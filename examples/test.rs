use quantic::{self, QuanticRunnerImpl};

fn main() {
	quantic::default()
		.add(690000000)
		.add(4200000)
		.run()
		.expect("QuanticInternal");
}