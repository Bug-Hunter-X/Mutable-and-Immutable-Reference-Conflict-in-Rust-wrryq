fn main() {
    let mut x = 5;
    {
        let y = &mut x; // y is a mutable reference to x
        *y += 1; // Modifies x through y
        println!("x = {}", x); // Prints x = 6
    }

    let z = &x;  // z is an immutable reference to x. Now it is safe since y is out of scope
    println!("z = {}", *z); // Prints z = 6
} 