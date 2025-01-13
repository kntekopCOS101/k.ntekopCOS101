struct Employee{
    name:String,
    company:String,
    age:u32

}

fn main(){
    let emp1 = Employee {
        company:String::from("emrst & young"),
        name:String::from("Edibiong Jess"),
        age: 25

    };
    println!("Name is {}", emp1.name);
    println!("Compamy is {}", emp1.company);
    println!("Age is {}", emp1.age);
}

