package at.bumzack.fractalthingi;


public class FractalRequest {
    ComplexNumber z1;
    ComplexNumber z2;
    int width;
    int max_iterations;
    int colors;
    int x_tiles;
    int y_tiles;

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

    public int getMax_iterations() {
        return max_iterations;
    }

    public void setMax_iterations(final int max_iterations) {
        this.max_iterations = max_iterations;
    }

    public int getColors() {
        return colors;
    }

    public void setColors(final int colors) {
        this.colors = colors;
    }

    public int getX_tiles() {
        return x_tiles;
    }

    public void setX_tiles(final int x_tiles) {
        this.x_tiles = x_tiles;
    }

    public int getY_tiles() {
        return y_tiles;
    }

    public void setY_tiles(final int y_tiles) {
        this.y_tiles = y_tiles;
    }
}
