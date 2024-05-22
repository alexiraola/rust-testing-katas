use std::fmt;

const WORLD_WIDTH: i32 = 10;
const WORLD_HEIGHT: i32 = 10;

#[derive(Clone)]
pub struct Location {
    x: i32,
    y: i32,
}

impl Location {
    pub fn create(x: i32, y: i32) -> Location {
        Location { x, y }
    }

    pub fn increase_x(&self) -> Location {
        Location {
            x: (self.x + 1) % WORLD_WIDTH,
            y: self.y,
        }
    }

    pub fn decrease_x(&self) -> Location {
        Location {
            x: (WORLD_WIDTH + self.x - 1) % WORLD_WIDTH,
            y: self.y,
        }
    }

    pub fn increase_y(&self) -> Location {
        Location {
            x: self.x,
            y: (self.y + 1) % WORLD_HEIGHT,
        }
    }

    pub fn decrease_y(&self) -> Location {
        Location {
            x: self.x,
            y: (WORLD_HEIGHT + self.y - 1) % WORLD_HEIGHT,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_a_location_with_initial_values() {
        let location = Location::create(0, 0);

        assert_eq!(format!("{}", location), "0:0");
    }

    #[test]
    fn increases_x() {
        let location = Location::create(0, 0);

        assert_eq!(format!("{}", location.increase_x()), "1:0");
    }

    #[test]
    fn decreases_x() {
        let location = Location::create(1, 0);

        assert_eq!(format!("{}", location.decrease_x()), "0:0");
    }

    #[test]
    fn increases_y() {
        let location = Location::create(0, 0);

        assert_eq!(format!("{}", location.increase_y()), "0:1");
    }

    #[test]
    fn decreases_y() {
        let location = Location::create(0, 1);

        assert_eq!(format!("{}", location.decrease_y()), "0:0");
    }

    #[test]
    fn rounds_x_on_edges() {
        assert_eq!(format!("{}", Location::create(0, 0).decrease_x()), "9:0");
        assert_eq!(format!("{}", Location::create(9, 0).increase_x()), "0:0");
    }

    #[test]
    fn rounds_y_on_edges() {
        assert_eq!(format!("{}", Location::create(0, 0).decrease_y()), "0:9");
        assert_eq!(format!("{}", Location::create(0, 9).increase_y()), "0:0");
    }
}
