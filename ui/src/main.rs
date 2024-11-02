use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow};
use emulator::cpu::CPUProcessor;

const APP_ID: &str = "com.lewissmith.lama";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    let processor: CPUProcessor = CPUProcessor::new(String::from("CPU0"), 0);
    println!("{}", processor);
    app.connect_activate(build_ui);
    app.run()
    // Just a stub for testing the processor stuff
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Lewis's ARM64 eMulator App")
        .build();
    window.present();
}
