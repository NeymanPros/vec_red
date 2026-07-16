use iced::Point;
use crate::{Message, VecRed};

impl VecRed {
    #[inline]
    pub(super) fn def_point(&mut self, point: Point) {
        let number = self.model.find_point(point, self.scale, self.app_config.zoom.scale);
        if self.mode == "Region" && number == self.model.points_len() {
            self.update(Message::CreateRegion(point))
        } else if self.mode == "Find" {
            self.update(Message::FindEverything(point.x as f64, point.y as f64))
        } else {
            if number == self.model.points_len() {
                self.journal.pushed_point();
                self.model.points_push(point, self.default_circle);
                self.state.redraw();
            }
            self.chosen_point = Some((self.model.points(number), self.model.points_r(number), number));
            self.point_string = vec![
                self.chosen_point.as_ref().unwrap().0.x.to_string(),
                self.chosen_point.as_ref().unwrap().0.y.to_string(),
                self.chosen_point.as_ref().unwrap().1.to_string()
            ]
        }
    }
    
    #[inline]
    pub(super) fn def_prim(&mut self, points: Vec<Point>, prim: (i32, i32, i32)) {
        let zoom_scale = self.app_config.zoom.scale;
        let add_point = |vec_red: &mut VecRed, point: Point| {
            let number = vec_red.model.find_point(point, vec_red.scale, zoom_scale);
            if number == vec_red.model.points_len() {
                vec_red.journal.pushed_point();
                vec_red.model.points_push(point, vec_red.default_circle);
            }
            number
        };
        let a = add_point(self, points[0]);
        let b = add_point(self, points[1]);
        if prim.2 == -1 {
            if a != b {
                self.journal.pushed_prim();
                self.model.prims_push([a as i32, b as i32, -1])
            }
        } else {
            let c = add_point(self, points[2]);
            if a != b && a != c && b != c {
                self.journal.pushed_prim();
                self.model.prims_push([a as i32, b as i32, c as i32])
            }
        }
        self.state.redraw();
        self.chosen_point = None
    }
}
