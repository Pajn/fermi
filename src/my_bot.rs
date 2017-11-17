extern crate pathfinding;
extern crate rand;
extern crate rusty_machine;
#[macro_use] extern crate text_io;

//Notice: due to Rust's extreme dislike of (even private!) global mutables, we do not reset the production values of each tile during get_frame.
//If you change them, you may not be able to recover the actual production values of the map, so we recommend not editing them.
//However, if your code calls for it, you're welcome to edit the production values of the sites of the map - just do so at your own risk.

mod hlt;
mod helpers;
use helpers::*;
use hlt::{ networking, types }; 
use std::collections::HashMap; 
use rand::Rng;
use std::iter::Iterator;
use rusty_machine::linalg::{BaseMatrix};
use std::io::prelude::*;
    use std::fs::File;

fn main() {
    let (my_id, mut game_map) = networking::get_init();
    let mut rng = rand::thread_rng();
    networking::send_init(format!("{}", "Fermi"));  

let mut file = File::create("test.debug").unwrap();
file.write_fmt(format_args!("{:?}", "test")).unwrap();
//     let distance = calculate_distance(&game_map); 
// let mut file = File::create("distance.debug").unwrap();
// file.write_fmt(format_args!("{:?}", distance.get_row(0))).unwrap();

    loop { 
        networking::get_frame(&mut game_map);

    let edges = game_map.contents.iter().flat_map(|row| row.iter().filter(|site| site.owner != my_id && game_map.get_neighbour_sites(site.location).iter().filter(|n| n.owner == my_id).count() > 0   ));



        let mut moves = HashMap::new();
        for a in 0..game_map.height {
            for b in 0..game_map.width {
                let l = hlt::types::Location { x: b, y: a };
                if game_map.get_site(l, types::STILL).owner == my_id {
                    moves.insert(l, (rng.gen::<u32>() % 5) as u8);
                }
            }
        } 
        networking::send_frame(moves);
    }
}