use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

     let P = "poundo yam and edikaikon soup";
    let F = "fried rice and chicken";
    let A = "amala and ewedu soup";
    let E = "eba and egusi soup";
    let W = "white rice and stew";

    let P_pri = 3200;
    let F_pri = 3000;
    let A_pri = 2500;
    let E_pri = 2000;
    let W_pri = 2500;

    println!("Welcome to FOOD FOR HEALTH cafe, will you like to see our menu?");
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    if input1.trim() == "yes" {
         println!("{} is {} input P to order", P, P_pri);
    println!("{} is {}, input F to order", F, F_pri);
    println!("{} is {} input A to order", A, A_pri);
    println!("{} is {} input E to order", E, E_pri);
    println!("{} is {}input W to order", W, W_pri);
    println!("Enter your orders once at a time");

    println!("What could we hekp you with?");
    io::stdin().read_line(&mut input2).expect("Failed to read input");

    println!("How many plates will you like to order?");
    io::stdin().read_line(&mut input3).expect("Failed to read input");

     let qty:f64 = input3.trim().parse().expect("Failed to read input");


    if input2.trim() == "P" {
        let p:f64 = 3200.0;
        let amount:f64 = p * qty;
        if amount > 10000.0 {
            let pd:f64 = 5.0/100.0;
            let discount = pd * amount;
            let cost = amount - discount;
            println!("WE have an attractive 5% discount for your purchase, your amount is {}", cost);

        }
        else if amount < 10000.0 {
            println!("That will cost you {}", amount);
        }
        }
    }
    
    else if input1.trim() == "no"{
    println!("Ok bye <3");
}
let qty:f64 = input3.trim().parse().expect("Failed to read input");

     if input2.trim() == "F" {
        let f:f64 = 3000.0;
        let amount:f64 = f * qty;
        if amount > 10000.0 {
            let pd:f64 = 5.0/100.0;
            let discount = pd * amount;
            let cost = amount - discount;
            println!("WE have an attractive 5% discount for your purchase, your amount is {}", cost);

        }
        else if amount < 10000.0 {
            println!("That will cost you {}", amount);
        }
    }

    let qty:f64 = input3.trim().parse().expect("Failed to read input");

     if input2.trim() == "A" {
        let a:f64 = 2500.0;
        let amount:f64 = a * qty;
        if amount > 10000.0 {
            let pd:f64 = 5.0/100.0;
            let discount = pd * amount;
            let cost = amount - discount;
            println!("WE have an attractive 5% discount for your purchase, your amount is {}", cost);

        }
        else if amount < 10000.0 {
            println!("That will cost you {}", amount);
}
}

let qty:f64 = input3.trim().parse().expect("Failed to read input");

     if input2.trim() == "E" {
        let e:f64 = 2000.0;
        let amount:f64 = e * qty;
        if amount > 10000.0 {
            let pd:f64 = 5.0/100.0;
            let discount = pd * amount;
            let cost = amount - discount;
            println!("WE have an attractive 5% discount for your purchase, your amount is {}", cost);

        }
        else if amount < 10000.0 {
            println!("That will cost you {}", amount);
}
}

let qty:f64 = input3.trim().parse().expect("Failed to read input");

     if input2.trim() == "W" {
        let w:f64 = 2500.0;
        let amount:f64 = w * qty;
        if amount > 10000.0 {
            let pd:f64 = 5.0/100.0;
            let discount = pd * amount;
            let cost = amount - discount;
            println!("WE have an attractive 5% discount for your purchase, your amount is {}", cost);

        }
        else if amount < 10000.0 {
            println!("That will cost you {}", amount);
}
}
}
    

    
    
