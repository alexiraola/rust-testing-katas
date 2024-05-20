const MAX_POINTS: i32 = 10;

trait Frame {
    fn score(&self) -> i32;
    fn rolls(&self, count: i32) -> i32;
    fn next_frame(&self) -> Option<&Box<dyn Frame>>;
    fn total_score(&self) -> i32 {
        if let Some(frame) = self.next_frame() {
            self.score() + frame.total_score()
        } else {
            self.score()
        }
    }
}

struct OpenFrame {
    first_roll: i32,
    second_roll: i32,
    next_frame: Option<Box<dyn Frame>>,
}

impl Frame for OpenFrame {
    fn score(&self) -> i32 {
        self.first_roll + self.second_roll
    }

    fn rolls(&self, count: i32) -> i32 {
        match count {
            0 => 0,
            1 => self.first_roll,
            _ => self.first_roll + self.second_roll,
        }
    }

    fn next_frame(&self) -> Option<&Box<dyn Frame>> {
        self.next_frame.as_ref()
    }
}

struct SpareFrame {
    first_roll: i32,
    second_roll: i32,
    next_frame: Option<Box<dyn Frame>>,
}

impl Frame for SpareFrame {
    fn score(&self) -> i32 {
        MAX_POINTS
            + match self.next_frame() {
                Some(frame) => frame.rolls(1),
                None => 0,
            }
    }

    fn rolls(&self, count: i32) -> i32 {
        match count {
            0 => 0,
            1 => self.first_roll,
            _ => self.first_roll + self.second_roll,
        }
    }

    fn next_frame(&self) -> Option<&Box<dyn Frame>> {
        self.next_frame.as_ref()
    }
}

struct StrikeFrame {
    next_frame: Option<Box<dyn Frame>>,
}

impl Frame for StrikeFrame {
    fn score(&self) -> i32 {
        MAX_POINTS
            + match &self.next_frame {
                Some(frame) => frame.rolls(2),
                None => 0,
            }
    }

    fn rolls(&self, count: i32) -> i32 {
        match count {
            0 => 0,
            1 => MAX_POINTS,
            _ => {
                MAX_POINTS
                    + match self.next_frame() {
                        Some(frame) => frame.rolls(1),
                        None => 0,
                    }
            }
        }
    }

    fn next_frame(&self) -> Option<&Box<dyn Frame>> {
        self.next_frame.as_ref()
    }
}

struct BonusFrame {
    first_roll: i32,
    second_roll: i32,
}

impl Frame for BonusFrame {
    fn score(&self) -> i32 {
        0
    }

    fn rolls(&self, count: i32) -> i32 {
        match count {
            0 => 0,
            1 => self.first_roll,
            _ => self.first_roll + self.second_roll,
        }
    }

    fn next_frame(&self) -> Option<&Box<dyn Frame>> {
        None
    }
}

pub struct BowlingGame {
    rolls: Vec<i32>,
}

impl BowlingGame {
    pub fn new() -> BowlingGame {
        BowlingGame { rolls: vec![] }
    }
    pub fn roll(&mut self, points: i32) {
        self.rolls.push(points);
    }

    pub fn calculate_result(&self) -> i32 {
        if let Some(frame) = self.build_frames(0, 0) {
            frame.total_score()
        } else {
            0
        }
    }

    fn build_frames(&self, frame: i32, frame_index: usize) -> Option<Box<dyn Frame>> {
        if frame == 10 {
            return Some(Box::new(BonusFrame {
                first_roll: *self.rolls.get(frame_index).unwrap_or(&0),
                second_roll: *self.rolls.get(frame_index + 1).unwrap_or(&0),
            }));
        }
        if self.is_strike(frame_index) {
            return Some(Box::new(StrikeFrame {
                next_frame: self.build_frames(frame + 1, frame_index + 1),
            }));
        }
        if self.is_spare(frame_index) {
            return Some(Box::new(SpareFrame {
                first_roll: self.rolls[frame_index],
                second_roll: self.rolls[frame_index + 1],
                next_frame: self.build_frames(frame + 1, frame_index + 2),
            }));
        }
        Some(Box::new(OpenFrame {
            first_roll: self.rolls[frame_index],
            second_roll: self.rolls[frame_index + 1],
            next_frame: self.build_frames(frame + 1, frame_index + 2),
        }))
    }

    fn is_spare(&self, frame_index: usize) -> bool {
        self.rolls[frame_index] + self.rolls[frame_index + 1] == MAX_POINTS
    }

    fn is_strike(&self, frame_index: usize) -> bool {
        self.rolls[frame_index] == MAX_POINTS
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roll_many(bowling_game: &mut BowlingGame, times: i32, points: i32) {
        for _ in 0..times {
            bowling_game.roll(points);
        }
    }

    #[test]
    fn zero_hits_get_zero_points() {
        let mut bowling_game = BowlingGame::new();

        roll_many(&mut bowling_game, 20, 0);

        assert_eq!(bowling_game.calculate_result(), 0);
    }

    #[test]
    fn all_ones_sums_twenty() {
        let mut bowling_game = BowlingGame::new();

        roll_many(&mut bowling_game, 20, 1);

        assert_eq!(bowling_game.calculate_result(), 20);
    }

    #[test]
    fn all_twos_sums_forty() {
        let mut bowling_game = BowlingGame::new();

        roll_many(&mut bowling_game, 20, 2);

        assert_eq!(bowling_game.calculate_result(), 40);
    }

    #[test]
    fn calculates_score_for_a_given_spare_extra_ball() {
        let mut bowling_game = BowlingGame::new();

        bowling_game.roll(5);
        bowling_game.roll(5);
        bowling_game.roll(5);

        roll_many(&mut bowling_game, 17, 0);

        assert_eq!(bowling_game.calculate_result(), 20);
    }

    #[test]
    fn calculates_score_for_a_given_strike_extra_ball() {
        let mut bowling_game = BowlingGame::new();

        bowling_game.roll(10);
        bowling_game.roll(2);
        bowling_game.roll(3);

        roll_many(&mut bowling_game, 16, 0);

        assert_eq!(bowling_game.calculate_result(), 20);
    }

    #[test]
    fn calculates_score_for_a_perfect_game() {
        let mut bowling_game = BowlingGame::new();

        roll_many(&mut bowling_game, 12, 10);

        assert_eq!(bowling_game.calculate_result(), 300);
    }

    #[test]
    fn calculates_socre_for_alternate_spare_strike() {
        let mut bowling_game = BowlingGame::new();

        for _ in 0..5 {
            bowling_game.roll(10);
            bowling_game.roll(3);
            bowling_game.roll(7);
        }
        bowling_game.roll(10);

        assert_eq!(bowling_game.calculate_result(), 200);
    }
}
