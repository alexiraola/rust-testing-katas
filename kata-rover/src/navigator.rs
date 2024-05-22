use crate::location::Location;

pub trait Navigator {
    fn format(&self) -> String;
    fn rotate_left(&self) -> Box<dyn Navigator>;
    fn rotate_right(&self) -> Box<dyn Navigator>;
    fn move_forward(&self) -> Box<dyn Navigator>;
}

pub struct NorthNavigator {
    pub location: Location,
}

impl Navigator for NorthNavigator {
    fn format(&self) -> String {
        format!("{}:N", self.location)
    }

    fn rotate_left(&self) -> Box<dyn Navigator> {
        Box::new(WestNavigator {
            location: self.location.clone(),
        })
    }

    fn rotate_right(&self) -> Box<dyn Navigator> {
        Box::new(EastNavigator {
            location: self.location.clone(),
        })
    }

    fn move_forward(&self) -> Box<dyn Navigator> {
        Box::new(NorthNavigator {
            location: self.location.increase_y(),
        })
    }
}

struct EastNavigator {
    location: Location,
}

impl Navigator for EastNavigator {
    fn format(&self) -> String {
        format!("{}:E", self.location)
    }

    fn rotate_left(&self) -> Box<dyn Navigator> {
        Box::new(NorthNavigator {
            location: self.location.clone(),
        })
    }

    fn rotate_right(&self) -> Box<dyn Navigator> {
        Box::new(SouthNavigator {
            location: self.location.clone(),
        })
    }

    fn move_forward(&self) -> Box<dyn Navigator> {
        Box::new(EastNavigator {
            location: self.location.increase_x(),
        })
    }
}

struct SouthNavigator {
    location: Location,
}

impl Navigator for SouthNavigator {
    fn format(&self) -> String {
        format!("{}:S", self.location)
    }

    fn rotate_left(&self) -> Box<dyn Navigator> {
        Box::new(EastNavigator {
            location: self.location.clone(),
        })
    }

    fn rotate_right(&self) -> Box<dyn Navigator> {
        Box::new(WestNavigator {
            location: self.location.clone(),
        })
    }

    fn move_forward(&self) -> Box<dyn Navigator> {
        Box::new(SouthNavigator {
            location: self.location.decrease_y(),
        })
    }
}

struct WestNavigator {
    location: Location,
}

impl Navigator for WestNavigator {
    fn format(&self) -> String {
        format!("{}:W", self.location)
    }

    fn rotate_left(&self) -> Box<dyn Navigator> {
        Box::new(SouthNavigator {
            location: self.location.clone(),
        })
    }

    fn rotate_right(&self) -> Box<dyn Navigator> {
        Box::new(NorthNavigator {
            location: self.location.clone(),
        })
    }

    fn move_forward(&self) -> Box<dyn Navigator> {
        Box::new(WestNavigator {
            location: self.location.decrease_x(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod facing_north {
        use super::{Location, Navigator, NorthNavigator};

        #[test]
        fn west_to_left() {
            let navigator = NorthNavigator {
                location: Location::create(0, 0),
            }
            .rotate_left();

            assert_eq!(navigator.format(), "0:0:W")
        }

        #[test]
        fn east_to_right() {
            let navigator = NorthNavigator {
                location: Location::create(0, 0),
            }
            .rotate_right();

            assert_eq!(navigator.format(), "0:0:E")
        }

        #[test]
        fn move_forward_increases_y() {
            let navigator = NorthNavigator {
                location: Location::create(0, 0),
            }
            .move_forward();

            assert_eq!(navigator.format(), "0:1:N")
        }
    }

    mod facing_east {
        use super::{EastNavigator, Location, Navigator};

        #[test]
        fn north_to_left() {
            let navigator = EastNavigator {
                location: Location::create(0, 0),
            }
            .rotate_left();

            assert_eq!(navigator.format(), "0:0:N")
        }

        #[test]
        fn south_to_right() {
            let navigator = EastNavigator {
                location: Location::create(0, 0),
            }
            .rotate_right();

            assert_eq!(navigator.format(), "0:0:S")
        }

        #[test]
        fn move_forward_increases_x() {
            let navigator = EastNavigator {
                location: Location::create(0, 0),
            }
            .move_forward();

            assert_eq!(navigator.format(), "1:0:E")
        }
    }

    mod facing_south {
        use super::{Location, Navigator, SouthNavigator};

        #[test]
        fn east_to_left() {
            let navigator = SouthNavigator {
                location: Location::create(0, 0),
            }
            .rotate_left();

            assert_eq!(navigator.format(), "0:0:E")
        }

        #[test]
        fn west_to_right() {
            let navigator = SouthNavigator {
                location: Location::create(0, 0),
            }
            .rotate_right();

            assert_eq!(navigator.format(), "0:0:W")
        }

        #[test]
        fn move_forward_decreases_y() {
            let navigator = SouthNavigator {
                location: Location::create(0, 0),
            }
            .move_forward();

            assert_eq!(navigator.format(), "0:9:S")
        }
    }

    mod facing_west {
        use super::{Location, Navigator, WestNavigator};

        #[test]
        fn south_to_left() {
            let navigator = WestNavigator {
                location: Location::create(0, 0),
            }
            .rotate_left();

            assert_eq!(navigator.format(), "0:0:S")
        }

        #[test]
        fn north_to_right() {
            let navigator = WestNavigator {
                location: Location::create(0, 0),
            }
            .rotate_right();

            assert_eq!(navigator.format(), "0:0:N")
        }

        #[test]
        fn move_forward_decreases_x() {
            let navigator = WestNavigator {
                location: Location::create(0, 0),
            }
            .move_forward();

            assert_eq!(navigator.format(), "9:0:W")
        }
    }
}
