fn borrowing_example() {
    let first_text: String = String::from("first text");
    let reference: &String = &first_text;

    println!("The first text is: {}", first_text);
    println!("The reference is: {}", reference);
}

fn mutable_borrowing_example() {
    let mut text: String = String::from("Hello");
    let extension: &mut String = &mut text;

    extension.push_str(", World!!!");
    println!("{}", text);
}

fn slicing_example() {
    let message: String = String::from("Hello World");
    let slice: &str = &message[0..=4];
    println!("{}", slice);
}

pub fn ownership_facade() {
    println!("=== Borrowing ===");
    borrowing_example();

    println!("=== Mutable Borrowing ===");
    mutable_borrowing_example();

    println!("=== Slicing ===");
    slicing_example();
}
