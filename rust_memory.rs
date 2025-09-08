fn main() {
    // Ownership example
    let s1 = String::from("Hello"); // s1 owns the string
    println!("s1 = {}", s1);

    // Move ownership to s2
    let s2 = s1;
    println!("s2 = {}", s2);

    // Borrowing example
    let s3 = String::from("Rust");
    print_length(&s3); // borrow s3 but ownership is not moved
    println!("s3 after borrowing = {}", s3); // s3 is still accessible

    // Mutable borrowing
    let mut s4 = String::from("Hello");
    append_world(&mut s4); // mutable borrow to modify the string
    println!("s4 after mutable borrow = {}", s4);
}

// Function that borrows a string immutably
fn print_length(s: &String) {
    println!("Length of '{}' is {}", s, s.len());
}

// Function that borrows a string mutably
fn append_world(s: &mut String) {
    s.push_str(", world!");
}