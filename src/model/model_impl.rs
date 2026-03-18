use iced::widget::canvas;
use iced::widget::canvas::{Path, Stroke, path::Arc};
use crate::app_config::app_config::{AppConfig, NodeMode};
use crate::app_config::zoom::Zoom;
use super::model::*;

impl Model {
    pub fn draw_model(&self, frame: &mut canvas::Frame, scale: f32, app_config: &AppConfig) {
        if app_config.prims_show {
            let prim_color = app_config.get_color("Prims");
            let lines = Path::new(|p| {
                for index in 0..self.prims_len() {
                    let prim = self.prims(index);
                    if prim[2] == -1 {
                        if app_config.is_line_inside(self.points(prim[0] as usize), self.points(prim[1] as usize)){
                            p.move_to(app_config.zoom.apply(self.points(prim[0] as usize)));
                            p.line_to(app_config.zoom.apply(self.points(prim[1] as usize)));
                        }
                    } else {
                        self.approx_arc(p, prim, &app_config.zoom)
                    }
                }
            });
            frame.stroke(&lines, Stroke::default().with_color(prim_color).with_width(scale));
        }
        
        if app_config.points_show || app_config.circles_show {
            let point_color = app_config.get_color("Points");
            let circle_color = app_config.get_color("Circles");
            for index in 0..self.points_len() {
                let point = self.points(index);
                if app_config.points_show {
                    if app_config.is_point_inside(point, scale * 2.) {
                        let point_draw = Path::circle(app_config.zoom.apply(point), scale * 2.);
                        frame.fill(&point_draw, point_color);
                    }
                }
                if app_config.circles_show {
                    if app_config.is_point_inside(point, self.points_r(index)) {
                        let dot = Path::circle(app_config.zoom.apply(point), self.points_r(index) * app_config.zoom.scale);
                        frame.stroke(&dot, Stroke::default().with_color(circle_color).with_width(2.0))
                    }
                }
            };
        }

        self.draw_nodes(app_config, frame, scale);
        if app_config.node_points_show {
            let node_point_color = app_config.get_color("Node points");
            for index in 0..self.nodes_len() {
                let node = self.nodes(index);
                if app_config.is_point_inside(node, scale) {
                    let point = Path::circle(app_config.zoom.apply(node), scale);
                    frame.fill(&point, node_point_color);
                }
            }
        }
    }

    /// Must be rewritten!
    fn approx_arc (&self, p: &mut canvas::path::Builder, prim: &[i32; 3], zoom: &Zoom) {
        let arc = Arc{
            center: zoom.apply(self.points(prim[2] as usize)),
            radius: 123.,
            start_angle: iced::Radians(0.0),
            end_angle: iced::Radians(360.0)
        };
        p.arc(arc);
    }

    fn draw_nodes (&self, app_config: &AppConfig, frame: &mut canvas::Frame, scale: f32) {
        let draw_triangle = |p1: i32, p2: i32, p3: i32| {
            Path::new(|path| {
                path.move_to(app_config.zoom.apply(self.nodes(p1 as usize)));
                path.line_to(app_config.zoom.apply(self.nodes(p2 as usize)));
                path.line_to(app_config.zoom.apply(self.nodes(p3 as usize)));
                path.line_to(app_config.zoom.apply(self.nodes(p1 as usize)));
            })
        };
        match app_config.node_mode {
            NodeMode::PureLines {} => {
                let node_line_color = app_config.get_color("Node lines");
                for index in 0..self.elems_len() {
                    let &[p1, p2, p3] = self.elems(index);
                    let is_visible = true; //model_config.is_line_inside()
                    if is_visible {
                        let triangle = draw_triangle(p1, p2, p3);
                        frame.stroke(&triangle, Stroke::default().with_color(node_line_color).with_width(scale / 2.0))
                    }
                }
            }

            NodeMode::Green { max } => {
                if self.is_borrowed() {
                    for index in 0..self.elems_len() {
                        let &[p1, p2, p3] = self.elems(index);
                        let is_visible = true; //model_config.is_lines_inside()
                        if is_visible {
                            let triangle = draw_triangle(p1, p2, p3);
                            let current_green = self.get_bm_only(index as i32);
                            frame.fill(&triangle, iced::Color::from_rgb(0.0, current_green / max, 0.0));
                        }
                    }
                }
                else {
                    println!("Model needs to be borrowed!")
                }
            }

            _ => {}
        }
    }
}
