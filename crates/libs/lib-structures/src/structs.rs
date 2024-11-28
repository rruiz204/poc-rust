struct User {
  id: i32,
  name: String,
  email: String
}

impl User {    
  fn get_name(&self) -> &str {
    &self.name
  }

  fn get_email(&self) -> &str {
    &self.email
  }

  fn change_email(&mut self, new_email: String) {
    self.email = new_email;
  }
}

pub fn structs_showcase() {
  let mut pato: User = User { id: 1, name: String::from("pato"), email: String::from("pato@example.com") };
  println!("ID: {}, Name: {}, Email: {}", pato.id, pato.get_name(), pato.get_email());

  pato.change_email(String::from("pato213@example.com"));
  println!("ID: {}, Name: {}, Email: {}", pato.id, pato.get_name(), pato.get_email());
}