use tgr::{engine::*, node2d};
use crate::BLACK;

pub fn main() -> Node2d {
    node2d!(
        rect("buttom", 10000., 100., 0.).position(0., -50.).color(BLACK).keep(Keep::Down),
    )
}