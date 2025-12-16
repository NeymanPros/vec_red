use iced::{keyboard, Point, Subscription, Vector};
use iced::keyboard::Key;
use iced::keyboard::key::Named;
use iced::widget::text_editor;
use libloading::Library;
use crate::{Message, VecRed};
use crate::app_settings::Change;
use crate::foreign_functions::*;
use crate::model::load_model;

impl VecRed {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChangeMode(new_mode) => {
                self.mode = new_mode;
            }

            Message::EditPath(edited) => {
                self.path_to_load.perform(edited)
            }

            Message::ExportModel => {
                if !load_model::export_model(self.path_to_load.text(), &self.model) {
                    println!("AAA")
                }
            }
            
            Message::ImportModel => {
                if let Some(lib) = &self.lib {
                    if load_model::import_model(lib, self.path_to_load.text(), &mut self.model) {
                        self.mode = "Move";
                        self.chosen_point = None;
                        self.journal.clear();
                        self.state.redraw();
                        println!("Done")
                    } else {
                        println!("AAA :(")
                    }
                }
            }

            Message::DefPoint(point) => {
                let number = self.model.find_point(point, self.scale);
                if number == self.model.points.len() {
                    self.journal.pushed_point();
                    self.model.points.push((point, self.default_circle));
                    self.state.redraw();
                }
                self.chosen_point = Some((self.model.points[number].0, self.model.points[number].1, number));
                self.change_point = [
                    text_editor::Content::with_text(self.chosen_point.as_ref().unwrap().0.x.to_string().as_str()),
                    text_editor::Content::with_text(self.chosen_point.as_ref().unwrap().0.y.to_string().as_str()),
                    text_editor::Content::with_text(self.chosen_point.as_ref().unwrap().1.to_string().as_str())
                ]
            }
            
