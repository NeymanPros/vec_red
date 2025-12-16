use iced::{Color, Point};
use iced::widget::canvas;
use iced::widget::canvas::{Path, Stroke};
use iced::widget::canvas::path::Arc;
use crate::app_settings::{AppSettings, Zoom};

/// Tools to draw [Framework].
#[derive(Clone, Debug, Default)]
pub struct Model {
    pub points: Vec<(Point, f32)>,
    pub prims: Vec<(i32, i32, i32)>,
    pub node_points: Vec<Point>,
    pub node_lines: Vec<(i32, i32)>
}

impl Model {
    pub fn draw_model(&self, frame: &mut canvas::Frame, scale: f32, app_settings: &AppSettings) {
        if app_settings.points_show || app_settings.circles_show {
            self.points.iter().for_each(|point| {
                if app_settings.points_show {
                    let dot = Path::circle(app_settings.zoom.apply(point.0), scale * 2.0);
                    frame.fill(&dot, Color::BLACK);
                }
                if app_settings.circles_show {
                    let dot = Path::circle(app_settings.zoom.apply(point.0), point.1 * app_settings.zoom.scale);
                    frame.stroke(&dot, Stroke::default().with_color(Color::from_rgb8(0, 0, 255)).with_width(2.0))
                }
            });
        }
        if app_settings.prims_show {
            let lines = Path::new(|p| {
                for i in &self.prims {
                    if i.2 == -1 {
                        p.move_to(app_settings.zoom.apply(self.points[i.0 as usize].0));
                        p.line_to(app_settings.zoom.apply(self.points[i.1 as usize].0));
                    } else {
                        self.approx_arc(p, i, &app_settings.zoom)
                    }
                }
            });
            frame.stroke(&lines, Stroke::default().with_color(Color::BLACK).with_width(scale));
        }
        if app_settings.node_point_show {
            for node in &self.node_points {
                let point = Path::circle(node.clone(), scale);
                frame.fill(&point, Color::from_rgb8(128, 128, 128));
            }
        }
        if app_settings.node_line_show {
        let lines = Path::new(|p| {
                for (start, end) in &self.node_lines {
                    p.move_to(app_settings.zoom.apply(self.node_points[*start as usize]));
                    p.line_to(app_settings.zoom.apply(self.node_points[*end as usize]));
                }
            });
            frame.stroke(&lines, Stroke::default().with_color(Color::from_rgb8(128, 128, 128)).with_width(scale / 2.0))
        }
    }

    fn approx_arc (&self, p: &mut canvas::path::Builder, i: &(i32, i32, i32), zoom: &Zoom) {
        let arc = Arc{
            center: zoom.apply(self.points[i.2 as usize].0),
            radius: self.points[i.2 as usize].0.distance(self.points[i.1 as usize].0) * zoom.scale,
            start_angle: iced::Radians(0.0),
            end_angle: iced::Radians(360.0)
        };
        p.arc(arc);
    }
}

impl Model {
    pub fn find_point(&self, point: Point, scale: f32) -> usize {
        self.points
            .iter()
            .position(|x| { x.0.distance(point) < scale * 2.0 })
            .unwrap_or(self.points.len())
    }

    pub fn find_min_max(&self) -> (Point, Point) {
        if let Some(min) = self.points.get(0) {
            let mut min = min.0;
            let mut max = min;

            for (point, _) in &self.points {
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

    pub fn replace_prim(&mut self, one: usize, two: usize) {
        self.prims.iter_mut().for_each(|x|{
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
