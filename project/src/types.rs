use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Args {
    pub my_map: [[i32;10];10],
	pub start_loc: [[i32;2];1],
	pub goal_loc: [[i32;2];1],
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Result {
    pub result: Option<Vec<Vec<Vec<i32>>>>,
}
