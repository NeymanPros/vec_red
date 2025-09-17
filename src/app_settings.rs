use iced::{Center, Fill, Point, Vector};
use iced::widget::{button, checkbox, column, text_input};
use crate::Message;
use crate::grid::Grid;


#[derive(Copy, Clone)]
pub struct Zoom {
    /// value * zoom + shift
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
    /// Creates a new [Point] by multiplying the old one's coordinates on a given number, then shifting its position.
    pub fn apply (&self, point: Point) -> Point {
        let mut result = point;
        result.x *= self.scale;
        result.y *= self.scale;
        result + self.shift
    }
    /// Draws back the effect of [Zoom::multi_and_shift].
    pub fn reverse(&self, point: Point) -> Point {
        let mut result = point;
        result = result - self.shift;
        result.x /= self.scale;
        result.y /= self.scale;
        result
    }
}



#[derive(Debug, Clone)]
pub enum Change {
    Shift(Vector),
    Resize(f32),
    Circles(bool),
    Dots(bool),
    Lines(bool),
    GridMode(&'static str)
}


pub struct AppSettings {
    pub shown: bool,
    pub zoom: Zoom,
    pub grid: Grid,
    pub dots_show: bool,
    pub lines_show: bool,
    pub circles_show: bool,
    grid_modes: [&'static str; 3]
}

impl AppSettings {
    pub fn update(&mut self, message: Change) {
        match message {
            Change::Resize(extent) => {
                if extent == 0.0 {
                    self.zoom.scale = 0.0;
                    self.zoom.shift = Vector::default()
                } else {
                    self.zoom.scale *= extent
                }
            }

            Change::Shift(add_shift) => {
                self.zoom.shift = self.zoom.shift + add_shift * (1.0 / self.zoom.scale);
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
            Change::GridMode(new) => {
                self.grid.set_display(new)
            }
        }
    }
    pub fn view(&self) -> iced::Element<'_, Message> {
        let go_back = button("Go back").on_press(Message::SettingsOpen(false));
        let circles = checkbox("Circles", self.circles_show).on_toggle(|a| Message::SettingsEdit(Change::Circles(a)));
        let dots = checkbox("Dots", self.dots_show).on_toggle(|a| Message::SettingsEdit(Change::Dots(a)));
        let lines = checkbox("Lines", self.lines_show).on_toggle(|a| Message::SettingsEdit(Change::Lines(a)));
        let grid_mode = iced::widget::PickList::new(self.grid_modes, Some(self.grid.get_display()), |a| Message::SettingsEdit(Change::GridMode(a)));

        column![go_back, circles, dots, lines, grid_mode].width(Fill).align_x(Center).into()
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
            grid_modes: ["Circles", "Squares", "None"]
        }
    }
}
