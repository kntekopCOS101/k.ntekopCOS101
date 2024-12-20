use std::fs::OpenOptions;
use std::io::Write;
fn main() {

    let announce = "Welcome to week 9 - Rust input and output";
    let dept = "Department of Software Engineering";
    let mut file = std::fs::File::create("data.txt").expect("Create Failed");
    file.write_all("Welcome to Rust Programming\n"
        .as_bytes()).expect("Write Failed");
    file.write_all(announce.as_bytes()).expect("Write Failed");
    file.write_all(dept.as_bytes()).expect("Write Failed");
    println!("Data Written To File");

    let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
    file.write_all("\nHello class".as_bytes()).expect("Failed");
    file.write_all("\nThis is the appendage to the document".as_bytes()).expect("FAILED");
    println!("File append success");
}
