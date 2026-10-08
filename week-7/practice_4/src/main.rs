use std::io;

fn add(a: i32, b:i32){
    let sum = a + b;

    println!("Sum of A and B = {}", sum);

}

fn main(){

    let mut a = String::new();
    println!("Enter input for parameter A:");
    io::stdin().read_line(&mut a ).expect("Failed to read input");
    let a:i32 = a.trim().parse().expect("Invalid input");

    let mut b = String::new();
    println!("Enter input for parameter B:");
    io::stdin().read_line(&mut b ).expect("Failed to read input");
    let b:i32 = b.trim().parse().expect("Invalid input");

    // call add function with arguments 

    add(a,b);
}