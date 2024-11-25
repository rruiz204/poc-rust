mod enums;
mod matching;

use crate::enums::enums_showcase;
use crate::matching::matching_showcase;

pub fn typing_facade() {
    enums_showcase();
    matching_showcase();
}