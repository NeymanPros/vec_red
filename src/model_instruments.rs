use iced::{Color, Point, Rectangle, Renderer};
use iced::mouse::Cursor;
use iced::widget::canvas;
use iced::widget::canvas::{Geometry, Path, Stroke};
use crate::app_settings::{AppSettings, Zoom};

/// Tools to draw [Framework]
#[derive(Clone, Debug, Default)]
pub struct Model {
    pub dots: Vec<(Point, f32)>,
    pub lines: Vec<(i32, i32)>
}

impl Model {
    pub fn draw_model(model: &Self, frame: &mut canvas::Frame, scale: f32, app_settings: &AppSettings) {
        let mut point: Path = Path::circle(Point::new(0., 0.), 1.0);
        if app_settings.dots_show || app_settings.circles_show {
            model.dots.iter().for_each(|dot| {
                if app_settings.dots_show {
                    point = Path::circle(app_settings.zoom.multi_and_shift(dot.0), scale * 2.0);
                    frame.fill(&point, Color::BLACK);
                }
                if app_settings.circles_show {
                    point = Path::circle(app_settings.zoom.multi_and_shift(dot.0), dot.1 * app_settings.zoom.scale);
                    frame.stroke(&point, Stroke::default().with_color(Color::from_rgb8(0, 0, 255)).with_width(2.0))
                }
            });
        }
        if app_settings.primitives_show {
            let lines = Path::new(|p| {
                for i in &model.lines {
                    p.move_to(app_settings.zoom.multi_and_shift(model.dots[i.0 as usize].0));
                    p.line_to(app_settings.zoom.multi_and_shift(model.dots[i.1 as usize].0));
                }
            });
            frame.stroke(&lines, Stroke::default().with_color(Color::BLACK).with_width(scale * app_settings.zoom.scale));
        }
    }

    pub fn find_point(&self, dot: Point, scale: f32) -> usize {
        self.dots
            .iter()
            .position(|x| { x.0.distance(dot) < scale * 2.0 })
            .unwrap_or(self.dots.len())
    }

    pub fn replace_primitive(&mut self, from: usize, to: usize) {
        self.lines.iter_mut().for_each(|x|{
            if x.0 == from as i32 {
                x.0 = to as i32;
            }
            if x.1 == from as i32 {
                x.1 = to as i32
            }
        })
    }
}

/// Is used to work with [Model] elements
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Drawing {
    Line {},
    LineDot { dot: Point, num: Option<usize> },
    Dot {},
    SelectDot { dot: Point, num: usize },
    None {}
}

impl Default for Drawing {
    fn default() -> Self {
        Drawing::None {}
    }
}

impl Drawing {
    pub fn editing(&self, dots: &Vec<(Point, f32)>, renderer: &Renderer, bounds: Rectangle, cursor: Cursor, scale: f32, zoom: &Zoom) -> Geometry<Renderer> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        match *self {
            Drawing::LineDot { dot, num } => {
                let cursor_pos = cursor.position_in(bounds).unwrap_or(dot);
                if num.is_none() || num.unwrap() >= dots.len() {
                    frame.fill(&Path::circle(zoom.multi_and_shift(dot), scale * 2.0), Color::from_rgb8(255, 0, 0));
                    frame.stroke(&Path::line(zoom.multi_and_shift(dot), cursor_pos),
                                 Stroke::default().with_width(2.0 * scale).with_color(Color::from_rgb8(255, 0, 0)));
                }
                else {
                    frame.fill(&Path::circle(zoom.multi_and_shift(dots[num.unwrap()].0), scale * 2.0), Color::from_rgb8(255, 0, 0));
                    frame.stroke(&Path::line(zoom.multi_and_shift(dots[num.unwrap()].0), cursor_pos),
                                 Stroke::default().with_width(scale).with_color(Color::from_rgb8(255, 0, 0)));
                }
            }
            Drawing::SelectDot { dot: _dot, num } => {
                if num < dots.len() {
                    frame.fill(&Path::circle(zoom.multi_and_shift(dots[num].0), scale * 2.0), Color::from_rgb8(255, 0, 0));
                }
            }
            _ => {}
        };

        frame.into_geometry()
    }
}

impl Drawing {
    pub fn as_str (&self) -> &'static str {
        match *self {
            Drawing::Dot {} => { "Dot" }
            Drawing::Line {} | Drawing::LineDot { .. } => { "Line" }
            _ => "Move"
        }
    }
}
