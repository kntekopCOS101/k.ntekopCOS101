fn main() {
    let mut colors = vec!["Red", "Green","Yellow","White"];
    println!("\nOriginal array = {:?}", colors);
    let sliced_colours = &mut colors[1..3];
    println!("First slice = {:?}", sliced_colours);
    sliced_colours[1] = "Purple";
    println!("Changed slice = {:?}", sliced_colours);
}
