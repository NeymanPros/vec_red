mod excel_parse;
mod framework;
mod app_settings;
mod model_instruments;
mod undo_manager;
mod grid;

use crate::undo_manager::UndoManager;
use crate::app_settings::{AppSettings, Change};
use crate::framework::{State, Framework};
use crate::model_instruments::{Model};
use crate::excel_parse::{excel_dots, excel_lines};

use iced::{Point, Vector, Size, Fill, Center, keyboard, Subscription};
use iced::keyboard::{Key, key::Named};
use iced::widget::{Column, column, row, container, horizontal_space, text, text_editor, button, Slider, stack};



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
    default_circle: f32
}


/// Messages produced by [VecRed]
#[derive(Debug, Clone)]
enum Message {
    ChangeMode(&'static str),
    ClearAll,
    EditPath(text_editor::Action),
    Undo,
    Def(Model),
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

impl VecRed {
    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeMode(new_mode) => {
                self.mode = new_mode;
            }

            Message::EditPath(edited) => {
                self.path_to_excel.perform(edited)
            }

            Message::GetData => {
                //self.path_to_excel;
                self.model.dots = excel_dots();
                self.model.lines = excel_lines();
                self.mode = "Move";
                self.chosen_dot = None;
                self.journal.clear();
                self.state.redraw()
            }

            Message::Def(add_model) => {
                if add_model.dots.len() == 0 && add_model.lines.len() == 0 {
                    self.chosen_dot = None
                }
                else if add_model.dots.len() == 1 && add_model.lines.len() == 0 {
                    let a = self.model.find_point(add_model.dots[0].0, self.scale);
                    if a >= self.model.dots.len() {
                        self.journal.pushed_dot();
                        self.model.dots.push((add_model.dots[0].0, self.default_circle));
                        self.state.redraw();
                    }
                    self.chosen_dot = Some((self.model.dots[a].0, self.model.dots[a].1, a));
                    self.change_dot = [
                        text_editor::Content::with_text(self.chosen_dot.as_ref().unwrap().0.x.to_string().as_str()),
                        text_editor::Content::with_text(self.chosen_dot.as_ref().unwrap().0.y.to_string().as_str()),
                        text_editor::Content::with_text(self.chosen_dot.as_ref().unwrap().1.to_string().as_str())
                    ]
                } else if add_model.lines.len() == 1 && add_model.lines[0].2 == -1 {
                    let a = self.model.find_point(add_model.dots[0].0, self.scale);
                    if a == self.model.dots.len() {
                        self.journal.pushed_dot();
                        self.model.dots.push((add_model.dots[0].0, self.default_circle));
                    }
                    let b = self.model.find_point(add_model.dots[1].0, self.scale);
                    if b == self.model.dots.len() {
                        self.journal.pushed_dot();
                        self.model.dots.push((add_model.dots[1].0, self.default_circle));
                    }
                    if a != b {
                        self.journal.pushed_line();
                        self.model.lines.push((a as i32, b as i32, -1))
                    }
                    self.state.redraw();
                    self.chosen_dot = None
                } else if add_model.lines.len() == 1 {
                    let a = self.model.find_point(add_model.dots[0].0, self.scale);
                    if a >= self.model.dots.len() {
                        self.journal.pushed_dot();
                        self.model.dots.push(add_model.dots[0])
                    }
                    let b = self.model.find_point(add_model.dots[1].0, self.scale);
                    if b >= self.model.dots.len() {
                        self.journal.pushed_dot();
                        self.model.dots.push(add_model.dots[1])
                    }
                    let c = self.model.find_point(add_model.dots[2].0, self.scale);
                    if c >= self.model.dots.len() {
                        self.journal.pushed_dot();
                        self.model.dots.push(add_model.dots[2])
                    }

                    if a != b && a != c && b != c {
                        self.journal.pushed_line();
                        self.model.lines.push((a as i32, b as i32, c as i32))
                    }
                    self.state.redraw()
                }
            }

            Message::ChangeDot(num, action) => {
                self.change_dot[num].perform(action);
                if let Ok(new_value) = self.change_dot[num].text().trim().parse::<f32>() {
                    match num {
                        0 => {
                            self.chosen_dot.as_mut().unwrap().0.x = new_value;
                        }
                        1 => {
                            self.chosen_dot.as_mut().unwrap().0.y = new_value;
                        }
                        2 => {
                            self.chosen_dot.as_mut().unwrap().1 = new_value;
                        }
                        _ => {}
                    }
                }
            }

            Message::ChangeApply => {
                if !self.chosen_dot.is_none()  {
                    let num = self.chosen_dot.as_ref().unwrap().2;
                    if self.model.dots[num].0 != self.chosen_dot.as_ref().unwrap().0 || self.model.dots[num].1 != self.chosen_dot.as_ref().unwrap().1 {
                        self.journal.changed_dot(self.model.dots[num], num);
                        self.model.dots[num].0 = self.chosen_dot.as_ref().unwrap().0;
                        self.model.dots[num].1 = self.chosen_dot.as_ref().unwrap().1;
                        self.state.redraw();
                    }
                }
            }

            Message::EditScale(name, new_value) => {
                if name == "scale" {
                    self.scale = new_value;
                    self.state.redraw();
                }
                else if name == "circle" {
                    self.default_circle = new_value
                }
            }

            Message::DeleteDot => {
                let Some((_, _, num)) = self.chosen_dot else {
                    return
                };
                self.model.lines
                    .iter()
                    .enumerate()
                    .rev()
                    .for_each(|(placement, x)| {
                        if x.0 == num as i32 || x.1 == num as i32 || x.2 == num as i32 {
                            self.journal.deleted_line(x.clone(), placement)
                        }
                    });
                self.model.lines.retain(|x| { x.0 != num as i32 && x.1 != num as i32 && x.2 != num as i32 });
                if self.model.dots.len() >= 1 && num != self.model.dots.len() - 1 {
                    self.journal.deleted_dot(self.model.dots[num], num);
                    self.model.dots.swap_remove(num);
                }
                else {
                    self.journal.deleted_dot(self.model.dots[num], num);
                    self.model.dots.pop();
                }
                if num != self.model.dots.len() {
                    self.model.replace_line(self.model.dots.len(), num);
                }
                self.chosen_dot = None;
                self.mode = "Move";
                self.state.redraw();
            }

            Message::Undo => {
                self.journal.undo()(&mut self.model);
                self.mode = "Move";
                self.state.redraw()
            }

            Message::ClearAll => {
                self.model.dots.clear();
                self.model.lines.clear();
                self.journal.clear();
                self.chosen_dot = None;
                self.state.redraw()
            }

            Message::Resize(extent) => {
                if extent == 0.0 {
                    self.app_settings.zoom.scale = 1.0;
                    self.app_settings.zoom.shift = Vector::default()
                } else {
                    self.app_settings.zoom.scale *= extent
                }
                self.mode = "Move";
                self.state.redraw()
            }

            Message::Shift(add_shift) => {
                self.app_settings.zoom.shift = self.app_settings.zoom.shift + add_shift * (1.0 / self.app_settings.zoom.scale);
                self.state.redraw()
            }

            Message::SettingsOpen(new_value) => {
                self.app_settings.shown = new_value;
                if new_value == false {
                    self.mode = "Move";
                    self.state.redraw();
                    self.app_settings.grid.redraw()
                }
                else {
                    self.app_settings.update(Change::Open)
                }
            }

            Message::SettingsEdit(action) => {
                self.app_settings.update(action)
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        if self.app_settings.shown {
            self.app_settings.view()
        }
        else {
            let blueprint = container(
                iced::widget::canvas(Framework {
                    state: &self.state,
                    model: &self.model,
                    scale: self.scale,
                    app_settings: &self.app_settings,
                    mode: &self.mode
                })
                    .width(Fill)
                    .height(Fill)
            )
                .width(Fill)
                .height(Fill);

            let grid = container(self.app_settings.grid.view())
                .width(Fill).height(Fill);

            row![stack![grid, blueprint], horizontal_space().height(Fill).width(10.0), self.side_panel().width(200).height(Fill)].into()
        }
    }
}

impl VecRed {
    fn side_panel(&self) -> Column<'_, Message> {
        let mode = iced::widget::PickList::new(self.modes, Some(self.mode), Message::ChangeMode);
        let for_path = text_editor(&self.path_to_excel).on_action(Message::EditPath);
        let get_data = button("Hello").on_press(Message::GetData);
        let clear_frame = button("Clear all").on_press(Message::ClearAll);
        let num = match self.chosen_dot {
            None => { String::from("") }
            _ => { self.chosen_dot.unwrap().2.to_string() }
        };
        let mut dot_info: Column<Message> = Column::new();
        if num != String::from("") {
            dot_info = self.about_dot(num);
        }
        let change_scale: Slider<f32, Message> = Slider::new(0.5..=20.0, self.scale, |x| Message::EditScale("scale", x)).step(0.25);
        let change_circle = Slider::new(self.scale..=100.0, self.default_circle, |x| Message::EditScale("circle", x)).step(1.0);
        let undo_button = button("Undo").on_press(Message::Undo);
        let settings = button("Settings").on_press(Message::SettingsOpen(true));

        column!(mode, for_path, dot_info, get_data, text("Change scale"), change_scale, text("Change default circle"), change_circle,
            clear_frame, undo_button, settings).spacing(5).align_x(Center)
    }

    fn about_dot(&self, num: String) -> Column<'_, Message> {
        let dot_number = text("Number of dot: ".to_owned() + num.as_str());
        let dot_x = row![text("X: "), text_editor(&self.change_dot[0]).on_action(|action| Message::ChangeDot(0, action))];
        let dot_y = row![text("Y: "), text_editor(&self.change_dot[1]).on_action(|action| Message::ChangeDot(1, action))];
        let dot_circle = row![text("R: "), text_editor(&self.change_dot[2]).on_action(|action| Message::ChangeDot(2, action))];
        let dot_apply = row![button("Apply").on_press(Message::ChangeApply)];
        let dot_delete = row![button("Delete").on_press(Message::DeleteDot)];
        column![dot_number, dot_x, dot_y, dot_circle, dot_apply, dot_delete].align_x(Center)
    }
    fn subscription(&self) -> Subscription<Message> {
        let keyboard_events = keyboard::on_key_press(|a, b| {
            Self::shortcuts(a, b)
        });
        Subscription::batch(vec![keyboard_events])
    }

