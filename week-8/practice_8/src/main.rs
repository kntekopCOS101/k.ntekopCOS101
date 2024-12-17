fn main() {
    let mut mountain_heights = ("everest", 8848, "Fishtail", 6339);
    println!("Original tuple = {:?}", mountain_heights);
    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8516;

    println!("changed tuple = {:?}", mountain_heights);
}
