use enigo::*;
use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};

pub trait Action<S> {
    fn perform_action(
        &self,
        client: &mut ActionClient<S>,
        input_state: InputState,
        amount: Option<f32>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum MouseAction {
    MouseX,
    MouseY,
    MouseButton(MouseButton),
}

impl <S> Action<S> for MouseAction {
    fn perform_action(
        &self,
        client: &mut ActionClient<S>,
        input_state: InputState,
        amount: Option<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let amount = amount.unwrap_or(1.0);
        match &self {
            MouseAction::MouseX => client.mouse_axis_state[0] = amount,
            MouseAction::MouseY => client.mouse_axis_state[1] = amount,
            MouseAction::MouseButton(mouse_button) => match input_state {
                InputState::Up => client.enigo.mouse_up(*mouse_button),
                InputState::Down => client.enigo.mouse_down(*mouse_button),
            },
        }
        Ok(())
    }
}

impl <S> Action<S> for Key {
    fn perform_action(
        &self,
        client: &mut ActionClient<S>,
        input_state: InputState,
        amount: Option<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(_amount) = amount {
            let action_state = client.axis_key_state.get_mut(self);
            if let Some(action_state) = action_state {
                if *action_state == input_state {
                    return Ok(());
                } else {
                    *action_state = input_state;
                }
            } else {
                client.axis_key_state.insert(*self, input_state);
            }
        }
        match input_state {
            InputState::Up => client.enigo.key_up(*self),
            InputState::Down => client.enigo.key_down(*self),
        };
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputState {
    Up,
    Down,
}
pub struct ActionClient<S> {
    pub enigo: Enigo,
    pub state: S,
    axis_key_state: FxHashMap<Key, InputState>,
    mouse_axis_state: [f32; 2],
}

impl <S> ActionClient<S> {
    pub fn new(state: S) -> Self {
        let enigo = Enigo::new();
        let axis_key_state = FxHashMap::default();

        ActionClient {
            enigo,
            state,
            axis_key_state,
            mouse_axis_state: [0_f32, 0_f32],
        }
    }

    pub fn perform_action<A: Action<S>>(
        &mut self,
        action: &A,
        input_state: InputState,
        amount: Option<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        action.perform_action(self, input_state, amount)
    }
    pub fn exec_mouse_loop(&mut self) {
        let [x_amount, y_amount] = self.mouse_axis_state;
        if x_amount != 0_f32 && y_amount != 0_f32 {
            self.enigo.mouse_move_relative(
                (x_amount * 20_f32.round()) as i32,
                (y_amount * -20_f32.round()) as i32,
            );
        }
    }
}
