package at.bumzack.fractalthingi;

import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

import java.time.Duration;

@SpringBootApplication
public class FractalThingiApplication {

    static final Logger logger = LogManager.getLogger(FractalThingiApplication.class);

    public static void main(String[] args) {
        SpringApplication.run(FractalThingiApplication.class, args);
    }


    private static Thread virtualThread(String name, Runnable runnable) {
        return Thread.ofVirtual()
                .name(name)
                .start(runnable);
    }

    static Thread bathTime() {
        return virtualThread(
                "Bath time",
                () -> {
                    log("I'm going to take a bath");
                    try {
                        sleep(Duration.ofMillis(500L));
                    } catch (InterruptedException e) {
                        throw new RuntimeException(e);
                    }
                    log("I'm done with the bath");
                });
    }

    static void log(String message) {
        logger.info("{} | " + message, Thread.currentThread());
    }

     private static void sleep(Duration duration) throws InterruptedException {
        Thread.sleep(duration);
    }


}
