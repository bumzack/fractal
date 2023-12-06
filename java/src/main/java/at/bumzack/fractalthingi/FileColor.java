package at.bumzack.fractalthingi;


public class FileColor {
    int color_id;
    String hex_string;
    Color rgb;
    Hsl hsl;
    String name;


    public FileColor() {
    }

    public int getColor_id() {
        return color_id;
    }

    public void setColor_id(final int color_id) {
        this.color_id = color_id;
    }

    public String getHex_string() {
        return hex_string;
    }

    public void setHex_string(final String hex_string) {
        this.hex_string = hex_string;
    }

    public Color getRgb() {
        return rgb;
    }

    public void setRgb(final Color rgb) {
        this.rgb = rgb;
    }

    public Hsl getHsl() {
        return hsl;
    }

    public void setHsl(final Hsl hsl) {
        this.hsl = hsl;
    }

    public String getName() {
        return name;
    }

    public void setName(final String name) {
        this.name = name;
    }
}
