use egui_winit_vulkano::egui::{self, Context, RichText, FontDefinitions, ProgressBar};
use sysinfo::System;
use std::sync::Arc;
use vulkano::device::Queue;

pub struct DebugUi<'a> {
    pub system: &'a System,
    pub queue: Arc<Queue>,
}

impl<'a> DebugUi<'a> {
    pub fn render(&self, ctx: &Context) {
        ctx.set_fonts(FontDefinitions::default());

        egui::Window::new("Despawn Debugger").resizable(true).show(ctx, |ui| {
            // System Info Header
            ui.heading("System Information");

            // CPU Info and Usage
            egui::CollapsingHeader::new("CPU")
                .default_open(false)
                .show(ui, |ui| {
                    let cpu = self.system.cpus().first()
                        .map(|cpu| (cpu.brand(), cpu.cpu_usage()))
                        .unwrap_or(("Unknown", 0.0));
                    let cpu_cores = self.system.cpus().len();

                    ui.add_space(5.0);
                    ui.label(RichText::new(format!("Model: {}", cpu.0)).strong());
                    ui.label(format!("Cores: {}", cpu_cores));
                    ui.add(ProgressBar::new(cpu.1 / 100.0)
                        .text(format!("Usage: {:.1}%", cpu.1))
                        .desired_width(250.0));
                });

            // RAM Info and Usage
            egui::CollapsingHeader::new("Memory")
                .default_open(false)
                .show(ui, |ui| {
                    let total_memory = self.system.total_memory();
                    let used_memory = self.system.used_memory();
                    let ram_usage = (used_memory as f64 / total_memory as f64) * 100.0;

                    ui.add_space(5.0);
                    ui.label(RichText::new(format!("Total: {:.1} GB", total_memory as f64 / 1_073_741_824.0)).strong());
                    ui.label(format!("Used: {:.1} GB ({:.1}%)", used_memory as f64 / 1_073_741_824.0, ram_usage));
                    ui.add(ProgressBar::new(ram_usage as f32 / 100.0)
                        .text(format!("{:.1}%", ram_usage))
                        .desired_width(250.0));
                });

            // GPU Info (from Vulkan)
            egui::CollapsingHeader::new("GPU")
                .default_open(false)
                .show(ui, |ui| {
                    let gpu_name = self.queue.device().physical_device().properties().device_name.to_string();
                    ui.add_space(5.0);
                    ui.label(RichText::new(format!("Model: {}", gpu_name)).strong());
                });

            // Game Specifics Header
            ui.heading("Game Specifics");
        });
    }
}
