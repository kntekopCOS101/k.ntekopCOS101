use std::io;
use std::io::Read;

fn administrator() {
    let mut file = std::fs::File::open("src/globacom_db.txt.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn employee() {
    let mut file = std::fs::File::open("src/Staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn projectman() {
    let mut file = std::fs::File::open("src/Project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn customer() {
    let mut file = std::fs::File::open("src/Customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn vendor() {
    let mut file = std::fs::File::open("src/Data_Plan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn main() {
    let mut input1 = String::new();
    println!("Type a if youre an administrator
    Type b if youre employee
    Type c if youre a project manager
    Type d if youre a customer
    Type e if youre a vendor" );
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    if input1.trim().to_lowercase() == "a" {
        administrator();
    }
    else if input1.trim() == "b" {
        employee();
    }
    else if input1.trim() == "c" {
        projectman();
    }
    else if input1.trim() == "d" {
        customer();
    }
    else if input1.trim() == "e"  {
        vendor();
    }

}
