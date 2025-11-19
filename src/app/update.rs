use iced::{keyboard, Point, Subscription, Vector};
use iced::keyboard::Key;
use iced::keyboard::key::Named;
use iced::widget::text_editor;
use libloading::Library;
use crate::{Message, VecRed};
use crate::app_settings::Change;
use crate::foreign_functions::*;

impl VecRed {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChangeMode(new_mode) => {
                self.mode = new_mode;
            }

            Message::EditPath(edited) => {
                self.path_to_excel.perform(edited)
            }

            Message::GetData => {
                //self.path_to_excel;
                self.model.dots = excel_dots();
                self.model.lines = excel_lines();
                self.mode = "Move";
                self.chosen_dot = None;
                self.journal.clear();
                self.state.redraw()
        }
            
            Message::DefDot(dot) => {
                let number = self.model.find_point(dot, self.scale);
                if number == self.model.dots.len() {
                    self.journal.pushed_dot();
                    self.model.dots.push((dot, self.default_circle));
                    self.state.redraw();
                }
                self.chosen_dot = Some((self.model.dots[number].0, self.model.dots[number].1, number));
                self.change_dot = [
                    text_editor::Content::with_text(self.chosen_dot.as_ref().unwrap().0.x.to_string().as_str()),
                    text_editor::Content::with_text(self.chosen_dot.as_ref().unwrap().0.y.to_string().as_str()),
                    text_editor::Content::with_text(self.chosen_dot.as_ref().unwrap().1.to_string().as_str())
                ]
            }
            
            Message::DefLine(dots, line) => {
                let add_dot = |vec_red: &mut VecRed, dot: Point| {
                    let number = vec_red.model.find_point(dot, vec_red.scale);
                    if number == vec_red.model.dots.len() {
                        vec_red.journal.pushed_dot();
                        vec_red.model.dots.push((dot, vec_red.default_circle));
                    }
                    number
                };
                let a = add_dot(self, dots[0]);
                let b = add_dot(self, dots[1]);
                if line.2 == -1 {
                    if a != b {
                        self.journal.pushed_line();
                        self.model.lines.push((a as i32, b as i32, -1))
                    }
                } else {
                    let c = add_dot(self, dots[2]);
                    if a != b && a != c && b != c {
                        self.journal.pushed_line();
                        self.model.lines.push((a as i32, b as i32, c as i32))
                    }
                }
                self.state.redraw();
                self.chosen_dot = None
            }
            
            Message::DefUnselect => {
                self.chosen_dot = None;
            }

            Message::ChangeDot(num, action) => {
                self.change_dot[num].perform(action);
                if let Ok(new_value) = self.change_dot[num].text().trim().parse::<f32>() {
                    match num {
                        0 => {
                            self.chosen_dot.as_mut().unwrap().0.x = new_value;
                        }
                        1 => {
                            self.chosen_dot.as_mut().unwrap().0.y = new_value;
                        }
                        2 => {
                            self.chosen_dot.as_mut().unwrap().1 = new_value;
                        }
                        _ => {}
                    }
                }
            }

            Message::ChangeApply => {
                if !self.chosen_dot.is_none()  {
                    let num = self.chosen_dot.as_ref().unwrap().2;
                    if self.model.dots[num].0 != self.chosen_dot.as_ref().unwrap().0 || self.model.dots[num].1 != self.chosen_dot.as_ref().unwrap().1 {
                        self.journal.changed_dot(self.model.dots[num], num);
                        self.model.dots[num].0 = self.chosen_dot.as_ref().unwrap().0;
                        self.model.dots[num].1 = self.chosen_dot.as_ref().unwrap().1;
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

            Message::DeleteDot => {
                let Some((_, _, num)) = self.chosen_dot else {
                    return
                };
                self.model.lines
                    .iter()
                    .enumerate()
                    .rev()
                    .for_each(|(placement, x)| {
                        if x.0 == num as i32 || x.1 == num as i32 || x.2 == num as i32 {
                            self.journal.deleted_line(x.clone(), placement)
                        }
                    });
                self.model.lines.retain(|x| { x.0 != num as i32 && x.1 != num as i32 && x.2 != num as i32 });
                if self.model.dots.len() >= 1 && num != self.model.dots.len() - 1 {
                    self.journal.deleted_dot(self.model.dots[num], num);
                    self.model.dots.swap_remove(num);
                }
                else {
                    self.journal.deleted_dot(self.model.dots[num], num);
                    self.model.dots.pop();
                }
                if num != self.model.dots.len() {
                    self.model.replace_line(self.model.dots.len(), num);
                }
                self.chosen_dot = None;
                self.mode = "Move";
                self.state.redraw();
            }

            Message::Undo => {
                self.journal.undo()(&mut self.model);
                self.mode = "Move";
                self.state.redraw()
            }

            Message::ClearAll => {
                self.model.dots.clear();
                self.model.lines.clear();
                self.journal.clear();
                self.chosen_dot = None;
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
                    self.lib = Some(Library::new("/home/alexe/Documents/DTLib/FLib.dll").expect("No lib found")); 
                }
                let lib = self.lib.as_ref().unwrap();
                f_init_model(lib);
                for i in &self.model.dots {
                    let out = f_create_point(lib, i);
                    println!("{out}")
                }
                for j in &self.model.lines {
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
                    (self.model.node_dots, self.model.node_lines) = get_nodes_full(lib);
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
                    Some(Message::DeleteDot)
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
