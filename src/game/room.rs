use ggez::graphics::{
    self,
    spritebatch::SpriteBatch,
};
use fnv::FnvHashMap;
use noise::{
    NoiseFn,
    Perlin,
};
use super::*;


pub struct Seed {
    pub step_size: f64,
    pub x_trans: f64,
    pub y_trans: f64,
}

pub struct RoomConInd {
    doors: FnvHashMap<Coord2D, RoomId>,
    seed: Seed,
}
impl RoomConInd {
    pub fn new(seed: Seed, doors: FnvHashMap<Coord2D, RoomId>) -> Self {
        RoomConInd {
            doors: doors,
            seed: seed,
        }
    }
}
pub struct RoomConDep {
    default_rocks: BitGrid,
}
impl RoomConDep {
    fn derive(con_ind: &RoomConInd) -> Self {
        let mut default_rocks = BitGrid::new(Room::ROOM_WIDTH as usize, Room::ROOM_HEIGHT as usize);
        let mut n = Perlin::new();
        for y in 0..Room::ROOM_HEIGHT {
            let nx = (y as f64) * con_ind.seed.step_size + con_ind.seed.x_trans;
            for x in 0..Room::ROOM_WIDTH {
                let ny = (y as f64) * con_ind.seed.step_size + con_ind.seed.y_trans;
                if n.get([nx, ny]) > 0.2 {
                    let coord = Coord2D::new(x, y);
                    default_rocks.set(coord, true);
                }
            }
        }
        RoomConDep {
            default_rocks: default_rocks,
        }
    }
}
pub struct RoomMutInd {
    players: FnvHashMap<PlayerId, Coord2D>,
    override_rocks: FnvHashMap<Coord2D, bool>,
}
impl RoomMutInd {
    pub fn new() -> Self {
        RoomMutInd {
            players: FnvHashMap::default(),
            override_rocks: FnvHashMap::default(),
        }
    }
}
pub struct RoomMutDep {
    //sprite batch
}
impl RoomMutDep {
    fn derive(con_ind: &RoomConInd, con_dep: &RoomConDep, mut_ind: &RoomMutInd) -> Self {
        RoomMutDep {

        }
    }
}

pub struct Room {
    con_ind: RoomConInd,
    con_dep: RoomConDep,
    mut_ind: RoomMutInd,
    mut_dep: RoomMutDep,
}

impl Room {
    pub const ROOM_WIDTH: u16 = 40;
    pub const ROOM_HEIGHT: u16 = 30;

    pub fn new(con_ind: RoomConInd, mut_ind: RoomMutInd) -> Self {
        let con_dep = RoomConDep::derive(&con_ind);
        let mut_dep = RoomMutDep::derive(&con_ind, &con_dep, &mut_ind);
        Room {
            con_ind: con_ind,
            con_dep: con_dep,
            mut_ind: mut_ind,
            mut_dep: mut_dep,
        }
    }
    pub fn iter_walls(&mut self, )

    pub fn update(&mut self, mut_ind: RoomMutInd) {
        unimplemented!()
    }
}

pub struct CoordIter {
    w: usize,
    h: usize,
    next: Coord2D,
}