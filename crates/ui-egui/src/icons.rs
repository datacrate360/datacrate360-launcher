//! Bundled images: the tools' icons and the launcher's own.

use egui::ImageSource;

/// The 128 px icon of a tool. A tool without an icon of its own shows the DataCrate360 mark.
pub fn app(id: &str) -> Option<ImageSource<'static>> {
    // One arm per tool with its own icon, e.g.
    // `"omni-agent" => egui::include_image!("../../../assets/app-icons/omni-agent.png"),`
    let _ = id;
    Some(datacrate360_mark())
}

/// The DataCrate360 mark (teal, on transparency): the header logo and the dialogs' icon.
pub fn datacrate360_mark() -> ImageSource<'static> {
    egui::include_image!("../../../assets/app-icon/mark-128.png")
}

/// A social service's logo, by its name in `catalog::SOCIAL`. None are bundled.
pub fn social(service: &str) -> Option<ImageSource<'static>> {
    let _ = service;
    None
}
