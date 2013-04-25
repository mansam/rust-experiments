fn main() {
	let mut count = 0;

	while count < 10 {
		io::println(fmt!("Count: %?", count));
		count += 1;
	}
}