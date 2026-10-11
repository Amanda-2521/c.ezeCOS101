use std::io::{self, Write};
use std::f64::consts::PI;

fn main() {
    loop {
        println!("\n=== Project: The Shape Calculator ===");
        println!("1. Trapezium (Area)");
        println!("2. Rhombus (Area)");
        println!("3. Parallelogram (Area)");
        println!("4. Cube (Surface Area)");
        println!("5. Cylinder (Volume)");
        println!("6. Exit");
        print!("Enter your choice (1-6): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "1" => {
                let result = calculate_trapezium_area();
                println!("The area of the trapezium is: {:.2}", result);
            }
            "2" => {
                let result = calculate_rhombus_area();
                println!("The area of the rhombus is: {:.2}", result);
            }
            "3" => {
                let result = calculate_parallelogram_area();
                println!("The area of the parallelogram is: {:.2}", result);
            }
            "4" => {
                let result = calculate_cube_surface_area();
                println!("The surface area of the cube is: {:.2}", result);
            }
            "5" => {
                let result = calculate_cylinder_volume();
                println!("The volume of the cylinder is: {:.2}", result);
            }
            "6" => {
                println!("Exiting program. Goodbye!");
                break;
            }
            _ => println!("Invalid choice! Please select a number between 1 and 6."),
        }
    }
}

// Helper function to read a floating-point number from user input
fn read_input(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

// 1. Trapezium: height / 2 * (base1 + base2)
fn calculate_trapezium_area() -> f64 {
    println!("\n--- Trapezium Area Calculator ---");
    let height = read_input("Enter height: ");
    let base1 = read_input("Enter base 1: ");
    let base2 = read_input("Enter base 2: ");
    (height / 2.0) * (base1 + base2)
}

// 2. Rhombus: 1/2 * diagonal1 * diagonal2
fn calculate_rhombus_area() -> f64 {
    println!("\n--- Rhombus Area Calculator ---");
    let diagonal1 = read_input("Enter diagonal 1: ");
    let diagonal2 = read_input("Enter diagonal 2: ");
    0.5 * diagonal1 * diagonal2
}

// 3. Parallelogram: base * altitude
fn calculate_parallelogram_area() -> f64 {
    println!("\n--- Parallelogram Area Calculator ---");
    let base = read_input("Enter base: ");
    let altitude = read_input("Enter altitude: ");
    base * altitude
}

// 4. Cube: 6 * side * side
fn calculate_cube_surface_area() -> f64 {
    println!("\n--- Cube Surface Area Calculator ---");
    let side = read_input("Enter side length: ");
    6.0 * side * side
}

// 5. Cylinder: pi * radius * radius * height
fn calculate_cylinder_volume() -> f64 {
    println!("\n--- Cylinder Volume Calculator ---");
    let radius = read_input("Enter radius: ");
    let height = read_input("Enter height: ");
    PI * radius * radius * height
}

