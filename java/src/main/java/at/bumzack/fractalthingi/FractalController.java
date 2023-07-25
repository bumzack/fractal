package at.bumzack.fractalthingi;

import com.google.gson.Gson;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.ResponseBody;


@Controller
public class FractalController {

    @PostMapping("/api/singlethreaded")
    @ResponseBody
    public FractalResponse singleThreaded(@RequestBody final FractalRequest request) {
        final Gson gson = new Gson();
        String json = gson.toJson(request);

        System.out.println("request in cintroller " + json);
        final var res = new FractalResponse();
        return res;
    }

}
