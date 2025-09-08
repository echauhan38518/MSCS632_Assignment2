public class JavaSimple {
    public static void main(String[] args) {
        {
            int[] numbers = new int[5]; // allocated on heap memory
            for (int i = 0; i < numbers.length; i++) {
                numbers[i] = i + 1;
            }
            System.out.println("Java: numbers[0] = " + numbers[0]);
            System.out.println("Java: numbers[0] = " + numbers[1]);
            System.out.println("Java: numbers[0] = " + numbers[2]);
            System.out.println("Java: numbers[0] = " + numbers[3]);
            System.out.println("Java: numbers[0] = " + numbers[4]);
        } // numbers becomes unreachable here

        System.out.println("Java: memory will be freed later by Garbage Collector");
    }
}