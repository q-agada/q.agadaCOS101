fn main() {
	let p: f64 = 520000000.0;
	let r: f64 = 10.0;
	let n: i32 = 5;

	let a = p * (1.0 + (r / 100.0)).powi(n);
	let ci = a - p;

	println!("Amount after {} years is : {:.2}", n, a);
	println!("Compound Interest is: {:.2}", ci);
	
}