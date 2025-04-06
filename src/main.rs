use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Enigo, MouseButton, MouseControllable};
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
    println!("r - left click");
    println!("t - right click");
    println!("Hold 'w' with movement keys for precise 1px movement");
    println!("press q+backslash to exit");

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();

        // Only process keys if 'q' is being held
        if keys.contains(&Keycode::Q) {
            let move_amount = if keys.contains(&Keycode::W) {
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
                    Keycode::R => enigo.mouse_click(MouseButton::Left),
                    Keycode::T => enigo.mouse_click(MouseButton::Right),
                    Keycode::BackSlash => {
                        println!("Exiting...");
                        return;
                    }
                    _ => {}
                }
            }
        }

        thread::sleep(Duration::from_millis(10));
    }
}
