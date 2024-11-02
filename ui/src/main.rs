use emulator::cpu::CPUProcessor;
fn main() {
    // Just a stub for testing the processor stuff
    let mut processor: CPUProcessor = CPUProcessor::new(String::from("CPU0"), 0);
    println!("Hello, world!");
    println!("{}", processor);
}
