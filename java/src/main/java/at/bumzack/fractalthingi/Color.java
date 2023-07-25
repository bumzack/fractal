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
}