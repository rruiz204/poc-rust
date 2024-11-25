#[derive(Debug)]
struct User {
    id: i32,
    username: String,
    email: String,
}

impl User {
    fn get_email(&self) -> &String {
        &self.email
    }
}

impl User {
    fn get_username(&self) -> &String {
        &self.username
    }
}

impl User {
    fn get_id(&self) -> &i32 {
        &self.id
    }
}

pub fn structures_facade() {
    println!("=== Structures ===");

    let change_email = |user: &mut User, email: String| user.email = email;

    let mut pato = User {
        id: 1,
        username: String::from("pato"),
        email: String::from("pato@example.com"),
    };

    change_email(&mut pato, String::from("pato213@example.com"));

    println!("The ID is: {}", pato.get_id());
    println!("The Username is: {}", pato.get_username());
    println!("The Email is: {}", pato.get_email());
}
