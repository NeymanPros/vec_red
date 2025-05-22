use iced::Point;
use rand::random_range;

pub fn excel_dots () -> Vec<(Point, f32)> {
    let mut dots: Vec<(Point, f32)> = Vec::with_capacity(1500);

    for _i in 0..1500 {
        dots.push((Point::new(random_range(10..=940) as f32, random_range(10..=940) as f32), 20.0));
    }

    dots
}

pub fn excel_lines() -> Vec<(i32, i32)> {
    vec![(0, 1), (0, 2), (1, 2)]
}
