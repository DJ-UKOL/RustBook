pub fn variables() {
    // Mutability
    println!("Mutability");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Shadowing
    println!("\nShadowing");
    let y = 5;  // y = 5
    let y = y + 1;  // y = 5 + 1 = 6

    {
        let y = y * 2;  // y = 6 * 2 = 12
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}"); // y = 6
}