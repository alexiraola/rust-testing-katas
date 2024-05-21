// 10x10 world
// L => '0:0:W'
// R => '0:0:E'
// F => '0:1:N'
// RFF => '2:0:E'
// LFF => '8:0:W'
// LLFF => '0:8:S'
// FRFFR => '2:1:S'

pub struct Rover {
    x: i32,
    y: i32,
    orientation: String,
}

impl Rover {
    pub fn position(&self) -> String {
        String::from("0:0:N")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_rover_at_initial_position() {
        let rover = Rover {
            x: 0,
            y: 0,
            orientation: String::from("N"),
        };
        assert_eq!(rover.position(), "0:0:N");
    }
}
