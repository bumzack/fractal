package at.bumzack.fractalthingi;

import java.util.ArrayList;
import java.util.List;

import static at.bumzack.fractalthingi.Fractal.calcColor;

public class MultiThreadedCalculation implements Runnable {
    private Thread t;
    private String threadName;
    private FractalRequest fractalRequest;
    private static volatile int yGlobal = 0;
    private final int height;
    private ComplexNumber upperLeft;
    private double xDelta;
    private double yDelta;
    private int maxIterations;
    private List<Color> colors;
    private List<Color> pixels;

    public Thread getT() {
        return t;
    }

    public void setT(final Thread t) {
        this.t = t;
    }

    public String getThreadName() {
        return threadName;
    }

    public void setThreadName(final String threadName) {
        this.threadName = threadName;
    }

    public FractalRequest getFractalRequest() {
        return fractalRequest;
    }

    public void setFractalRequest(final FractalRequest fractalRequest) {
        this.fractalRequest = fractalRequest;
    }

    public static int getyGlobal() {
        return yGlobal;
    }

    public static void setyGlobal(final int yGlobal) {
        MultiThreadedCalculation.yGlobal = yGlobal;
    }

    public int getHeight() {
        return height;
    }

    public ComplexNumber getUpperLeft() {
        return upperLeft;
    }

    public void setUpperLeft(final ComplexNumber upperLeft) {
        this.upperLeft = upperLeft;
    }

    public double getxDelta() {
        return xDelta;
    }

    public void setxDelta(final double xDelta) {
        this.xDelta = xDelta;
    }

    public double getyDelta() {
        return yDelta;
    }

    public void setyDelta(final double yDelta) {
        this.yDelta = yDelta;
    }

    public int getMaxIterations() {
        return maxIterations;
    }

    public void setMaxIterations(final int maxIterations) {
        this.maxIterations = maxIterations;
    }

    public List<Color> getColors() {
        return colors;
    }

    public void setColors(final List<Color> colors) {
        this.colors = colors;
    }

    public List<Color> getPixels() {
        return pixels;
    }

    public void setPixels(final List<Color> pixels) {
        this.pixels = pixels;
    }

    public MultiThreadedCalculation(final String threadName,
                                    final FractalRequest fractalRequest,
                                    final int height,
                                    final ComplexNumber upperLeft,
                                    final double xDelta,
                                    final double yDelta,
                                    final int maxIterations,
                                    final List<Color> colors,
                                    final List<Color> pixels) {
        this.threadName = threadName;
        this.fractalRequest = fractalRequest;
        this.height = height;
        this.maxIterations = maxIterations;
        this.upperLeft = upperLeft;
        this.xDelta = xDelta;
        this.yDelta = yDelta;
        this.colors = colors;
        this.pixels = pixels;
    }

    public void run() {
        int y = -1;
        System.out.println("run    width " + fractalRequest.width + "     height " + height + "Pixels.len()" + pixels.size());
        final var pixelsThread = new ArrayList<Color>(fractalRequest.width);
        try {
            while (yGlobal < height) {
//               System.out.println("Thread " + Thread.currentThread().threadId() + " while   yGlobal  " + yGlobal + ", y " + y);
                if (yGlobal < height) {
//                    System.out.println("Thread " + Thread.currentThread().threadId() + " if   yGlobal  " + yGlobal + ", y " + y);
                    synchronized (this) {
//                        System.out.println("Thread " + Thread.currentThread().threadId() + "  synchronized    yGlobal  " + yGlobal + ", y " + y);
                        if (yGlobal < height) {
                            y = yGlobal;
                            yGlobal++;
                        }
                    }
                    if (yGlobal < height) {
                        pixelsThread.clear();
                        for (int x = 0; x < fractalRequest.width; x++) {
                            final var c = calcColor(x, y, upperLeft, xDelta, yDelta, maxIterations, colors);
                            pixelsThread.add(c);
                        }

                        synchronized (this) {
                            for (int x = 0; x < fractalRequest.width; x++) {
                                int idx = y * getFractalRequest().width + x;
                                pixels.set(idx, pixelsThread.get(x));
                            }
                        }
                    }
                }
//                System.out.println("Thread " + Thread.currentThread().threadId() + " is done sleeping. y = " + y);
            }
        } catch (final Exception e) {
            // Throwing an exception
            System.out.println("Exception is caught");
            System.err.println("error " + e.getMessage());
            System.err.println(e);
        }
    }

    public void start() {

        System.out.println("Starting " + threadName);
        if (t == null) {
            t = new Thread(this, threadName);
            t.start();
        }
    }
}

