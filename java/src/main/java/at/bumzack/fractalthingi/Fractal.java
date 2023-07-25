package at.bumzack.fractalthingi;

import com.google.gson.Gson;
import org.springframework.data.util.Pair;
import org.springframework.util.StopWatch;

import java.io.File;
import java.io.IOException;
import java.net.URISyntaxException;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

import static java.lang.Math.abs;
import static java.lang.Math.round;

public class Fractal {

    public static final Color BLACK = Color.from(0, 0, 0);

    public static Color calcColor(int x, int y, ComplexNumber upperLeft, double xDelta, double yDelta, int maxIterations, List<Color> colors) {
        int cntIterations = 0;
        final var c = new ComplexNumber();
        c.a = upperLeft.a + x * xDelta;
        c.b = upperLeft.b - y * yDelta;

        var z = new ComplexNumber();
        z.a = 0;
        z.b = 0;

        while (z.lengthSquared() < 4.0 && cntIterations < maxIterations) {
            z = z.pow2().add(c);
            cntIterations += 1;
        }

        if (cntIterations >= maxIterations) {
            return BLACK;
        } else {
            final var idx = cntIterations % colors.size();
            // System.out.println("COLOR    " + colors.get(idx) + "        idx " + idx);
            return colors.get(idx);
        }
    }

    public static Pair<FractalImage, Long> calc_image_singlethreaded(final FractalRequest request) throws IOException, URISyntaxException {
        final var start = new StopWatch();
        start.start();

//        final var r = new ClassPathResource("classpath:/resources/256-colors.json");
        final var r = Fractal.class.getClassLoader().getResource("256-colors.json");
        final var f = new File(r.toURI());

        final String json = new String(Files.readAllBytes(f.toPath()));

        final Gson gson = new Gson(); // Or use new GsonBuilder().create();
        final List<FileColor> fileColors = List.of(gson.fromJson(json, FileColor[].class));
        final List<Color> colors = new ArrayList<>();
        fileColors.forEach(c -> {
            final var co = Color.from(c.rgb.r, c.rgb.g, c.rgb.b);
            colors.add(co);
        });

        final double xDiff = abs(request.z1.a) + abs(request.z2.a);
        final double yDiff = abs(request.z2.b) + abs(request.z2.b);
        System.out.println("xDiff " + xDiff + ", yDiff:   " + yDiff);
        System.out.println("request.width " + request.width);
        System.out.println("xDiff * request.width / yDiff " + (xDiff * request.width / yDiff));

        final int height = (int) round(xDiff * request.width / yDiff);

        final double xDelta = xDiff / request.width;
        final double yDelta = yDiff / height;

        final var pixels = new ArrayList<Color>();

        System.out.println("width " + request.width + ", height:   " + height);
        for (int y = 0; y < height; y++) {
            for (int x = 0; x < request.width; x++) {
                final var c = calcColor(x, y, request.z1, xDelta, yDelta, request.maxIterations, colors);
                pixels.add(c);
            }
        }
        start.stop();
        final var duration = start.getTotalTimeMillis();

        System.out.println("single threaded took " + duration + " ms");

        final var image = new FractalImage();
        image.width = request.width;
        image.height = height;
        image.pixels = pixels;

        return Pair.of(image, duration);
    }


    public static Pair<FractalImage, Long> calc_image_multithreaded(final FractalRequest request) throws IOException, URISyntaxException, InterruptedException {
        int cores = Runtime.getRuntime().availableProcessors();
        System.out.println("number of cores " + cores);

        final var start = new StopWatch();
        start.start();

//        final var r = new ClassPathResource("classpath:/resources/256-colors.json");
        final var r = Fractal.class.getClassLoader().getResource("256-colors.json");
        final var f = new File(r.toURI());

        final String json = new String(Files.readAllBytes(f.toPath()));

        final Gson gson = new Gson(); // Or use new GsonBuilder().create();
        final List<FileColor> fileColors = List.of(gson.fromJson(json, FileColor[].class));
        final List<Color> colors = new ArrayList<>();
        fileColors.forEach(c -> {
            final var co = Color.from(c.rgb.r, c.rgb.g, c.rgb.b);
            colors.add(co);
        });

        final double xDiff = abs(request.z1.a) + abs(request.z2.a);
        final double yDiff = abs(request.z2.b) + abs(request.z2.b);
        System.out.println("xDiff " + xDiff + ", yDiff:   " + yDiff);
        System.out.println("request.width " + request.width);
        System.out.println("xDiff * request.width / yDiff " + (xDiff * request.width / yDiff));

        final int height = (int) round(xDiff * request.width / yDiff);
        System.out.println("height " + height);

        final var threads = new ArrayList<MultiThreadedCalculation>();
        final double xDelta = xDiff / request.width;
        final double yDelta = yDiff / height;

        final var pixels = new ArrayList<Color>(Collections.nCopies(request.width * height, BLACK));

        System.out.println("calc_image_multithreaded            width " + request.width + "       height " + height + "     pixels.size( " + pixels.size());


        for (int i = 0; i < cores; i++) {
            final MultiThreadedCalculation t = new MultiThreadedCalculation("Thread-" + i, request, height, request.z1, xDelta, yDelta, request.maxIterations, colors, pixels);
            threads.add(t);
        }

        for (int i = 0; i < cores; i++) {
            threads.get(i).start();
        }

        for (int i = 0; i < cores; i++) {
            final var t = threads.get(i);
            t.getT().join();
        }

        start.stop();
        final var duration = start.getTotalTimeMillis();
        System.out.println("multi threaded took " + duration + " ms");

        final var image = new FractalImage();
        image.width = request.width;
        image.height = height;
        image.pixels = pixels;

        return Pair.of(image, duration);
    }
}
