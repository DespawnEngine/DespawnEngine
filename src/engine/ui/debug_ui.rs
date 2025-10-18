use egui::{
    self, Color32, Context, CornerRadius, FontData, FontDefinitions, FontFamily, FontTweak, Label,
    Margin, ProgressBar, RichText, Shadow, Stroke,
};
use egui_plot::{Line, Plot, PlotPoints, uniform_grid_spacer};
use serde_json::de;
use std::{collections::VecDeque, sync::Arc, time::Duration};
use sysinfo::{Process, System};
use vulkano::device::Queue;

use crate::engine::core::input;

type MemoryUsageBytes = u64;
type CpuUsagePercent = f64;

const AMOUNT_OF_STATS_SAVED: u64 = 1024;

pub struct DebugUi {
    pub queue: Arc<Queue>,
    fonts: FontDefinitions,
    mem_usage_stats: VecDeque<Option<MemoryUsageBytes>>,
    cpu_usage_stats: VecDeque<Option<CpuUsagePercent>>,
    fps_stats: VecDeque<Option<f32>>,
}

impl DebugUi {
    pub fn new(queue: Arc<Queue>) -> Self {
        let mut mem_usage: VecDeque<Option<MemoryUsageBytes>> =
            VecDeque::with_capacity(AMOUNT_OF_STATS_SAVED as usize);
        mem_usage.resize_with(AMOUNT_OF_STATS_SAVED as usize, || None);

        let mut cpu_usage: VecDeque<Option<CpuUsagePercent>> =
            VecDeque::with_capacity(AMOUNT_OF_STATS_SAVED as usize);
        cpu_usage.resize_with(AMOUNT_OF_STATS_SAVED as usize, || None);

        let mut fps_stats: VecDeque<Option<f32>> =
            VecDeque::with_capacity(AMOUNT_OF_STATS_SAVED as usize);
        fps_stats.resize_with(AMOUNT_OF_STATS_SAVED as usize, || None);

        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "default".into(),
            Arc::new(FontData::from_static(include_bytes!(
                "../../../assets/fonts/default.ttf"
            ))),
        );

        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "default".to_owned());

        DebugUi {
            queue,
            fonts,
            mem_usage_stats: mem_usage,
            cpu_usage_stats: cpu_usage,
            fps_stats,
        }
    }

    // has an implicit lifetime due to using a System ref. Doesn't do much though.
    pub fn render(&mut self, ctx: &Context, system: &System, delta_time: Duration) {
        ctx.set_fonts(self.fonts.clone());

        let fps: f32 = 1.0 / delta_time.as_secs_f64() as f32;

        egui::TopBottomPanel::top("Fps Counter")
            .frame(egui::Frame {
                inner_margin: Margin::ZERO,
                outer_margin: Margin::ZERO,
                fill: Color32::from_rgba_premultiplied(0, 0, 0, 0),
                stroke: Stroke::NONE,
                corner_radius: CornerRadius::ZERO,
                shadow: Shadow::NONE,
            })
            .show_separator_line(false)
            .show(ctx, |ui| {
                let fps_label = Label::new(format!("FPS: {}", fps.trunc())).selectable(false);
                ui.add(fps_label);
            });

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

                egui::scroll_area::ScrollArea::vertical().show(ui, |ui| {
                    egui::CollapsingHeader::new("CPU")
                        .default_open(false)
                        .show(ui, |ui| {
                            let cpu = system
                                .cpus()
                                .first()
                                .map(|cpu| (cpu.brand(), cpu.cpu_usage()))
                                .unwrap_or(("Unknown", 0.0));
                            // let cpu_cores = system.cpus().len();

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
                                .show_x(false)
                                .y_axis_label("Percentage")
                                .allow_boxed_zoom(false)
                                .allow_double_click_reset(false)
                                .x_grid_spacer(uniform_grid_spacer(|_| {
                                    [
                                        AMOUNT_OF_STATS_SAVED as f64 / 16.0,
                                        AMOUNT_OF_STATS_SAVED as f64 / 4.0,
                                        AMOUNT_OF_STATS_SAVED as f64 / 2.0,
                                    ]
                                }))
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
                                .show_x(false)
                                .y_axis_label("Percentage")
                                .allow_boxed_zoom(false)
                                .allow_double_click_reset(false)
                                .x_grid_spacer(uniform_grid_spacer(|_| {
                                    [
                                        AMOUNT_OF_STATS_SAVED as f64 / 16.0,
                                        AMOUNT_OF_STATS_SAVED as f64 / 4.0,
                                        AMOUNT_OF_STATS_SAVED as f64 / 2.0,
                                    ]
                                }))
                                .default_y_bounds(0.0, 100.0)
                                .default_x_bounds(0.0, AMOUNT_OF_STATS_SAVED as f64)
                                .show(ui, |plot_ui| {
                                    plot_ui.line(Line::new("Memory Usage Percentage", mem_usage))
                                });
                        });

                    self.update_fps_stats(fps);

                    egui::CollapsingHeader::new("FPS")
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.add_space(5.0);
                            ui.label(RichText::new(format!("Current FPS: {fps:.1}")).strong());

                            let max_fps = self
                                .fps_stats
                                .clone()
                                .into_iter()
                                .map(|input| input.unwrap_or_default() as u16)
                                .max()
                                .unwrap() as f32;

                            let fps_over_time: PlotPoints = self
                                .fps_stats
                                .clone()
                                .into_iter()
                                .enumerate()
                                .map(|(index, value)| {
                                    let x = value.unwrap_or_default();
                                    [index as f64, x as f64]
                                })
                                .collect();
                            Plot::new("my_plot")
                                .view_aspect(2.0)
                                .allow_zoom(false)
                                .allow_scroll(false)
                                .allow_drag(false)
                                .show_x(false)
                                .y_axis_label("FPS")
                                .allow_boxed_zoom(false)
                                .allow_double_click_reset(false)
                                .x_grid_spacer(uniform_grid_spacer(|_| {
                                    [
                                        AMOUNT_OF_STATS_SAVED as f64 / 16.0,
                                        AMOUNT_OF_STATS_SAVED as f64 / 4.0,
                                        AMOUNT_OF_STATS_SAVED as f64 / 2.0,
                                    ]
                                }))
                                .default_y_bounds(0.0, max_fps as f64 * 1.2)
                                .default_x_bounds(0.0, AMOUNT_OF_STATS_SAVED as f64)
                                .show(ui, |plot_ui| plot_ui.line(Line::new("FPS", fps_over_time)));
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
    fn update_fps_stats(&mut self, current_fps: f32) {
        self.fps_stats.pop_front();
        self.fps_stats.extend([Some(current_fps)]);
    }
}
