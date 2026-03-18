use iced::Point;
use libloading::Library;
use crate::app::undo_manager::UndoManager;
use super::borrow_model::*;
use super::own_model::*;

/// Tools to draw [Framework].
#[derive(Debug)]
pub enum Model {
    Own {model: OwnModel},
    Borrow {model: BorrowModel}
}

impl Model {
    pub fn make_borrow(&mut self, lib: std::rc::Rc<Library>, 
                       points_ref: (*const *mut TBPoint, /* *mut*/ i32),
                       prims_ref: (*const *mut TPrimitive, i32),
                       nodes_ref: (*const *mut TNode, i32),
                       elems_ref: (*const *mut TElement, i32)) {
        *self = Self::Borrow {
            model: BorrowModel::new (
                lib, 
                points_ref,
                prims_ref, 
                nodes_ref, 
                elems_ref
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

impl Model {
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
            Self::Borrow { model } => model.points_len()
        }
    }
    pub fn prims_len(&self) -> usize {
        match self {
            Self::Own { model } => model.prims.len(),
            Self::Borrow { model } => model.prims_len()
        }
    }
    pub fn nodes_len(&self) -> usize {
        match self {
            Self::Own { model } => model.node_points.len(),
            Self::Borrow { model } => model.nodes_len()
        }
    }
    pub fn elems_len(&self) -> usize {
        match self {
            Self::Own { model } => model.node_lines.len(),
            Self::Borrow { model } => model.elems_len()
        }
    }
}

impl Model {
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
            Self::Borrow { model } => { model.points_pop(); }
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
            Self::Borrow { model } => { model.prims_pop(); }
        }
    }
    pub fn prims_insert(&mut self, index: usize, element: [i32; 3]) {
        match self {
            Self::Own { model } => model.prims.insert(index, element),
            Self::Borrow { model } => model.prims_insert(index, element)
        }
    }
}

impl Model {
    pub fn get_bm_only(&self, index: i32) -> f32 {
        match self {
            Self::Borrow {model} => model.get_bm_only(index),
            _ => 0.
        }
    }
}

impl Model {
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

impl Default for Model {
    fn default() -> Self {
        Self::Own {model: OwnModel::default()}
    }
}
