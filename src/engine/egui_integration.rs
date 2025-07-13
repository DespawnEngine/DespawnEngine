use std::sync::Arc;
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
        EguiStruct { gui: egui, system, queue }
    }

    pub fn update(&mut self, event: &WindowEvent) {
        self.gui.update(event);
    }

    pub fn redraw(&mut self) {
        self.system.refresh_all(); // Refresh system info
        let fonts: FontDefinitions = FontDefinitions::default();
        self.gui.immediate_ui(|gui| {
            let ctx = gui.context();
            ctx.set_fonts(fonts);
            egui::Window::new("Despawn Debugger").resizable(true).show(&ctx, |ui| {
                ui.heading("System Information");

                // CPU Info and Usage
                let cpu = self.system.cpus().first().map(|cpu| (cpu.brand(), cpu.cpu_usage())).unwrap_or(("Unknown", 0.0));
                let cpu_cores = self.system.cpus().len();
                ui.horizontal(|ui| {
                    ui.label(format!("CPU: {} ({} cores, {:.1}% usage)", cpu.0, cpu_cores, cpu.1));
                });

                // RAM Info and Usage
                let total_memory = self.system.total_memory();
                let used_memory = self.system.used_memory();
                let ram_usage = (used_memory as f64 / total_memory as f64) * 100.0;
                ui.horizontal(|ui| {
                    ui.label(format!("RAM: {:.1} GB ({:.1}% used, {:.1} GB / {:.1} GB)",
                                     total_memory as f64 / 1_073_741_824.0,
                                     ram_usage,
                                     used_memory as f64 / 1_073_741_824.0,
                                     total_memory as f64 / 1_073_741_824.0));
                });

                // GPU Info (from Vulkan)
                let gpu_name = self.queue.device().physical_device().properties().device_name.to_string();
                ui.horizontal(|ui| {
                    ui.label(format!("GPU: {}", gpu_name));
                });
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