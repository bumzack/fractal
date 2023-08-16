package at.bumzack.fractalthingi;


import com.fasterxml.jackson.annotation.JsonProperty;

public class FractalRequest {
    ComplexNumber center;
    int width;
    int height;

    @JsonProperty("complex_width")
    double complexWidth;
    @JsonProperty("max_iterations")
    int maxIterations;

    int colors;

    @JsonProperty("x_tiles")
    int xTiles;

    @JsonProperty("y_tiles")
    int yTiles;


    double zoom;
    String name;


    public FractalRequest() {
    }

    public ComplexNumber getCenter() {
        return center;
    }

    public void setCenter(final ComplexNumber center) {
        this.center = center;
    }

    public int getWidth() {
        return width;
    }

    public void setWidth(final int width) {
        this.width = width;
    }

    public int getHeight() {
        return height;
    }

    public void setHeight(final int height) {
        this.height = height;
    }

    public double getComplexWidth() {
        return complexWidth;
    }

    public void setComplexWidth(final double complexWidth) {
        this.complexWidth = complexWidth;
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

    public double getZoom() {
        return zoom;
    }

    public void setZoom(final double zoom) {
        this.zoom = zoom;
    }

    public String getName() {
        return name;
    }

    public void setName(final String name) {
        this.name = name;
    }

    @Override
    public String toString() {
        return "FractalRequest{" +
                "center=" + center +
                ", width=" + width +
                ", height=" + height +
                ", complexWidth=" + complexWidth +
                ", maxIterations=" + maxIterations +
                ", colors=" + colors +
                ", xTiles=" + xTiles +
                ", yTiles=" + yTiles +
                ", zoom=" + zoom +
                ", name='" + name + '\'' +
                '}';
    }
}
