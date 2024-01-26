# str_tools

## Использование:

```rust
mod str_tools;

fn main() {
	let s: &str = "catsta";
	let r = str_tools::palindrome::is_palindrome(s);
	// false
	println!("{}", { r });

	let s: &str = "catstac";
	let r = str_tools::palindrome::is_palindrome(s);
	// true
	println!("{}", { r });
}
```
