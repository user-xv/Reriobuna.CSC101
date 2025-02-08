fn main() {
	let p:f64 = 210000.00;
	let t:f64 = 3.00;
	let r:f64 = 5.00;
	let a:f64 = p * (1.00 - (r / 100.00)).powf (t);
	let d:f64 = a - p;
	println!("your amount is {} and your depreciation is {}",a,d);
}