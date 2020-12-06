use crate::vent::{select_vent, VentAction};
use enigo::*;
use joystick_mapper_lib::ActionClient;
use joystick_mapper_lib::{Action, InputState, JoystickClient, MappingConfiguration};
use joystick_mapper_lib::{Key, MouseAction};
use scrap::*;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::io::ErrorKind::WouldBlock;
use std::{env, fs, thread, time};

mod vent;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
enum KeyMouseVentAction {
    Key(Key),
    Layout(char),
    Mouse(MouseAction),
    Vent(VentAction),
}

impl Action<Capturer> for VentAction {
    fn perform_action(
        &self,
        client: &mut ActionClient<Capturer>,
        input_state: InputState,
        _amount: Option<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if input_state == InputState::Up {
            return Ok(());
        }
        let (w, h) = (client.state.width(), client.state.height());
        loop {
            let buffer = match client.state.frame() {
                Ok(buffer) => buffer,
                Err(error) => {
                    if error.kind() == WouldBlock {
                        thread::sleep(time::Duration::new(1, 0) / 60);
                        continue;
                    } else {
                        panic!("Error: {}", error);
                    }
                }
            };

            // there are maximum 3 vent direction in game
            let max_vents = 3_u8;
            let vent = select_vent(&buffer, w as i32, h as i32, max_vents, self)?;
            if let Some(vent) = vent {
                client.enigo.mouse_move_to(vent.0, vent.1);
                client.enigo.mouse_click(MouseButton::Left);
            }
            break;
        }
        Ok(())
    }
}

impl Action<Capturer> for KeyMouseVentAction {
    fn perform_action(
        &self,
        client: &mut ActionClient<Capturer>,
        input_state: InputState,
        amount: Option<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            KeyMouseVentAction::Key(key) => key.perform_action(client, input_state, amount),
            KeyMouseVentAction::Layout(c) => {
                Key::Layout(*c).perform_action(client, input_state, amount)
            }
            KeyMouseVentAction::Mouse(mouse) => mouse.perform_action(client, input_state, amount),
            KeyMouseVentAction::Vent(vent) => vent.perform_action(client, input_state, amount),
        }
    }
}

fn print_gamepads(client: &JoystickClient<KeyMouseVentAction, Capturer>) {
    let gamepads = client.gamepads();
    println!(
        "Found {} joystick{}",
        gamepads.len(),
        if gamepads.len() == 1 { "" } else { "s" }
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = env::args()
        .nth(1)
        .unwrap_or("joystick-mapper.conf".to_string());
    let conf_content = fs::read_to_string(filename).expect("Failed reading the file");
    let conf: MappingConfiguration<KeyMouseVentAction> = serde_yaml::from_str(&conf_content)?;
    let display = Display::primary()?;
    let capturer = Capturer::new(display)?;
    let mut joystick_client: JoystickClient<KeyMouseVentAction, Capturer> =
        JoystickClient::new(conf, capturer);
    let pause = time::Duration::from_millis(15);
    print_gamepads(&joystick_client);
    let on_connected = || {
        println!("New joystick connected!");
    };
    let on_disconnected = || {
        println!("Joystick disconnected!");
    };
    loop {
        joystick_client.exec_event_loop(Some(&on_connected), Some(&on_disconnected))?;
        thread::sleep(pause);
    }
}
