mod str_tools;

fn main() {
	let s:&str = "catstac";
	let r = str_tools::palindrome::is_palindrome(s);
	println!("{}", {r});
}
