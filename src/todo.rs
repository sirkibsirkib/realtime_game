
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate mio;
extern crate mio_extras;
extern crate middleman;
extern crate clap;
extern crate rand;
extern crate ggez;
extern crate bitset;

use clap::App;
use std::{
	net::SocketAddr,
};

mod game;
use game::Moniker;

mod common;
mod server;
mod client;

fn main() {
	let matches = App::new("Realtime Game")
	        .version("1.0")
	        .author("C. Esterhuyse <christopher.esterhuyse@gmail.com>")
	        .about("A rust game designed to test clientside dead-reckoning and real-time synchronization.")
	        .args_from_usage("-m, --moniker=[CHAR] 'Choose a character-moniker for this game session. eg: `$`'
	                         <ip> 'Sets the bind/connect addr'")
	        .get_matches();
	        
    if let Some(ip) = matches.value_of("ip") {
        println!("Value for server: {}", ip);
    }

    let ip = matches.value_of("ip").unwrap();
	if let Ok(addr) = ip.parse::<SocketAddr>() {
		println!("ADDR {:?}", &addr);
		match matches.value_of("moniker") {
	    	Some(moniker) => {
	    		if moniker.len() != 1 {
	    			println!("You need to provide a 1-char ascii moniker!");
	    			return;
	    		}
	    		let my_moniker = Moniker(moniker.chars().next().unwrap());
	    		println!("Welcome, player `{}`.", my_moniker.0);
				client::client_enter(&addr, my_moniker);
	    	},
	    	None => server::server_enter(&addr),
	    };
	} else {
		println!("Couldn't parse ip string `{}`. Good example: `127.0.0.1:8000`", ip);
	}
}

