// 10x10 world
// L => '0:0:W'
// R => '0:0:E'
// F => '0:1:N'
// RFF => '2:0:E'
// LFF => '8:0:W'
// LLFF => '0:8:S'
// FRFFR => '2:1:S'

#[derive(Debug)]
enum Orientation {
    N,
    E,
    S,
    W,
}

pub struct Rover {
    x: i32,
    y: i32,
    orientation: Orientation,
}

impl Rover {
    pub fn position(&self) -> String {
        format!("{}:{}:{:#?}", self.x, self.y, self.orientation)
    }

    pub fn execute(&mut self, commands: Vec<String>) {
        if commands[0].eq("L") {
            self.orientation = Orientation::W;
        }
        if commands[0].eq("R") {
            self.orientation = Orientation::E;
        }
    }
}

impl Default for Rover {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
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
        assert_eq!(rover.position(), "0:0:N");
    }

    #[test]
    fn turns_left() {
        let mut rover = Rover::default();

        rover.execute(vec!["L".to_string()]);

        assert_eq!(rover.position(), "0:0:W");
    }

    #[test]
    fn turns_right() {
        let mut rover = Rover::default();

        rover.execute(vec!["R".to_string()]);

        assert_eq!(rover.position(), "0:0:E");
    }
}
