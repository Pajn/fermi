#![allow(unused)]

use hlt::types;
use std::io;
use std::collections::HashMap;
use std::io::Write;
use std::str::FromStr;

//Persistant between moves, that way if the user screws up the map it won't persist.
static mut _WIDTH: u16 = 0;
static mut _HEIGHT: u16 = 0;

fn serialize_move_set(moves: HashMap<types::Location, u8>) -> String {
  let mut s: String = String::new();
  for (l, d) in moves {
    s = format!("{}{} {} {} ", s, l.x, l.y, d);
  }
  s
}

fn deserialize_map_size(s: String) -> () {
  let splt: Vec<&str> = s.split(" ").collect();
  unsafe {
    _WIDTH = u16::from_str(splt[0]).unwrap();
    _HEIGHT = u16::from_str(splt[1]).unwrap();
  }
}

fn deserialize_productions(player_tag: u8, s: String) -> types::GameMap {
  let splt: Vec<&str> = s.split(" ").collect();
  let mut gmp = types::GameMap {
    width: 0,
    height: 0,
    contents: Vec::new(),
    my_id: player_tag,
  };
  unsafe {
    gmp.width = _WIDTH;
    gmp.height = _HEIGHT;
  }
  gmp.contents.resize(gmp.height as usize, Vec::new());
  let mut loc = 0;
  let mut y = 0;
  for v in &mut gmp.contents {
    for x in 0..gmp.width {
      v.push(types::Site {
        owner: 0,
        strength: 0,
        production: u8::from_str(splt[loc]).unwrap(),
        location: types::Location { x, y },
      });
      loc += 1;
    }
    y += 1;
  }
  gmp
}

fn deserialize_map(s: String, gmp: &mut types::GameMap) -> () {
  let splt: Vec<&str> = s.split(" ").collect();
  unsafe {
    let mut counter = 0;
    let mut owner = 0;
    let mut loc: usize = 0;
    for a in 0.._HEIGHT {
      for b in 0.._WIDTH {
        if counter == 0 {
          counter = u16::from_str(splt[loc]).unwrap();
          loc += 1;
          owner = u8::from_str(splt[loc]).unwrap();
          loc += 1;
        }
        gmp
          .get_site_mut(types::Location { x: b, y: a }, types::STILL)
          .owner = owner;
        counter -= 1;
      }
    }
    for a in 0.._HEIGHT {
      for b in 0.._WIDTH {
        gmp
          .get_site_mut(types::Location { x: b, y: a }, types::STILL)
          .strength = u8::from_str(splt[loc]).unwrap();
        loc += 1;
      }
    }
  }
}


fn send_string(s: String) -> () {
  println!("{}", s);
  io::stdout().flush();
}

fn get_string() -> String {
  read!("{}\n")
}

pub fn get_init() -> (u8, types::GameMap) {
  let player_tag: u8 = u8::from_str(&get_string()).unwrap();
  deserialize_map_size(get_string());
  let mut gmp = deserialize_productions(player_tag, get_string());
  deserialize_map(get_string(), &mut gmp);
  (player_tag, gmp)
}

pub fn send_init(name: String) -> () {
  send_string(name);
}

pub fn get_frame(gmp: &mut types::GameMap) -> () {
  deserialize_map(get_string(), gmp);
}

pub fn send_frame(moves: HashMap<types::Location, u8>) -> () {
  send_string(serialize_move_set(moves));
}
