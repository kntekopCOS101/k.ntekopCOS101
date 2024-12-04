fn main() {
    let A:i32 = 10;
    let B:i32 = 20;

    println!("The value of A is: {}", A);
    println!("The value of B is: {}", B);

    let mut res = A<B;
    println!("A is lesser than B {}", res);

    res = A>B;
    println!("Ais greater than B {}", res);

    res = A>=B;
    println!("A is greater than or equal to B {}", res);

    res = A<=B;
    println!("A is less than or equal to B {}", res);

    res = A==B;
    println!("A is equal to B {}", res);

    res = A!=B;
    println!("A is not equal to B {}", res);


}
