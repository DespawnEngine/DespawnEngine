use crate::engine::{rendering::vswapchain::IMAGE_FORMAT, ui::debug_ui};
use crate::engine::ui::debug_ui::DebugUi;
use egui_winit_vulkano::{
    egui::{self, FontDefinitions},
    Gui, GuiConfig,
};
use std::sync::Arc;
use std::time::{Duration, Instant};
use sysinfo::System;
use vulkano::{
    command_buffer::SecondaryAutoCommandBuffer, device::Queue, render_pass::Subpass,
    swapchain::Surface,
};
use winit::{event::WindowEvent, event_loop::ActiveEventLoop};

pub struct EguiStruct {
    gui: Gui,
    pub system: System,
    queue: Arc<Queue>, // For GPU info
    last_update: Instant,
    update_interval: Duration, // frequently to update the UI window
    debug_ui: DebugUi,
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


        let debug_ui = DebugUi::new(queue.clone());

        EguiStruct {
            gui: egui,
            system,
            queue,
            last_update: Instant::now(),
            update_interval: Duration::from_secs_f64(0.5),
            debug_ui,
        }
    }

    pub fn update(&mut self, event: &WindowEvent) {
        self.gui.update(event);
    }

    pub fn redraw(&mut self) {
        if self.last_update.elapsed() >= self.update_interval {
            self.system.refresh_all();
            self.last_update = Instant::now();
        }

        self.gui.immediate_ui(|gui| {
            let ctx = gui.context();
            self.debug_ui.render(&ctx, &self.system);
        });
    }

    pub fn draw_on_subpass_image(
        &mut self,
        image_dimensions: [u32; 2],
    ) -> Arc<SecondaryAutoCommandBuffer> {
        self.gui.draw_on_subpass_image(image_dimensions)
    }
}
