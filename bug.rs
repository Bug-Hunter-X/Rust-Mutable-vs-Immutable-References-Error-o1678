fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    *y += 1; // Modifying x through y is allowed
    println!("x = {}", x); // Output: x = 6

    // This line will cause a compile-time error
    // Because z is an immutable reference
    // *z += 1;
}