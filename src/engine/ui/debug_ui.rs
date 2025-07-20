use egui_plot::{log_grid_spacer, uniform_grid_spacer, GridInput, Line, Plot, PlotPoints};
use egui_winit_vulkano::egui::{self, Context, FontDefinitions, ProgressBar, RichText};
use memory_stats::{self, MemoryStats};
use std::{collections::VecDeque, sync::Arc};
use sysinfo::System;
use vulkano::device::Queue;

use crate::engine::ui::egui_integration::EguiStruct;

pub struct DebugUi {
    pub queue: Arc<Queue>,
    mem_usage_stats: VecDeque<Option<MemoryStats>>,
}

impl DebugUi {
    pub fn new(queue: Arc<Queue>) -> Self {
        let mut mem_usage: VecDeque<Option<MemoryStats>> = VecDeque::with_capacity(128);
        mem_usage.resize_with(128, || None);

        DebugUi {
            queue,
            mem_usage_stats: mem_usage,
        }
    }

    // has an implicit lifetime due to using a System ref. Doesn't do much though.
    pub fn render(&mut self, ctx: &Context, system: &System) {
        
        ctx.set_fonts(FontDefinitions::default());

        egui::Window::new("Despawn Debugger")
            .resizable(true)
            .show(ctx, |ui| {
                // System Info Header
                ui.heading("System Information");

                // CPU Info and Usage
                egui::CollapsingHeader::new("CPU")
                    .default_open(false)
                    .show(ui, |ui| {
                        let cpu = system
                            .cpus()
                            .first()
                            .map(|cpu| (cpu.brand(), cpu.cpu_usage()))
                            .unwrap_or(("Unknown", 0.0));
                        let cpu_cores = system.cpus().len();

                        ui.add_space(5.0);
                        ui.label(RichText::new(format!("Model: {}", cpu.0)).strong());
                        ui.label(format!("Cores: {}", cpu_cores));
                        ui.add(
                            ProgressBar::new(cpu.1 / 100.0)
                                .text(format!("Usage: {:.1}%", cpu.1))
                                .desired_width(250.0),
                        );
                    });

                // RAM Info and Usage
                //
                self.update_memory_usage();

                let total_memory = system.total_memory();
                egui::CollapsingHeader::new("Memory")
                    .default_open(false)
                    .show(ui, |ui| {
                        ui.add_space(5.0);
                        ui.label(
                            RichText::new(format!(
                                "Total: {:.1} GiB",
                                total_memory as f64 / 1_073_741_824.0
                            ))
                            .strong(),
                        );

                        let mem_usage: PlotPoints = self
                            .mem_usage_stats
                            .clone()
                            .into_iter()
                            .enumerate()
                            .map(|(index, value)| {
                                let x = match value {
                                    Some(memory_data) => memory_data.physical_mem,
                                    None => 0_usize,
                                };
                                [index as f64, x as f64 / total_memory as f64 * 100.0]
                            })
                            .collect();
                        Plot::new("my_plot")
                            .view_aspect(2.0)
                            .allow_zoom(false)
                            .allow_scroll(false)
                            .allow_drag(false)
                            .allow_boxed_zoom(false)
                            .allow_double_click_reset(false)
                            .x_grid_spacer(uniform_grid_spacer(|_| {[4.0, 16.0, 64.0]}))
                            .default_y_bounds(0.0, 100.0)
                            .default_x_bounds(0.0, 128.0)
                            .show(ui, |plot_ui| {
                                plot_ui.line(Line::new("Memory Usage Percentage", mem_usage))
                            });
                    });

                // GPU Info (from Vulkan)
                egui::CollapsingHeader::new("GPU")
                    .default_open(false)
                    .show(ui, |ui| {
                        let gpu_name = self
                            .queue
                            .device()
                            .physical_device()
                            .properties()
                            .device_name
                            .to_string();
                        ui.add_space(5.0);
                        ui.label(RichText::new(format!("Model: {}", gpu_name)).strong());
                    });

                // Game Specifics Header
                ui.heading("Game Specifics");
            });
    }

    fn update_memory_usage(&mut self) {
        let latest_memory_stats = memory_stats::memory_stats();
        self.mem_usage_stats.pop_front();
        self.mem_usage_stats.extend(Some(latest_memory_stats));
    }
}
