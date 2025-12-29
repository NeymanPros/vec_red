use iced::{Color, Point, Rectangle, Renderer};
use iced::mouse::Cursor;
use iced::widget::canvas;
use iced::widget::canvas::{Geometry, Path, Stroke};
use crate::app_settings::Zoom;

/// Is used to work with [Model] elements
#[derive(Debug, Clone, Copy)]
pub enum Drawing {
    None {},
    Line {},
    LinePoint { point: Point, num: Option<usize> },
    Arc,
    ArcPoint { point: Point, num: Option<usize>},
    ArcTwoPoints { point_one: Point, num_one: Option<usize>, point_two: Point, num_two: Option<usize> },
    Point {},
    SelectPoint { point: Point, num: usize },
    Scaling { starting_point: Point}
}

impl Default for Drawing {
    fn default() -> Self {
        Drawing::None {}
    }
}

impl Drawing {
    pub fn editing(&self, points: &Vec<(Point, f32)>, renderer: &Renderer, bounds: Rectangle, cursor: Cursor, scale: f32, zoom: &Zoom) -> Geometry {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        match *self {
            Self::LinePoint { point, num } => {
                let cursor_pos = cursor.position_in(bounds).unwrap_or(point);
                Self::default_point(&mut frame, zoom, scale, points, point, num);
                if num.is_none() || num.unwrap() >= points.len() {
                    frame.stroke(&Path::line(zoom.apply(point), cursor_pos),
                                 Stroke::default().with_width(2.0 * scale).with_color(Color::from_rgb8(255, 0, 0)));
                }
                else {
                    frame.stroke(&Path::line(zoom.apply(points[num.unwrap()].0), cursor_pos),
                                 Stroke::default().with_width(scale).with_color(Color::from_rgb8(255, 0, 0)));
                }
            }
            Self::SelectPoint { point, num } => {
                if num < points.len() {
                    Self::default_point(&mut frame, zoom, scale, points, point, Some(num));
                }
            }
            Self::ArcPoint { point, num} => {
                Self::default_point(&mut frame, zoom, scale, points, point, num)
            }
            Self::ArcTwoPoints { point_one, num_one, point_two, num_two} => {
                Self::default_point(&mut frame, zoom, scale, points, point_one, num_one);
                Self::default_point(&mut frame, zoom, scale, points, point_two, num_two)
            }
            Self::Scaling { starting_point } => {
                let cursor_pos = cursor.position_in(bounds).unwrap_or(starting_point);
                let path = Path::new(|builder| {
                    builder.move_to(starting_point);
                    builder.line_to(Point{x: starting_point.x, y: cursor_pos.y});
                    builder.line_to(cursor_pos);
                    builder.line_to(Point{x: cursor_pos.x, y: starting_point.y});
                    builder.line_to(starting_point);
                });
                frame.stroke(&path, Stroke::default().with_color(Color::from_rgb8(0, 32, 192)))
            }
            _ => {}
        };

        frame.into_geometry()
    }

    fn default_point(frame: &mut canvas::Frame, zoom: &Zoom, scale: f32, points: &Vec<(Point, f32)>, point: Point, num: Option<usize>) {
        let real_point = match num {
            Some(index) => {
                if index < points.len() {
                    points[index].0
                } else {
                    point
                }
            },
            None => point,
        };
        frame.fill(&Path::circle(zoom.apply(real_point), scale * 2.0), Color::from_rgb8(255, 0, 0));
    }
}

impl Drawing {
    pub fn as_str (&self) -> &'static str {
        match *self {
            Self::Point {} => { "Point" }
            Self::Line {} | Self::LinePoint { .. } => { "Line" }
            Self::Arc {} | Self::ArcPoint { .. } | Self::ArcTwoPoints { .. } => { "Arc" }
            _ => "Move"
        }
    }
}
