mod model;
mod app;
mod foreign_functions;
mod app_config;

use std::rc::Rc;
use app::undo_manager::UndoManager;
use app_config::app_config::{AppConfig, Change};
use model::framework::State;
use crate::model::Model;

use iced::{Point, Size, Vector};
use iced::widget::text_editor;

use libloading::Library;

/// Main event loop.
struct VecRed {
    path_to_load: text_editor::Content,
    journal: UndoManager,

    state: State,
    model: Model,

    /// Point, radius, number in dots.
    chosen_point: Option<(Point, f32, usize)>,
    /// Stands for X, Y, radius.
    change_point: [text_editor::Content; 3],

    modes: [&'static str; 5],
    mode: &'static str,

    app_config: AppConfig,
    scale: f32,
    default_circle: f32,

    lib: Option<Rc<Library>>
}

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
    ChangePoint(usize, text_editor::Action),
    ChangeApply,
    EditScale(&'static str, f32),
    DeletePoint,
    
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

impl Default for VecRed {
    fn default() -> Self {
        Self {
            journal: UndoManager::default(),
            path_to_load: text_editor::Content::default(),
            modes: ["Move", "Point", "Line", "Arc", "Region"],
            mode: "Move",
            chosen_point: None,
            change_point: [text_editor::Content::default(), text_editor::Content::default(), text_editor::Content::default()],

            state: State::default(),
            model: Model::default(),

            app_config: AppConfig::default(),
            scale: 1.0,
            default_circle: 20.0,

            lib: None
        }
    }
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
