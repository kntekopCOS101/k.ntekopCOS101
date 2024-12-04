use std ::io;
fn main() {
    let  mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
     let mut input4 = String::new();
     let mut input5 = String::new();
     let mut input6 = String::new();
     let mut input7 = String::new();
     let mut input8 = String::new();
     let mut input9 = String::new();
     let mut input10 = String::new();
     let mut input11 = String::new();

     let mut b1 = "base one";
     let mut b2 = "base two";
     let mut h = "height";
     let mut d1 = "diagonal one";
     let mut d2 = "diagonal two";
     let mut a = "altitude";
     let mut l = "length";
     let mut r = "radius";


    println!("Welcome 2 math 101 class, these are the various formulas");
    println!("Area of trapezium = height/2*(base1 + base2)");
println!("Area of rhombus = 0.5 * diagonal1 * diagonal2");
println!("Area of paralellogram = base * altitude");
println!("Area of cube = 6 * (length)^2");
println!("Volume of cylinder = pi * rad^2 * height");

println!("If you want to calculate the Area Of Trapeziun, kindly type AOT");
io::stdin().read_line(&mut input1).expect("Failed to read input");
println!("If you want to calculate the Area Of Rhombus, kindly type AOR");
println!("If you want to calculate the Area Of paralellogram, kindly type AOP");
println!("If you want to calculate the Area Of Cube, kindly type AOC");
println!("If you want to calculate the Volume Of cylinder, kindly type VOC");


}




