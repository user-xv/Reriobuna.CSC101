use std::io;
fn main() {
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken -      N3,000");
    println!("A = Amala & Ewedu Soup -        N2,500");
    println!("E = Eba & Egusi Soup -          N2,000");
    println!("W = White Rice & Stew -         N2,500");

    let p:i32 = 3200;
    let f:i32 = 3000;
    let a:i32 = 2500;
    let e:i32 = 2000;
    let w:i32 = 2500;

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("insert letter assigned to what you want to order");
    io::stdin().read_line(&mut input1);
    let x:&str = input1.trim();
    
    println!("how many");
    io::stdin().read_line(&mut input2).expect("not a valid string");
    let y:i32 = input2.trim().parse().expect("not a valid number");

    let mut j:i32 = 0;
    if x == "P" {
      j = p;
    } else if x == "F" {
      j = f;
    } else if x == "A" {
      j = a;
    } else if x == "E" {
      j = e;
    } else if x == "W" {
      j = w;
    } else {
        println!("error")
    }

    let mut t:i32 = 0;

    let z = j * y ;
    if z>10000 {
        println!("you have a 5% discount");
        t = (5 * z) / 100 ;
    } else {
        println!("no discount");
    }
    let q:i32 = z - t;
    println!("your total is {} and you are paying {}",z,q)
}
