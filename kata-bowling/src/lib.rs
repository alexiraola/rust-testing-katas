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
        let mut frame_index = 0;
        let mut points = 0;

        for _ in 0..10 {
            if self.is_strike(frame_index) {
                points += 10 + self.rolls[frame_index + 1] + self.rolls[frame_index + 2];
                frame_index += 1;
            } else if self.is_spare(frame_index) {
                points += 10 + self.rolls[frame_index + 2];
                frame_index += 2;
            } else {
                points += self.rolls[frame_index] + self.rolls[frame_index + 1];
                frame_index += 2;
            }
        }
        points
    }

    fn is_spare(&self, frame_index: usize) -> bool {
        self.rolls[frame_index] + self.rolls[frame_index + 1] == 10
    }

    fn is_strike(&self, frame_index: usize) -> bool {
        self.rolls[frame_index] == 10
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
}
