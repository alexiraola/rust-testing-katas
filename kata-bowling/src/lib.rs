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
        return self.rolls.iter().sum::<i32>();
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
}
