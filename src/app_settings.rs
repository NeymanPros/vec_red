use iced::{Center, Fill, Point, Vector};
use iced::widget::{button, checkbox, column, pick_list, row, text, text_editor};
use crate::Message;
use crate::Message::SettingsEdit;
use crate::grid::Grid;


#[derive(Copy, Clone)]
pub struct Zoom {
    pub scale: f32,
    pub shift: Vector
}

impl Default for Zoom {
    fn default () -> Self {
        Self {
            scale: 1.0,
            shift: Vector::new(0.0, 0.0)
        }
    }
}

impl Zoom {
    /// Creates a new [Point]. Adds shift and multiplies coordinates by scale.
    pub fn apply (&self, point: Point) -> Point {
        let mut result = point;
        result = result - self.shift;
        result.x *= self.scale;
        result.y *= self.scale;
        result
    }
    /// Draws back the effect of [Zoom::apply].
    pub fn reverse(&self, point: Point) -> Point {
        let mut result = point;
        result.x /= self.scale;
        result.y /= self.scale;
        result = result + self.shift;
        result
    }
}

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
    Point(bool),
    Lines(bool),
    Bound(bool),
    NodePointShow(bool),
    NodeLineMode(String),
    GridMode(&'static str)
}


pub struct AppSettings {
    pub shown: bool,
    
    /// (value - shift) / scale
    pub zoom: Zoom,
    pub grid: Grid,
    pub points_show: bool,
    pub prims_show: bool,
    pub circles_show: bool,
    pub node_point_show: bool,
    pub node_mode: NodeMode,
    pub bound: bool,
    
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

            Change::Circles(new) => {
                self.circles_show = new
            }
            Change::Point(new) => {
                self.points_show = new
            }
            Change::Lines(new) => {
                self.prims_show = new
            }
            Change::NodePointShow(new) => {
                    self.node_point_show = new;
            }
            Change::NodeLineMode(new) => {
                    self.node_mode = match new.as_str() {
                        "Pure lines" => NodeMode::PureLines {},
                        "Green" => NodeMode::Green { max: 2.2 },
                        _ => NodeMode::None {}
                    }
            }
            Change::GridMode(new) => {
                self.grid.set_display(new)
            }
            Change::Bound(new) => {
                self.bound = new
            }
        }
    }
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
        let circles = checkbox("Circles", self.circles_show).on_toggle(|a| SettingsEdit(Change::Circles(a)));
        let points = checkbox("Points", self.points_show).on_toggle(|a| SettingsEdit(Change::Point(a)));
        let prims = checkbox("Prims", self.prims_show).on_toggle(|a| SettingsEdit(Change::Lines(a)));
        let node_point = checkbox("Node points", self.node_point_show).on_toggle(|a| SettingsEdit(Change::NodePointShow(a)));
        
        let node_line = pick_list(self.node_mode.options(), Some(self.node_mode.as_str()), |a| SettingsEdit(Change::NodeLineMode(a.to_string())));
        
        let bound_grid = checkbox("Bound to grid", self.bound).on_toggle(|a| SettingsEdit(Change::Bound(a)));
        
        column![circles, points, prims, node_point, node_line, bound_grid, ]
    }
}

impl Default for AppSettings {
    fn default () -> Self {
        Self {
            shown: false,
            zoom: Zoom::default(),
            grid: Grid::default(),
            points_show: true,
            prims_show: true,
            circles_show: true,
            node_point_show: true,
            node_mode: NodeMode::None {},
            bound: false,
            grid_modes: ["Circles", "Squares", "None"],
            write_zoom: [text_editor::Content::default(), text_editor::Content::default(), text_editor::Content::default()]
        }
    }
}
