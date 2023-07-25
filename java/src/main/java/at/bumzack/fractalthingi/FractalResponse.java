package at.bumzack.fractalthingi;

public class FractalResponse {
    String duration_calculation;
    FractalImage fractal;

    public FractalResponse() {
    }

    public String getDuration_calculation() {
        return duration_calculation;
    }

    public void setDuration_calculation(final String duration_calculation) {
        this.duration_calculation = duration_calculation;
    }

    public FractalImage getFractal() {
        return fractal;
    }

    public void setFractal(final FractalImage fractal) {
        this.fractal = fractal;
    }
}



