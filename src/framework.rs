use iced::{Fill, mouse, Point, Rectangle, Renderer, Theme};
use iced::event::Status;
use iced::mouse::{Cursor, Interaction};
use iced::widget::canvas;
use iced::widget::canvas::{Event, Geometry};
use crate::app_settings::{AppSettings};
use crate::model_instruments::{Model, Drawing};


/// Canvas, that draws a model
pub struct Framework<'a> {
    state: &'a State,
    model: &'a Model,
    scale: f32,
    app_settings: &'a AppSettings,
    mode: &'static str
}

impl Framework<'_> {
    fn mouse_events(&self, state: &mut Drawing, mouse_event: mouse::Event, cursor_pos: Point) -> (Status, Option<Model>){
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
                match *state {
                    Drawing::Dot {} => {
                        let a = self.model.find_point(zoom.apply(cursor_pos), self.scale);
                        *state = Drawing::SelectDot { dot: zoom.reverse(cursor_pos), num: a };
                        Some(Model { dots: vec![(zoom.reverse(cursor_pos), 0.0)], lines: vec![] })
                    }
                    Drawing::Line {} => {
                        *state = Drawing::LineDot { dot: zoom.reverse(cursor_pos), num: None };

                        None
                    }
                    Drawing::LineDot { dot, num } => {
                        *state = Drawing::Line {};

                        Some(Model { dots: vec![(dot, 0.0), (zoom.reverse(cursor_pos), 0.0)], lines: vec![(0, 1, -1)] })
                    }
                    Drawing::Arc {} => {
                        *state = Drawing::ArcDot { dot: zoom.reverse(cursor_pos), num: None };

                        None
                    }
                    Drawing::ArcDot { dot, num } => {
                        *state = Drawing::ArcTwoDots { dot_one: dot, num_one: num, dot_two: zoom.reverse(cursor_pos), num_two: None };

                        None
                    }
                    Drawing::ArcTwoDots { dot_one, dot_two, .. } => {
                        *state = Drawing::Arc {};

                        Some(Model { dots: vec![(dot_one, 0.0), (dot_two, 0.0), (zoom.reverse(cursor_pos), 0.0)], lines: vec![(0, 1, 2)] })
                    }
                    _ => {
                        *state = Drawing::None {};
                        Some(Model { dots: vec![], lines: vec![] })
                    }
                }
            }
            mouse::Event::ButtonPressed(mouse::Button::Right) => {
                let a = self.model.find_point(zoom.reverse(cursor_pos), self.scale);
                if a >= self.model.dots.len() {
                    None
                } else {
                    match *state {
                        Drawing::LineDot { dot, .. } => {
                            *state = Drawing::Line {};
                            Some(Model { dots: vec![(dot, 0.0), (zoom.reverse(cursor_pos), 0.0)], lines: vec![(0, 1, -1)] })
                        }

                        Drawing::ArcDot { dot, num } => {
                            *state = Drawing::ArcTwoDots { dot_one: dot, num_one: num, dot_two: self.model.dots[a].0, num_two: Some(a) };

                            None
                        }

                        Drawing::ArcTwoDots { dot_one, dot_two, .. } => {
                            *state = Drawing::Arc {};

                            Some(Model {dots: vec![(dot_one, 0.0), (dot_two, 0.0), self.model.dots[a]], lines: vec![(0, 1, 2)]})
                        }

                        _ => {
                            *state = Drawing::SelectDot { dot: self.model.dots[a].0, num: a };
                            Some(Model { dots: vec![(zoom.reverse(cursor_pos), 0.0)], lines: vec![] })
                        }
                    }
                }
            }
            _ => None,
        };

        (Status::Captured, message)
    }
}

impl canvas::Program<Model> for Framework<'_> {
    type State = Drawing;

    fn update(&self, state: &mut Self::State, event: Event, bounds: Rectangle, cursor: Cursor) -> (Status, Option<Model>) {
        if self.mode == "Line" {
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
                self.mouse_events(state, mouse_event, cursor_pos)
            }
            _ => (Status::Ignored, None),
        }
    }

    fn draw(&self, state: &Self::State, renderer: &Renderer, _theme: &Theme, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
        let content = self.state.cache.draw(renderer, bounds.size(), |frame| {
            self.app_settings.grid.draw_grid(frame, &self.app_settings.zoom);
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
    cache: canvas::Cache,
}

impl State {
    pub fn view<'a>(&'a self, model: &'a Model, scale: f32, app_settings: &'a AppSettings, mode: &'static str) -> iced::Element<'a, Model> {
        canvas(Framework {
            state: self,
            model,
            scale,
            app_settings,
            mode
        })
            .width(Fill)
            .height(Fill)
            .into()
    }

    ///Deletes cache and draws from scratch.
    pub fn request_redraw(&mut self) {
        self.cache.clear();
    }
}
