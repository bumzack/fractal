package at.bumzack.fractalthingi;

import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
public class FractalThingiApplication {

    static final Logger logger = LogManager.getLogger(FractalThingiApplication.class);

    public static void main(String[] args) {
        SpringApplication.run(FractalThingiApplication.class, args);
    }
}
