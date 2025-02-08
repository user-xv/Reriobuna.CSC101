fn main() {
	let p:f64 = 520000000.00;
	let t:f64 = 5.00;
	let r:f64 = 10.00;
	let a:f64 = p * (1.00 + (r / 100.00)).powf (t);
	let ci:f64 = a - p;
	println!("your amount is {} and your interest is {}",a,ci);
}