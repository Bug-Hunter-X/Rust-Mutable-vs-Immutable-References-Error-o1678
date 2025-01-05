fn main() {
    let mut x = 5;
    let y = &mut x; // Mutable reference
    
    *y += 1;
    println!("x = {}", x); // Output: x = 6
    
    //To use immutable reference,  we can clone value 
    let z = x.clone();
    println!("z = {}", z); // Output: z = 6

}