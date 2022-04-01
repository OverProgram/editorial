use crate::style::Style;
use crate::utils::{AABB};

pub struct Command {
    command_type: CommandType,
    style: Style,
}

pub enum CommandType {
    Square(AABB)
}
