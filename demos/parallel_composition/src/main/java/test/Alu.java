package test;

public class Alu {
    public double singleAlu(int times) {
        int a = (int) (Math.random() * 91 + 10);
        int b = (int) (Math.random() * 91 + 10);
        double temp = 0;
        for (int i = 0; i < 20; i++) {
            for (long j = 0; j < times; j++) {
                switch ((int)(j % 4)) {
                    case 0:
                        temp = a + b;
                        break;
                    case 1:
                        temp = a - b;
                        break;
                    case 2:
                        temp = (long) a * b;
                        break;
                    case 3:
                        temp = a / b;
                        break;
                }
            }
        }
        return temp;
    }
}
