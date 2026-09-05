fn main() {
	let p: f64 = 210000.0;
	let r: f64 = 5.0;
	let n: i32 = 3;

	let a = p * (1.0 - (r/ 100.0)).powi(n);
	let depreciation = p - a;

	println!("value after {} years: {:.2}", n, a);
	println!("Total depreciation: {:.2}", depreciation);
}
