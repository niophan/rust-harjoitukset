fn main() {
	let mut a: u128 = 0;
	let mut b: u128 = 1;
	let mut sum: u128;
	
	for i in 0..100 {
		if i <= 1 {
			println!("{}: {}", i, i);
		} else {
			sum = a + b;
			a = b;
			b = sum;
			println!("{} : {}", i, b);
		}
	}
}


