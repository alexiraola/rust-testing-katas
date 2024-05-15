const MAX_POINTS: i32 = 10;
const OPEN_FRAME_ROLLS: usize = 2;
const SPARE_ROLLS: usize = 2;
const STRIKE_ROLLS: usize = 1;

trait Frame {
    fn score(&self) -> i32;
    fn roll_count(&self) -> usize;
}

struct OpenFrame {
    rolls: Vec<i32>,
    index: usize,
}

impl Frame for OpenFrame {
    fn score(&self) -> i32 {
        self.rolls[self.index] + self.rolls[self.index + 1]
    }

    fn roll_count(&self) -> usize {
        OPEN_FRAME_ROLLS
    }
}

struct SpareFrame {
    rolls: Vec<i32>,
    index: usize,
}

impl Frame for SpareFrame {
    fn score(&self) -> i32 {
        MAX_POINTS + self.rolls[self.index + 2]
    }

    fn roll_count(&self) -> usize {
        SPARE_ROLLS
    }
}

struct StrikeFrame {
    rolls: Vec<i32>,
    index: usize,
}

impl Frame for StrikeFrame {
    fn score(&self) -> i32 {
        MAX_POINTS + self.rolls[self.index + 1] + self.rolls[self.index + 2]
    }

    fn roll_count(&self) -> usize {
        STRIKE_ROLLS
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
        let (points, _) = (0..10).fold((0, 0), |(points, frame_index), _| {
            let frame = self.get_frame(frame_index);
            (points + frame.score(), frame_index + frame.roll_count())
        });

        points
    }

    fn get_frame(&self, frame_index: usize) -> Box<dyn Frame> {
        if self.is_strike(frame_index) {
            return Box::new(StrikeFrame {
                rolls: self.rolls.clone(),
                index: frame_index,
            });
        }
        if self.is_spare(frame_index) {
            return Box::new(SpareFrame {
                rolls: self.rolls.clone(),
                index: frame_index,
            });
        }
        Box::new(OpenFrame {
            rolls: self.rolls.clone(),
            index: frame_index,
        })
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
