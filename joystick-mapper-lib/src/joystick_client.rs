use crate::action_client::{Action, ActionClient, InputState};

use fxhash::FxHashMap;
use gilrs::{Axis, Button, Event, Gilrs};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyMapping<A> {
    pub buttons: FxHashMap<Button, A>,
    pub axis: FxHashMap<Axis, [A; 2]>,
}

pub struct JoystickClient<A: Action<S>, S> {
    gilrs: Gilrs,
    keymapping: KeyMapping<A>,
    pub action_client: ActionClient<S>, // TODO remove pub
    axis_sensitivity: f32,
}

impl<A: Action<S>, S> JoystickClient<A, S> {
    pub fn new(keymapping: KeyMapping<A>, state: S) -> JoystickClient<A, S> {
        let gilrs = Gilrs::new().unwrap();
        let axis_sensitivity = 0.3_f32;
        let action_client = ActionClient::new(state);
        JoystickClient {
            gilrs,
            keymapping,
            action_client,
            axis_sensitivity,
        }
    }

    pub fn axis_sensitivity(mut self, axis_sensitivity: f32) -> Self {
        self.axis_sensitivity = axis_sensitivity;
        self
    }

    pub fn exec_event_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(Event { event, .. }) = self.gilrs.next_event() {
            match event {
                gilrs::EventType::ButtonPressed(button, _) => {
                    if let Some(action) = self.keymapping.buttons.get(&button) {
                        self.action_client
                            .perform_action(action, InputState::Down, None)?;
                    }
                }
                gilrs::EventType::ButtonReleased(button, _) => {
                    if let Some(action) = self.keymapping.buttons.get(&button) {
                        self.action_client
                            .perform_action(action, InputState::Up, None)?;
                    }
                }
                gilrs::EventType::AxisChanged(axis, amount, _) => {
                    if let Some([negative_action, positive_action]) =
                        self.keymapping.axis.get(&axis)
                    {
                        let negative_input_state = if amount >= -self.axis_sensitivity {
                            InputState::Up
                        } else {
                            InputState::Down
                        };
                        let positive_input_state = if amount <= self.axis_sensitivity {
                            InputState::Up
                        } else {
                            InputState::Down
                        };
                        self.action_client.perform_action(
                            negative_action,
                            negative_input_state,
                            Some(amount),
                        )?;
                        self.action_client.perform_action(
                            positive_action,
                            positive_input_state,
                            Some(amount),
                        )?;
                    }
                }
                gilrs::EventType::ButtonRepeated(_, _) => {}
                gilrs::EventType::ButtonChanged(_, _, _) => {}
                gilrs::EventType::Connected => {}
                gilrs::EventType::Disconnected => {}
                gilrs::EventType::Dropped => {}
            }
        }
        self.action_client.exec_mouse_loop();
        Ok(())
    }
}
