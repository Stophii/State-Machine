use turbo::*;

use state::*;
mod state;

#[turbo::game]
struct GameState {
    screen: Screen,
    frames: u32,
}

impl GameState {
    pub fn new() -> Self {
        // initialize your game state
        Self { 
            screen: Screen::Title,
            frames: 0,  
        }
    }
    pub fn update(&mut self) {
        state_of_game(self);
        // This is where your main game loop code goes
        // The stuff in this block will run ~60x per sec
    }
}