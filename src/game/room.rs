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
use std::iter::Iterator;


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
        let mut default_rocks = BitGrid::new(Room::WIDTH as usize, Room::HEIGHT as usize);
        let mut n = Perlin::new();
        for y in 0..Room::HEIGHT {
            let nx = (y as f64) * con_ind.seed.step_size + con_ind.seed.x_trans;
            for x in 0..Room::WIDTH {
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
    pub const WIDTH: u16 = 40;
    pub const HEIGHT: u16 = 30;

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

    pub fn wall_at(&self, coord: Coord2D) -> bool {
        self.mut_ind.override_rocks.get(&coord)
        .map(|x| *x)
        .unwrap_or_else(|| {
            self.con_dep.default_rocks.test(coord)
        })
    }

    pub fn iter_walls(&self) -> impl Iterator<Item=Coord2D> {
        CoordIter::new::<_>(
            Self::WIDTH as usize,
            Self::HEIGHT as usize,
            |coord| {
                self.wall_at(coord)
            },
        )
    }

    pub fn update(&mut self, mut_ind: RoomMutInd) {
        unimplemented!()
    }
}

struct CoordIter<F: Fn(Coord2D)->bool> {
    w: usize,
    h: usize,
    next: Coord2D,
    filter_func: F,
}

impl<F> CoordIter<F> where F: Fn(Coord2D) -> bool {
    pub fn new(w: usize, h: usize, filter_func: F {
        CoordIter {
            h: h,
            w: w,
            next: Coord2D::new(0, 0),
            filter_func, filter_func,
        }
    }
}

impl Iterator for CoordIter { 
    type Item = Coord2D;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.next.x >= Room::WIDTH {
                self.next.y += 1;
                if self.next.y >= Room::HEIGHT {
                    return None;
                }
                self.next.x = 0;
            }
            let was = self.next;
            self.next = Coord2D::new(was.x+1, was.y);
            if self.filter_func(was) {
                return Some(was);
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let x = self.w * self.h;
        (x, Some(x))
    }
}