            Message::DefPrim(points, prim) => {
                let add_point = |vec_red: &mut VecRed, point: Point| {
                    let number = vec_red.model.find_point(point, vec_red.scale);
                    if number == vec_red.model.points.len() {
                        vec_red.journal.pushed_point();
                        vec_red.model.points.push((point, vec_red.default_circle));
                    }
                    number
                };
                let a = add_point(self, points[0]);
                let b = add_point(self, points[1]);
                if prim.2 == -1 {
                    if a != b {
                        self.journal.pushed_prim();
                        self.model.prims.push((a as i32, b as i32, -1))
                    }
                } else {
                    let c = add_point(self, points[2]);
                    if a != b && a != c && b != c {
                        self.journal.pushed_prim();
                        self.model.prims.push((a as i32, b as i32, c as i32))
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
                if !self.chosen_point.is_none()  {
                    let num = self.chosen_point.as_ref().unwrap().2;
                    if self.model.points[num].0 != self.chosen_point.as_ref().unwrap().0 || self.model.points[num].1 != self.chosen_point.as_ref().unwrap().1 {
                        self.journal.changed_point(self.model.points[num], num);
                        self.model.points[num].0 = self.chosen_point.as_ref().unwrap().0;
                        self.model.points[num].1 = self.chosen_point.as_ref().unwrap().1;
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
                self.model.prims
                    .iter()
                    .enumerate()
                    .rev()
                    .for_each(|(placement, x)| {
                        if x.0 == num as i32 || x.1 == num as i32 || x.2 == num as i32 {
                            self.journal.deleted_prim(x.clone(), placement)
                        }
                    });
                self.model.prims.retain(|x| { x.0 != num as i32 && x.1 != num as i32 && x.2 != num as i32 });
                if self.model.points.len() >= 1 && num != self.model.points.len() - 1 {
                    self.journal.deleted_point(self.model.points[num], num);
                    self.model.points.swap_remove(num);
                }
                else {
                    self.journal.deleted_point(self.model.points[num], num);
                    self.model.points.pop();
                }
                if num != self.model.points.len() {
                    self.model.replace_prim(self.model.points.len(), num);
                }
                self.chosen_point = None;
                self.mode = "Move";
                self.state.redraw();
            }

            Message::Undo => {
                self.journal.undo()(&mut self.model);
                self.mode = "Move";
                self.state.redraw()
            }

            Message::ClearAll => {
                self.model.points.clear();
                self.model.prims.clear();
                self.journal.clear();
                self.chosen_point = None;
                self.state.redraw()
            }

            Message::Scale(extent) => {
                if extent == 0.0 {
                    self.app_settings.zoom.scale = 1.0;
                    self.app_settings.zoom.shift = Vector::default()
                } else {
                    self.app_settings.zoom.scale *= extent
                }
                self.mode = "Move";
                self.state.redraw()
            }

            Message::Shift(add_shift) => {
                self.app_settings.zoom.shift = self.app_settings.zoom.shift + add_shift * (1.0 / self.app_settings.zoom.scale);
                self.state.redraw()
            }
            
            Message::SetZoom(start, end, force) => {
                let scale = self.app_settings.zoom.scale;
                let big_enough = ((end.x - start.x) * scale).abs() > 25.0 &&
                    ((end.y - start.y) * scale).abs() > 25.0;
                
                if force || big_enough {
                    let min_x = start.x.min(end.x);
                    let max_x = start.x.max(end.x);
                    let min_y = start.y.min(end.y);
                    let max_y = start.y.max(end.y);

                    self.app_settings.zoom.shift.x = min_x;
                    self.app_settings.zoom.shift.y = min_y;
                    self.app_settings.zoom.scale = 900.0/f32::max(max_x - min_x, max_y - min_y).abs();
                    self.state.redraw()
                }
            }

            Message::SettingsOpen(new_value) => {
                self.app_settings.shown = new_value;
                if new_value == false {
                    self.mode = "Move";
                    self.state.redraw();
                    self.app_settings.grid.redraw()
                }
                else {
                    self.app_settings.update(Change::Open)
                }
            }

            Message::SettingsEdit(action) => {
                self.app_settings.update(action)
            }

            Message::SendModel => {
                unsafe { 
                    self.lib = Some(Library::new("C:/Users/alexe/Downloads/FLib/FLib.dll").expect("No lib found")); 
                }
                let lib = self.lib.as_ref().unwrap();
                f_init_model(lib);
                for i in &self.model.points {
                    let out = f_create_point(lib, i);
                    println!("{out}")
                }
                for j in &self.model.prims {
                    let out = f_create_prim(lib, j);
                    println!("primo {out}")
                }
            }

            Message::BuildFM(point) => {
                if let lib = self.lib.as_ref().unwrap() {
                    let out = f_create_region(lib, &point);
                    println!("aaa is {out}");
                }
            }

            Message::CreateTriangle => {
                if let lib = self.lib.as_ref().unwrap() {
                    let out = f_build_fm(lib);
                    println!("Triangle is {}", out);
                    (self.model.node_points, self.model.node_lines) = get_nodes_full(lib);
                    self.state.redraw()
                }
            }
        }
    }
}

impl VecRed {
    pub(crate) fn subscription(&self) -> Subscription<Message> {
        let keyboard_events = keyboard::on_key_press(|a, b| {
            Self::shortcuts(a, b)
        });
        //let mouse_events = iced::mouse::Event::CursorEntered;
        Subscription::batch(vec![keyboard_events])
    }

    fn shortcuts (key: Key, modifiers: keyboard::Modifiers) -> Option<Message> {
        if modifiers.is_empty() {
            return match key {
                Key::Named(Named::Delete) => {
                    Some(Message::DeletePoint)
                }
                Key::Named(Named::ArrowLeft) => {
                    Some(Message::Shift(Vector::new(-100.0, 0.0)))
                }
                Key::Named(Named::ArrowRight) => {
                    Some(Message::Shift(Vector::new(100.0, 0.0)))
                }
                Key::Named(Named::ArrowUp) => {
                    Some(Message::Shift(Vector::new(0.0, -100.0)))
                }
                Key::Named(Named::ArrowDown) => {
                    Some(Message::Shift(Vector::new(0.0, 100.0)))
                }
                _ => { None }
            }
        }
        match modifiers {
            keyboard::Modifiers::SHIFT => {
                match key {
                    Key::Named(Named::ArrowLeft) => {
                        Some(Message::Shift(Vector::new(-10.0, 0.0)))
                    }
                    Key::Named(Named::ArrowRight) => {
                        Some(Message::Shift(Vector::new(10.0, 0.0)))
                    }
                    Key::Named(Named::ArrowUp) => {
                        Some(Message::Shift(Vector::new(0.0, -10.0)))
                    }
                    Key::Named(Named::ArrowDown) => {
                        Some(Message::Shift(Vector::new(0.0, 10.0)))
                    }
                    _ => { None }
                }
            }
            keyboard::Modifiers::CTRL => {
                match key.as_ref() {
                    Key::Character("z") => {
                        Some(Message::Undo)
                    }
                    Key::Character("=") => {
                        Some(Message::Scale(1.1))
                    }
                    Key::Character("-") => {
                        Some(Message::Scale(0.9))
                    }
                    _ => { None }
                }
            }
            _ => { None }
        }
    }
}
