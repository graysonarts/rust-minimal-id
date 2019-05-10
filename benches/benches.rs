	#![feature(test)]
	extern crate test;
	use test::Bencher;
	use minimal_id::Generator;

	#[bench]
	fn baseline_benchmark(b: &mut Bencher) {
		let generator = Generator::default();
		b.iter(|| generator.generate());
	}