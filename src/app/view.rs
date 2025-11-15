use iced::{Center, Fill};
use iced::widget::{button, container, horizontal_space, row, stack, text, text_editor, Column, column, Slider};
use crate::{Message, VecRed};
use crate::framework::Framework;

impl VecRed {
    pub fn view(&self) -> iced::Element<'_, Message> {
        if self.app_settings.shown {
            self.app_settings.view()
        }
        else {
            let blueprint = container(
                iced::widget::canvas(Framework {
                    state: &self.state,
                    model: &self.model,
                    scale: self.scale,
                    app_settings: &self.app_settings,
                    mode: &self.mode
                })
                    .width(Fill)
                    .height(Fill)
            )
                .width(Fill)
                .height(Fill);

            let grid = container(self.app_settings.grid.view())
                .width(Fill).height(Fill);

            row![stack![grid, blueprint], 
                horizontal_space().height(Fill).width(10.0), 
                self.side_panel().width(200).height(Fill)].into()
        }
    }
}


impl VecRed {
    fn side_panel(&self) -> Column<'_, Message> {
        let mode = iced::widget::PickList::new(self.modes, Some(self.mode), Message::ChangeMode);
        let for_path = text_editor(&self.path_to_excel).on_action(Message::EditPath);
        let get_data = button("Hello").on_press(Message::GetData);
        let clear_frame = button("Clear all").on_press(Message::ClearAll);
        let num = match self.chosen_dot {
            None => { String::from("") }
            _ => { self.chosen_dot.unwrap().2.to_string() }
        };
        let mut dot_info: Column<Message> = Column::new();
        if num != String::from("") {
            dot_info = self.about_dot(num);
        }
        let change_scale: Slider<f32, Message> = Slider::new(0.5..=20.0, self.scale, |x| Message::EditScale("scale", x)).step(0.25);
        let change_circle = Slider::new(self.scale..=100.0, self.default_circle, |x| Message::EditScale("circle", x)).step(1.0);
        let undo_button = button("Undo").on_press(Message::Undo);
        let settings = button("Settings").on_press(Message::SettingsOpen(true));

        column!(mode, for_path, dot_info, get_data, text("Change scale"), change_scale, text("Change default circle"), change_circle,
            clear_frame, undo_button, settings).spacing(5).align_x(Center)
    }

    fn about_dot(&self, num: String) -> Column<'_, Message> {
        let dot_number = text("Number of dot: ".to_owned() + num.as_str());
        let dot_x = row![text("X: "), text_editor(&self.change_dot[0]).on_action(|action| Message::ChangeDot(0, action))];
        let dot_y = row![text("Y: "), text_editor(&self.change_dot[1]).on_action(|action| Message::ChangeDot(1, action))];
        let dot_circle = row![text("R: "), text_editor(&self.change_dot[2]).on_action(|action| Message::ChangeDot(2, action))];
        let dot_apply = row![button("Apply").on_press(Message::ChangeApply)];
        let dot_delete = row![button("Delete").on_press(Message::DeleteDot)];
        column![dot_number, dot_x, dot_y, dot_circle, dot_apply, dot_delete].align_x(Center)
    }
    
    fn outer_functions(&self) -> Column<'_, Message> {
        todo!()
    }
}
