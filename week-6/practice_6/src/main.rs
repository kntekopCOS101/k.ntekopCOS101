fn main() {
    let n1 = "Electrical".to_string();
    let n2 = "Electronic".to_string();
    let n3 = "engineering".to_string();
    let n4 = n1 + &n2 + &n3;

    println!("\nThe {} is informed by the aspiration to train electronic/electric engineering professionals in areas of design, building and maintenance of electrical control sysytems,", n4);


    let v1 = "Computer".to_string();
    let v2 = "Science".to_string();
    let v3 = v1 + &v2;
    println!();
    println!("{} is aimed at developing competent professionals", v3);

}