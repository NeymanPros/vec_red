use iced::{Center, Color, Point, Vector};
use iced::widget::{button, canvas, checkbox, column, text_input};
use iced::widget::canvas::{Path, Stroke};
use crate::Message;


#[derive(Copy, Clone)]
pub struct Zoom {
    pub scale: f32, // value * zoom
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


pub struct Grid {
    display: &'static str,
    distance: f32,
    thickness: f32,
    color: Color
}

impl Grid {
    pub fn draw_grid(&self, frame: &mut canvas::Frame, zoom: &Zoom) {
        if self.display == "Squares" {
            let mut build = Point::default() + zoom.shift;
            let str = Stroke::default().with_width(self.thickness).with_color(self.color);
            while build.x < zoom.shift.x + 2000.0 {
                let line = Path::line(build, build + Vector::new(0.0, 2000.0));
                frame.stroke(&line, str);

                build.x += self.distance;
            }
            build = Point::new(self.distance, self.distance) + zoom.shift;
            while build.y < zoom.shift.y + 2000.0 {
                let line = Path::line(build, build + Vector::new(2000.0, 0.0));
                frame.stroke(&line, str);

                build.y += self.distance;
            }
        }
        else if self.display == "Circles" {
            let mut build = Point::new(self.distance, self.distance) + zoom.shift;
            while build.x < zoom.shift.x + 2000.0 {
                build.y = zoom.shift.y;
                while build.y < zoom.shift.y + 2000.0 {
                    let point = Path::circle(build, self.thickness * 2.0);
                    frame.fill(&point, self.color);

                    build.y += self.distance
                }
                build.x += self.distance
            }
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            thickness: 2.0,
            distance: 50.0,
            display: "None",
            color: Color::from_rgb8(100, 100, 100)
        }
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
                self.grid.display = new
            }
        }
    }
    pub fn view(&self) -> iced::Element<Message> {
        let go_back = button("Go back").on_press(Message::SettingsOpen(false));
        let circles = checkbox("Circles", self.circles_show).on_toggle(|a| Message::SettingsEdit(Change::Circles(a)));
        let dots = checkbox("Dots", self.dots_show).on_toggle(|a| Message::SettingsEdit(Change::Dots(a)));
        let lines = checkbox("Lines", self.lines_show).on_toggle(|a| Message::SettingsEdit(Change::Lines(a)));
        let grid_mode = iced::widget::pick_list(self.grid_modes, Some(self.grid.display), |a| Message::SettingsEdit(Change::GridMode(a)));

        column![go_back, circles, dots, lines, grid_mode].align_x(Center).into()
    }
}

impl Default for AppSettings {
    fn default () -> Self {
        Self {
            shown: false,
            zoom: Default::default(),
            grid: Default::default(),
            dots_show: true,
            lines_show: true,
            circles_show: true,
            grid_modes: ["Circles", "Squares", "None"]
        }
    }
}
