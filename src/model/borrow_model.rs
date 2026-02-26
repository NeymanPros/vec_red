use iced::Point;
use crate::app::undo_manager::UndoManager;

#[derive(Debug)]
pub(super) struct BorrowModel {
    pub(super) points: Vec<TBPoint>, 
    pub(super) prims: Vec<TPrimitive>,
    pub(super) nodes: Vec<TNode>,
    pub(super) elems: Vec<TElement>,
    points_ref: *mut *mut TBPoint,
    points_len: *mut i32
}

impl BorrowModel {
    pub(super) fn new(points_ref: (*mut *mut TBPoint, *mut i32)) -> Self {
        unsafe {
            let points = Vec::from_raw_parts(*points_ref.0, *points_ref.1 as usize, *points_ref.1 as usize);
            Self {
                points,
                prims: vec![],
                nodes: vec![],
                elems: vec![],
                points_ref: points_ref.0,
                points_len: points_ref.1
            }
        }
    }
    pub(super) fn sync_points(&mut self) {
        unsafe {
            *self.points_ref = self.points.as_mut_ptr();
            *self.points_len = self.points.len() as i32;
        };
    }
}

impl BorrowModel {
    pub(super) fn get_point(&self, index: usize) -> Point {
        Point::new(self.points[index].x as f32, self.points[index].y as f32)
    }
    pub(super) fn get_point_r(&self, index: usize) -> f32 {
        self.points[index].r as f32
    }
    pub(super) fn get_prim(&self, index: usize) -> &[i32; 3] {
        &self.prims[index].p
    }
    pub(super) fn get_node(&self, index: usize) -> Point {
        Point::new(self.nodes[index].x as f32, self.nodes[index].y as f32)
    }
    pub(super) fn get_elem(&self, index: usize) -> &[i32; 3] {
        &self.elems[index].m
    }
    
    pub(super) fn point_set(&mut self, num: usize, point: (Point, f32)) {
        self.points[num].x = point.0.x as f64;
        self.points[num].y = point.0.y as f64;
        self.points[num].r = point.1 as f64;

    }
    
    pub(super) fn points_push(&mut self, point: Point, circle: f32) {
        self.points.push(TBPoint {x: point.x as f64, y: point.y as f64, r: circle as f64, ..Default::default()})
    }
    pub(super) fn prims_push(&mut self, prim: [i32; 3]) {
        self.prims.push(TPrimitive {p: prim, ..Default::default()})
    }
    pub(super) fn prims_insert(&mut self, index: usize, element: [i32; 3]) {
        self.prims.insert(index, TPrimitive {p: element, ..Default::default()})
    }
    pub(super) fn points_swap(&mut self, a: usize, b: usize) { self.points.swap(a, b); }
}

impl BorrowModel {
    #[allow(duplicate_macro_attributes)]
    pub(super) fn clear(&mut self) {
        self.points.clear();
        self.prims.clear();
        self.nodes.clear();
        self.elems.clear();
    }
    pub(super) fn find_point(&self, point: Point, scale: f32, zoom_scale: f32) -> usize {
        self.points
            .iter()
            .position(|big_point| { Point::new(big_point.x as f32, big_point.y as f32).distance(point) < scale / zoom_scale * 2.0 })
            .unwrap_or(self.points.len())
    }
    pub(super) fn find_min_max(&self) -> (Point, Point) {
        if let Some(min) = self.points.get(0) {
            let mut min = Point::new(min.x, min.y);
            let mut max = min;

            for point in self.points.iter() {
                min.x = min.x.min(point.x);
                min.y = min.y.min(point.y);
                max.x = max.x.max(point.x);
                max.y = max.y.max(point.y);
            }
            let min = Point::new(min.x as f32, min.y as f32);
            let max = Point::new(max.x as f32, max.y as f32);
            (min, max)
        }
        else {
            (Point::new(0., 0.), Point::new(1000., 1000.))
        }
    }
    pub(crate) fn replace_prim(&mut self, one: i32, two: i32) {
        self.prims.iter_mut().for_each(|prim|{
            if prim.p[0] == one {
                prim.p[0] = two
            } else if prim.p[0] == two {
                prim.p[0] = one
            }

            if prim.p[1] == one {
                prim.p[1] = two
            } else if prim.p[1] == two {
                prim.p[1] = one
            }

            if prim.p[2] == one {
                prim.p[2] = two
            } else if prim.p[2] == two {
                prim.p[2] = one
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
                if !f(&x.p) {
                    journal.deleted_prim(placement, x.p.clone())
                }
            });
        self.prims.retain(|x|{
            f(&x.p)
        });
    }
}

#[derive(Debug, Default)]
pub struct TBPoint {
    x: f64, 
    y: f64, 
    r: f64, 
}

#[derive(Debug, Default)]
pub struct TPrimitive {
    p: [i32; 3]
}

#[derive(Debug)]
pub struct TNode {
    x: f64, 
    y: f64
}

#[derive(Debug)]
pub struct TElement {
    m: [i32; 3]
}
