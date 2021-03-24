type PlayerId = u32;
const GAME_SIZE: usize = 8;
type WaitingList = Vec<PlayerId>;

use std::sync::Arc;
use std::sync::Mutex;

struct FernEmpireApp {
  waiting_list: Mutex<WaitingList>,
}

impl FernEmpireApp {
  fn join_waiting_list(&self, player: PlayerId) {
    let mut guard = self.waiting_list.lock().unwrap();
    guard.push(player);
    if guard.len() == GAME_SIZE {
      let players = guard.split_off(0);
      drop(guard);
      self.start_game(players);
    }
  }
  
  fn start_game(&self, players: Vec<PlayerId>) {
    
  }
}
fn main() {
  let app = Arc::new(FernEmpireApp {
    waiting_list: Mutex::new(vec![]),
  });
}
