	#![feature(test)]
	extern crate test;
	use test::Bencher;
	use minimal_id::Generator;
	use std::thread::sleep_ms;

	#[bench]
	fn baseline_benchmark(b: &mut Bencher) {
		let generator = Generator::default();
		b.iter(|| generator.generate());
	}

	#[bench]
	/// This test validates that if we generate 1 million ids in fast order,
	/// that we hit no collisions.
	fn validate_uniqueness(b: &mut Bencher) {
		let generator = Generator::default();
		let mut generated = std::collections::HashSet::new();
		b.iter(|| {
			(0..1000000).for_each(|_| { generated.insert(generator.generate()); });
		});
	}