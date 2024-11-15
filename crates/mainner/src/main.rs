use lib_variables::greeting;
use lib_functions::*;

fn main() {
    greeting("Morderkay");

    let result: i32 = sum(10, 20);
    println!("The reuslt is: {}", result);
}
