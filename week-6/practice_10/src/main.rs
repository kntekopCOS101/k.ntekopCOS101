fn main() {
    let a = 20;
    let b = 30;

    if (a > 10) && (b > 10) {
        println!("TRUE");
    }
    
    let c = 0;
    let d = 30;
    if (c>30) || (d>10) {
        println!("TRUE");
    }
    let is_elder = false;
    if !is_elder {
        println!("NOT ELDER");
    }
}