fn main() {
	let t:f64 = 450000.00;
	let m:f64 = 1500000.00;
	let h:f64 = 750000.00;
    let d:f64 = 2850000.00;
    let a:f64 = 250000.00;
	let sum:f64 = t + m + h + d + a ;
    let avg:f64 = sum / 5.0;
	println!("your sum and average is {} and {}",sum,avg);
}
