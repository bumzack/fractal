package at.bumzack.fractalthingi;

import java.io.FileWriter;
import java.io.IOException;
import java.util.List;

public class FractalComputer4 {

    private static final int MAX_ITER = 1000;
    private static final double ZOOM = 150;
    private static final int WIDTH = 1920;
    private static final int HEIGHT = 1080;

    public static void main(String[] args) throws InterruptedException {
        List<Thread> threads = new java.util.ArrayList<>();
        int[][] image = new int[WIDTH][HEIGHT];

        for (int y = 0; y < HEIGHT; y++) {
            int finalY = y;
            Thread thread = Thread.startVirtualThread(() -> {
                for (int x = 0; x < WIDTH; x++) {
                    double zx, zy, cX, cY;
                    zx = zy = 0;
                    cX = (x - WIDTH / 2) / ZOOM;
                    cY = (finalY - HEIGHT / 2) / ZOOM;
                    int iter = MAX_ITER;
                    while (zx * zx + zy * zy < 4 && iter > 0) {
                        double tmp = zx * zx - zy * zy + cX;
                        zy = 2.0 * zx * zy + cY;
                        zx = tmp;
                        iter--;
                    }
                    image[x][finalY] = iter | (iter << 8);
                }
            });
            threads.add(thread);
        }

        for (Thread thread : threads) {
            thread.join();
        }

        try (FileWriter writer = new FileWriter("image.ppm")) {
            writePPM(image, writer);
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    private static void writePPM(int[][] image, FileWriter writer) throws IOException {
        writer.write("P3\n" + WIDTH + " " + HEIGHT + "\n255\n");
        for (int y = 0; y < HEIGHT; y++) {
            for (int x = 0; x < WIDTH; x++) {
                Color color = new Color(image[x][y]);
                writer.write(color.getBlue() + " " + color.getGreen() + " " + color.getRed() + " ");
            }
            writer.write("\n");
        }
    }

    private static void writePPM2(int[][] image, FileWriter writer) throws IOException {
        writer.write("P3\n" + WIDTH + " " + HEIGHT + "\n255\n");
        for (int y = 0; y < HEIGHT; y++) {
            int count = 0;
            for (int x = 0; x < WIDTH; x++) {
                Color color = new Color(image[x][y]);
                String pixel = color.getBlue() + " " + color.getGreen() + " " + color.getRed() + " ";
                if (count + pixel.length() > 70) {
                    writer.write("\n");
                    count = 0;
                }
                writer.write(pixel);
                count += pixel.length();
            }
            writer.write("\n");
        }
    }

}