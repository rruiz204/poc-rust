mod structs;
mod traits;

use crate::structs::structs_showcase;
use crate::traits::traits_showcase;

pub fn structures_facade() {
    println!("=== Structures ===");
    structs_showcase();
    traits_showcase();
}
