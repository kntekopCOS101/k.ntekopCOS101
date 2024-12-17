fn main() { 
    let numbers = [1, 2, 3, 4, 5];
    println!("Original array = {:?}", numbers);
    let slice1 = &numbers[1..3];
    println!("Second and third elements sliced = {:?}", slice1);
    let slice2 = &numbers[..3];
    println!("index 0 to index 3 sliced = {:?}", slice2);
    let slice3 = &numbers[2..];
    println!("index 2 to index 3 = {:?}", slice3);
    let slice4 = &numbers[..];
    println!("Index 0 to index 5 = {:?}", slice4);
}

