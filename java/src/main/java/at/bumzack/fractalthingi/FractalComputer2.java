package at.bumzack.fractalthingi;

import java.awt.Graphics2D;
import java.awt.image.BufferedImage;
import java.util.concurrent.ForkJoinPool;
import java.util.concurrent.RecursiveAction;

public class FractalComputer2 extends RecursiveAction {

    private static final int MAX_ITER = 10000;
    private static final double ZOOM = 150;
    private double zx, zy, cX, cY;
    private int x, y, w, h;
    private BufferedImage image;

    public FractalComputer2(int x, int y, int w, int h, double cX, double cY, BufferedImage image) {
        this.x = x;
        this.y = y;
        this.w = w;
        this.h = h;
        this.cX = cX;
        this.cY = cY;
        this.image = image;
    }

    @Override
    protected void compute() {
        if ((w - x) * (h - y) < 500_000) {
            computeDirectly();
            return;
        }
        int midX = (x + w) / 2;
        int midY = (y + h) / 2;
        invokeAll(new FractalComputer2(x, y, midX, midY, cX, cY, image),
                new FractalComputer2(x, midY, midX, h, cX, cY, image),
                new FractalComputer2(midX, y, w, midY, cX, cY, image),
                new FractalComputer2(midX, midY, w, h, cX, cY, image));
    }

    private void computeDirectly() {
        for (x = 0; x < w; x++) {
            for (y = 0; y < h; y++) {
                zx = zy = 0;
                cX = (x - 400) / ZOOM;
                cY = (y - 300) / ZOOM;
                int iter = MAX_ITER;
                while (zx * zx + zy * zy < 4 && iter > 0) {
                    double tmp = zx * zx - zy * zy + cX;
                    zy = 2.0 * zx * zy + cY;
                    zx = tmp;
                    iter--;
                }
                image.setRGB(x, y, iter | (iter << 8));
            }
        }
    }

    public static void main(String[] args) {
        BufferedImage image = new BufferedImage(800, 600, BufferedImage.TYPE_INT_RGB);
        ForkJoinPool pool = new ForkJoinPool();
        pool.invoke(new FractalComputer2(0, 0, 800, 600, 0, 0, image));
        Graphics2D g = (Graphics2D) image.getGraphics();
        g.dispose();
    }
}