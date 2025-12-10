use iced::{Color, Point};
use iced::widget::canvas;
use iced::widget::canvas::{Path, Stroke};
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

impl Model {
    pub fn export (&self) -> (
        Vec<[f32; 3]>, 
        Vec<[i32; 3]>, 
        Vec<[f32; 2]>, 
        Vec<[i32; 2]>
    ) 
    {
        let points: Vec<[f32; 3]> = self.dots
            .iter()
            .map(|&(point, r)| 
                [point.x, point.y, r]
            ).collect();

        let lines: Vec<[i32; 3]> = 
            self.lines.iter().map(|&l| l.into() ).collect();
        
        let node_dots: Vec<[f32; 2]> = self.node_dots
            .iter()
            .map(|&dots| dots.into())
            .collect();
        
        let node_lines: Vec<[i32; 2]> = self.node_lines
            .iter()
            .map(|&line| line.into())
            .collect();

        (points, lines, node_dots, node_lines)
    }
}

