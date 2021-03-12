extern crate serde;

use serde::{Deserialize, Serialize};
use serde_json::Serializer;
use std::collections::HashMap;
use std::io;

#[derive(Serialize, Deserialize)]
struct Player {
  location: String,
  items: Vec<String>,
  health: u32,
}
fn main() -> std::io::Result<()> {
  type RoomId = String; // each room has a unique name
  type RoomExits = Vec<(char, RoomId)>; // ...and a list of exits
  type RoomMap = HashMap<RoomId, RoomExits>; // room names and exits, simple
                                             // Create a simple map.
  let mut map = RoomMap::new();
  map.insert(
    "Cobble Crawl".to_string(),
    vec![('W', "Debris Room".to_string())],
  );
  map.insert(
    "Debris Room".to_string(),
    vec![
      ('E', "Cobble Crawl".to_string()),
      ('W', "Sloping Canyon".to_string()),
    ],
  );
  let mut serializer = Serializer::pretty(io::stdout());
  map.serialize(&mut serializer)?;
  let player = Player {
    location: "city".to_string(),
    items: vec!["a".to_string()],
    health: 33,
  };
  println!();
  player.serialize(&mut serializer)?;
  Ok(())
}
