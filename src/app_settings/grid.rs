use iced::{Color, Point, Rectangle, Renderer, Theme, Vector, Fill};
use iced::mouse::Cursor;
use iced::widget::canvas;
use iced::widget::canvas::{Geometry, Path, Stroke};
use crate::Message;

struct GridDrawing<'a> {
    grid_info: &'a GridInfo,
    grid_display: &'a GridDisplay
}

impl canvas::Program<Message> for GridDrawing<'_> {
    type State = ();

    fn draw(&self, _state: &Self::State, renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry<Renderer>> {
        let content = self.grid_display.cache.draw(renderer, bounds.size(), |frame| {
            self.grid_info.draw_grid(frame, &self.grid_info);
        });

        vec![content]
    }
}

#[derive(Default)]
struct GridDisplay {
    cache: canvas::Cache
}

impl GridDisplay {
    fn view<'a>(&'a self, grid_info: &'a GridInfo) -> iced::Element<'a, Message> {
        canvas(GridDrawing {
            grid_info: &grid_info,
            grid_display: self
        })
            .height(Fill)
            .width(Fill)
            .into()
    }

    fn redraw_grid(&mut self) {
        self.cache.clear()
    }
}

struct GridInfo {
    display: &'static str,
    distance: f32,
    thickness: f32,
    color: Color,
}

impl GridInfo {
    fn draw_grid(&self, frame: &mut canvas::Frame, grid_info: &GridInfo) {
        if grid_info.display == "Squares" {
            let mut build = Point::new(0.0, 0.0);
            let str = Stroke::default().with_width(grid_info.thickness).with_color(grid_info.color);
            while build.x < 2000.0 {
                let line = Path::line(build, build + Vector::new(0.0, 2000.0));
                frame.stroke(&line, str);

                build.x += grid_info.distance;
            }
            build = Point::new(0.0, 0.0);
            while build.y < 2000.0 {
                let line = Path::line(build, build + Vector::new(2000.0, 0.0));
                frame.stroke(&line, str);

                build.y += grid_info.distance;
            }
        }
        else if grid_info.display == "Circles" {
            let mut point = Point::new(0.0, 0.0);
            while point.x < 2000.0 {
                point.y = 0.0;
                while point.y < 2000.0 {
                    let build = Path::circle(point, grid_info.thickness * 2.0);
                    frame.fill(&build, grid_info.color);

                    point.y += grid_info.distance
                }
                point.x += grid_info.distance
            }
        }
    }
}

impl Default for GridInfo {
    fn default() -> Self {
        Self {
            thickness: 1.0,
            distance: 25.0,
            display: "None",
            color: Color::from_rgba8(100, 100, 100, 0.5)
        }
        
    }
}

pub struct Grid {
    display: GridDisplay,
    info: GridInfo
}

impl Grid {
    pub fn view(&self) -> iced::Element<'_, Message> {
        self.display.view(&self.info)
    }
    
    pub fn redraw (&mut self) {
        self.display.redraw_grid()
    }
    
    pub fn set_display(&mut self, new_value: &'static str) {
        self.info.display = new_value
    }
    
    pub fn get_display(&self) -> &'static str {
        self.info.display
    }
}

impl Grid {
    pub fn bound(&self, point: &Point) -> Point {
        if self.get_display() != "None" {
            let mut new_point = Point::default();
            new_point.x = (point.x / self.info.distance).round() * self.info.distance;
            new_point.y = (point.y / self.info.distance).round() * self.info.distance;
            new_point
        }
        else {
            *point
        }
    }
}

impl Default for Grid {
    fn default () -> Self {
        Self {
            display: GridDisplay::default(),
            info: GridInfo::default()
        }
    }
}
