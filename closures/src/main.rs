use std::collections::HashMap;

struct Cacher<T> where T : Fn(u32) -> u32 {
	calculator: T,
	values: HashMap<u32, u32>,
	value: Option<u32>,
}

impl<T> Cacher<T> where T : Fn(u32) -> u32 {
	fn new(calculator: T) -> Cacher<T> {
		Cacher {
			calculator,
			value: None,
			values: HashMap::new()
		}
	}
	fn value(&mut self, arg: u32) -> u32 {
		match self.value {
			Some(v) => v,
			None => {
				let val = (self.calculator)(arg);
				self.value = Some(val);
				val
			}
		}
	}
	fn value2(&mut self, arg: u32) -> u32 {
		let values = &mut self.values;
		{
			let entry = values.get(&arg);
			if let Some(&v) = entry {
				return v;
			}
		}
		let v = (self.calculator)(arg);
		values.insert(arg, v);
		v
	}
}

/// Counter structure for interaction from 1 to 5 including
/// 
pub struct Counter {
	count: u32,
}
impl Counter {
	pub fn new() -> Counter {
		Counter { count: 0 }
	}
}
impl Iterator for Counter {
	type Item = u32;
	fn next(&mut self) -> Option<Self::Item> {
		self.count += 1;
		if self.count < 6 {
			Some(self.count)
		} else {
			None
		}
	}
}

fn main() {
	let mut cacher = Cacher::new(|x| { x * x });
	for value in vec![1, 2, 3, 4, 5] {
		println!("{} -> {}, {}", value, cacher.value(value), cacher.value2(value));
	}
	
	fn f(v:u32) -> u32 { 2 * v };
	println!("{}->{}", 5, f(5));
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn iterator_demo() {
		let v = vec![1, 2, 3];
		
		let mut i = v.iter();
		
		assert_eq!(i.next(), Some(&1));
		assert_eq!(i.next(), Some(&2));
		assert_eq!(i.next(), Some(&3));
		assert_eq!(i.next(), None);
		
		let v2: Vec<_> = v.iter().map(|x| x + 1).collect();
		assert_eq!(v2, vec![2, 3, 4]);
		
		let mut cnt = Counter::new();
		
		assert_eq!(cnt.next(), Some(1));
		assert_eq!(cnt.next(), Some(2));
	}
}
