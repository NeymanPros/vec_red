mod excel_parse;
mod framework;
mod app_settings;
mod model_instruments;
mod undo_manager;
mod grid;
mod app;

use std::mem::MaybeUninit;
use crate::undo_manager::UndoManager;
use crate::app_settings::{AppSettings, Change};
use crate::framework::State;
use crate::model_instruments::{Model};

use iced::{Point, Vector, Size};
use iced::widget::text_editor;

use libloading::Library;

/// Main event loop.
struct VecRed {
    path_to_excel: text_editor::Content,
    journal: UndoManager,

    state: State,
    model: Model,

    ///Point, radius, number in dots.
    chosen_dot: Option<(Point, f32, usize)>,
    /// Stands for X, Y, radius.
    change_dot: [text_editor::Content; 3],

    modes: [&'static str; 4],
    mode: &'static str,

    app_settings: AppSettings,
    scale: f32,
    default_circle: f32,

    lib: Box<MaybeUninit<Library>>
}


/// Messages produced by [VecRed]
#[derive(Debug, Clone)]
enum Message {
    ChangeMode(&'static str),
    ClearAll,
    EditPath(text_editor::Action),
    Undo,
    
    DefDot(Point),
    DefLine(Vec<Point>, (i32, i32, i32)),
    DefUnselect,
    
    GetData,
    ChangeDot(usize, text_editor::Action),
    ChangeApply,
    EditScale(&'static str, f32),
    DeleteDot,
    Resize(f32),
    Shift(Vector),
    SettingsOpen(bool),
    SettingsEdit(Change)
}

impl Default for VecRed {
    fn default() -> Self {
        Self {
            journal: UndoManager::default(),
            path_to_excel: text_editor::Content::default(),
            modes: ["Move", "Dot", "Line", "Arc"],
            mode: "Move",
            chosen_dot: None,
            change_dot: [text_editor::Content::default(), text_editor::Content::default(), text_editor::Content::default()],

            state: State::default(),
            model: Model::default(),

            app_settings: AppSettings::default(),
            scale: 1.0,
            default_circle: 20.0,

            lib: Box::new_uninit()
        }
    }
}


fn main() -> iced::Result {
    iced::application("VecRed", VecRed::update, VecRed::view)
        .antialiasing(true)
        .window_size(Size::new(1200.0, 1000.0))
        .subscription(VecRed::subscription)
        .run()
}
