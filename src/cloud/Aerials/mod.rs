mod dair;
mod nair;
mod uair;

use crate::MARKED_COLORS;

pub fn install() {
    dair::install();
    nair::install();
    uair::install();
}