package at.bumzack.fractalthingi;

  import org.springframework.data.util.Pair;
  import org.springframework.util.StopWatch;

import java.util.ArrayList;
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
            //  info!("BLACK       z = {}, c = {} ,  cnt_iterations {}, max_iterations {}", &z, &c, cnt_iterations, max_iterations);
            return BLACK;
        } else {
            final var idx = cntIterations % colors.size();
            return colors.get(idx);
        }

    }

    public static Pair<FractalImage, Long> calc_image_singlethreaded(final FractalRequest request) {
        final var start = new StopWatch();

        final double xDiff = abs(request.z1.a) + abs(request.z1.b);
        final double yDiff = abs(request.z2.a) + abs(request.z2.b);

        final int height = (int) round(xDiff * request.width / yDiff);

        final double xDelta = xDiff / request.width;
        final double yDelta = yDiff / height;

        final var pixels = new ArrayList<Color>();
        final var colors = new ArrayList<Color>();

        for (int y = 0; y < height; y++) {
            for (int x = 0; x < request.width; x++) {
                final var c = calcColor(x, y, request.z1, xDelta, yDelta, request.max_iterations, colors);
                pixels.add(c);
            }
        }

        final var duration = start.getTotalTimeMillis();

        System.out.println("single threaded took " + duration  + " ms" );

        final var image = new FractalImage();
        return Pair.of(image, duration);
    }

}
