package at.bumzack.fractalthingi;

import java.util.List;

public class FractalImage {
    int width;
    int height;
    List<Color> pixels;

    public FractalImage() {
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

    public List<Color> getPixels() {
        return pixels;
    }

    public void setPixels(final List<Color> pixels) {
        this.pixels = pixels;
    }
}
