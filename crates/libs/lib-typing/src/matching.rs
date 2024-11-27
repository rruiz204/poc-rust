struct User {
  id: i32,
  user: String,
}

fn get_user(user_id: i32, users: Vec<User>) -> Option<User> {      
  users.into_iter().find(|user| user.id == user_id)
}

pub fn matching_showcase() {
  println!("=== Matching ===");
  
  let users: Vec<User> = vec![
    User { id: 1, user: String::from("user1") },
    User { id: 2, user: String::from("user2") },
    User { id: 3, user: String::from("user3") },
  ];

  let user = get_user(4, users);

  if let Some(value) = user {
    println!("ID: {} User: {}", value.id, value.user);
  } else {
    println!("User not found");
  }
}
