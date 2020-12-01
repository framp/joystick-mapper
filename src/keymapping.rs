use serde::{Serialize, Deserialize};
use fxhash::FxHashMap;
use gilrs::{Axis, Button};
use crate::action_client::Action;

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyMapping {
    pub buttons: FxHashMap<Button, Action>,
    pub axis: FxHashMap<Axis, [Action; 2]>
}