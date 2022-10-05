#[derive(Default)]
pub struct Main {
	value: u32
}
impl quantic::script::Script for Main {
	fn start(mut self) {
		self.value = 0;
	}
	fn update(mut self) {
		self.value += 1;
		println!("{}", self.value);
	}
}