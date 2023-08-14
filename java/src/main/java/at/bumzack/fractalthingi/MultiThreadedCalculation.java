package at.bumzack.fractalthingi;

import java.util.ArrayList;
import java.util.List;

import static at.bumzack.fractalthingi.Fractal.calcColor;

public class MultiThreadedCalculation implements Runnable {
    private final static GlobalInt yGlobal = new GlobalInt(0);
    private final int height;
    private final double reMin;
    private final double imgMin;
    private Thread t;
    private final String threadName;
    private final FractalRequest fractalRequest;
    private final double xDelta;
    private final double yDelta;
    private final int maxIterations;
    private final List<Color> colors;
    private final List<Color> pixels;


    public MultiThreadedCalculation(final String threadName,
                                    final FractalRequest fractalRequest,
                                    final int height,
                                    final double reMin,
                                    final double imgMin,
                                    final double xDelta,
                                    final double yDelta,
                                    final int maxIterations,
                                    final List<Color> colors,
                                    final List<Color> pixels) {
        this.threadName = threadName;
        this.fractalRequest = fractalRequest;
        this.height = height;
        this.reMin = reMin;
        this.imgMin = imgMin;
        this.maxIterations = maxIterations;
        this.xDelta = xDelta;
        this.yDelta = yDelta;
        this.colors = colors;
        this.pixels = pixels;
        yGlobal.reset();
    }

    public void run() {
        System.out.println("request " + fractalRequest);
        int y = -1;
        System.out.println("run. width " + fractalRequest.width + ", height " + height + ",  Pixels.len()" + pixels.size());
        final var pixelsThread = new ArrayList<Color>(fractalRequest.width);
        try {
            while (yGlobal.getValue() < height) {
                synchronized (yGlobal) {
                    if (yGlobal.getValue() < height) {
                        System.out.println("Thread " + Thread.currentThread().threadId() + "  if (yGlobal < height)  is true   yGlobal  " + yGlobal + ", y " + y);
                        y = yGlobal.getValue();
                        System.out.println("Thread " + Thread.currentThread().threadId() + "  if (yGlobal < height)  is true   yGlobal  " + yGlobal + ", y " + y);
                        yGlobal.increment();
                    } else {
                        System.out.println("Thread " + Thread.currentThread().threadId() + "  if (yGlobal < height)  is false   yGlobal  " + yGlobal + ", y " + y);
                    }
                }
                if (y < height) {
                    System.out.println("Thread " + Thread.currentThread().threadId() + "   if (y < height)   is true   yGlobal  " + yGlobal + ", y " + y);
                    pixelsThread.clear();
                    for (int x = 0; x < fractalRequest.width; x++) {
                        final var c = calcColor(x, y, reMin, imgMin, xDelta, yDelta, maxIterations, colors);
                        pixelsThread.add(c);
                    }

                    synchronized (pixels) {
                        for (int x = 0; x < fractalRequest.width; x++) {
                            int idx = y * fractalRequest.width + x;
                            pixels.set(idx, pixelsThread.get(x));
                        }
                    }
                }
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

    public Thread getT() {
        return t;
    }
}
