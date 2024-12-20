use std::io::Write;
use std::io::Read;
fn main()  {

let mut file = std::fs::File::create("PAU SMIS.txt").expect("Failed to create file");
let  data = vec![" Students name", "     Matric number", "     Department", "     Level"];
let  students_name = vec!["  Oluchi Mordi", "  Adams Aliyu", "  Shania Bolade", "  Adekunle Gold", "  Blanca Edemoh"];
let  matric_number = vec!["     ACC10211111", "      ECO101101101", "     CSC10328828", "     EE11020202", "     ME10202001"];
let  dept = vec!["       Accounting", "       Economics", "      Computer", "       Electrical", "       Mechanical"];
let  level = vec!["      300", "      100", "        200", "      200", "      100"];
let d1 = data[0];
let d2 = data[1];
let d3 = data[2];
let d4 = data[3];
let s1 = students_name[0];
let s2 = students_name[1];
let s3 = students_name[2];
let s4 = students_name[3];
let s5 = students_name[4];
let m1 = matric_number[0];
let m2 = matric_number[1];
let m3 = matric_number[2];
let m4 = matric_number[3];
let m5 = matric_number[4];
let f1 = dept[0];
let f2 = dept[1];
let f3 = dept[2];
let f4 = dept[3];
let f5 = dept[4];
let l1 = level[0];
let l2 = level[1];
let l3 = level[2];
let l4 = level[3];
let l5 = level[4];

file.write_all("BELOW IS PAU SMIS\n".as_bytes()).expect("Failed");
file.write_all("PAU SMIS\n".as_bytes()).expect("Failed");
file.write_all(d1.as_bytes()).expect("Failed");
file.write_all(d2.as_bytes()).expect("Failed");
file.write_all(d3.as_bytes()).expect("Failed");
file.write_all(d4.as_bytes()).expect("Failed");
file.write_all("\n".as_bytes()).expect("Failed");

file.write_all(s1.as_bytes()).expect("Failed");
file.write_all(m1.as_bytes()).expect("Failed");
file.write_all(f1.as_bytes()).expect("Failed");
file.write_all(l1.as_bytes()).expect("Failed");

file.write_all("\n".as_bytes()).expect("Failed");

file.write_all(s2.as_bytes()).expect("Failed");
file.write_all(m2.as_bytes()).expect("Failed");
file.write_all(f2.as_bytes()).expect("Failed");
file.write_all(l2.as_bytes()).expect("Failed");

file.write_all("\n".as_bytes()).expect("Failed");

file.write_all(s3.as_bytes()).expect("Failed");
file.write_all(m3.as_bytes()).expect("Failed");
file.write_all(f3.as_bytes()).expect("Failed");
file.write_all(l3.as_bytes()).expect("Failed");

file.write_all("\n".as_bytes()).expect("Failed");

file.write_all(s4.as_bytes()).expect("Failed");
file.write_all(m4.as_bytes()).expect("Failed");
file.write_all(f4.as_bytes()).expect("Failed");
file.write_all(l4.as_bytes()).expect("Failed");

file.write_all("\n".as_bytes()).expect("Failed");

file.write_all(s5.as_bytes()).expect("Failed");
file.write_all(m5.as_bytes()).expect("Failed");
file.write_all(f5.as_bytes()).expect("Failed");
file.write_all(l5.as_bytes()).expect("Failed");

println!("DOC CREATED");

 let mut file = std::fs::File::open("PAU SMIS.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);


}




