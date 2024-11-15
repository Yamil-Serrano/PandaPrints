use eframe::egui;

#[derive(Default)]
pub struct PrinterCostApp {
    filament_price: f64,
    filament_used: f64,
    hours: f64,
    minutes: f64,
    electricity_price: f64,
    printer_consumption: f64,
    failure_rate: f64,
    result: Option<String>,
}

impl eframe::App for PrinterCostApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Set the background color
        ctx.set_visuals(egui::Visuals {
            window_fill: egui::Color32::from_rgb(30, 30, 30), // Dark gray background
            ..Default::default()
        });


        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("3D Printer Cost Calculator");

            // Input fields
            ui.horizontal(|ui| {
                ui.label("Filament price (per kg):");
                ui.add(egui::DragValue::new(&mut self.filament_price).speed(0.1));
            });

            ui.horizontal(|ui| {
                ui.label("Filament used (grams):");
                ui.add(egui::DragValue::new(&mut self.filament_used).speed(1.0));
            });

            ui.horizontal(|ui| {
                ui.label("Hours:");
                ui.add(egui::DragValue::new(&mut self.hours).speed(0.1));
                ui.label("Minutes:");
                ui.add(egui::DragValue::new(&mut self.minutes).speed(1.0));
            });

            ui.horizontal(|ui| {
                ui.label("Electricity price (per kWh):");
                ui.add(egui::DragValue::new(&mut self.electricity_price).speed(0.01));
            });

            ui.horizontal(|ui| {
                ui.label("Printer power consumption (Watts):");
                ui.add(egui::DragValue::new(&mut self.printer_consumption).speed(0.1));
            });

            ui.horizontal(|ui| {
                ui.label("Failure rate (%):");
                ui.add(egui::DragValue::new(&mut self.failure_rate).speed(0.1));
            });

            // Button to calculate
            if ui.button("Calculate").clicked() {
                self.result = Some(self.calculate_cost());
            }

            // Display result
            if let Some(result) = &self.result {
                ui.separator();
                ui.label("Result:");
                ui.monospace(result);
            }
        });
    }
}

impl PrinterCostApp {
    fn calculate_cost(&self) -> String {
        let total_time_hours = self.hours + (self.minutes / 60.0);
        let filament_cost = (self.filament_price / 1000.0) * self.filament_used;
        let consumption_kwh = self.printer_consumption / 1000.0;
        let energy_cost = consumption_kwh * total_time_hours * self.electricity_price;
        let total_cost = filament_cost + energy_cost;
        let total_cost_with_failure = total_cost + (total_cost * (self.failure_rate / 100.0));

        format!(
            "Filament Cost: {:.2}\nEnergy Cost: {:.2}\nTotal Cost: {:.2}\nTotal Cost with Failure Rate: {:.2}",
            filament_cost, energy_cost, total_cost, total_cost_with_failure
        )
    }
}
