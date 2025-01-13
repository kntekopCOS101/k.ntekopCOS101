struct Laptop {
    brand:String,
    price:u32,
    quantity:u32
}
impl Laptop{
    fn cost(&self) -> u32{
        self.price * self.quantity
    }
}

fn main(){
    let h_p = Laptop{
        brand:String::from("HP"),
        price:650000,
        quantity:3,
    };

    let ibm = Laptop{
        brand:String::from("IBM"),
        price:755000,
        quantity:3,
    };

    let dell = Laptop{
        brand:String::from("DELL"),
        price:850000,
        quantity:3,
    };


    let tosh= Laptop{
        brand:String::from("TOSHIBA"),
        price:550000,
        quantity:3,
    };

    let total = h_p.cost() + ibm.cost() + dell.cost() + tosh.cost();

    println!("You got {}; {}, {}, {} and {} laptops \n Costing {}, {}, {} and {} individually. everything costing {}", dell.quantity, h_p.brand, ibm.brand, dell.brand, tosh.brand, h_p.price, ibm.price, dell.price, tosh. price, total );
}

