use iced::{Center, Fill};
use iced::widget::{button, container, horizontal_space, row, stack, text, text_editor, Column, column, Slider};
use crate::{Message, VecRed};
use crate::model::framework::Framework;

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
                    mode: &self.mode,
                    lib: &self.lib
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
    /// Summary for the right panel
    fn side_panel(&self) -> Column<'_, Message> {
        let mode = iced::widget::PickList::new(self.modes, Some(self.mode), Message::ChangeMode);
        let for_path = text_editor(&self.path_to_load).on_action(Message::EditPath);
        let export_model = button("Export model").on_press(Message::ExportModel);
        let open_model = button("Open model").on_press(Message::OpenModel);
        let clear_frame = button("Clear all").on_press(Message::ClearAll);
        
        let num = match self.chosen_point {
            None => { String::from("") }
            _ => { self.chosen_point.unwrap().2.to_string() }
        };
        let mut point_info: Column<Message> = Column::new();
        if num != String::from("") {
            point_info = self.about_point(num);
        }
        let change_scale: Slider<f32, Message> = Slider::new(0.5..=20.0, self.scale, |x| Message::EditScale("scale", x)).step(0.25);
        let change_circle = Slider::new(self.scale..=100.0, self.default_circle, |x| Message::EditScale("circle", x)).step(1.0);
        let undo_button = button("Undo").on_press(Message::Undo);
        let settings = button("Settings").on_press(Message::SettingsOpen(true));
        
        let foreign_functions = self.foreign_functions();
        let shrink = self.shrink_to_fit();
        column!(mode, for_path, point_info, export_model, open_model, text("Change scale"), change_scale, text("Change default circle"), change_circle,
            clear_frame, undo_button, settings, foreign_functions, shrink).spacing(5).align_x(Center)
    }

    /// Part of the panel about selected [Point].
    fn about_point(&self, num: String) -> Column<'_, Message> {
        let point_number = text("Number of point: ".to_owned() + num.as_str());
        let point_x = row![text("X: "), text_editor(&self.change_point[0]).on_action(|action| Message::ChangePoint(0, action))];
        let point_y = row![text("Y: "), text_editor(&self.change_point[1]).on_action(|action| Message::ChangePoint(1, action))];
        let point_circle = row![text("R: "), text_editor(&self.change_point[2]).on_action(|action| Message::ChangePoint(2, action))];
        let point_apply = row![button("Apply").on_press(Message::ChangeApply)];
        let point_delete = row![button("Delete").on_press(Message::DeletePoint)];
        column![point_number, point_x, point_y, point_circle, point_apply, point_delete].align_x(Center)
    }
    
    /// Part of the panel calling foreign functions
    fn foreign_functions(&self) -> Column<'_, Message> {
        let send_model = button("Send model").on_press(Message::SendModel);
        let triangle = button("Create triangle").on_press(Message::CreateTriangle);
        
        column![send_model, triangle]
    }
    
    /// Changes [app_settings::Zoom] so that every Point from [Self::model] fits inside
    fn shrink_to_fit(&self) -> button::Button<'_, Message> {
        let (min, max) = self.model.find_min_max();
        let button = button("Shrink").on_press(Message::SetZoom(min, max, true));
        
        button
    }
}
