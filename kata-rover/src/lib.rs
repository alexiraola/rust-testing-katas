mod location;
mod navigator;

use location::Location;
use navigator::{Navigator, NorthNavigator};

pub enum Command {
    R,
    L,
    F,
}

pub struct Rover {
    navigator: Box<dyn Navigator>,
}

impl Rover {
    pub fn format_position(&self) -> String {
        self.navigator.format()
    }

    pub fn execute(&self, commands: Vec<Command>) -> Rover {
        commands.iter().fold(
            Rover {
                navigator: self.navigator.clone(),
            },
            |r, c| r.execute_command(c),
        )
    }

    fn execute_command(&self, command: &Command) -> Rover {
        match command {
            Command::R => Rover {
                navigator: self.navigator.rotate_right(),
            },
            Command::L => Rover {
                navigator: self.navigator.rotate_left(),
            },
            Command::F => Rover {
                navigator: self.navigator.move_forward(),
            },
        }
    }
}

impl Default for Rover {
    fn default() -> Self {
        Self {
            navigator: Box::new(NorthNavigator {
                location: Location::create(0, 0),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_rover_at_initial_position() {
        let rover = Rover::default();
        assert_eq!(rover.format_position(), "0:0:N");
    }

    #[test]
    fn turns_left() {
        let rover = Rover::default().execute(vec![Command::L]);

        assert_eq!(rover.format_position(), "0:0:W");
    }

    #[test]
    fn turns_right() {
        let rover = Rover::default().execute(vec![Command::R]);

        assert_eq!(rover.format_position(), "0:0:E");
    }

    #[test]
    fn moves_forward() {
        let rover = Rover::default().execute(vec![Command::F]);

        assert_eq!(rover.format_position(), "0:1:N");
    }

    #[test]
    fn moves_forward_twice_to_the_right() {
        let rover = Rover::default().execute(vec![Command::R, Command::F, Command::F]);

        assert_eq!(rover.format_position(), "2:0:E");
    }

    #[test]
    fn turns_twice_right() {
        let rover = Rover::default().execute(vec![Command::R, Command::R]);

        assert_eq!(rover.format_position(), "0:0:S");
    }

    #[test]
    fn turns_twice_left() {
        let rover = Rover::default().execute(vec![Command::L, Command::L]);

        assert_eq!(rover.format_position(), "0:0:S");
    }

    #[test]
    fn executes_many_commands() {
        let rover = Rover::default().execute(vec![Command::R, Command::F, Command::F]);

        assert_eq!(rover.format_position(), "2:0:E");
    }

    #[test]
    fn rounds_on_horizontal_edges() {
        let rover = Rover::default().execute(vec![Command::L, Command::F, Command::F]);

        assert_eq!(rover.format_position(), "8:0:W");
    }

    #[test]
    fn rounds_on_vertical_edges() {
        let rover = Rover::default().execute(vec![Command::L, Command::L, Command::F, Command::F]);

        assert_eq!(rover.format_position(), "0:8:S");
    }
}
