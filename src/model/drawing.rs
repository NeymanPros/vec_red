use iced::{Color, Point, Rectangle, Renderer};
use iced::mouse::Cursor;
use iced::widget::canvas;
use iced::widget::canvas::{Geometry, Path, Stroke};
use crate::app_settings::Zoom;
use crate::model::model_main::Model;

/// Is used to work with [Model] elements
#[derive(Debug, Clone, Copy)]
pub enum Drawing {
    None {},
    Line {},
    LineDot { dot: Point, num: Option<usize> },
    Arc,
    ArcDot { dot: Point, num: Option<usize>},
    ArcTwoDots { dot_one: Point, num_one: Option<usize>, dot_two: Point, num_two: Option<usize> },
    Dot {},
    SelectDot { dot: Point, num: usize },
    Scaling {starting_dot: Point}
}

impl Default for Drawing {
    fn default() -> Self {
        Drawing::None {}
    }
}

impl Drawing {
    pub fn editing(&self, dots: &Vec<(Point, f32)>, renderer: &Renderer, bounds: Rectangle, cursor: Cursor, scale: f32, zoom: &Zoom) -> Geometry {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        match *self {
            Self::LineDot { dot, num } => {
                let cursor_pos = cursor.position_in(bounds).unwrap_or(dot);
                Self::default_point(&mut frame, zoom, scale, dots, dot, num);
                if num.is_none() || num.unwrap() >= dots.len() {
                    frame.stroke(&Path::line(zoom.apply(dot), cursor_pos),
                                 Stroke::default().with_width(2.0 * scale).with_color(Color::from_rgb8(255, 0, 0)));
                }
                else {
                    frame.stroke(&Path::line(zoom.apply(dots[num.unwrap()].0), cursor_pos),
                                 Stroke::default().with_width(scale).with_color(Color::from_rgb8(255, 0, 0)));
                }
            }
            Self::SelectDot { dot, num } => {
                if num < dots.len() {
                    Self::default_point(&mut frame, zoom, scale, dots, dot, Some(num));
                }
            }
            Self::ArcDot {dot, num} => {
                Self::default_point(&mut frame, zoom, scale, dots, dot, num)
            }
            Self::ArcTwoDots {dot_one, num_one, dot_two, num_two} => {
                Self::default_point(&mut frame, zoom, scale, dots, dot_one, num_one);
                Self::default_point(&mut frame, zoom, scale, dots, dot_two, num_two)
            }
            Self::Scaling {starting_dot} => {
                let cursor_pos = cursor.position_in(bounds).unwrap_or(starting_dot);
                let path = Path::new(|builder| {
                    builder.move_to(starting_dot);
                    builder.line_to(Point{x: starting_dot.x, y: cursor_pos.y});
                    builder.line_to(cursor_pos);
                    builder.line_to(Point{x: cursor_pos.x, y: starting_dot.y});
                    builder.line_to(starting_dot);
                });
                frame.stroke(&path, Stroke::default().with_color(Color::from_rgb8(0, 32, 192)))
            }
            _ => {}
        };

        frame.into_geometry()
    }

    fn default_point(frame: &mut canvas::Frame, zoom: &Zoom, scale: f32, dots: &Vec<(Point, f32)>, dot: Point, num: Option<usize>) {
        let real_dot = match num {
            Some(index) => {
                if index < dots.len() {
                    dots[index].0
                } else {
                    dot
                }
            },
            None => dot,
        };
        frame.fill(&Path::circle(zoom.apply(real_dot), scale * 2.0), Color::from_rgb8(255, 0, 0));
    }
}

impl Drawing {
    pub fn as_str (&self) -> &'static str {
        match *self {
            Self::Dot {} => { "Dot" }
            Self::Line {} | Self::LineDot { .. } => { "Line" }
            Self::Arc {} | Self::ArcDot { .. } | Self::ArcTwoDots { .. } => { "Arc" }
            _ => "Move"
        }
    }
}
