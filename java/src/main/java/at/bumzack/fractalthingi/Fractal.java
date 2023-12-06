package at.bumzack.fractalthingi;

import com.google.gson.Gson;
import org.springframework.data.util.Pair;
import org.springframework.util.StopWatch;

import java.io.*;
import java.net.URISyntaxException;
import java.net.URL;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Fractal {

    public static final Color BLACK = Color.from(0, 0, 0);

    public static Color calcColor(final int x, final int y, final double reMin, final double imgMin, final double xDelta, final double yDelta, final int maxIterations, final List<Color> colors) {
        int cntIterations = 0;
        final var c = new ComplexNumber();
        c.a = reMin + x * xDelta;
        c.b = imgMin + y * yDelta;

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

        final var is = Fractal.class.getClassLoader().getResourceAsStream("256-colors.json");
        final StringBuilder textBuilder = new StringBuilder();
        try (final Reader reader = new BufferedReader(new InputStreamReader(is, StandardCharsets.UTF_8))) {
            int c = 0;
            while ((c = reader.read()) != -1) {
                textBuilder.append((char) c);
            }
        }
        final var json = textBuilder.toString();

        final Gson gson = new Gson(); // Or use new GsonBuilder().create();
        final List<FileColor> fileColors = List.of(gson.fromJson(json, FileColor[].class));
        final List<Color> colors = new ArrayList<>();
        fileColors.forEach(c -> {
            final var co = Color.from(c.rgb.r, c.rgb.g, c.rgb.b);
            colors.add(co);
        });

        final var complexWidth = request.complexWidth / request.zoom;
        final double ratio = (double) request.width / request.height;
        final double complexHeight = complexWidth / ratio;

        final double reMin = request.getCenter().getA() - complexWidth / 2.0;
        final double reMax = request.getCenter().getA() + complexWidth / 2.0;

        final double imgMin = request.getCenter().getB() - complexHeight / 2.0;
        final double imgMax = request.getCenter().getB() + complexHeight / 2.0;

        final double xDelta = (reMax - reMin) / request.width;
        final double yDelta = (imgMax - imgMin) / request.height;

        final var pixels = new ArrayList<Color>();

        for (int y = 0; y < request.getHeight(); y++) {
            for (int x = 0; x < request.getWidth(); x++) {
                final var c = calcColor(x, y, reMin, imgMin, xDelta, yDelta, request.maxIterations, colors);
                pixels.add(c);
            }
        }
        start.stop();
        final var duration = start.getTotalTimeMillis();

        System.out.println("single threaded took " + duration + " ms");

        final var image = new FractalImage();
        image.width = request.width;
        image.height = request.height;
        image.pixels = pixels;

        return Pair.of(image, duration);
    }


    public static Pair<FractalImage, Long> calc_image_multithreaded(final FractalRequest request) throws IOException, URISyntaxException, InterruptedException {
        int cores = Runtime.getRuntime().availableProcessors();
        System.out.println("number of cores " + cores);

        final var start = new StopWatch();
        start.start();

//        final var r = new ClassPathResource("classpath:/resources/256-colors.json");
        final var is = Fractal.class.getClassLoader().getResourceAsStream("256-colors.json");
        final StringBuilder textBuilder = new StringBuilder();
        try (final Reader reader = new BufferedReader(new InputStreamReader(is, StandardCharsets.UTF_8))) {
            int c = 0;
            while ((c = reader.read()) != -1) {
                textBuilder.append((char) c);
            }
        }
        final var json = textBuilder.toString();

        final Gson gson = new Gson(); // Or use new GsonBuilder().create();
        final List<FileColor> fileColors = List.of(gson.fromJson(json, FileColor[].class));
        final List<Color> colors = new ArrayList<>();
        fileColors.forEach(c -> {
            final var co = Color.from(c.rgb.r, c.rgb.g, c.rgb.b);
            colors.add(co);
        });

        final var complexWidth = request.complexWidth / request.zoom;
        final double ratio = (double) request.width / request.height;
        final double complexHeight = complexWidth / ratio;

        final double reMin = request.getCenter().getA() - complexWidth / 2.0;
        final double reMax = request.getCenter().getA() + complexWidth / 2.0;

        final double imgMin = request.getCenter().getB() - complexHeight / 2.0;
        final double imgMax = request.getCenter().getB() + complexHeight / 2.0;

        final double xDelta = (reMax - reMin) / request.width;
        final double yDelta = (imgMax - imgMin) / request.height;

        final var threads = new ArrayList<MultiThreadedCalculation>();
        final var pixels = new ArrayList<>(Collections.nCopies(request.getWidth() * request.getHeight(), BLACK));

        System.out.println("calc_image_multithreaded.  width " + request.width + ", height " + request.height + ", pixels.size " + pixels.size());

        for (int i = 0; i < cores; i++) {
            final MultiThreadedCalculation t = new MultiThreadedCalculation("Thread-" + i, request, request.height, reMin, imgMin, xDelta, yDelta, request.maxIterations, colors, pixels);
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
        image.height = request.height;
        image.pixels = pixels;

        return Pair.of(image, duration);
    }
}
