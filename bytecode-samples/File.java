import java.util.Arrays;

public class File {

  public static class Data {
    private final int value;
    private int count;

    public Data(int value) {
      this.value = value;
      this.count = 0;
    }
  }

  public static void main(String[] args) {
    Data d = new Data(12);

    final int[] t = new int[] {
      123,
      456,
      67890
    };
    System.out.println(Arrays.toString(t));

    final long[] l = new long[] {
      1L,
      234L,
      567890L
    };
    System.out.println(Arrays.toString(l));
  }

}
