fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x;  // z is an immutable reference to x

    *y += 1; // Modifies x through y
    println!("x = {}", x); // Prints x = 6

    // This line will cause a compile-time error:
    // println!("z = {}", *z); // Cannot dereference an immutable reference
}