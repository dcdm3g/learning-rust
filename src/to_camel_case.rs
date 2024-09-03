use regex::{Captures, Regex};

#[allow(dead_code)]
pub fn execute(text: &str) -> String {
	let re = Regex::new(r"[-_]([a-zA-Z])").unwrap();

	re
		.replace_all(text, | caps: &Captures | {
			caps[1].to_uppercase()
		})
		.to_string()
}

#[cfg(test)]
mod tests {
	use super::execute;

	fn dotest(text: &str, expected: &str) {
		let actual = execute(text);
		assert_eq!(actual, expected, "With text = {text:?}\n Expected {expected} but got {actual}")
	}

	#[test]
	fn test_to_camel_case() {
		dotest("hello_world", "helloWorld");
		dotest("hello-world", "helloWorld");
	}
}