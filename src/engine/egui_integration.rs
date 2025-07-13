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
    event_loop::{ActiveEventLoop },
};

use crate::engine::vswapchain::IMAGE_FORMAT;

pub struct EguiStruct {
    gui: Gui,
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
            queue,
            subpass,
            IMAGE_FORMAT,
            GuiConfig {
                allow_srgb_render_target: true,
                ..Default::default()
            },
        );
        EguiStruct { gui: egui }
    }

    pub fn update(&mut self, event: &WindowEvent) {
        self.gui.update(event);
    }

    pub fn redraw(&mut self) {
        let fonts: FontDefinitions = FontDefinitions::default();
        self.gui.immediate_ui(|gui| {
            let ctx = gui.context();
            ctx.set_fonts(fonts);
            egui::Window::new("IT WORKS").show(&ctx, |ui| {
                ui.heading("Yeah I wrote this.");
                ui.horizontal(|ui| ui.label("This too. Its now just a matter of design (mostly?)"));
                ui.horizontal(|ui| ui.label("It wont resize, idk whats up with that tbh"));
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
