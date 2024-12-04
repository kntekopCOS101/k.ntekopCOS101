fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "Pan Atlantic University";
    let add:&str = "Km 52 lekki epe expressway, ibeju lekki";
    println!("Name: {}", name);
    println!("University: {}, /nAddress: {}", uni, add);
    let department:&'static str = "Computer science";
    let school:&'static str = "school of science and technology";
    println!("Department: {}, \nSchool: {}", department, school);



}
