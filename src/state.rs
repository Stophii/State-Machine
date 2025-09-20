//state.rs

use crate::*;

#[turbo::serialize]

pub enum Screen {
    Title,
    Loading,
    Game,
}

pub fn state_of_game(state: &mut GameState) {
    match state.screen {
        Screen::Title => {

            text!("Title Screen!", x = 100, y = 60);
            text!("Press z to begin!", x = 100, y = 90);
            if gamepad::get(0).a.just_pressed() {
                state.screen = Screen::Loading
            }

        }
        Screen::Loading => {

            if state.frames >= 150 {
                text!("Loading...", x = 100, y = 90);

            } else if state.frames >= 75 {
                text!("Loading..", x = 100, y = 90);

            } else {
                text!("Loading.", x = 100, y = 90);
            }

            state.frames += 1;

            if state.frames >= 200 {
                state.screen = Screen::Game;
                state.frames = 0;
            }
        }
        Screen::Game => {

            text!("Hello, world!!!", x = 100, y = 60);
            if gamepad::get(0).b.just_pressed() {
                state.screen = Screen::Title
            }
            
        }
    }
}

