package at.bumzack.fractalthingi;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.*;

public class FractalComputer {

    private static final int WIDTH = 800;
    private static final int HEIGHT = 800;
    private static final int MAX_THREADS = 4;

    private final Fractal fractal;
    private final ExecutorService executorService;

    public FractalComputer(Fractal fractal) {
        this.fractal = fractal;
        executorService = Executors.newFixedThreadPool(MAX_THREADS);
    }

    public int[][] computeImage() throws InterruptedException, ExecutionException {
        int[][] image = new int[WIDTH][HEIGHT];

        List<Future<Integer[]>> futures = new ArrayList<>();
        for (int x = 0; x < WIDTH; x++) {
            for(int y = 0; y < HEIGHT; y++) {
                int finalX = x;
                int finalY = y;
                futures.add(executorService.submit(() -> fractal.compute(finalX, finalY)));
            }
        }

        for (Future<Integer[]> future : futures) {
            Integer[] result = future.get();
            // result[0] and result[1] are the x and y coordinates. result[2] is the computed color.
            image[result[0]][result[1]] = result[2];
        }

        executorService.shutdown(); // Always remember to shutdown the executor when you are done.
        return image;
    }

}