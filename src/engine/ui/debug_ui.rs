use egui::{self, Context, RichText, FontDefinitions, ProgressBar};
use sysinfo::System;
use std::sync::Arc;
use egui_plot::{uniform_grid_spacer, Line, Plot, PlotPoints};
use egui::{self, Context, FontDefinitions, RichText};
use std::{collections::VecDeque, sync::Arc};
use sysinfo::{Process, System};
use vulkano::device::Queue;

type MemoryUsageBytes = u64;
type CpuUsagePercent = f64;

const AMOUNT_OF_STATS_SAVED: u64 = 1024;

pub struct DebugUi {
    pub queue: Arc<Queue>,
    mem_usage_stats: VecDeque<Option<MemoryUsageBytes>>,
    cpu_usage_stats: VecDeque<Option<CpuUsagePercent>>,
}

impl DebugUi {
    pub fn new(queue: Arc<Queue>) -> Self {
        let mut mem_usage: VecDeque<Option<MemoryUsageBytes>> = VecDeque::with_capacity(AMOUNT_OF_STATS_SAVED as usize);
        mem_usage.resize_with(AMOUNT_OF_STATS_SAVED as usize, || None);

        let mut cpu_usage: VecDeque<Option<CpuUsagePercent>> = VecDeque::with_capacity(AMOUNT_OF_STATS_SAVED as usize);
        cpu_usage.resize_with(AMOUNT_OF_STATS_SAVED as usize, || None);

        DebugUi {
            queue,
            mem_usage_stats: mem_usage,
            cpu_usage_stats: cpu_usage,
        }
    }

    // has an implicit lifetime due to using a System ref. Doesn't do much though.
    pub fn render(&mut self, ctx: &Context, system: &System) {
        ctx.set_fonts(FontDefinitions::default());

        egui::Window::new("Despawn Debugger")
            .resizable(true)
            .show(ctx, |ui| {
                let proc = system
                    .process(sysinfo::get_current_pid().expect("Failed to get current Pid"))
                    .expect("Failed to get current process");

                // System Info Header
                ui.heading("System Information");

                // CPU Info and Usage

                self.update_cpu_usage(proc);

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

                        let cpu_usage: PlotPoints = self
                            .cpu_usage_stats
                            .clone()
                            .into_iter()
                            .enumerate()
                            .map(|(index, value)| {
                                let x = value.unwrap_or_default();
                                [index as f64, x]
                            })
                            .collect();
                        Plot::new("my_plot")
                            .view_aspect(2.0)
                            .allow_zoom(false)
                            .allow_scroll(false)
                            .allow_drag(false)
                            .allow_boxed_zoom(false)
                            .allow_double_click_reset(false)
                            .x_grid_spacer(uniform_grid_spacer(|_| [AMOUNT_OF_STATS_SAVED as f64 / 16.0, AMOUNT_OF_STATS_SAVED as f64 / 4.0, AMOUNT_OF_STATS_SAVED as f64 / 2.0]))
                            .default_y_bounds(0.0, 100.0)
                            .default_x_bounds(0.0, AMOUNT_OF_STATS_SAVED as f64)
                            .show(ui, |plot_ui| {
                                plot_ui.line(Line::new("Cpu Usage Percentage", cpu_usage))
                            });
                    });

                // RAM Info and Usage

                self.update_memory_usage(proc);

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
                                let x = value.unwrap_or_default();
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
                            .x_grid_spacer(uniform_grid_spacer(|_| [AMOUNT_OF_STATS_SAVED as f64 / 16.0, AMOUNT_OF_STATS_SAVED as f64 / 4.0, AMOUNT_OF_STATS_SAVED as f64 / 2.0]))
                            .default_y_bounds(0.0, 100.0)
                            .default_x_bounds(0.0, AMOUNT_OF_STATS_SAVED as f64)
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

    fn update_memory_usage(&mut self, process: &Process) {
        let latest_memory_stats: MemoryUsageBytes = process.memory();
        self.mem_usage_stats.pop_front();
        self.mem_usage_stats.extend([Some(latest_memory_stats)]);
    }
    fn update_cpu_usage(&mut self, process: &Process) {
        let latest_cpu_usage: CpuUsagePercent = process.cpu_usage() as f64;
        self.cpu_usage_stats.pop_front();
        self.cpu_usage_stats.extend([Some(latest_cpu_usage)]);
    }
}
