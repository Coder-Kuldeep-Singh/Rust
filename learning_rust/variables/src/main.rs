// Variables and Mutability
fn main() {
    let mut x = 5; // immutable
    println!("The value of x is: {}", x);
    x = 6; // can't modify the value of immutable variables
    println!("The value of x is: {}", x);

    // Const variables are immutable by nature you can't make them mutable
    const MAX_POINTS: u32 = 10_000_0;
    println!("Const Value is: {}", MAX_POINTS);

    // Shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2; // 5 , 5 + 1 = 6, 6 * 2 = 12 result
    println!("The value of the y is: {}", y);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("spaces count: {}", spaces);
}
