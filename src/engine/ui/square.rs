use crate::{
    engine::ui::{native_gui_material::NativeGuiMaterial, native_gui_vertex::GuiVertex, ui_element::UiElement},
    utils::math::{Vec2, Vec3, Vec4},
};

pub struct SquareGuiElement {
    pub side_length: f32,
    pub position: Vec2,
    pub color: Vec4,
    pub depth: f32,
}

impl SquareGuiElement {
    pub fn new_from_rgb_a<P: Into<Vec2>, C: Into<Vec3>>(pos: P, col: C, alpha:f32, side_length: f32) -> Self {
        SquareGuiElement {
            side_length,
            position: pos.into(),
            color: (col.into(), alpha).into(),
            depth: 0.0,
        }
    }
}

impl UiElement for SquareGuiElement {
    fn get_pos(&self) -> Vec2 {
        self.position
    }
    fn get_mesh(&self) -> Vec<super::native_gui_vertex::GuiVertex> {
        let verts = [
            GuiVertex::new(
                [self.position.clone().x(), self.position.clone().y()],
                self.color,
                self.depth,
                [0.0, 0.0],
            ),
            GuiVertex::new(
                [
                    self.position.clone().x(),
                    self.position.clone().y() + self.side_length,
                ],
                self.color,
                self.depth,
                [0.0, 1.0],
            ),
            GuiVertex::new(
                [
                    self.position.clone().x() + self.side_length,
                    self.position.clone().y(),
                ],
                self.color,
                self.depth,
                [1.0, 0.0],
            ),
            GuiVertex::new(
                [
                    self.position.clone().x() + self.side_length,
                    self.position.clone().y() + self.side_length,
                ],
                self.color,
                self.depth,
                [1.0, 1.0],
            ),
        ];

        let index_order = [0, 1, 2, 1, 2, 3];

        index_order.iter().map(|&i| verts[i]).collect()
    }

    fn get_depth(&self) -> f32 {
        self.depth
    }
}
