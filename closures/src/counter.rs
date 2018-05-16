pub struct Counter {
	index: u32,
	end: u32,
}
impl Counter {
	pub fn new(start: u32, end: u32) -> Counter {
		Counter { index: start, end: end }
	}
}
impl Iterator for Counter {
	type Item = u32;
	fn next(&mut self) -> Option<Self::Item> {
		if self.index < self.end {
			let index = self.index;
			self.index += 1;
			Some(index)
		} else {
			None
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn iterator_demo() {
		let mut cnt = Counter::new(1, 3);
		
		assert_eq!(cnt.next(), Some(1));
		assert_eq!(cnt.next(), Some(2));
		assert_eq!(cnt.next(), None);
	}
}
