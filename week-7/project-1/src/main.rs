use std::io;

fn read_number(prompt: &str) -> f64 {
loop {
let mut input = String::new();
println!("{}", prompt);

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().parse::<f64>() {
        Ok(number) if number > 0.0 => return number,
        _ => println!("Invalid input! Please enter a positive number."),
    }
}

}

// Function to calculate the area of a trapezium
fn trapezium_area() -> f64 {
let height = read_number("Enter the height:");
let base1 = read_number("Enter the first base:");
let base2 = read_number("Enter the second base:");

(height / 2.0) * (base1 + base2)

}

// Function to calculate the area of a rhombus
fn rhombus_area() -> f64 {
let diagonal1 = read_number("Enter the first diagonal:");
let diagonal2 = read_number("Enter the second diagonal:");

0.5 * diagonal1 * diagonal2

}

// Function to calculate the area of a parallelogram
fn parallelogram_area() -> f64 {
let base = read_number("Enter the base:");
let altitude = read_number("Enter the altitude:");

base * altitude

}

// Function to calculate the surface area of a cube
fn cube_surface_area() -> f64 {
let side = read_number("Enter the side length:");

6.0 * side * side

}

// Function to calculate the volume of a cylinder
fn cylinder_volume() -> f64 {
let radius = read_number("Enter the radius:");
let height = read_number("Enter the height:");

std::f64::consts::PI * radius * radius * height

}

fn main() {
loop {
println!("\n================================");
println!("       SHAPE CALCULATOR");
println!("================================");
println!("1. Area of a Trapezium");
println!("2. Area of a Rhombus");
println!("3. Area of a Parallelogram");
println!("4. Surface Area of a Cube");
println!("5. Volume of a Cylinder");
println!("================================");

    let mut choice = String::new();

    println!("Enter your choice (1-5):");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let choice: u32 = match choice.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid choice! Please enter a number from 1 to 5.");
            continue;
        }
    };

    let result = match choice {
        1 => Some(("Area of Trapezium", trapezium_area())),
        2 => Some(("Area of Rhombus", rhombus_area())),
        3 => Some(("Area of Parallelogram", parallelogram_area())),
        4 => Some(("Surface Area of Cube", cube_surface_area())),
        5 => Some(("Volume of Cylinder", cylinder_volume())),
        _ => {
            println!("Invalid choice! Please select from 1 to 5.");
            continue;
        }
    };

    if let Some((name, answer)) = result {
        println!("\n================================");
        println!("Calculation: {}", name);
        println!("Result: {:.2}", answer);
        println!("================================");
    }

    println!("\nWould you like to calculate another shape? (y/n)");

    let mut again = String::new();

    io::stdin()
        .read_line(&mut again)
        .expect("Failed to read input");

    match again.trim().to_lowercase().as_str() {
        "y" => continue,
        "n" => {
            println!("Thank you for using the Shape Calculator!");
            break;
        }
        _ => {
            println!("Invalid response! Returning to the menu.");
        }
    }
}

}