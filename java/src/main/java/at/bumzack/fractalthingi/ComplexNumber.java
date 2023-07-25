package at.bumzack.fractalthingi;

public class ComplexNumber {
    double a;
    double b;

    public double lengthSquared() {
        return a * a + b * b;
    }

    public ComplexNumber pow2() {
        var result = new ComplexNumber();
        result.a = a * a - b * b;
        result.b = 2 * a * b;
        return result;
    }

    public ComplexNumber add(final ComplexNumber z) {
        var result = new ComplexNumber();
        result.a = a + z.a;
        result.b = b + z.b;
        return result;
    }

    public ComplexNumber() {
    }

    public double getA() {
        return a;
    }

    public void setA(final double a) {
        this.a = a;
    }

    public double getB() {
        return b;
    }

    public void setB(final double b) {
        this.b = b;
    }
}
