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

import static at.bumzack.fractalthingi.Fractal.BLACK;

public class FractalVirtualThreads {

    public static Pair<FractalImage, Long> calc_image_multithreaded_virtual_threads(final FractalRequest request) throws IOException, URISyntaxException, InterruptedException {
        int cores = Runtime.getRuntime().availableProcessors();
        System.out.println("number of cores " + cores);

        final var start = new StopWatch();
        start.start();

//        final var r = new ClassPathResource("classpath:/resources/256-colors.json");
        final var r = FractalVirtualThreads.class.getClassLoader().getResource("256-colors.json");
        final var f = new File(r.toURI());

        final String json = new String(Files.readAllBytes(f.toPath()));

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

        final var threads = new ArrayList<Thread>();

        final var pixels = new ArrayList<>(Collections.nCopies(request.getWidth() * request.getHeight(), BLACK));

        System.out.println("calc_image_multithreaded.  width " + request.width + ", height " + request.height + ", pixels.size " + pixels.size());

        for (int i = 0; i < cores; i++) {
            final Thread t = Thread.ofVirtual().start(new MultiThreadedCalculation("Thread-" + i, request, request.height, reMin, imgMin, xDelta, yDelta, request.maxIterations, colors, pixels));
            threads.add(t);
        }

//        for (int i = 0; i < cores; i++) {
//            threads.get(i).start();
//        }

        for (int i = 0; i < cores; i++) {
            final var t = threads.get(i);
            t.join();
        }

        start.stop();
        final var duration = start.getTotalTimeMillis();
        System.out.println("multi threaded virtual threads took " + duration + " ms");

        final var image = new FractalImage();
        image.width = request.width;
        image.height = request.height;
        image.pixels = pixels;

        return Pair.of(image, duration);
    }
}
