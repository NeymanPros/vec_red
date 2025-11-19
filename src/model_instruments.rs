use iced::{Color, Point, Rectangle, Renderer};
use iced::mouse::Cursor;
use iced::widget::canvas;
use iced::widget::canvas::{Geometry, Path, Stroke};
use iced::widget::canvas::path::Arc;
use crate::app_settings::{AppSettings, Zoom};

/// Tools to draw [Framework].
#[derive(Clone, Debug, Default)]
pub struct Model {
    pub dots: Vec<(Point, f32)>,
    pub lines: Vec<(i32, i32, i32)>,
    pub node_dots: Vec<Point>,
    pub node_lines: Vec<(i32, i32)>
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
            frame.stroke(&lines, Stroke::default().with_color(Color::BLACK).with_width(scale));
        }
        if app_settings.node_dot_show {
            for node in &self.node_dots {
                let point = Path::circle(node.clone(), scale);
                frame.fill(&point, Color::from_rgb8(128, 128, 128));
            }
        }
        if app_settings.node_line_show {
        let lines = Path::new(|p| {
                for (start, end) in &self.node_lines {
                    p.move_to(app_settings.zoom.apply(self.node_dots[*start as usize]));
                    p.line_to(app_settings.zoom.apply(self.node_dots[*end as usize]));
                }
            });
            frame.stroke(&lines, Stroke::default().with_color(Color::from_rgb8(128, 128, 128)).with_width(scale / 2.0))
        }
    }

    fn approx_arc (&self, p: &mut canvas::path::Builder, i: &(i32, i32, i32), zoom: &Zoom) {
        let arc = Arc{
            center: zoom.apply(self.dots[i.2 as usize].0),
            radius: self.dots[i.2 as usize].0.distance(self.dots[i.1 as usize].0) * zoom.scale,
            start_angle: iced::Radians(0.0),
            end_angle: iced::Radians(360.0)
        };
        p.arc(arc);
    }
}

impl Model {
    pub fn find_point(&self, dot: Point, scale: f32) -> usize {
        self.dots
            .iter()
            .position(|x| { x.0.distance(dot) < scale * 2.0 })
            .unwrap_or(self.dots.len())
    }

    pub fn find_min_max(&self) -> (Point, Point) {
        if let Some(min) = self.dots.get(0) {
            let mut min = min.0;
            let mut max = min;

            for (point, _) in &self.dots {
                min.x = min.x.min(point.x);
                min.y = min.y.min(point.y);
                max.x = max.x.max(point.x);
                max.y = max.y.max(point.y);
            }

            (min, max)
        }
        else {
            (Point::new(0., 0.), Point::new(1000., 1000.))
        }
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

#[allow(dead_code)] 
impl Model {
    pub fn import (&mut self) {
        
    }
    pub fn export (&self) {
        let mut output = String::new();
        output += "p1\tp2\tr\n";
        for point in &self.dots {
            let p1 = point.0.x.to_string();
            let p2 = point.0.y.to_string();
            let p3 = point.1.to_string();
            output += (p1 + "\t" + p2.as_str() + "\t" + p3.as_str() + "\n").as_str();
        }
        output += "l1\tl2\tl3\n";
        for line in &self.lines {
            let l1 = line.0.to_string();
            let l2 = line.1.to_string();
            let l3 = line.2.to_string();
            output += (l1 + "\t" + l2.as_str() + "\t" + l3.as_str() + "\n").as_str();
        }
        
        println!("{}", output);
    }
}

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
