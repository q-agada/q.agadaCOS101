use std::io;

fn main(){

    println!("           RESTAURANT MENU");
    println!("+---------+----------------------------+-----");
    println!("FOOD CODE      FOOD                     PRICE");
    println!("+---------+----------------------------+----\n");
    println!("P         |Poundo Yam / Edinkaiko Soup | N3200");
    println!("F         |Fried Rice & Chicken        | N3000");
    println!("A         |Amala & Ewedu Soup          | N2500");
    println!("E         |Eba & Egusi Soup            | N2000");
    println!("W         |White Rice & Stew           | N2500");

    println!("\n Enter food choice");
    let mut food_choice = String::new();
    io::stdin().read_line(& mut food_choice).expect("failed to read input");

    println!("Enter quantity");
    let mut quantity = String::new();
    io::stdin().read_line(& mut quantity).expect("failed to read input");
    let quantity: i32 = quantity.trim().parse().expect("please enter a number");

    
    let price: i32 = match food_choice.trim().to_uppercase().as_str(){
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("invalid  food choice");
            return;
        }
    };

// input the formula
let total: i32= price * quantity;
println!("TOTAL AMOUNT IS N{}", total);


//discount formula for 5 % 

if total > 10000{
let discount = total * 5 / 100;
let discount_amount = total - discount;

println!("FOR BUYING MORE THAN 10,000 PAY {} AT A 5% DISCOUNT RATE, YAYYY!!!!!!", discount_amount);
}
}

























