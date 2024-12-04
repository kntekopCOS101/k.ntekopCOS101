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

println!("What will you like to calculate?");
io::stdin().read_line(&mut input1).expect("Failed to input value");

if input1.trim() == "AOT" {
    println!("Input the height of the trapezium");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let h:f64 = input2.trim().parse().expect("Failed to read input");
    println!("Input base 1 of the trapezium");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let b1:f64 = input3.trim().parse().expect("Failed to read input");
    println!("Input base 2 of the trapezium");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let b2:f64 = input4.trim().parse().expect("Failed to read input");

    let sum:f64 = b1 + b2;
    let aot:f64 = h / 2.0 * sum;
    println!("AREA = {}", aot);
}

else if input1.trim() == "AOR" {
     println!("Input daigonal 1 of the rhombus");
    io::stdin().read_line(&mut input5).expect("Failed to read input");
    let d1:f64 = input5.trim().parse().expect("Failed to read input");
    println!("Input diagonal 2 of the rhombus");
    io::stdin().read_line(&mut input6).expect("Failed to read input");
    let d2:f64 = input6.trim().parse().expect("Failed to read input");

    let aor:f64 = 0.5 * d1 * d2;
    println!("AREA = {}", aor);

}
else if input1.trim() == "AOP"  {
     println!("Input the base of the parallelogram");
    io::stdin().read_line(&mut input7).expect("Failed to read input");
    let b:f64 = input7.trim().parse().expect("Failed to read input");
    println!("Input the altitude of the parallelogram");
    io::stdin().read_line(&mut input8).expect("Failed to read input");
    let a:f64 = input8.trim().parse().expect("Failed to read input");


    let aop:f64 = b * a;
    println!("AREA = {}", aop);
}
 
 else if input1.trim() == "AOC"  {
     println!("Input the length of the cube");
    io::stdin().read_line(&mut input9).expect("Failed to read input");
    let l:f64 = input9.trim().parse().expect("Failed to read input");


    let aoc:f64 = 6.0  * l * l;
    println!("AREA = {}", aoc);
 }

 else if input1.trim() == "VOC" {
     println!("Input the value of thr radius");
    io::stdin().read_line(&mut input10).expect("Failed to read input");
    let r:f64 = input10.trim().parse().expect("Failed to read input");
    println!("Input the value of the height");
    io::stdin().read_line(&mut input11).expect("Failed to read input");
    let h:f64 = input11.trim().parse().expect("Failed to read input");
    

    let pi:f64 = 3.142;
    let voc:f64 = pi * r * r * h;
    println!("AREA = {}", voc);
 }



}




