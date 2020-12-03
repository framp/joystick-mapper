use enigo::*;
use fxhash::FxHashMap;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum MouseAction {
    MouseX,
    MouseY,
    MouseButton(MouseButton)
}
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Action {
    Key(Key),
    Layout(char),
    Mouse(MouseAction),
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up, 
    Down
}
pub struct ActionClient {
    enigo: Enigo,
    axis_key_state: FxHashMap<Action, Direction>,
    mouse_axis_state: [f32; 2] 
}

impl Default for ActionClient {
    fn default() -> Self { 
        let enigo = Enigo::new();
        let axis_key_state = FxHashMap::default();

        ActionClient {
            enigo,
            axis_key_state,
            mouse_axis_state: [0_f32, 0_f32],
        }
    }
}

impl ActionClient {
    fn perform_mouse_action(&mut self, mouse: MouseAction, _action: Action, direction: Direction, amount: Option<f32>) {
        let amount = amount.unwrap_or(1.0);
        match mouse {
            MouseAction::MouseX => { 
                self.mouse_axis_state[0] = amount
            }
            MouseAction::MouseY => { 
                self.mouse_axis_state[1] = amount
            }
            MouseAction::MouseButton(mouse_button) => {
                match direction {
                    Direction::Up => { self.enigo.mouse_up(mouse_button) }
                    Direction::Down => { self.enigo.mouse_down(mouse_button) }
                }
            }
        }
    }
    fn perform_key_action(&mut self, key: Key, action: Action, direction: Direction, amount: Option<f32>) {
        if let Some(_amount) = amount {
            let action_state = self.axis_key_state.get_mut(&action);
            if let Some(action_state) = action_state {
                if *action_state == direction {
                    return;
                } else {
                    *action_state = direction;
                }
            } else {
                self.axis_key_state.insert(action, direction);
            }
        }
        match direction {
            Direction::Up => { self.enigo.key_up(key) }
            Direction::Down => { self.enigo.key_down(key) }
        };
    }
    pub fn perform_action(&mut self, action: Action, direction: Direction, amount: Option<f32>) {
        let (key, mouse) = match action {
            Action::Key(key) => { (Some(key), None) }
            Action::Layout(char) => {(Some(Key::Layout(char)), None) }
            Action::Mouse(mouse ) => { (None, Some(mouse)) }
        };
        if let Some(mouse) = mouse {
            self.perform_mouse_action(mouse, action, direction, amount);
        }
        if let Some(key) = key {
            self.perform_key_action(key, action, direction, amount);
        }
    }
    pub fn exec_mouse_loop(&mut self) {
        let [x_amount, y_amount] = self.mouse_axis_state;
        if x_amount != 0_f32 && y_amount != 0_f32 {
            self.enigo.mouse_move_relative((x_amount*10_f32.round()) as i32, (y_amount*-10_f32.round()) as i32);
        }
    }
}