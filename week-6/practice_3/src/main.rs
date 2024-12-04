fn main() {
    let name1 = "David Bamidele";
    println!("My name is {}", name1);


    let name2 = name1.replace("David", "AY");
    println!("You can also call me {}", name2);
    let faculty = "Faculty of science and tech";


    let school = faculty.replace("Faculty", "school");
    println!("I am a student of the {}", school);
}
