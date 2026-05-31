use std::rc::Rc;
use iced::Point;
use iced::widget::text_editor;
use libloading::Library;
use crate::app::undo_manager::UndoManager;
use crate::app_config::AppConfig;
use crate::model::framework::State;
use crate::model::Model;

/// Main event loop.
pub struct VecRed {
    pub path_to_load: text_editor::Content,
    pub journal: UndoManager,

    pub state: State,
    pub model: Model,

    /// Point, radius, number in points.
    pub chosen_point: Option<(Point, f32, usize)>,
    
    pub point_string: Vec<String>,

    pub modes: [&'static str; 5],
    pub mode: &'static str,

    pub app_config: AppConfig,
    pub scale: f32,
    pub default_circle: f32,

    pub lib: Option<Rc<Library>>
}

impl Default for VecRed {
    fn default() -> Self {
        Self {
            journal: UndoManager::default(),
            path_to_load: text_editor::Content::default(),
            modes: ["Move", "Point", "Line", "Arc", "Region"],
            mode: "Move",
            chosen_point: None,
            
            point_string: vec![],
            
            state: State::default(),
            model: Model::default(),

            app_config: AppConfig::default(),
            scale: 1.0,
            default_circle: 20.0,

            lib: None
        }
    }
}
