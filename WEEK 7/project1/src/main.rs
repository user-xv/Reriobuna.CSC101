use std::io;
fn trapezium(){
    println!("\nAREA OF TRAPEZIUM:");


    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("enter the height:");
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let a:f32 = input1.trim().parse().expect("not a valid number");
    
    println!("enter the base 1:");
    io::stdin().read_line(&mut input2).expect("not a valid string");
    let b:f32 = input2.trim().parse().expect("not a valid number");
    
    println!("enter the base 2:");
    io::stdin().read_line(&mut input3).expect("not a valid string");
    let c:f32 = input3.trim().parse().expect("not a valid number");
    
    let z:f32 = (a * (b + c)) / 2.0;
    println!("area of trapezium is {}",z);
}

fn rhombus(){
    println!("\nAREA OF RHOMBUS:");


    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("enter first diagonal:");
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let a:f32 = input1.trim().parse().expect("not a valid number");
    
    println!("enter second diagonal:");
    io::stdin().read_line(&mut input2).expect("not a valid string");
    let b:f32 = input2.trim().parse().expect("not a valid number");
    
    let z:f32 = (a * b) / 2.0;
    println!("area of rhombus is {}",z);
}

fn parallelogram(){
    println!("\nAREA OF PARALLELOGRAM:");


    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("enter base:");
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let a:f32 = input1.trim().parse().expect("not a valid number");
    
    println!("enter altitude:");
    io::stdin().read_line(&mut input2).expect("not a valid string");
    let b:f32 = input2.trim().parse().expect("not a valid number");
    
    let z:f32 = a * b;
    println!("area of rhombus is {}",z);
}

fn cube(){
    println!("\nAREA OF CUBE:");

    let mut input1 = String::new();

    println!("enter length:");
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let a:f32 = input1.trim().parse().expect("not a valid number");
    
    let z:f32 = 6.0 * a;
    println!("area of rhombus is {}",z);
}

fn cylinder(){
    println!("\nVOLUME OF CYLINDER:");

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("enter radius:");
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let a:f32 = input1.trim().parse().expect("not a valid number");
    
    println!("enter height:");
    io::stdin().read_line(&mut input2).expect("not a valid string");
    let b:f32 = input2.trim().parse().expect("not a valid number");
    
    let z:f32 = (22.0 * a.powf (2.0) * b) / 7.0;
    println!("area of rhombus is {}",z);
}

fn main() {
    println!("\nMENSURATION CALCULATOR:");



   loop{
    let mut input1 = String::new();
    
    println!("\nT- Area of Trapezium formula = height/2*(base1+base2)
\nR- Area of the rhombus formula = ½ × diagonal1 × diagonal2
\nP- Area of Parallelogram formula = base x altitude
\nC- Area of Cube formula = 6 x (length of the side)2
\nV- Volume of Cylinder formula = π*radius2*height
\nQ- Quit");

println!("\nInsert letter assigned to what you wish to calculate");
    io::stdin().read_line(&mut input1);
    let x:&str = input1.trim();

if x == "T"{
trapezium()
}else if x == "R"{
    rhombus()
}else if x == "P"{
    parallelogram()
}else if x == "C"{
    cube()
}else if x == "V"{
    cylinder()
}else if x == "Q"{
    break;
}else {
    println!("error")
}
}
}