use std::collections::HashMap;

pub struct Cacher<T> where T : Fn(u32) -> u32 {
	calculator: T,
	values: HashMap<u32, u32>,
}

impl<T> Cacher<T> where T : Fn(u32) -> u32 {
	pub fn new(calculator: T) -> Cacher<T> {
		Cacher {
			calculator,
			values: HashMap::new(),
		}
	}
	pub fn value(&mut self, arg: u32) -> u32 {
		let values = &mut self.values;
		{
			let entry = values.get(&arg);
			if let Some(&v) = entry {
				println!("from cache");
				return v;
			}
		}
		let v = (self.calculator)(arg);
		println!("calculated");
		values.insert(arg, v);
		v
	}
}
