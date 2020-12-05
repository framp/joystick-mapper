use joystick_mapper_lib::ActionClient;
use joystick_mapper_lib::{Action, InputState, JoystickClient, KeyMapping};
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

impl Action for KeyMouseAction {
    fn perform_action(
        &self,
        client: &mut ActionClient,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exec = env::args().nth(0).unwrap();
    let filename = env::args().nth(1);
    if filename.is_none() {
        println!("USAGE: {} <configuration-file>", exec);
        std::process::exit(0);
    }
    let conf_content = fs::read_to_string(filename.unwrap()).expect("Failed reading the file");
    let conf: KeyMapping<KeyMouseAction> = serde_yaml::from_str(&conf_content)?;

    let mut joystick_client: JoystickClient<KeyMouseAction> = JoystickClient::new(conf);
    let pause = time::Duration::from_millis(15);
    loop {
        joystick_client.exec_event_loop()?;
        thread::sleep(pause);
    }
}
