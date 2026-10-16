pub fn is_crawford(score: [u32; 2], target: u32, already_played: bool) -> bool {
    target > 1 && !already_played && score.iter().any(|&s| s + 1 == target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crawford_game_once_at_match_point() {
        assert!(is_crawford([4, 2], 5, false));
        assert!(!is_crawford([4, 2], 5, true));
        assert!(!is_crawford([3, 2], 5, false));
        assert!(!is_crawford([0, 0], 1, false));
    }
}
