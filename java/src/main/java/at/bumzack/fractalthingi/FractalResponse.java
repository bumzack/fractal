package at.bumzack.fractalthingi;

import com.fasterxml.jackson.annotation.JsonProperty;

public class FractalResponse {
    @JsonProperty("duration_calculation")
    String durationCalculation;
    FractalImage fractal;
    @JsonProperty("duration_ms")
    Long durationMs;

    public FractalResponse() {
    }

    public String getDurationCalculation() {
        return durationCalculation;
    }

    public void setDurationCalculation(final String durationCalculation) {
        this.durationCalculation = durationCalculation;
    }

    public FractalImage getFractal() {
        return fractal;
    }

    public void setFractal(final FractalImage fractal) {
        this.fractal = fractal;
    }
}



