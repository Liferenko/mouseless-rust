use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Enigo, MouseControllable, MouseButton};
use std::{thread, time::Duration};

const NORMAL_MOVE: i32 = 15;
const PRECISE_MOVE: i32 = 2;

fn main() {
    let mut enigo = Enigo::new();
    let device_state = DeviceState::new();

    println!("Mouseless started!");
    println!("Hold 'q' and use:");
    println!("y - move left");
    println!("u - move down");
    println!("i - move up");
    println!("o - move right");
    println!("c - left click");
    println!("v - right click");
    println!("Hold 'e' with movement keys for precise 1px movement");
    println!("Press ` to exit");

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        
        // Only process keys if 'q' is being held
        if keys.contains(&Keycode::Q) {
            let move_amount = if keys.contains(&Keycode::E) {
                PRECISE_MOVE
            } else {
                NORMAL_MOVE
            };

            for key in keys.iter() {
                match key {
                    Keycode::Y => enigo.mouse_move_relative(-move_amount, 0),
                    Keycode::U => enigo.mouse_move_relative(0, move_amount),
                    Keycode::I => enigo.mouse_move_relative(0, -move_amount),
                    Keycode::O => enigo.mouse_move_relative(move_amount, 0),
                    Keycode::C => enigo.mouse_click(MouseButton::Left),
                    Keycode::V => enigo.mouse_click(MouseButton::Right),
                    _ => {}
                }
            }
        }

        // Always check for escape, regardless of 'q' state
        if keys.contains(&Keycode::Escape) {
            println!("Exiting...");
            return;
        }

        thread::sleep(Duration::from_millis(10));
    }
}
