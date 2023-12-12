package at.bumzack.fractalthingi;

import javax.swing.*;
import java.awt.*;
import java.awt.image.BufferedImage;
import java.util.concurrent.Executor;

public final class FractalComputer6 extends JFrame {

    private static final int MAX_ITER = 1000;
    private static final double ZOOM = 150;
    private static final int WIDTH = 800;
    private static final int HEIGHT = 600;
    private BufferedImage I;
    private JPanel panel;

    public FractalComputer6() {
        super("Mandelbrot Set");
        setBounds(100, 100, WIDTH, HEIGHT);
        setResizable(false);
        setDefaultCloseOperation(EXIT_ON_CLOSE);

        I = new BufferedImage(getWidth(), getHeight(), BufferedImage.TYPE_INT_RGB);
        I.getGraphics().drawImage(I, 0, 0, this);

        panel = new JPanel() {
            @Override
            public void paintComponent(Graphics g) {
                super.paintComponent(g);
                g.drawImage(I, 0, 0, this);
            }
        };

        add(panel);
        setVisible(true);

        Executor executor = (Runnable command) -> Thread.ofVirtual().start(command);

        for (int y = 0; y < getHeight(); y++) {
            final int finalY = y;
            executor.execute(() -> {
                for (int x = 0; x < getWidth(); x++) {
                    double zx, zy, cX, cY, tmp;
                    zx = zy = 0;
                    cX = (x - 400) / ZOOM;
                    cY = (finalY - 300) / ZOOM;
                    int iter = MAX_ITER;
                    while (zx * zx + zy * zy < 4 && iter > 0) {
                        tmp = zx * zx - zy * zy + cX;
                        zy = 2.0 * zx * zy + cY;
                        zx = tmp;
                        iter--;
                    }
                    I.setRGB(x, finalY, iter | (iter << 8));
                }
                panel.repaint();
            });
        }
    }

    public static void main(String[] args) {
        new FractalComputer6();
    }
}