    fn shortcuts (key: Key, modifiers: keyboard::Modifiers) -> Option<Message> {
        if modifiers.is_empty() {
            return match key {
                Key::Named(Named::Delete) => {
                    Some(Message::DeleteDot)
                }
                Key::Named(Named::ArrowLeft) => {
                    Some(Message::Shift(Vector::new(100.0, 0.0)))
                }
                Key::Named(Named::ArrowRight) => {
                    Some(Message::Shift(Vector::new(-100.0, 0.0)))
                }
                Key::Named(Named::ArrowUp) => {
                    Some(Message::Shift(Vector::new(0.0, 100.0)))
                }
                Key::Named(Named::ArrowDown) => {
                    Some(Message::Shift(Vector::new(0.0, -100.0)))
                }
                _ => { None }
            }
        }
        match modifiers {
            keyboard::Modifiers::SHIFT => {
                match key {
                    Key::Named(Named::ArrowLeft) => {
                        Some(Message::Shift(Vector::new(10.0, 0.0)))
                    }
                    Key::Named(Named::ArrowRight) => {
                        Some(Message::Shift(Vector::new(-10.0, 0.0)))
                    }
                    Key::Named(Named::ArrowUp) => {
                        Some(Message::Shift(Vector::new(0.0, 10.0)))
                    }
                    Key::Named(Named::ArrowDown) => {
                        Some(Message::Shift(Vector::new(0.0, -10.0)))
                    }
                    _ => { None }
                }
            }
            keyboard::Modifiers::CTRL => {
                match key.as_ref() {
                    Key::Character("z") => {
                        Some(Message::Undo)
                    }
                    Key::Character("=") => {
                        Some(Message::Resize(1.1))
                    }
                    Key::Character("-") => {
                        Some(Message::Resize(0.9))
                    }
                    _ => { None }
                }
            }
            _ => { None }
        }
    }
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
            default_circle: 20.0
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
