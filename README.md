# State Machine

## Description

A video guide on [`state machines`](https://www.youtube.com/watch?v=6XMg5csFccw&t=2s) is located here if you want to follow along!

## Starting off

Every new Turbo project begins with a `lib.rs`, a `GameState` struct, and an impl block that defines the `new()` and `update()` functions. 

To set up a simple state machine, create a new file named `state.rs` and add a `Screen` enum to it. 

You can create a new file directly from Visual Studio Code (or any editor you prefer).

<img width="351" height="407" alt="Screenshot 2025-09-20 at 2 15 03 PM" src="https://github.com/user-attachments/assets/bd17eaba-7d50-46af-b0c4-eda42cd423f6" />

Name the file `state.rs`. 

At the very top of `state.rs`, add the following line so it can reference items from lib.rs:

```rust
use crate::*;
```

Next, at the top of `lib.rs`, add these lines:

```rust
use state::*;
mod state;
```
`mod state;` tells Rust to include the `state.rs` module, and `use state::*;` brings its public items into scope. The word “state” is used because of the filename `state.rs`.

> [!TIP]
> When you add more files, repeat the same pattern in `lib.rs`:
>
> ```rust
> use state::*;
> mod state;
> use {file_name}::*; // <-- Add this for new files!
> mod {file_name}; // <-- Add this for new files!
> ```

## Adding fields to GameState

Next up, we are going to add a `screen` field to your `GameState` struct as well as a `frames` field. 

> The `frames` field is for for demo purposes, so you can skip it if you want.

```rust
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
        // This is where your main game loop code goes
        // The stuff in this block will run ~60x per sec
    }
}
```

We define `screen` as Screen and `frames` as u32.

We initialize `screen` as Screen::Title and `frames` as 0 in `GameState::new`.


Let's add the Screen `enum` to `state.rs`:

```rust
#[turbo::serialize] // add this above structs and enums

pub enum Screen {
    Title,
    Loading,
    Game,
}
```
Now make sure to save your project with Cmd+S or Ctrl+S

> [!TIP]
> Try to have your project running and open while programming in Turbo with: 
>
> ```bash
> turbo run -w
> ```
>
> One of Turbos biggest strengths is its hot reload to view saves and changes in real time!


## Initializing the state machine

Now that we have an `enum` for Screen and the `state.rs` file let's add the actual `state_of_game` function.
> **Tip** Make sure `state_of_game` is `pub` (public) so `lib.rs` can read it and use it!
```rust
pub fn state_of_game(state: &mut GameState) {
    match state.screen {
        Screen::Title => {

        }
        Screen::Loading => {

        }
        Screen::Game => {

        }
    }
}
```

Right now it is empty but we'll fix that as soon as we "turn it on" in our `GameState::update`, add this in `lib.rs`:

```rust
    pub fn update(&mut self) {
        state_of_game(self); //<-- add this!
        // This is where your main game loop code goes
        // The stuff in this block will run ~60x per sec
    }
```

Now you have a fully operational state machine! Let's add some substance to the `state_of_game` function as well as ways to switch between screens.

```rust
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
```

If you press `Z` on your keyboard or `A` on gamepad you'll switch from the Title screen to the Loading screen.

After 200 `frames` accumulate thanks to `state.frames += 1` you'll pass the Loading screen to the Game screen.

At the Game screen you can press `x` on your keyboard or `b` on gamepad to head back to the Title.

This is a simple and very modular way to setup up screen switching in your project and I highly encourage its use!

> [!TIP]
> Adding more variants to the Screen `enum` will give you more states to change to just make sure to add them to the `state_of_game` function or add a `_ => {}` as a placeholder to avoid errors or crashes!


## Ending Notes

State machines help you keep your code clean and have you returning to `state.rs` to expand your screens or fix screen-specific issues.

Staying organized is important when your project starts to scale and you'll thank yourself later when you can find and isolate where a crash/error exists.

Make sure to like and subscribe if you watched the video and join our [Discord](https://discord.gg/V5YWWvQvKW) for active help! 


