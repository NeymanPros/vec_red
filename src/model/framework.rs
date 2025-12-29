use iced::{mouse, Point, Rectangle, Renderer, Theme};
use iced::event::Status;
use iced::mouse::{Cursor, Interaction};
use iced::widget::canvas;
use iced::widget::canvas::{Event, Geometry};
use libloading::Library;
use crate::Message;
use crate::app_settings::AppSettings;
use super::model_main::Model;
use super::drawing::Drawing;


/// Canvas, that draws a model
pub struct Framework<'a> {
    pub state: &'a State,
    pub model: &'a Model,
    pub scale: f32,
    pub app_settings: &'a AppSettings,
    pub mode: &'static str,
    
    pub lib: &'a Option<Library>
}


impl canvas::Program<Message> for Framework<'_> {
    type State = Drawing;

    fn update(&self, state: &mut Self::State, event: Event, bounds: Rectangle, cursor: Cursor) -> (Status, Option<Message>) {
        if self.mode == state.as_str() {
        } else if self.mode == "Line" {
            match *state {
                Drawing::SelectPoint { point, num } => {
                    *state = Drawing::LinePoint { point, num: Some(num) }
                }
                Drawing::ArcPoint { point, num} => {
                    *state = Drawing::LinePoint { point, num }
                }
                _ => {}
            }
        } else if self.mode == "Arc" {
            match *state {
                Drawing::SelectPoint { point, num } => {
                    *state = Drawing::ArcPoint { point, num: Some(num)}
                }
                Drawing::LinePoint { point, num} => {
                    *state = Drawing::ArcPoint { point, num }
                }
                _ => {}
            }
        } else {
            match *state {
                Drawing::LinePoint { .. } | Drawing::ArcPoint { .. } | Drawing::ArcTwoPoints { .. } => { *state = Drawing::None {} }
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
                self.simple_mouse_events(state, mouse_event, cursor_pos)
            }
            _ => (Status::Ignored, None),
        }
    }

    fn draw(&self, state: &Self::State, renderer: &Renderer, _theme: &Theme, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
        let content = self.state.cache.draw(renderer, bounds.size(), |frame| {
            self.model.draw_model(frame, self.scale, &self.app_settings, &self.lib);
        });

        vec![content, state.editing(&self.model.points, renderer, bounds, cursor, self.scale, &self.app_settings.zoom)]
    }
    
    fn mouse_interaction(&self, _state: &Self::State, bounds: Rectangle, cursor: Cursor) -> Interaction {
        if cursor.is_over(bounds) {
            Interaction::Crosshair
        } else {
            Interaction::default()
        }
    }
}

impl Framework<'_> {
    fn simple_mouse_events(&self, state: &mut Drawing, mouse_event: mouse::Event, cursor_pos: Point) -> (Status, Option<Message>){
        let real_cursor = self.app_settings.zoom.reverse(cursor_pos);
        let message = match mouse_event {
            mouse::Event::ButtonPressed(mouse::Button::Left) => {
                if state.as_str() != self.mode {
                    *state = match self.mode {
                        "Point" | "Region" => { Drawing::Point {} }
                        "Line" => { Drawing::Line {} }
                        "Arc" => { Drawing::Arc }
                        _ => Drawing::None {}
                    };
                }
                match *state {
                    Drawing::Point {} => {
                        let a = self.model.find_point(real_cursor, self.scale, self.app_settings.zoom.scale);
                        *state = Drawing::SelectPoint { point: real_cursor, num: a };
                        Some(Message::DefPoint(real_cursor))
                    }
                    Drawing::Line {} => {
                        *state = Drawing::LinePoint { point: real_cursor, num: None };

                        None
                    }
                    Drawing::LinePoint { mut point, num } => {
                        *state = Drawing::Line {};
                        if num.is_some() && num.unwrap() < self.model.points.len() {
                            point = self.model.points[num.unwrap()].0
                        }

                        Some(Message::DefPrim(vec![point, real_cursor], (0, 1, -1)))
                    }
                    Drawing::Arc {} => {
                        *state = Drawing::ArcPoint { point: real_cursor, num: None };

                        None
                    }
                    Drawing::ArcPoint { point, num } => {
                        *state = Drawing::ArcTwoPoints { point_one: point, num_one: num, point_two: real_cursor, num_two: None };

                        None
                    }
                    Drawing::ArcTwoPoints { mut point_one, mut point_two, num_one, num_two } => {
                        *state = Drawing::Arc {};

                        if num_one.is_some() && num_one.unwrap() < self.model.points.len() {
                            point_one = self.model.points[num_one.unwrap()].0
                        }
                        if num_two.is_some() && num_two.unwrap() < self.model.points.len() {
                            point_two = self.model.points[num_two.unwrap()].0
                        }
                        
                        Some(Message::DefPrim(vec![point_one, point_two, real_cursor], (0, 1, 2)))
                    }
                    _ => {
                        *state = Drawing::Scaling { starting_point: cursor_pos};
                        
                        Some(Message::DefUnselect)
                    }
                }
            }
            mouse::Event::ButtonReleased(mouse::Button::Left) => {
                if let Drawing::Scaling{ starting_point } = *state {
                    *state = Drawing::None {};
                    Some(Message::SetZoom(self.app_settings.zoom.reverse(starting_point), real_cursor, false))
                }
                else {
                    None
                }
            }
            mouse::Event::ButtonPressed(mouse::Button::Right) => {
                let a = self.model.find_point(real_cursor, self.scale, self.app_settings.zoom.scale);
                if a >= self.model.points.len() {
                    None
                } else {
                    match *state {
                        Drawing::LinePoint { point, .. } => {
                            *state = Drawing::Line {};
                            Some(Message::DefPrim(vec![point, real_cursor], (0, 1, -1)))
                        }

                        Drawing::ArcPoint { point, num } => {
                            *state = Drawing::ArcTwoPoints { point_one: point, num_one: num, point_two: self.model.points[a].0, num_two: Some(a) };

                            None
                        }

                        Drawing::ArcTwoPoints { point_one, point_two, .. } => {
                            *state = Drawing::Arc {};

                            Some(Message::DefPrim(vec![point_one, point_two, real_cursor], (0, 1, 2)))
                        }

                        _ => {
                            *state = Drawing::SelectPoint { point: self.model.points[a].0, num: a };
                            Some(Message::DefPoint(real_cursor))
                        }
                    }
                }
            }
            _ => None,
        };

        (Status::Captured, message)
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
