mod enums;
mod matching;
mod generics;

use crate::enums::enums_showcase;
use crate::matching::matching_showcase;
use crate::generics::generics_showcase;

pub fn typing_facade() {
    enums_showcase();
    matching_showcase();
    generics_showcase();
}