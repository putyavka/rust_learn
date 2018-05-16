extern crate closures;

use closures::Cacher;
use closures::Counter;

fn main() {
	let mut cacher = Cacher::new(|x| { x * x });
	for value in vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5] {
		println!("{} -> {}", value, cacher.value(value));
	}
	println!("Counter(3, 8):");
	for value in Counter::new(3, 8) {
		println!("{}", value);
	}
}
