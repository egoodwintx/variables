fn main() {
    let x = 5; 
    println!("The value of x is {x}.");
    let x = 6;
    println!("The value of x is {x}.");

    {
        let x = x * 2;
        println!("The value in the inner loop is {x}");
    }
}
