use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("enter your a:");
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let a:f32 = input1.trim().parse().expect("not a valid number");
    
    println!("enter your b:");
    io::stdin().read_line(&mut input2).expect("not a valid string");
    let b:f32 = input2.trim().parse().expect("not a valid number");
    
    println!("enter your c:");
    io::stdin().read_line(&mut input3).expect("not a valid string");
    let c:f32 = input3.trim().parse().expect("not a valid number");
    
    let d:f32 = b.powf (2.0) - 4.0 * a * c;
    if d < 0.0 {
    println!("there is no real root");
    } else if d > 0.0 {
        println!("there are 2 roots");
    } else {
        println!("there is one real root");
    }
}
