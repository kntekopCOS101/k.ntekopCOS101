fn main() {
    let full_name = "Ntekop Kendara Rhoni";
    let dept = "Software Eng";
    let uni = "PAU";
    let mut school = "school of sciences".to_string();
    school.push_str("and technology");
    println!("My name is {}", full_name);
    println!("The length of my fullname is {}", full_name.len());
    println!("I am a student of {} department", dept);
    println!("{}", school);
    println!("{}", uni);
}