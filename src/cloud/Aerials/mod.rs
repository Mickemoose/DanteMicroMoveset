mod dair;
mod nair;
mod uair;

use crate::MARKED_COLORS;

pub fn install(agent: &mut smashline::Agent) {
    dair::install(agent);
    nair::install(agent);
    uair::install(agent);
}