package at.bumzack.fractalthingi;

import com.google.gson.FieldNamingPolicy;
import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
public class FractalThingiApplication {
    public static void main(String[] args) {
        final var z1 = new ComplexNumber();
        final var z2 = new ComplexNumber();
        final var req = new FractalRequest();

        req.z1 = z1;
        req.z2 = z2;
        req.width = 200;
        req.colors = 256;
        req.maxIterations = 1000;
        req.xTiles = 5;
        req.yTiles = 19;

        final Gson gson = new GsonBuilder()
                .setFieldNamingPolicy(FieldNamingPolicy.LOWER_CASE_WITH_UNDERSCORES)
                .create();
        String json = gson.toJson(req);

        System.out.println("request: " + json);

        SpringApplication.run(FractalThingiApplication.class, args);
    }

}
