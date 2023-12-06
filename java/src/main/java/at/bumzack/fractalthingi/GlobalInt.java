package at.bumzack.fractalthingi;

public class GlobalInt {
    private int value;

    public GlobalInt(final int value) {
        this.value = value;
    }

    public int getValue() {
        return value;
    }

    public void setValue(final int value) {
        this.value = value;
    }

    public void increment() {
        value++;
    }

    public void reset() {
        value = 0;
    }

    @Override
    public String toString() {
        return "GlobalInt{" +
                "value=" + value +
                '}';
    }


}
