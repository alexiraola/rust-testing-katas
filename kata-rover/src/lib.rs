// 10x10 world
// L => '0:0:W'
// R => '0:0:E'
// F => '0:1:N'
// RFF => '2:0:E'
// LFF => '8:0:W'
// LLFF => '0:8:S'
// FRFFR => '2:1:S'
//
mod location;

use location::Location;

#[derive(Debug)]
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

pub struct Rover {
    location: Location,
    orientation: Orientation,
}

impl Rover {
    pub fn format_position(&self) -> String {
        format!("{}:{:#?}", self.location, self.orientation)
    }

    pub fn execute(&mut self, commands: Vec<Command>) {
        commands.iter().for_each(|c| self.execute_command(c));
    }

    fn execute_command(&mut self, command: &Command) {
        match command {
            Command::R => self.rotate_right(),
            Command::L => self.rotate_left(),
            Command::F => self.move_forward(),
        }
    }

    fn rotate_right(&mut self) {
        match self.orientation {
            Orientation::N => self.orientation = Orientation::E,
            Orientation::E => self.orientation = Orientation::S,
            Orientation::S => self.orientation = Orientation::W,
            Orientation::W => self.orientation = Orientation::N,
        }
    }

    fn rotate_left(&mut self) {
        match self.orientation {
            Orientation::N => self.orientation = Orientation::W,
            Orientation::E => self.orientation = Orientation::N,
            Orientation::S => self.orientation = Orientation::E,
            Orientation::W => self.orientation = Orientation::S,
        }
    }

    fn move_forward(&mut self) {
        self.location = match self.orientation {
            Orientation::N => self.location.increase_y(),
            Orientation::E => self.location.increase_x(),
            Orientation::S => self.location.decrease_y(),
            Orientation::W => self.location.decrease_x(),
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
        let mut rover = Rover::default();

        rover.execute(vec![Command::L]);

        assert_eq!(rover.format_position(), "0:0:W");
    }

    #[test]
    fn turns_right() {
        let mut rover = Rover::default();

        rover.execute(vec![Command::R]);

        assert_eq!(rover.format_position(), "0:0:E");
    }

    #[test]
    fn moves_forward() {
        let mut rover = Rover::default();

        rover.execute(vec![Command::F]);

        assert_eq!(rover.format_position(), "0:1:N");
    }

    #[test]
    fn moves_forward_twice_to_the_right() {
        let mut rover = Rover::default();

        rover.execute(vec![Command::R, Command::F, Command::F]);

        assert_eq!(rover.format_position(), "2:0:E");
    }

    #[test]
    fn turns_twice_right() {
        let mut rover = Rover::default();

        rover.execute(vec![Command::R, Command::R]);

        assert_eq!(rover.format_position(), "0:0:S");
    }

    #[test]
    fn turns_twice_left() {
        let mut rover = Rover::default();

        rover.execute(vec![Command::L, Command::L]);

        assert_eq!(rover.format_position(), "0:0:S");
    }
}
