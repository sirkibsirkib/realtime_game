pub mod room;

use fnv::FnvHashMap;
use bitset::BitSet;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct RoomId(u32);
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct PlayerId(u32);
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct MonsterId(u32);
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]

pub struct Player {
    coord: Coord2D,
}

pub struct Game {
    players: FnvHashMap<PlayerId, Player>,
} 


#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Coord2D {
    x: u16, y: u16,
}
impl Coord2D {
    pub fn new(x: u16, y: u16) -> Self {
        Coord2D { x: x, y: y, }
    }
}


pub struct BitGrid {
    w: usize,
    h: usize,
    bits: Vec<BitSet>,
}
impl BitGrid {
    pub fn new(w: usize, h: usize) -> Self {
        BitGrid {
            w: w,
            h: h,
            bits: (0..h).map(|_| BitSet::with_capacity(w)).collect(),
        }
    }

    pub fn set(&mut self, coord: Coord2D, value: bool) {
        self.bits[coord.y as usize].set(coord.x as usize, value);
    }

    pub fn test(&self, coord: Coord2D) -> bool {
        self.bits[coord.y as usize].test(coord.x as usize)
    }
}