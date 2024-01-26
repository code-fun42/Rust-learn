/// Функция проверки строки на палиндром. Возвращает true, если строка является палиндромом и
/// false в обратном случае.
/// * `s` - входная строка.
pub fn is_palindrome(s: &str) -> bool {
	if s.len() <= 1 {
		return false;
	}

	let chars: Vec<char> = s.chars().collect();

	let mut left: usize = 0;
	let mut right: usize = s.len() - 1;

	while left <= right && left < chars.len() {
		if !chars[left].is_alphabetic() {
			left += 1;
		}

		if !chars[right].is_alphabetic() {
			right -= 1;
		}

		if chars[left].to_ascii_lowercase() == chars[right].to_ascii_lowercase() {
			if left == right || left + 1 == right || left == right - 1 {
				return true;
			}

			left += 1;
			right -= 1;
		} else {
			return false;
		}
	}

	return false;
}
