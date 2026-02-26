use iced::widget::text_editor;
use libloading::Library;
use crate::{Message, VecRed};
use crate::app_config::app_config::Change;
use crate::foreign_functions::*;
use crate::model::load_model;

impl VecRed<'_> {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChangeMode(new_mode) => {
                self.mode = new_mode;
            }

            Message::EditPath(edited) => {
                self.path_to_load.perform(edited)
            }

            Message::ExportModel => {
                if !load_model::export_model(&self.lib, self.path_to_load.text(), &self.model) {
                    println!("Not done!")
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

            Message::DefPoint(point) => {
                let number = self.model.find_point(point, self.scale, self.app_config.zoom.scale);
                if self.mode == "Region" && number == self.model.points_len() {
                    self.update(Message::CreateRegion(point))
                }
                else {
                    if number == self.model.points_len() {
                        self.journal.pushed_point();
                        self.model.points_push(point, self.default_circle);
                        self.state.redraw();
                    }
                    self.chosen_point = Some((self.model.points(number), self.model.points_r(number), number));
                    self.change_point = [
                        text_editor::Content::with_text(self.chosen_point.as_ref().unwrap().0.x.to_string().as_str()),
                        text_editor::Content::with_text(self.chosen_point.as_ref().unwrap().0.y.to_string().as_str()),
                        text_editor::Content::with_text(self.chosen_point.as_ref().unwrap().1.to_string().as_str())
                    ]
                }
            }
            
            Message::DefPrim(points, prim) => {
                let zoom_scale = self.app_config.zoom.scale.clone();
                let add_point = |vec_red: &mut VecRed, point: iced::Point| {
                    let number = vec_red.model.find_point(point, vec_red.scale, zoom_scale);
                    if number == vec_red.model.points_len() {
                        vec_red.journal.pushed_point();
                        vec_red.model.points_push(point, vec_red.default_circle);
                    }
                    number
                };
                let a = add_point(self, points[0]);
                let b = add_point(self, points[1]);
                if prim.2 == -1 {
                    if a != b {
                        self.journal.pushed_prim();
                        self.model.prims_push([a as i32, b as i32, -1])
                    }
                } else {
                    let c = add_point(self, points[2]);
                    if a != b && a != c && b != c {
                        self.journal.pushed_prim();
                        self.model.prims_push([a as i32, b as i32, c as i32])
                    }
                }
                self.state.redraw();
                self.chosen_point = None
            }
            
            Message::DefUnselect => {
                self.chosen_point = None;
            }

            Message::ChangePoint(num, action) => {
                self.change_point[num].perform(action);
                if let Ok(new_value) = self.change_point[num].text().trim().parse::<f32>() {
                    match num {
                        0 => {
                            self.chosen_point.as_mut().unwrap().0.x = new_value;
                        }
                        1 => {
                            self.chosen_point.as_mut().unwrap().0.y = new_value;
                        }
                        2 => {
                            self.chosen_point.as_mut().unwrap().1 = new_value;
                        }
                        _ => {}
                    }
                }
            }

            Message::ChangeApply => {
                if let Some((chosen_p, chosen_r, chosen_num)) = self.chosen_point {
                    if self.model.points(chosen_num) != chosen_p || self.model.points_r(chosen_num) != chosen_r {
                        self.journal.changed_point((self.model.points(chosen_num), self.model.points_r(chosen_num)), chosen_num);
                        self.model.point_set(chosen_num, (chosen_p, chosen_r));
                        self.state.redraw();
                    }
                }
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
                    self.journal.deleted_point((self.model.points(num), self.model.points_r(num)), num);
                    self.model.points_swap(num, self.model.points_len() - 1);
                    self.model.points_pop();
                }
                else {
                    self.journal.deleted_point((self.model.points(num), self.model.points_r(num)), num);
                    self.model.points_pop();
                }
                if num != self.model.points_len() {
                    self.model.replace_prim(self.model.points_len() as i32, num as i32);
                }
                if num < self.model.points_len() {
                    self.chosen_point.as_mut().unwrap().0 = self.model.points(num);
                    self.chosen_point.as_mut().unwrap().1 = self.model.points_r(num);
                }
                else{
                    self.chosen_point = None;
                }
                self.mode = "Move";
                self.state.redraw();
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

            Message::OpenMathCore => {
                self.lib = unsafe { 
                    let temp_lib = Library::new("C:/Users/alexe/Downloads/FLib/FLib.dll");
                    if temp_lib.is_err() {
                        println!("No library");
                        return;
                    }
                    Some(temp_lib.unwrap())
                };
                let lib = self.lib.as_ref().unwrap();
                f_init_model(lib);
                for i in 0..self.model.points_len() {
                    f_create_point(lib, (self.model.points(i), self.model.points_r(i)));
                }
                for j in 0..self.model.prims_len() {
                    f_create_prim(lib, self.model.prims(j));
                }
                
                let points_ref = get_points_ref(lib);
                self.model.make_borrow(points_ref);
            }

            Message::CreateRegion(point) => {
                if let Some(lib) = self.lib.as_ref() {
                    let out = f_create_region(lib, &point);
                    println!("Region is {out}");
                }
            }

            Message::CreateTriangle => {
                if let Some(lib) = self.lib.as_ref() {
                    let out = f_build_fm(lib);
                    println!("Triangle is {}", out);
                    //(self.model.node_points, self.model.node_lines) = get_nodes_full(lib);
                    self.state.redraw()
                }
            }
        }
    }
}
