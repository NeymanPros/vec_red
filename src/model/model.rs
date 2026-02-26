use iced::Point;
use crate::app::undo_manager::UndoManager;
use super::borrow_model::*;
use super::own_model::*;

/// Tools to draw [Framework].
#[derive(Debug)]
pub enum Model {
    Own {model: OwnModel},
    Borrow {model: BorrowModel<'a>}
}

impl<'a> Model<'a> {
    pub fn make_borrow(&mut self, points_ref: (*mut *mut TBPoint, *mut i32)) {
        *self = Self::Borrow {
            model: BorrowModel::new (
                points_ref
            )
        }
    }
    pub(super) fn is_borrowed(&self) -> bool {
        match self {
            Model::Own{..} => false,
            _ => true
        }
    }
}

impl Model<'_> {
    pub fn points(&self, index: usize) -> Point {
        match self {
            Self::Own { model } => model.points[index].0,
            Self::Borrow { model } => model.get_point(index)
        }
    }
    pub fn points_r(&self, index: usize) -> f32 {
        match self {
            Self::Own { model } => model.points[index].1,
            Self::Borrow { model } => model.get_point_r(index)
        }
    }
    pub fn prims(&self, index: usize) -> &[i32; 3] {
        match self {
            Self::Own { model } => &model.prims[index],
            Self::Borrow { model } => model.get_prim(index)
        }
    }
    pub fn nodes(&self, index: usize) -> Point {
        match self {
            Self::Own { model } => model.node_points[index],
            Self::Borrow { model } => model.get_node(index)
        }
    }
    pub fn node_lines_bm(&self, index: usize) -> f32 {
        0.
    }
    pub fn elems(&self, index: usize) -> &[i32; 3] {
        match self {
            Self::Own { model } => &model.node_lines[index],
            Self::Borrow { model } => model.get_elem(index)
        }
    }
    pub fn point_set(&mut self, num: usize, point: (Point, f32)) {
        match self {
            Self::Own { model } => model.points[num] = point,
            Self::Borrow { model } => model.point_set(num, point)
        }
    }

    pub fn points_len(&self) -> usize {
        match self {
            Self::Own { model } => model.points.len(),
            Self::Borrow { model } => model.points.len()
        }
    }
    pub fn prims_len(&self) -> usize {
        match self {
            Self::Own { model } => model.prims.len(),
            Self::Borrow { model } => model.prims.len()
        }
    }
    pub fn nodes_len(&self) -> usize {
        match self {
            Self::Own { model } => model.node_points.len(),
            Self::Borrow { model } => model.nodes.len()
        }
    }
    pub fn elems_len(&self) -> usize {
        match self {
            Self::Own { model } => model.node_lines.len(),
            Self::Borrow { model } => model.elems.len()
        }
    }
}

impl Model<'_> {
    pub fn points_push(&mut self, point: Point, circle: f32) {
        match self {
            Self::Own { model } => model.points.push((point, circle)),
            Self::Borrow { model } => model.points_push(point, circle)
        }
    }
    pub fn prims_push(&mut self, prim: [i32; 3]) {
        match self {
            Self::Own { model } => model.prims.push(prim),
            Self::Borrow { model } => model.prims_push(prim)
        }
    }
    pub fn points_pop(&mut self) {
        match self {
            Self::Own { model } => { model.points.pop(); },
            Self::Borrow { model } => { model.points.pop(); }
        }
    }
    pub fn points_swap(&mut self, a: usize, b: usize) {
        match self {
            Self::Own { model } => { model.points.swap(a, b) },
            Self::Borrow { model } => { model.points_swap(a, b) }
        }
    }
    pub fn prims_pop(&mut self) {
        match self {
            Self::Own { model } => { model.prims.pop(); },
            Self::Borrow { model } => { model.prims.pop(); }
        }
    }
    pub fn prims_insert(&mut self, index: usize, element: [i32; 3]) {
        match self {
            Self::Own { model } => model.prims.insert(index, element),
            Self::Borrow { model } => model.prims_insert(index, element)
        }
    }
}


impl Model<'_> {
    pub fn clear(&mut self) {
        match self {
            Self::Own {model} => model.clear(), 
            Self::Borrow {model} => model.clear()
        };
    }
    pub fn find_point(&self, point: Point, scale: f32, zoom_scale: f32) -> usize {
        match self {
            Self::Own {model} => model.find_point(point, scale, zoom_scale), 
            Self::Borrow {model} => model.find_point(point, scale, zoom_scale)
        }
    }
    pub fn find_min_max(&self) -> (Point, Point) {
        match self {
            Self::Own {model} => model.find_min_max(),
            Self::Borrow {model} => model.find_min_max()
        }
    }
    pub fn replace_prim(&mut self, one: i32, two: i32) {
        match self {
            Self::Own {model} => model.replace_prim(one, two), 
            Self::Borrow {model} => model.replace_prim(one, two)
        }
    }
    pub fn prims_retain_safe<F>(&mut self, f: F, journal: &mut UndoManager) 
    where
        F: FnMut(&[i32; 3]) -> bool
    {
        match self {
            Self::Own { model } => model.prims_retain_safe(f, journal),
            Self::Borrow { model } => model.prims_retain_safe(f, journal)
        }
    }
}

impl Default for Model<'_> {
    fn default() -> Self {
        Self::Own {model: OwnModel::default()}
    }
}
