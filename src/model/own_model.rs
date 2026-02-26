use iced::Point;
use crate::app::undo_manager::UndoManager;

#[derive(Debug, Default)]
pub(super) struct OwnModel {
    pub(super) points: Vec<(Point, f32)>,
    pub(super) prims: Vec<[i32; 3]>,
    pub(super) node_points: Vec<Point>,
    pub(super) node_lines: Vec<[i32; 3]>
}

impl OwnModel {
    pub(super) fn find_point(&self, point: Point, scale: f32, zoom_scale: f32) -> usize {
        self.points
            .iter()
            .position(|x| { x.0.distance(point) < scale / zoom_scale * 2.0 })
            .unwrap_or(self.points.len())
    }

    pub(super) fn find_min_max(&self) -> (Point, Point) {
        if let Some(min) = self.points.get(0) {
            let mut min = min.0;
            let mut max = min;

            for (point, _) in &self.points {
                min.x = min.x.min(point.x);
                min.y = min.y.min(point.y);
                max.x = max.x.max(point.x);
                max.y = max.y.max(point.y);
            }

            (min, max)
        }
        else {
            (Point::new(0., 0.), Point::new(1000., 1000.))
        }
    }

    pub(super) fn replace_prim(&mut self, one: i32, two: i32) {
        self.prims.iter_mut().for_each(|x|{
            if x[0] == one {
                x[0] = two
            } else if x[0] == two {
                x[0] = one
            }

            if x[1] == one {
                x[1] = two
            } else if x[1] == two {
                x[1] = one
            }

            if x[2] == one {
                x[2] = two
            } else if x[2] == two {
                x[2] = one
            }
        })
    }
    
    pub(super) fn prims_retain_safe<F>(&mut self, mut f: F, journal: &mut UndoManager) 
    where
        F: FnMut(&[i32; 3]) -> bool
    {
        self.prims
            .iter()
            .enumerate()
            .rev()
            .for_each(|(placement, x)| {
                if !f(x) {
                    journal.deleted_prim(placement, x.clone())
                }
            });
        self.prims.retain(f);
    }
}

impl OwnModel {
    pub(super) fn clear(&mut self) {
        self.points.clear();
        self.prims.clear();
        self.node_points.clear();
        self.node_lines.clear();
    }
}