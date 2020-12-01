mod action_client;
mod keymapping;

use std::{fs, thread, time, env};
use gilrs::{Event, Gilrs};
use serde_yaml;
use action_client::Direction;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>>  {
    let exec = env::args().nth(0).unwrap();
    let filename = env::args().nth(1);
    if filename.is_none() {
        println!("USAGE: {} <configuration-file>", exec);
        std::process::exit(0);
    }
    let conf_content = fs::read_to_string(filename.unwrap()).expect("Failed reading the file");
    let conf: keymapping::KeyMapping = serde_yaml::from_str(&conf_content)?;
    
    let mut gilrs = Gilrs::new().unwrap();

    let mut client = action_client::ActionClient::default();

    let pause = time::Duration::from_millis(5);
    loop {
        while let Some(Event { event, .. }) = gilrs.next_event() {
            //println!("{:?} New event from {}: {:?}", time, id, event);
            match event {
                gilrs::EventType::ButtonPressed(button, _) => {
                    if let Some(action) = conf.buttons.get(&button) {
                        client.perform_action(*action, Direction::Down,  None);
                    }
                }
                gilrs::EventType::ButtonReleased(button, _) => {
                    if let Some(action) = conf.buttons.get(&button) {
                        client.perform_action(*action, Direction::Up,  None);
                    }
                }
                gilrs::EventType::AxisChanged(axis, amount, _) => {
                    if let Some([negative_action, positive_action]) = conf.axis.get(&axis) {
                            let negative_direction = if amount >= -0.3_f32 { Direction::Up } else { Direction::Down };
                            let positive_direction = if amount <= 0.3_f32 { Direction::Up } else { Direction::Down };
                            client.perform_action(*negative_action, negative_direction, Some(amount));
                            client.perform_action(*positive_action, positive_direction,  Some(amount));
                    }
                }
                gilrs::EventType::ButtonRepeated(_, _) => {}
                gilrs::EventType::ButtonChanged(_, _, _) => {}
                gilrs::EventType::Connected => {}
                gilrs::EventType::Disconnected => {}
                gilrs::EventType::Dropped => {}
            }
        }
        client.exec_mouse_loop();
        thread::sleep(pause);
    }
}