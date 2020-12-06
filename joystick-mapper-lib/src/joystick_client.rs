use crate::action_client::{Action, ActionClient, InputState};

use fxhash::FxHashMap;
use gilrs::{Axis, Button, Event, Gamepad, GamepadId, Gilrs};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MappingConfiguration<A> {
    pub buttons: FxHashMap<Button, A>,
    pub axis: FxHashMap<Axis, [A; 2]>,
    pub axis_sensitivity: Option<f32>,
    pub mouse_speed: Option<f32>,
}

pub struct JoystickClient<A: Action<S>, S> {
    gilrs: Gilrs,
    configuration: MappingConfiguration<A>,
    action_client: ActionClient<S>,
}

impl<A: Action<S>, S> JoystickClient<A, S> {
    pub fn new(configuration: MappingConfiguration<A>, state: S) -> JoystickClient<A, S> {
        let gilrs = Gilrs::new().unwrap();
        let mouse_speed = configuration.mouse_speed.unwrap_or(20.0);
        let action_client = ActionClient::new(state, mouse_speed);
        JoystickClient {
            gilrs,
            configuration,
            action_client,
        }
    }

    pub fn gamepads(&self) -> Vec<(GamepadId, Gamepad)> {
        self.gilrs.gamepads().collect::<Vec<(GamepadId, Gamepad)>>()
    }

    pub fn exec_event_loop(
        &mut self,
        on_connected: Option<&dyn Fn() -> ()>,
        on_disconnected: Option<&dyn Fn() -> ()>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(Event { event, .. }) = self.gilrs.next_event() {
            match event {
                gilrs::EventType::ButtonPressed(button, _) => {
                    if let Some(action) = self.configuration.buttons.get(&button) {
                        self.action_client
                            .perform_action(action, InputState::Down, None)?;
                    }
                }
                gilrs::EventType::ButtonReleased(button, _) => {
                    if let Some(action) = self.configuration.buttons.get(&button) {
                        self.action_client
                            .perform_action(action, InputState::Up, None)?;
                    }
                }
                gilrs::EventType::AxisChanged(axis, amount, _) => {
                    if let Some([negative_action, positive_action]) =
                        self.configuration.axis.get(&axis)
                    {
                        let axis_sensitivity =
                            self.configuration.axis_sensitivity.unwrap_or(0.3_f32);
                        let negative_input_state = if amount >= -axis_sensitivity {
                            InputState::Up
                        } else {
                            InputState::Down
                        };
                        let positive_input_state = if amount <= axis_sensitivity {
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
                gilrs::EventType::Connected => {
                    on_connected.and_then(|cb| Some(cb())).unwrap_or(())
                }
                gilrs::EventType::Disconnected => {
                    on_disconnected.and_then(|cb| Some(cb())).unwrap_or(())
                }
                gilrs::EventType::Dropped => {}
            }
        }
        self.action_client.exec_mouse_loop();
        Ok(())
    }
}
