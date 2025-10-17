use fontdue::{
    Font,
    layout::{Layout, LayoutSettings, TextStyle},
};

use crate::engine::ui::ui_element::UiElement;

pub struct Textbox {
    pub contents: String,
    pub color: [f32; 4], //RGBA_SNORM
    //anchor: maybe coming?
    position: [f32; 2], //from top left
    depth: f32,
    fontdue_layout: Layout,
}

impl Textbox {
    pub fn new(text: String, position: [f32; 2]) -> Self {
        let font =
            include_bytes!("/home/onezoop/Projects/Despawn/DespawnEngine/assets/fonts/default.ttf")
                as &[u8];

        let font = Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();

        let fonts = &[font.clone()];

        let mut fontdue_layout = Layout::new(fontdue::layout::CoordinateSystem::PositiveYUp);

        // layout.reset(&LayoutSettings {
        //     ..LayoutSettings::default()
        // });

        fontdue_layout.append(fonts, &TextStyle::new(&text, 60.0, 0));

        // println!("{:?}", fontdue_layout.glyphs().first().unwrap().key.glyph_index);

        println!("{:?}", font.glyph_count());

        todo!();

        Textbox {
            contents: text,
            color: [0.0, 0.0, 0.0, 1.0],
            position,
            depth: 0.0,
            fontdue_layout,
        }
    }
}

impl UiElement for Textbox {
    fn get_pos(&self) -> crate::utils::math::Vec2 {
        self.position.into()
    }

    fn get_depth(&self) -> f32 {
        self.depth
    }

    fn get_mesh(&self) -> Vec<super::native_gui_vertex::GuiVertex> {
        todo!()
    }
}
