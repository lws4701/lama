use emulator::cpu::CPUProcessor;
fn main() {
    // Just a stub for testing the processor stuff
    let processor: CPUProcessor = CPUProcessor::new(String::from("CPU0"), 0);
    println!("{}", processor);
}
