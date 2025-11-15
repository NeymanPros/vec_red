use iced::{mouse, Point, Rectangle, Renderer, Theme};
use iced::event::Status;
use iced::mouse::{Cursor, Interaction};
use iced::widget::canvas;
use iced::widget::canvas::{Event, Geometry};
use crate::Message;
use crate::app_settings::{AppSettings};
use crate::model_instruments::{Model, Drawing};


/// Canvas, that draws a model
pub struct Framework<'a> {
    pub state: &'a State,
    pub model: &'a Model,
    pub scale: f32,
    pub app_settings: &'a AppSettings,
    pub mode: &'static str
}

impl Framework<'_> {
    fn mouse_events(&self, state: &mut Drawing, mouse_event: mouse::Event, cursor_pos: Point) -> (Status, Option<Message>){
        let zoom = &self.app_settings.zoom;
        let message = match mouse_event {
            mouse::Event::ButtonPressed(mouse::Button::Left) => {
                if state.as_str() != self.mode {
                    *state = match self.mode {
                        "Dot" => { Drawing::Dot {} }
                        "Line" => { Drawing::Line {} }
                        "Arc" => { Drawing::Arc }
                        _ => Drawing::None {}
                    };
                }
                let real_cursor = zoom.reverse(cursor_pos);
                match *state {
                    Drawing::Dot {} => {
                        let a = self.model.find_point(real_cursor, self.scale);
                        *state = Drawing::SelectDot { dot: real_cursor, num: a };
                        Some(Message::DefDot(real_cursor))
                    }
                    Drawing::Line {} => {
                        *state = Drawing::LineDot { dot: real_cursor, num: None };

                        None
                    }
                    Drawing::LineDot { mut dot, num } => {
                        *state = Drawing::Line {};
                        if num.is_some() && num.unwrap() < self.model.dots.len() {
                            dot = self.model.dots[num.unwrap()].0
                        }

                        Some(Message::DefLine(vec![dot, real_cursor], (0, 1, -1)))
                    }
                    Drawing::Arc {} => {
                        *state = Drawing::ArcDot { dot: real_cursor, num: None };

                        None
                    }
                    Drawing::ArcDot { dot, num } => {
                        *state = Drawing::ArcTwoDots { dot_one: dot, num_one: num, dot_two: real_cursor, num_two: None };

                        None
                    }
                    Drawing::ArcTwoDots { mut dot_one, mut dot_two, num_one, num_two } => {
                        *state = Drawing::Arc {};

                        if num_one.is_some() && num_one.unwrap() < self.model.dots.len() {
                            dot_one = self.model.dots[num_one.unwrap()].0
                        }
                        if num_two.is_some() && num_two.unwrap() < self.model.dots.len() {
                            dot_two = self.model.dots[num_two.unwrap()].0
                        }


                        Some(Message::DefLine(vec![dot_one, dot_two, real_cursor], (0, 1, 2)))
                    }
                    _ => {
                        *state = Drawing::None {};
                        Some(Message::DefUnselect)
                    }
                }
            }
            mouse::Event::ButtonPressed(mouse::Button::Right) => {
                let real_cursor = zoom.reverse(cursor_pos);
                let a = self.model.find_point(real_cursor, self.scale);
                if a >= self.model.dots.len() {
                    None
                } else {
                    match *state {
                        Drawing::LineDot { dot, .. } => {
                            *state = Drawing::Line {};
                            Some(Message::DefLine(vec![dot, real_cursor], (0, 1, -1)))
                        }

                        Drawing::ArcDot { dot, num } => {
                            *state = Drawing::ArcTwoDots { dot_one: dot, num_one: num, dot_two: self.model.dots[a].0, num_two: Some(a) };

                            None
                        }

                        Drawing::ArcTwoDots { dot_one, dot_two, .. } => {
                            *state = Drawing::Arc {};

                            Some(Message::DefLine(vec![dot_one, dot_two, real_cursor], (0, 1, 2)))
                        }

                        _ => {
                            *state = Drawing::SelectDot { dot: self.model.dots[a].0, num: a };
                            Some(Message::DefDot(real_cursor))
                        }
                    }
                }
            }
            _ => None,
        };

        (Status::Captured, message)
    }
}

impl canvas::Program<Message> for Framework<'_> {
    type State = Drawing;

    fn update(&self, state: &mut Self::State, event: Event, bounds: Rectangle, cursor: Cursor) -> (Status, Option<Message>) {
        if self.mode == state.as_str() {
        } else if self.mode == "Line" {
            match *state {
                Drawing::SelectDot { dot, num } => {
                    *state = Drawing::LineDot { dot, num: Some(num) }
                }
                Drawing::ArcDot { dot, num} => {
                    *state = Drawing::LineDot { dot, num }
                }
                _ => {}
            }
        } else if self.mode == "Arc" {
            match *state {
                Drawing::SelectDot { dot, num } => {
                    *state = Drawing::ArcDot { dot, num: Some(num)}
                }
                Drawing::LineDot {dot, num} => {
                    *state = Drawing::ArcDot { dot, num }
                }
                _ => {}
            }
        } else {
            match *state {
                Drawing::LineDot { .. } | Drawing::ArcDot { .. } | Drawing::ArcTwoDots { .. } => { *state = Drawing::None {} }
                _ => {}
            }
        }

        match event {
            Event::Mouse(mouse_event) => {
                let Some(cursor_pos) = cursor.position_in(bounds) else {
                    return (Status::Ignored, None);
                };
                let cursor_pos = if self.app_settings.bound {
                    self.app_settings.grid.bound(&cursor_pos)
                }
                else {
                    cursor_pos
                };
                self.mouse_events(state, mouse_event, cursor_pos)
            }
            _ => (Status::Ignored, None),
        }
    }

    fn draw(&self, state: &Self::State, renderer: &Renderer, _theme: &Theme, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
        let content = self.state.cache.draw(renderer, bounds.size(), |frame| {
            self.model.draw_model(frame, self.scale, &self.app_settings);
        });

        vec![content, state.editing(&self.model.dots, renderer, bounds, cursor, self.scale, &self.app_settings.zoom)]
    }
    
    fn mouse_interaction(&self, _state: &Self::State, bounds: Rectangle, cursor: Cursor) -> Interaction {
        if cursor.is_over(bounds) {
            Interaction::Crosshair
        } else {
            Interaction::default()
        }
    }
}


/// [canvas::Cache], contains already drawn [Framework].
#[derive(Default)]
pub struct State {
    cache: canvas::Cache
}

impl State {
    ///Deletes cache and draws from zero.
    pub fn redraw(&mut self) {
        self.cache = canvas::Cache::new()
    }
}
