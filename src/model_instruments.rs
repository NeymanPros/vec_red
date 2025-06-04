use iced::{Color, Point, Rectangle, Renderer};
use iced::mouse::Cursor;
use iced::widget::canvas;
use iced::widget::canvas::{Geometry, Path, Stroke};
use iced::widget::canvas::path::Arc;
use crate::app_settings::{AppSettings, Zoom};

/// Tools to draw [Framework]
#[derive(Clone, Debug, Default)]
pub struct Model {
    pub dots: Vec<(Point, f32)>,
    pub lines: Vec<(i32, i32, i32)>
}

impl Model {
    pub fn draw_model(&self, frame: &mut canvas::Frame, scale: f32, app_settings: &AppSettings) {
        let mut point: Path = Path::circle(Point::new(0., 0.), 1.0);
        if app_settings.dots_show || app_settings.circles_show {
            self.dots.iter().for_each(|dot| {
                if app_settings.dots_show {
                    point = Path::circle(app_settings.zoom.apply(dot.0), scale * 2.0);
                    frame.fill(&point, Color::BLACK);
                }
                if app_settings.circles_show {
                    point = Path::circle(app_settings.zoom.apply(dot.0), dot.1 * app_settings.zoom.scale);
                    frame.stroke(&point, Stroke::default().with_color(Color::from_rgb8(0, 0, 255)).with_width(2.0))
                }
            });
        }
        if app_settings.lines_show {
            let lines = Path::new(|p| {
                for i in &self.lines {
                    if i.2 == -1 {
                        p.move_to(app_settings.zoom.apply(self.dots[i.0 as usize].0));
                        p.line_to(app_settings.zoom.apply(self.dots[i.1 as usize].0));
                    } else {
                        self.approx_arc(p, i, &app_settings.zoom)
                    }
                }
            });
            frame.stroke(&lines, Stroke::default().with_color(Color::BLACK).with_width(scale * app_settings.zoom.scale));
        }
    }

    fn approx_arc (&self, p: &mut canvas::path::Builder, i: &(i32, i32, i32), zoom: &Zoom) {
        let arc = Arc{
            center: zoom.apply(self.dots[i.2 as usize].0),
            radius: self.dots[i.2 as usize].0.distance(self.dots[i.1 as usize].0) * zoom.scale,
            start_angle: iced::Radians(0.0),
            end_angle: iced::Radians(360.0)
        };
        p.arc(arc);//  line_to(zoom.apply(self.dots[i.1 as usize].0));
    }
}

impl Model {
    pub fn find_point(&self, dot: Point, scale: f32) -> usize {
        self.dots
            .iter()
            .position(|x| { x.0.distance(dot) < scale * 2.0 })
            .unwrap_or(self.dots.len())
    }

    pub fn replace_line(&mut self, one: usize, two: usize) {
        self.lines.iter_mut().for_each(|x|{
            if x.0 == one as i32 {
                x.0 = two as i32
            } else if x.0 == two as i32 {
                x.0 = one as i32
            }

            if x.1 == one as i32 {
                x.1 = two as i32
            } else if x.1 == two as i32 {
                x.1 = one as i32
            }

            if x.2 == one as i32 {
                x.2 = two as i32
            } else if x.2 == two as i32 {
                x.2 = one as i32
            }
        })
    }
}

/// Is used to work with [Model] elements
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Drawing {
    Line {},
    LineDot { dot: Point, num: Option<usize> },
    Arc,
    ArcDot { dot: Point, num: Option<usize>},
    ArcTwoDots { dot_one: Point, num_one: Option<usize>, dot_two: Point, num_two: Option<usize> },
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
            Self::LineDot { dot, num } => {
                let cursor_pos = cursor.position_in(bounds).unwrap_or(dot);
                Self::default_dot(&mut frame, zoom, scale, dots, dot, num);
                if num.is_none() || num.unwrap() >= dots.len() {
                    frame.stroke(&Path::line(zoom.apply(dot), cursor_pos),
                                 Stroke::default().with_width(2.0 * scale).with_color(Color::from_rgb8(255, 0, 0)));
                }
                else {
                    frame.stroke(&Path::line(zoom.apply(dots[num.unwrap()].0), cursor_pos),
                                 Stroke::default().with_width(scale).with_color(Color::from_rgb8(255, 0, 0)));
                }
            }
            Self::SelectDot { dot, mut num } => {
                if num < dots.len() {
                    Self::default_dot(&mut frame, zoom, scale, dots, dot, Some(num));
                }
            }
            Self::ArcDot {dot, num} => {
                Self::default_dot(&mut frame, zoom, scale, dots, dot, num)
            }
            Self::ArcTwoDots {dot_one, num_one, dot_two, num_two} => {
                Self::default_dot(&mut frame, zoom, scale, dots, dot_one, num_one);
                Self::default_dot(&mut frame, zoom, scale, dots, dot_two, num_two)
            }
            _ => {}
        };

        frame.into_geometry()
    }

    fn default_dot(frame: &mut canvas::Frame, zoom: &Zoom, scale: f32, dots: &Vec<(Point, f32)>, dot: Point, num: Option<usize>) {
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
