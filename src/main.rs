mod model;
mod app;
mod foreign_functions;
mod app_config;

use app_config::app_config::Change;

use iced::{Point, Size, Vector};
use iced::widget::text_editor;
use app::core::VecRed;


/// Messages produced by [VecRed]
#[derive(Debug, Clone)]
enum Message {
    ChangeMode(&'static str),
    ClearAll,
    EditPath(text_editor::Action),
    Undo,
    
    DefPoint(Point),
    DefPrim(Vec<Point>, (i32, i32, i32)),
    DefUnselect,
    /// Flush model into a file.
    ExportModel,
    /// Load model from a file.
    OpenModel,
    EditScale(&'static str, f32),
    DeletePoint,

    ChangeApply,
    /// What, index, new_value, number of a field.
    ChangeParams(&'static str, usize, String, usize), 
    
    WindowResized(Size),
    ZoomScale(f32),
    ZoomShift(Vector),
    SetZoom(Point, Point, bool),
    
    ConfigOpen(bool),
    ConfigEdit(Change),

    OpenMathCore,
    CreateRegion(Point),
    CreateTriangle
}


fn main() -> iced::Result {
    let mut settings = iced::window::Settings::default();
    settings.min_size = Some(Size{width: 700., height: 500.});
    iced::application("VecRed", VecRed::update, VecRed::view)
        .antialiasing(true)
        .window_size(Size::new(1100.0, 900.0))
        .window(settings)
        .theme(|_| iced::Theme::Light)
        .subscription(VecRed::subscription)
        .run()
}
