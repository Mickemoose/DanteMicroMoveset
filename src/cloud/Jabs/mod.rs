mod jab1;
mod jab2;
mod jab3;

use crate::MARKED_COLORS;

pub fn install() {
    jab1::install();
    jab2::install();
    jab3::install();
}