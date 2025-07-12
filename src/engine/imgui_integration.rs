use imgui::{Condition, Context, FontSource};
pub use imgui_winit_support::{HiDpiMode, WinitPlatform};
use winit::window::Window;

pub struct ImguiStruct {
    pub context: Context,
    pub platform: WinitPlatform,
}

impl ImguiStruct {
    pub fn new() -> ImguiStruct {
        let mut imgui_context: Context = Context::create();
        let platform: WinitPlatform = WinitPlatform::new(&mut imgui_context);
        let font_atlas = imgui_context.fonts();
        font_atlas.add_font(&[FontSource::DefaultFontData { config: None }]);
        font_atlas.build_rgba32_texture();
        ImguiStruct {
            context: imgui_context,
            platform,
        }
    }
    pub fn paltform_attach_window(&mut self, window: &Window) {
        self.platform
            .attach_window(self.context.io_mut(), window, HiDpiMode::Default);
    }

    pub fn draw_to_window(&mut self, window: &Window) {
        let ui = self.context.frame();
        ui.window("Hello world")
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(|| {
                ui.text("Hello world!");
                ui.text("こんにちは世界！");
                ui.text("This...is...imgui-rs!");
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
        self.platform.prepare_render(ui, window);
        let _draw_data = self.context.render();
    }

    pub fn prepare_frame(&mut self, window: &Window) {
        let _ = self.platform.prepare_frame(self.context.io_mut(), window);
    }
}
