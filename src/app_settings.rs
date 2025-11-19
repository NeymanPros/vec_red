use iced::{Center, Fill, Point, Vector};
use iced::widget::{button, checkbox, column, row, text, text_editor};
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



#[derive(Debug, Clone)]
pub enum Change {
    Open,
    ZoomWrite(usize, text_editor::Action),
    Circles(bool),
    Dots(bool),
    Lines(bool),
    Bound(bool),
    NodeDotShow(bool),
    NodeLineShow(bool),
    GridMode(&'static str)
}


pub struct AppSettings {
    pub shown: bool,
    
    /// (value - shift) * scale
    pub zoom: Zoom,
    pub grid: Grid,
    pub dots_show: bool,
    pub lines_show: bool,
    pub circles_show: bool,
    pub node_dot_show: bool,
    pub node_line_show: bool,
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
            Change::Dots(new) => {
                self.dots_show = new
            }
            Change::Lines(new) => {
                self.lines_show = new
            }
            Change::NodeDotShow(new) => {
                    self.node_dot_show = new;
            }
            Change::NodeLineShow(new) => {
                    self.node_line_show = new;
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
        let dots = checkbox("Dots", self.dots_show).on_toggle(|a| SettingsEdit(Change::Dots(a)));
        let lines = checkbox("Lines", self.lines_show).on_toggle(|a| SettingsEdit(Change::Lines(a)));
        let node_dot = checkbox("Node dots", self.node_dot_show).on_toggle(|a| SettingsEdit(Change::NodeDotShow(a)));
        let node_line = checkbox("Node lines", self.node_line_show).on_toggle(|a| SettingsEdit(Change::NodeLineShow(a)));
        let bound_grid = checkbox("Bound to grid", self.bound).on_toggle(|a| SettingsEdit(Change::Bound(a)));
        
        column![circles, dots, lines, node_dot, node_line, bound_grid, ]
    }
}

impl Default for AppSettings {
    fn default () -> Self {
        Self {
            shown: false,
            zoom: Zoom::default(),
            grid: Grid::default(),
            dots_show: true,
            lines_show: true,
            circles_show: true,
            node_dot_show: true,
            node_line_show: true,
            bound: false,
            grid_modes: ["Circles", "Squares", "None"],
            write_zoom: [text_editor::Content::default(), text_editor::Content::default(), text_editor::Content::default()]
        }
    }
}
