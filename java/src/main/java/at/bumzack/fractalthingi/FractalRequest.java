package at.bumzack.fractalthingi;


import com.fasterxml.jackson.annotation.JsonProperty;
import com.google.gson.annotations.SerializedName;

public class FractalRequest {
    ComplexNumber z1;
    ComplexNumber z2;
    int width;

    @JsonProperty("max_iterations")
    int maxIterations;

    int colors;

    @JsonProperty("x_tiles")
    int xTiles;

    @JsonProperty("y_tiles")
    int yTiles;

    public FractalRequest() {
    }

    public ComplexNumber getZ1() {
        return z1;
    }

    public void setZ1(final ComplexNumber z1) {
        this.z1 = z1;
    }

    public ComplexNumber getZ2() {
        return z2;
    }

    public void setZ2(final ComplexNumber z2) {
        this.z2 = z2;
    }

    public int getWidth() {
        return width;
    }

    public void setWidth(final int width) {
        this.width = width;
    }

    public int getMaxIterations() {
        return maxIterations;
    }

    public void setMaxIterations(final int maxIterations) {
        this.maxIterations = maxIterations;
    }

    public int getColors() {
        return colors;
    }

    public void setColors(final int colors) {
        this.colors = colors;
    }

    public int getxTiles() {
        return xTiles;
    }

    public void setxTiles(final int xTiles) {
        this.xTiles = xTiles;
    }

    public int getyTiles() {
        return yTiles;
    }

    public void setyTiles(final int yTiles) {
        this.yTiles = yTiles;
    }
}
