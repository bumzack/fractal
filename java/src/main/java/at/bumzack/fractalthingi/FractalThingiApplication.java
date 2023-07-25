package at.bumzack.fractalthingi;

import com.google.gson.Gson;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
public class FractalThingiApplication {
    public static void main(String[] args) {

        final var z1 = new ComplexNumber();
        final var z2 = new ComplexNumber();
        final var req = new FractalRequest();
        req.z1  = z1;
        req.z2  = z2;
        req.width = 200;
        req.colors = 256;
        req.max_iterations = 1000;
        req.x_tiles=5;
        req .y_tiles=19;

        final Gson gson = new Gson();
        String json = gson.toJson(req);



        System.out.println("request: " +json);

        SpringApplication.run(FractalThingiApplication.class, args);
    }

}
