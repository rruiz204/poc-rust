mod structs;
mod traits;
mod lifetimes;

use crate::structs::structs_showcase;
use crate::traits::traits_showcase;
use crate::lifetimes::lifetimes_showcase;

pub fn structures_facade() {
    println!("=== Structures ===");
    structs_showcase();
    traits_showcase();
    lifetimes_showcase();
}
