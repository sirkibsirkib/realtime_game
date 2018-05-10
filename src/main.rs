
extern crate rand;
extern crate ggez;
extern crate bitset;
extern crate fnv;
extern crate noise;
#[macro_use] extern crate lazy_static;

use fnv::FnvHashMap;
// use std::collections::HashMap;

mod game;
use game::room::*;
use game::*;

mod client;

fn main() {
	let seed = Seed {
	    step_size: 1.3,
	    x_trans: 1.04,
	    y_trans: 6.0,
	};
	let con_ind = RoomConInd::new(seed, FnvHashMap::default());
	let mut_ind = RoomMutInd::new();
	let room = Room::new(con_ind, mut_ind);
	client::client_go(room);
}