fn main() {
	let mut a = 0;
	let mut b = 1;
	let mut sum;
	
	for i in 0..10 {
		if i <= 1 {
			println!("{}", i);
		} else {
			sum = a + b;
			a = b;
			b = sum;
			println!("{}", b);
		}
	}
}


