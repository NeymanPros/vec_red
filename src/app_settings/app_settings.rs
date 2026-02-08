use iced::{Center, Fill, FillPortion};
use iced::widget::{button, checkbox, column, container, pick_list, row, text, text_editor, Slider};
use crate::Message;
use crate::Message::SettingsEdit;
use super::grid::Grid;
use super::zoom::Zoom;

pub enum NodeMode {
    None {},
    PureLines {},
    Green { max: f32 } // Triangles
}

impl NodeMode {
    fn as_str(&self) -> &str {
        match self {
            Self::None {} => "None",
            Self::PureLines {} => "Pure lines",
            Self::Green { .. } => "Green"
        }
    }
    
    fn options(&self) -> Vec<&str> {
        vec!["None", "Pure lines", "Green"]
    }
}


#[derive(Debug, Clone)]
pub enum Change {
    Open,
    ZoomWrite(usize, text_editor::Action),
    Circles(bool),
    Points(bool),
    Prims(bool),
    ChangeColor(&'static str, usize, u8),
    Bound(bool),
    NodePointsShow(bool),
    NodeLineMode(String),
    GridMode(&'static str),
}


pub struct AppSettings {
    pub shown: bool,
    
    /// (value - shift) / scale
    pub zoom: Zoom,
    pub grid: Grid,
    pub bound: bool,

    pub points_show: bool,
    pub prims_show: bool,
    pub circles_show: bool,
    pub node_points_show: bool,
    pub node_mode: NodeMode,

    circle_color: [u8; 3],
    point_color: [u8; 3],
    prim_color: [u8; 3],
    node_point_color: [u8; 3],
    node_line_color: [u8; 3],

    grid_modes: [&'static str; 3], 
    write_zoom: [text_editor::Content; 3]
}

impl AppSettings {
    pub fn update(&mut self, message: Change) {
        match message {
            Change::Open => {
                self.write_zoom = [
                    text_editor::Content::with_text(self.zoom.shift.x.to_string().as_str()),
                    text_editor::Content::with_text(self.zoom.shift.y.to_string().as_str()),
                    text_editor::Content::with_text(self.zoom.scale.to_string().as_str())
                ]
            }

            Change::ZoomWrite(num, action) => {
                self.write_zoom[num].perform(action);
                if let Ok(new_value) = self.write_zoom[num].text().trim().parse::<f32>() {
                    match num {
                        0 => {
                            self.zoom.shift.x = new_value
                        }
                        1 => {
                            self.zoom.shift.y = new_value
                        }
                        2 => {
                            self.zoom.scale = new_value
                        }
                        _ => {}
                    }
                }
            }

            Change::GridMode(new) => {
                self.grid.set_display(new)
            }

            Change::Circles(new) => {
                self.circles_show = new
            }
            Change::Points(new) => {
                self.points_show = new
            }
            Change::Prims(new) => {
                self.prims_show = new
            }
            Change::NodePointsShow(new) => {
                self.node_points_show = new;
            }
            Change::NodeLineMode(new) => {
                self.node_mode = match new.as_str() {
                    "Pure lines" => NodeMode::PureLines {},
                    "Green" => NodeMode::Green { max: 2.2 },
                    _ => NodeMode::None {}
                }
            }
            Change::ChangeColor(name, num, new) => {
                match name {
                    "Circles" => self.circle_color[num] = new,
                    "Points" => self.point_color[num] = new,
                    "Prims" => self.prim_color[num] = new,
                    "Node points" => self.node_point_color[num] = new,
                    _ => println!("No such color for {}", name)
                }
            }
            Change::Bound(new) => {
                self.bound = new
            }
        }
    }
}

impl AppSettings {
    pub fn view(&self) -> iced::Element<'_, Message> {
        let go_back = button("Go back").on_press(Message::SettingsOpen(false));
        let showing = self.showing_settings();
        
        let grid_mode = iced::widget::PickList::new(self.grid_modes, Some(self.grid.get_display()), |a| SettingsEdit(Change::GridMode(a)));
        let write_zoom_x = row![text("Shift x: "), text_editor(&self.write_zoom[0]).on_action(|action| SettingsEdit(Change::ZoomWrite(0, action)))];
        let write_zoom_y = row![text("Shift y: "), text_editor(&self.write_zoom[1]).on_action(|action| SettingsEdit(Change::ZoomWrite(1, action)))];
        let write_zoom_mul = row![text("Mul: "), text_editor(&self.write_zoom[2]).on_action(|action| SettingsEdit(Change::ZoomWrite(2, action)))];

        column![go_back, showing, grid_mode,
            write_zoom_mul, write_zoom_x, write_zoom_y].width(Fill).align_x(Center).into()
    }
    
    fn showing_settings(&self) -> iced::widget::Column<'_, Message> {
        let circles = self.color_element("Circles");
        let points = self.color_element("Points");
        let prims = self.color_element("Prims");
        let node_point = self.color_element("Node points");
        
        let node_line = pick_list(self.node_mode.options(), Some(self.node_mode.as_str()), |a| SettingsEdit(Change::NodeLineMode(a.to_string())));
        
        let bound_grid = checkbox("Bound to grid", self.bound).on_toggle(|a| SettingsEdit(Change::Bound(a)));
        
        column![circles, points, prims, node_point, node_line, bound_grid, ]
    }

    fn color_element(&self, name: &'static str) -> container::Container<Message> {
        let (show_tick, colors) = match name {
            "Circles" => (checkbox(name, self.circles_show).on_toggle(|a| SettingsEdit(Change::Circles(a))), self.circle_color),
            "Points" => (checkbox(name, self.points_show).on_toggle(|a| SettingsEdit(Change::Points(a))), self.point_color),
            "Prims" => (checkbox(name, self.prims_show).on_toggle(|a| SettingsEdit(Change::Prims(a))), self.prim_color),
            "Node points" => (checkbox(name, self.node_points_show).on_toggle(|a| SettingsEdit(Change::NodePointsShow(a))), self.node_point_color),
            _ => (checkbox(name, false), [0, 0, 0])
        };

        let change_color = move |num: usize| {
            let color = Slider::new(0..=255, colors[num], move |a| SettingsEdit(Change::ChangeColor(name, num, a)));
            let letter = match num {
                0 => 'R',
                1 => 'G',
                _ => 'B'
            };
            let text = text(format!("{} {}", letter, colors[num]));
            row![color, text].width(Fill)
        };

        let color = self.get_color(name);

        container(
            row![
                column![
                    show_tick,
                    text(name).width(Fill).align_x(Center),
                    change_color(0),
                    change_color(1),
                    change_color(2)
                ]
                    .spacing(4)
                    .width(FillPortion(2))
                    .height(Fill),
                button(
                    text("")
                ).style(move |_theme, _status| button::Style {
                        background: Some(color.into()),
                        ..Default::default()
                    })
                .height(20)
                .width(20)
            ].align_y(Center)
        )
            .width(Fill)
            .height(130)
            .align_y(Center)
            .padding(8)
    }
}

impl AppSettings {
    pub fn get_color(&self, name: &'static str) -> iced::Color {
        let array = match name {
            "Circles" => self.circle_color,
            "Points" => self.point_color,
            "Prims" => self.prim_color,
            "Node points" => self.node_point_color,
            _ => {
                println!("No such color for {}", name);
                [0, 0, 0] }
        };
        iced::Color::from_rgb8(array[0], array[1], array[2])
    }
}

impl Default for AppSettings {
    fn default () -> Self {
        Self {
            shown: false,
            zoom: Zoom::default(),
            grid: Grid::default(),
            bound: false,

            points_show: true,
            prims_show: true,
            circles_show: true,
            node_points_show: true,
            node_mode: NodeMode::None {},

            circle_color: [0, 0, 255],
            point_color: [0, 0, 0],
            prim_color: [0, 0, 0],
            node_point_color: [128, 12, 128],
            node_line_color: [128, 128, 128],

            grid_modes: ["Circles", "Squares", "None"],
            write_zoom: [text_editor::Content::default(), text_editor::Content::default(), text_editor::Content::default()]
        }
    }
}
