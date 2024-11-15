mod user_interface;

fn main() -> Result<(), eframe::Error> {
    // Start the GUI application
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "3D Printer Cost Calculator",
        options,
        Box::new(|_| Ok(Box::new(user_interface::PrinterCostApp::default()))), // Wrap the Box in Ok
    )
}
