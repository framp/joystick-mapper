use joystick_mapper_lib::ActionClient;
use joystick_mapper_lib::{Action, InputState, JoystickClient, MappingConfiguration};
use joystick_mapper_lib::{Key, MouseAction};

use serde::{Deserialize, Serialize};

use serde_yaml;
use std::{env, fs, thread, time};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum KeyMouseAction {
    Key(Key),
    Layout(char),
    Mouse(MouseAction),
}

impl Action<()> for KeyMouseAction {
    fn perform_action(
        &self,
        client: &mut ActionClient<()>,
        input_state: InputState,
        amount: Option<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            KeyMouseAction::Key(key) => key.perform_action(client, input_state, amount),
            KeyMouseAction::Layout(c) => {
                Key::Layout(*c).perform_action(client, input_state, amount)
            }
            KeyMouseAction::Mouse(mouse) => mouse.perform_action(client, input_state, amount),
        }
    }
}

fn print_gamepads(client: &JoystickClient<KeyMouseAction, ()>) {
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
    let conf: MappingConfiguration<KeyMouseAction> = serde_yaml::from_str(&conf_content)?;
    let mut joystick_client: JoystickClient<KeyMouseAction, ()> = JoystickClient::new(conf, ());
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
