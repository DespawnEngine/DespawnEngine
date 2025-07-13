use std::sync::Arc;
use std::time::{Duration, Instant};
use egui_winit_vulkano::{
    egui::{self, FontDefinitions},
    Gui, GuiConfig,
};
use vulkano::{
    command_buffer::SecondaryAutoCommandBuffer, device::Queue, render_pass::Subpass,
    swapchain::Surface,
};
use winit::{
    event::WindowEvent,
    event_loop::{ActiveEventLoop},
};
use sysinfo::{System};

use crate::engine::vswapchain::IMAGE_FORMAT;

pub struct EguiStruct {
    gui: Gui,
    system: System,
    queue: Arc<Queue>, // For GPU info
    last_update: Instant,
    update_interval: Duration, // frequently to update the UI window
}

impl EguiStruct {
    pub fn new(
        event_loop: &ActiveEventLoop,
        surface: Arc<Surface>,
        queue: Arc<Queue>,
        subpass: Subpass,
    ) -> EguiStruct {
        let egui: Gui = Gui::new_with_subpass(
            event_loop,
            surface,
            queue.clone(),
            subpass,
            IMAGE_FORMAT,
            GuiConfig {
                allow_srgb_render_target: true,
                ..Default::default()
            },
        );
        let mut system = System::new_all();
        system.refresh_all(); // Initial refresh
        EguiStruct { gui: egui, system, queue, last_update: Instant::now(), update_interval: Duration::from_secs_f64(0.5) }
    }

    pub fn update(&mut self, event: &WindowEvent) {
        self.gui.update(event);
    }

    pub fn redraw(&mut self) {
        // Only refresh system info if enough time has passed
        if self.last_update.elapsed() >= self.update_interval {
            self.system.refresh_all();
            self.last_update = Instant::now();
        }


        let fonts: FontDefinitions = FontDefinitions::default();
        self.gui.immediate_ui(|gui| {
            let ctx = gui.context();
            ctx.set_fonts(fonts);
            egui::Window::new("Despawn Debugger").resizable(true).show(&ctx, |ui| {

                // System Info Header
                ui.heading("System Information");

                // CPU Info and Usage
                egui::CollapsingHeader::new("CPU")
                    .default_open(false)
                    .show(ui, |ui| {
                        let cpu = self.system.cpus().first().map(|cpu| (cpu.brand(), cpu.cpu_usage())).unwrap_or(("Unknown", 0.0));
                        let cpu_cores = self.system.cpus().len();
                        ui.add_space(5.0);
                        ui.label(egui::RichText::new(format!("Model: {}", cpu.0)).strong());
                        ui.label(format!("Cores: {}", cpu_cores));
                        ui.add(egui::ProgressBar::new(cpu.1 / 100.0)
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
                        ui.label(egui::RichText::new(format!("Total: {:.1} GB", total_memory as f64 / 1_073_741_824.0)).strong());
                        ui.label(format!("Used: {:.1} GB ({:.1}%)", used_memory as f64 / 1_073_741_824.0, ram_usage));
                        ui.add(egui::ProgressBar::new(ram_usage as f32 / 100.0)
                            .text(format!("{:.1}%", ram_usage))
                            .desired_width(250.0));
                    });

                // GPU Info (from Vulkan)
                egui::CollapsingHeader::new("GPU")
                    .default_open(false)
                    .show(ui, |ui| {
                        let gpu_name = self.queue.device().physical_device().properties().device_name.to_string();
                        ui.add_space(5.0);
                        ui.label(egui::RichText::new(format!("Model: {}", gpu_name)).strong());
                    });

                // Game Specifics Header
                ui.heading("Game Specifics");
            });
        })
    }

    pub fn draw_on_subpass_image(
        &mut self,
        image_dimensions: [u32; 2],
    ) -> Arc<SecondaryAutoCommandBuffer> {
        self.gui.draw_on_subpass_image(image_dimensions)
    }
}