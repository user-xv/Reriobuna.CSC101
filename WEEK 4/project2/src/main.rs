use std::io;
fn main() {
    loop{
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("enter your age:");
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let a:f32 = input1.trim().parse().expect("not a valid number");
    
    println!("enter your years of work experience:");
    io::stdin().read_line(&mut input2).expect("not a valid string");
    let b:f32 = input2.trim().parse().expect("not a valid number");
    
    if b <= 0.0{
        println!("your incentive is 100,000");
    } else if a >= 40.0 {
        println!("your incentive is 1,560,000");
    } else if a >= 30.0 {
        println!("your incentive is 1,480,000");
    } else if a <= 28.0 {
        println!("your incentive is 1,300,000");
    } else {
        println!("your incentive is pending")
    }
}
    }