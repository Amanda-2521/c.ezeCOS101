use std::io;

fn main() {
    // Display the menu
    println!("--- MENU ---");
    println!("P - Poundo Yam / Edinkaiko Soup : N3,200");
    println!("F - Fried Rice & Chicken        : N3,000");
    println!("A - Amala & Ewedu Soup          : N2,500");
    println!("E - Eba & Egusi Soup            : N2,000");
    println!("W - White Rice & Stew           : N2,500");

    // Read food selection
    println!("\nEnter food letter (P, F, A, E, W):");
    let mut food_input = String::new();
    io::stdin().read_line(&mut food_input).unwrap();
    let food = food_input.trim();

    // Set price based on selection
    let mut price = 0;

    if food == "P" || food == "p" {
        price = 3200;
    } else if food == "F" || food == "f" {
        price = 3000;
    } else if food == "A" || food == "a" {
        price = 2500;
    } else if food == "E" || food == "e" {
        price = 2000;
    } else if food == "W" || food == "w" {
        price = 2500;
    } else {
        println!("Invalid food choice!");
        return;
    }

    // Read quantity
    println!("Enter quantity:");
    let mut qty_input = String::new();
    io::stdin().read_line(&mut qty_input).unwrap();
    let quantity: i32 = qty_input.trim().parse().unwrap();

    // Calculate total charge
    let mut total = (price * quantity) as f64;
    println!("Subtotal: N{}", total);

    // Apply 5% discount if total > 10,000
    if total > 10000.0 {
        let discount = total * 0.05;
        total = total - discount;
        println!("5% Discount Applied! Discount amount: N{}", discount);
    }

    println!("Total Amount to Pay: N{}", total);
}
