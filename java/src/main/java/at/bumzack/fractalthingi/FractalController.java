package at.bumzack.fractalthingi;

import com.google.gson.FieldNamingPolicy;
import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import org.springframework.data.util.Pair;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.ResponseBody;

import java.io.IOException;
import java.net.URISyntaxException;

import static at.bumzack.fractalthingi.Fractal.calc_image_multithreaded;
import static at.bumzack.fractalthingi.Fractal.calc_image_singlethreaded;


@Controller
public class FractalController {

    @PostMapping("/api/singlethreaded")
    @ResponseBody
    public FractalResponse singleThreaded(@RequestBody final FractalRequest request) throws IOException, URISyntaxException {
        final Gson gson = new GsonBuilder()
                .setFieldNamingPolicy(FieldNamingPolicy.LOWER_CASE_WITH_UNDERSCORES)
                .create();
        final String json = gson.toJson(request);

        System.out.println("request in controller " + json);
        final Pair<FractalImage, Long> img = calc_image_singlethreaded(request);

        final var res = new FractalResponse();
        res.fractal = img.getFirst();
        res.durationCalculation = String.format("java singlethreaded took %s ms", img.getSecond());
        return res;
    }

    @PostMapping("/api/multithreaded")
    @ResponseBody
    public FractalResponse multiThreaded(@RequestBody final FractalRequest request) throws IOException, URISyntaxException, InterruptedException {
        final Gson gson = new GsonBuilder()
                .setFieldNamingPolicy(FieldNamingPolicy.LOWER_CASE_WITH_UNDERSCORES)
                .create();
        final String json = gson.toJson(request);

        System.out.println("request in controller " + json);
        final Pair<FractalImage, Long> img = calc_image_multithreaded(request);

        final var res = new FractalResponse();
        res.fractal = img.getFirst();
        res.durationCalculation = String.format("java multiThreaded took %s ms", img.getSecond());
        return res;
    }
}
