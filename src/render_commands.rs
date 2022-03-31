use crate::style::Style;
use crate::utils::{AABB};

struct Command {
    command_type: CommandType,
    style: Style,
}

enum CommandType {
    Square(AABB)
}
