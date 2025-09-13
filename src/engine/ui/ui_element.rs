use std::sync::Arc;

use crate::{engine::ui::native_gui_vertex::GuiVertex, utils::math::Vec2};


pub trait UiElement{
    fn get_pos(&self) -> Vec2;
    fn get_mesh(&self) -> Vec<GuiVertex>;
    fn get_depth(&self) -> f32;
}
