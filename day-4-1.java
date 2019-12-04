public class Day4 {
	public static void main(String[] args) {
		int min = 178416;
		int max = 676461;
		int counter = 0;
		boolean cond1;
		boolean cond2;

		for (int i = min; i <= max; i++) {
			cond1 = false;
			cond2 = true;
			String curr = Integer.toString(i);
			for (int j = 0; j < 5; j++) {
				String a = String.valueOf(curr.charAt(j));
				String b = String.valueOf(curr.charAt(j+1));
				if (Integer.parseInt(a) == Integer.parseInt(b)) {
					cond1 = true;
				}
				if (Integer.parseInt(a) > Integer.parseInt(b)) {
					cond2 = false;
				}
			}
			if (cond1 && cond2) {
				counter++;
			}
		}
		System.out.println(counter);
	}

}
