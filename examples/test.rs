use quantic;
use quantic::RunnerImpl;

fn main() {
	quantic::default()
		.run()
		.expect("QuanticInternal");
}