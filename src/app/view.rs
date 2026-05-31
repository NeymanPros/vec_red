use iced::{Center, Fill};
use iced::widget::{container, row, stack, Scrollable, button, text, text_editor, Column, column, Slider, text_input};
use crate::{Message, VecRed};
use crate::model::framework::Framework;
use std::default::Default;

impl VecRed {
    pub fn view(&self) -> iced::Element<'_, Message> {
        if self.app_config.showing {
            self.app_config.view()
        }
        else {
            let model = container(
                iced::widget::canvas(Framework {
                    state: &self.state,
                    model: &self.model,
                    scale: self.scale,
                    app_config: &self.app_config,
                    mode: &self.mode
                })
                    .width(Fill)
                    .height(Fill)
            )
                .width(Fill)
                .height(Fill);

            let grid = container(self.app_config.grid.view())
                .width(Fill).height(Fill);

            let space = container("")
                .height(Fill).width(3)
                .style(|_| container::Style {
                    background: Some(iced::Color::BLACK.into()),
                    ..Default::default()
                });
            
            row![stack![grid, model], 
                space, 
                self.side_panel().width(200).height(Fill)]
                .spacing(2).into()
        }
    }
}


impl VecRed {
    /// Summary for the right panel
    fn side_panel(&self) -> Scrollable<'_, Message> {
        let make_separator = || -> container::Container<_> {
            container("").height(2).width(Fill).padding(4).style(|_| container::Style {
                background: Some(iced::Color::from_rgb8(96, 96, 96).into()),
                ..Default::default()
            })
        };
        
        let mode = iced::widget::PickList::new(self.modes, Some(self.mode), Message::ChangeMode);
        let sep_1 = make_separator();

        let change_scale: Slider<f32, Message> = Slider::new(0.5..=20.0, self.scale, |x| Message::EditScale("scale", x)).step(0.25);
        let change_circle = Slider::new(self.scale..=100.0, self.default_circle, |x| Message::EditScale("circle", x)).step(1.0);
        let sep_2 = make_separator();
        
        let num = match self.chosen_point {
            None => { None }
            _ => { Some(self.chosen_point.unwrap().2) }
        };
        let mut point_info: Column<Message> = Column::new();
        if let Some(num) = num {
            point_info = self.about_point(num);
            point_info = point_info.push(make_separator())
        }

        let undo_button = button("Undo").on_press(Message::Undo);
        let shrink = self.shrink_to_fit();
        let clear_all = button("Clear all").on_press(Message::ClearAll);
        let sep_3 = make_separator();

        let for_path = text_editor(&self.path_to_load).on_action(Message::EditPath).placeholder("/path/to/model");
        let open_model = button("Open model").on_press(Message::OpenModel);
        let export_model = button("Export model").on_press(Message::ExportModel);
        let sep_4 = make_separator();
        
        let foreign_functions = self.foreign_functions();
        let sep_5 = make_separator();
        
        let settings = button("Settings").on_press(Message::ConfigOpen(true));
        
        let full_panel = column!(mode, sep_1, 
            text("Change scale"), change_scale, text("Change default circle"), change_circle, sep_2, 
            point_info, 
            undo_button, shrink, clear_all, sep_3, 
            for_path, open_model, export_model, sep_4, 
            foreign_functions, sep_5, 
            settings).spacing(5).align_x(Center);
        
        Scrollable::new(full_panel)
    }

    /// Part of the panel about selected [Point].
    fn about_point(&self, num: usize) -> Column<'_, Message> {
        let point_number = text(format!("Number of point: {}", num));
        
        let input = |order: usize| { 
            text_input("", &self.point_string[order]).on_input(move |text| Message::ChangeParams("point", num, text, order)) 
        };
        let point_x = row![text("X: "), input(0)];
        let point_y = row![text("Y: "), input(1)];
        let point_circle = row![text("R: "), input(2)];
        
        let point_apply = row![button("Apply").on_press(Message::ChangeApply)];
        let point_delete = row![button("Delete").on_press(Message::DeletePoint)];
        
        column![
            point_number, 
            point_x, point_y, point_circle, 
            point_apply, point_delete, 
            self.full_point(self.chosen_point.unwrap().2)
        ].align_x(Center).spacing(5)
    }
    
    /// Part of the panel calling foreign functions
    fn foreign_functions(&self) -> Column<'_, Message> {
        let send_model = button("Send model").on_press(Message::OpenMathCore);
        let triangle = button("Create triangle").on_press(Message::CreateTriangle);
        
        column![send_model, triangle].align_x(Center).spacing(5)
    }
    
    /// Changes [Zoom] so that every Point from [Model] fits inside
    fn shrink_to_fit(&self) -> button::Button<'_, Message> {
        let (min, max) = self.model.find_min_max();
        let button = button("Shrink").on_press(Message::SetZoom(min, max, true));
        
        button
    }
}
