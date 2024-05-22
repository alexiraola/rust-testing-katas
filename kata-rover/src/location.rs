use std::fmt;

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
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn decrease_x(&self) -> Location {
        Location {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn increase_y(&self) -> Location {
        Location {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn decrease_y(&self) -> Location {
        Location {
            x: self.x,
            y: self.y - 1,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.x, self.y)
    }
}
