mod location;

use location::Location;

#[derive(Debug, Clone)]
enum Orientation {
    N,
    E,
    S,
    W,
}

pub enum Command {
    R,
    L,
    F,
}

#[derive(Clone)]
pub struct Rover {
    location: Location,
    orientation: Orientation,
}

impl Rover {
    pub fn format_position(&self) -> String {
        format!("{}:{:#?}", self.location, self.orientation)
    }

    pub fn execute(&self, commands: Vec<Command>) -> Rover {
        commands
            .iter()
            .fold(self.clone(), |r, c| r.execute_command(c))
    }

    fn execute_command(&self, command: &Command) -> Rover {
        match command {
            Command::R => self.rotate_right(),
            Command::L => self.rotate_left(),
            Command::F => self.move_forward(),
        }
    }

    fn rotate_right(&self) -> Rover {
        Rover {
            location: self.location.clone(),
            orientation: match self.orientation {
                Orientation::N => Orientation::E,
                Orientation::E => Orientation::S,
                Orientation::S => Orientation::W,
                Orientation::W => Orientation::N,
            },
        }
    }

    fn rotate_left(&self) -> Rover {
        Rover {
            location: self.location.clone(),
            orientation: match self.orientation {
                Orientation::N => Orientation::W,
                Orientation::E => Orientation::N,
                Orientation::S => Orientation::E,
                Orientation::W => Orientation::S,
            },
        }
    }

    fn move_forward(&self) -> Rover {
        Rover {
            location: match self.orientation {
                Orientation::N => self.location.increase_y(),
                Orientation::E => self.location.increase_x(),
                Orientation::S => self.location.decrease_y(),
                Orientation::W => self.location.decrease_x(),
            },
            orientation: self.orientation.clone(),
        }
    }
}

impl Default for Rover {
    fn default() -> Self {
        Self {
            location: Location::create(0, 0),
            orientation: Orientation::N,
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
