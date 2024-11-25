use lib_ownership::ownership_facade;
use lib_functions::functions_facade;
use lib_variables::variables_facade;
use lib_structures::structures_facade;

fn main() {
    println!("Starting...");
    variables_facade();
    functions_facade();
    ownership_facade();
    structures_facade();
}
