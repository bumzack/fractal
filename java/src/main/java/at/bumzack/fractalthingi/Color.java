package at.bumzack.fractalthingi;

public class Color {
    int r;
    int g;
    int b;

    public static Color from(int r, int g, int b) {
        final var c = new Color();
        c.r = r;
        c.g = g;
        c.b = b;
        return c;
    }

    public int getR() {
        return r;
    }

    public void setR(final int r) {
        this.r = r;
    }

    public int getG() {
        return g;
    }

    public void setG(final int g) {
        this.g = g;
    }

    public int getB() {
        return b;
    }

    public void setB(final int b) {
        this.b = b;
    }

    public Color() {
    }

    @Override
    public String toString() {
        return "Color{" +
                "r=" + r +
                ", g=" + g +
                ", b=" + b +
                '}';
    }
}