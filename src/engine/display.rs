use winit::window::Icon;
use image::GenericImageView; // "image" crate uses this for loading images

// This display script will contain almost all window functionality later, hopefully. Need to make sure I didn't break linux though first.

// Helper function for loading an icon for the window icon. Code will likely be changed, but I wanted to experiment to learn more.
pub fn load_icon(path: &str) -> Icon {
    // Load the image
    let image = image::open(path).expect("Failed to open icon file");

    let (width, height) = image.dimensions();
    let rgba = image.into_rgba8().into_raw(); // Convert to raw RGBA bytes

    // Create winit Icon
    Icon::from_rgba(rgba, width, height).expect("Failed to create icon")
}