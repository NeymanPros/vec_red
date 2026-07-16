use crate::{Message, VecRed};
use crate::app_config::app_config::Change;
use crate::foreign_functions::*;
use crate::model::load_model;
use super::core::CallByName;

impl VecRed {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChangeMode(new_mode) => {
                self.mode = new_mode;
            }

            Message::EditPath(edited) => {
                self.path_to_load.perform(edited)
            }

            Message::DefPoint(point) => {
                self.def_point(point)
            }
            
            Message::DefPrim(points, prim) => {
                self.def_prim(points, prim)
            }
            
            Message::DefUnselect => {
                self.chosen_point = None;
            }

            Message::EditScale(name, new_value) => {
                if name == "scale" {
                    self.scale = new_value;
                    self.state.redraw();
                }
                else if name == "circle" {
                    self.default_circle = new_value
                }
            }

            Message::DeletePoint => {
                let Some((_, _, num)) = self.chosen_point else {
                    return
                };
                
                let pred = |x: &[i32; 3]| { x[0] != num as i32 && x[1] != num as i32 && x[2] != num as i32 };
                self.model.prims_retain_safe(pred, &mut self.journal);
                if self.model.points_len() >= 1 && num != self.model.points_len() - 1 {
                    self.journal.deleted_point(self.model.points(num), self.model.points_r(num), num);
                    self.model.points_swap(num, self.model.points_len() - 1);
                    self.model.points_pop();
                }
                else {
                    self.journal.deleted_point(self.model.points(num), self.model.points_r(num), num);
                    self.model.points_pop();
                }
                if num != self.model.points_len() {
                    self.model.replace_prim(self.model.points_len() as i32, num as i32);
                }
                if num < self.model.points_len() {
                    self.chosen_point.as_mut().unwrap().0 = self.model.points(num);
                    self.chosen_point.as_mut().unwrap().1 = self.model.points_r(num);
                }
                else {
                    self.chosen_point = None;
                }
                self.mode = "Move";
                self.state.redraw();
            }
            
            Message::FindEverything(x, y) => {
                if let Some(lib) = self.lib.as_ref() {
                    let prim = f_get_prim_xy(lib.clone(), x, y);
                    let node = f_get_node_xy(lib.clone(), x, y);
                    let region = f_get_region_xy(lib.clone(), x, y);
                    
                    self.chosen_elems = Some(CallByName{prim, node, region})
                }
                else {
                    println!("Math core must be!")
                }
            }

            Message::ChangeParams(what, index, new_value, order) => {
                self.change_params(what, index, new_value, order)
            }

            Message::ChangeApply => {
                self.move_point_apply()
            }

            Message::Undo => {
                self.journal.undo()(&mut self.model);
                self.mode = "Move";
                self.state.redraw()
            }

            Message::ClearAll => {
                self.model.clear();
                self.journal.clear();
                self.chosen_point = None;
                self.state.redraw()
            }
            
            Message::WindowResized(new_size) => 
                self.app_config.model_size = 
                    new_size - iced::Size::new(200., 0.),

            Message::ZoomScale(extent) => {
                if extent == 0.0 {
                    self.app_config.zoom.scale = 1.0;
                    self.app_config.zoom.shift = iced::Vector::default()
                } else {
                    self.app_config.zoom.scale *= extent
                }
                self.mode = "Move";
                self.state.redraw()
            }

            Message::ZoomShift(add_shift) => {
                self.app_config.zoom.shift = self.app_config.zoom.shift + add_shift * (1.0 / self.app_config.zoom.scale);
                self.state.redraw()
            }
            
            Message::SetZoom(start, end, force) => {
                let scale = self.app_config.zoom.scale;
                let big_enough = ((end.x - start.x) * scale).abs() > 25.0 &&
                    ((end.y - start.y) * scale).abs() > 25.0;
                
                if force || big_enough {
                    let min_x = f32::min(start.x, end.x);
                    let max_x = f32::max(start.x, end.x);
                    let min_y = f32::min(start.y, end.y);
                    let max_y = f32::max(start.y, end.y);

                    self.app_config.zoom.shift.x = min_x;
                    self.app_config.zoom.shift.y = min_y;
                    
                    let x_scale = self.app_config.model_size.width/(max_x - min_x).abs();
                    let y_scale = self.app_config.model_size.height/(max_y - min_y).abs();
                    
                    self.app_config.zoom.scale = f32::min(x_scale, y_scale);
                    self.state.redraw()
                }
            }

            Message::ConfigOpen(start_showing) => {
                self.app_config.showing = start_showing;
                if !start_showing {
                    self.mode = "Move";
                    self.state.redraw();
                    self.app_config.grid.redraw()
                }
                else {
                    self.app_config.update(Change::Open)
                }
            }

            Message::ConfigEdit(action) => {
                self.app_config.update(action)
            }

            Message::ExportModel => {
                if !load_model::export_model(&self.lib, self.path_to_load.text(), &self.model) {
                    println!("Not done!")
                } else {
                    println!("Done")
                }
            }

            Message::OpenModel => {
                if load_model::open_model(&self.lib, self.path_to_load.text(), &mut self.model) {
                    let (min, max) = self.model.find_min_max();
                    self.update(Message::SetZoom(min, max, true));

                    self.mode = "Move";
                    self.chosen_point = None;
                    self.journal.clear();
                    self.state.redraw();

                    println!("Done")
                } else {
                    println!("Not done :(")
                }
            }

            Message::OpenMathCore => {
                self.open_math_core()
            }
            
            Message::CreateRegion(point) => {
                self.create_region(point)
            }

            Message::CreateTriangle => {
                self.create_triangle()
            }
        }
    }
}
