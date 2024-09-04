#[allow(dead_code)]
fn execute(bullets: u16, dragons: u16) -> bool {
	bullets / 2 >= dragons
}

#[cfg(test)]
mod tests {
	use super::execute;

	#[test]
	fn test_hero() {
		assert_eq!(execute(10, 5), true);
		assert_eq!(execute(7, 4), false);
    assert_eq!(execute(4, 5), false);
    assert_eq!(execute(100, 40), true);
    assert_eq!(execute(1500, 751), false);
    assert_eq!(execute(0, 1), false);
	}
}