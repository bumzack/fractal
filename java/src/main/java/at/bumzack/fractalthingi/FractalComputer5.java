package at.bumzack.fractalthingi;

import java.util.concurrent.ForkJoinPool;
import java.util.concurrent.TimeUnit;

public class FractalComputer5 {

    private static final int WIDTH = 800;
    private static final int HEIGHT = 800;

    private final Fractal fractal;
    private final ForkJoinPool forkJoinPool;

    public FractalComputer5(Fractal fractal) {
        this.fractal = fractal;
        this.forkJoinPool = new ForkJoinPool(10);  // Set the thread limit to 10
    }

    public int[][] computeImage() {
        int[][] image = new int[WIDTH][HEIGHT];

        for (int y = 0; y < HEIGHT; y++) {
            int finalY = y;
            forkJoinPool.submit(() -> computeRow(finalY, image));
        }

        forkJoinPool.shutdown(); // always remember to shutdown the pool
        try {
            forkJoinPool.awaitTermination(Long.MAX_VALUE, TimeUnit.SECONDS); // wait for all tasks to finish
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
        return image;
    }

    private void computeRow(int y, int[][] image) {
        for (int x = 0; x < WIDTH; x++) {
            image[x][y] = fractal.compute(x, y);
        }
    }
}