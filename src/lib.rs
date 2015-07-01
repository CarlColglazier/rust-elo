/// Elo.
pub trait Elo {
    /// Get the rating.
    fn get_rating(&self) -> f32;
    /// Set the rating.
    fn change_rating(&mut self, rating: f32);
}

fn expected_rating<T: Elo>(player_one: &T, player_two: &T) -> f32 {
    return 1.0f32 / (1.0f32 + 10f32.powf(
        (player_two.get_rating() - player_one.get_rating()) / 400f32
    ));
}

/// EloRanking.
pub struct EloRanking {
    k_factor: usize,
}

impl EloRanking {
    /// Create a new Elo ranking system.
    ///
    /// # Example
    ///
    /// ```
    /// # use elo::EloRanking;
    /// let k_factor: usize = 32;
    /// let elo_ranking = EloRanking::new(k_factor);
    /// ```
    pub fn new(k: usize) -> EloRanking {
        return EloRanking {
            k_factor: k,
        }
    }

    /// Change the K factor.
    ///
    /// # Example
    ///
    /// ```
    /// # use elo::EloRanking;
    /// # let mut elo_ranking = EloRanking::new(32);
    /// elo_ranking.set_k_factor(25);
    /// ```
    pub fn set_k_factor(&mut self, k: usize) {
        self.k_factor = k;
    }

    /// Returns the K factor.
    ///
    /// # Example
    ///
    /// ```
    /// # use elo::EloRanking;
    /// # let elo_ranking = EloRanking::new(32);
    /// assert_eq!(32, elo_ranking.get_k_factor());
    /// ```
    pub fn get_k_factor(&self) -> usize {
        return self.k_factor;
    }

    /// Internal method for generic calculations.
    fn calculate_rating<T: Elo>(&self,
                                      player_one: &mut T,
                                      player_two: &mut T,
                                      score: f32) {
        let change = self.k_factor as f32 *
            (score - expected_rating::<T>(player_one, player_two));
        player_one.change_rating(change);
        player_two.change_rating(-change);
    }

    pub fn win<T: Elo>(&self, winner: &mut T, loser: &mut T) {
        self.calculate_rating(winner, loser, 1.0);
    }

    pub fn tie<T: Elo>(&self, player_one: &mut T, player_two: &mut T) {
        self.calculate_rating(player_one, player_two, 0.5);
    }

    pub fn loss<T: Elo>(&self, loser: &mut T, winner: &mut T) {
        self.win::<T>(winner, loser);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct RatingObject {
        rating: f32,
    }

    impl RatingObject {
        pub fn new() -> RatingObject {
            return RatingObject {
                rating: 1400f32,
            };
        }
    }

    impl Elo for RatingObject {
        fn get_rating(&self) -> f32 {
            return self.rating;
        }
        fn change_rating(&mut self, rating: f32) {
            self.rating += rating;
        }
    }

    #[test]
    fn rating() {
        let mut player_one = RatingObject::new();
        let mut player_two = RatingObject::new();
        let rating_system = EloRanking::new(32);
        assert_eq!(1400f32, player_one.get_rating());
        assert_eq!(1400f32, player_two.get_rating());
        player_one.change_rating(100f32);
        assert_eq!(1500f32, player_one.get_rating());
        player_one.change_rating(-100f32);
        assert_eq!(1400f32, player_one.get_rating());
        // In a tie, the ratings should stay the same.
        rating_system.tie::<RatingObject>(&mut player_one, &mut player_two);
        assert_eq!(1400f32, player_one.get_rating());
        assert_eq!(1400f32, player_two.get_rating());
        // With a win, player_one should gain an advantage.
        rating_system.win::<RatingObject>(&mut player_one, &mut player_two);
        assert_eq!(1416f32, player_one.get_rating());
        assert_eq!(1384f32, player_two.get_rating());
        // With a loss, this should reset to normal.
        rating_system.loss::<RatingObject>(&mut player_one, &mut player_two);
        assert_eq!(1398.5305f32, player_one.get_rating());
        assert_eq!(1401.4695f32, player_two.get_rating());
    }
}
