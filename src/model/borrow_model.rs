use std::rc::Rc;
use iced::Point;
use libloading::Library;
use crate::app::undo_manager::UndoManager;
use crate::foreign_functions::*;
use super::borrow_types::*;

#[allow(unused_lifetimes)]
#[derive(Debug)]
pub(super) struct BorrowModel {
    points_ref: *const *mut TBPoint,
    points_len: i32,
    prims_ref: *const *mut TPrimitive,
    prims_len: i32,
    nodes_ref: *const *mut TNode,
    nodes_len: i32,
    elems_ref: *const *mut TElement,
    elems_len: i32,
    regions_ref: *const *mut TRegion,

    lib: Rc<Library>
}

impl BorrowModel {
    pub(super) fn new(lib: Rc<Library>, 
                      points_ref: (*const *mut TBPoint, i32), 
                      prims_ref: (*const *mut TPrimitive, i32), 
                      nodes_ref: (*const *mut TNode, i32),
                      elems_ref: (*const *mut TElement, i32),
                      regions_ref: *const *mut TRegion) -> Self {
        Self {
            lib: lib.clone(),
            points_ref: points_ref.0,
            points_len: points_ref.1,
            prims_ref: prims_ref.0,
            prims_len: prims_ref.1,
            nodes_ref: nodes_ref.0,
            nodes_len: nodes_ref.1,
            elems_ref: elems_ref.0,
            elems_len: elems_ref.1,
            regions_ref
        }
    }
    pub(super) fn sync_points(&mut self) {
        let points = get_points_ref(self.lib.clone());
        self.points_len = points.1;
    }
    pub(super) fn sync_prims(&mut self) {
        let prims = get_prims_ref(self.lib.clone());
        //self.prims_ref = prims.0;
        self.prims_len = prims.1;
    }
    pub(super) fn sync_nodes(&mut self) {
        let nodes = get_nodes_ref(self.lib.clone());
        //self.nodes_ref = nodes.0;
        self.nodes_len = nodes.1;
    }
    pub(super) fn sync_elems(&mut self) {
        let elems = get_elems_ref(self.lib.clone());
        self.elems_len = elems.1;
    }
    
    pub(super) fn sync_everything(&mut self) {
        self.sync_points();
        self.sync_prims();
        self.sync_nodes();
        self.sync_elems();
    }
}

impl BorrowModel {
    pub(super) fn tb_point_ref(&self, index: usize) -> Option<&mut TBPoint> {
        if index < self.points_len as usize {
            unsafe {
                Some(
                    &mut *(*self.points_ref).add(index)
                )
            }
        } else {
            None
        }
    }
    pub(super) fn get_point(&self, index: usize) -> Point {
        assert!(index < self.points_len as usize);
        unsafe {
            let tb_point = (*self.points_ref).add(index);
            Point::new((*tb_point).x as f32, (*tb_point).y as f32)
        }
    }
    pub(super) fn get_point_r(&self, index: usize) -> f32 {
        assert!(index < self.points_len as usize);
        unsafe {
            (*(*self.points_ref).add(index)).r as f32
        }
    }
    pub(super) fn point_set(&mut self, num: usize, point: Point, point_r: f32) {
        unsafe {
            assert!(num < self.points_len as usize);
            let points = std::slice::from_raw_parts_mut(*self.points_ref, self.points_len as usize);
            f_set_point(self.lib.clone(), num as i32, &point);
            points[num].r = point_r as f64;
        }
    }
    pub(super) fn points_len(&self) -> usize { self.points_len as usize }
    pub(super) fn points_push(&mut self, point: Point, circle: f32) {
        f_create_point(self.lib.clone(), (point, circle));
        self.sync_points()
    }
    pub(super) fn points_swap(&mut self, a: usize, b: usize) {
        unsafe {
            assert!(a < self.points_len as usize);
            assert!(b < self.points_len as usize);
            let points = std::slice::from_raw_parts_mut(*self.points_ref, self.points_len as usize);
            points.swap(a, b);
        }
    }
    pub(super) fn points_pop(&mut self) {
        if self.points_len >= 1 {
            f_del_point(self.lib.clone(), self.points_len - 1);
            self.sync_points()
        }
    }
    
    pub(super) fn t_primitive_ref(&self, index: usize) -> Option<&mut TPrimitive> {
        if index < self.prims_len as usize {
            unsafe {
                Some(
                    &mut *(*self.prims_ref).add(index)
                )
            }
        } else {
            None
        }
    }
    pub(super) fn get_prim(&self, index: usize) -> &[i32; 3] {
        assert!((index as i32) < self.prims_len);
        unsafe { 
            &(*(*self.prims_ref).add(index)).p
        }
    }

    pub(super) fn prims_len(&self) -> usize { self.prims_len as usize }

    pub(super) fn prims_push(&mut self, prim: [i32; 3]) {
        f_create_prim(self.lib.clone(), &prim);
        self.sync_everything()
    }
    pub(super) fn prims_insert(&mut self, index: usize, element: [i32; 3]) {
        todo!();
        //self.prims.insert(index, TPrimitive {p: element, ..Default::default()})
        
    }
    pub(super) fn prims_pop(&mut self) {
        if self.prims_len >= 1 {
            f_del_prim(self.lib.clone(), self.prims_len - 1);
            self.sync_prims();
        }
    }

    pub(super) fn t_node_ref(&self, index: usize) -> Option<&mut TNode> {
        if index < self.nodes_len as usize {
            unsafe {
                Some(
                    &mut *(*self.nodes_ref).add(index)
                )
            }
        } else {
            None
        }
    }
    pub(super) fn nodes_len(&self) -> usize {
        self.nodes_len as usize
    }
    pub(super) fn node(&self, index: usize) -> Point {
        assert!((index as i32) < self.nodes_len);
        unsafe {
            let t_node = &(*(*self.nodes_ref).add(index));
            Point::new(t_node.x as f32, t_node.y as f32)
        }
    }

    pub(super) fn elems_len(&self) -> usize {
        self.elems_len as usize
    }
    pub(super) fn elem(&self, index: usize) -> &[i32; 3] {
        assert!((index as i32) < self.elems_len);
        unsafe {
            &(*(*self.elems_ref).add(index)).m
        }
    }
    
    pub(super) fn t_region_ref(&self, index: usize) -> Option<&mut TRegion> {
        if index < self.elems_len as usize {
            unsafe {
                Some(
                    &mut *(*self.regions_ref).add(index)
                )
            }
        } else {
            None
        }
    }
}

impl BorrowModel {
    pub(super) fn get_bm_only(&self, index: i32) -> f32 {
        get_bm_only(self.lib.clone(), index)
    }
}

impl BorrowModel {
    pub(super) fn clear(&mut self) {
        f_del_memo_model(self.lib.clone());
        self.points_len = 0;
        self.prims_len = 0;
        self.nodes_len = 0;
        self.elems_len = 0;
    }
    pub(super) fn find_point(&self, point: Point, scale: f32, zoom_scale: f32) -> usize {
        if self.points_len == 0 {
            return 0;
        }
        unsafe {
            let points = std::slice::from_raw_parts(*self.points_ref, self.points_len as usize);
            points
                .iter()
                .position(|big_point| { Point::new(big_point.x as f32, big_point.y as f32).distance(point) < scale / zoom_scale * 2.0 })
                .unwrap_or(self.points_len as usize)
        }
    }
    pub(super) fn find_min_max(&self) -> (Point, Point) {
        if self.points_len == 0 {
            return (Point::new(0., 0.), Point::new(1000., 1000.))
        }
        unsafe {
            let points = std::slice::from_raw_parts(*self.points_ref, self.points_len as usize);
            let mut min = Point::new(points[0].x, points[0].y);
            let mut max = min;

            for point in points.iter() {
                min.x = min.x.min(point.x);
                min.y = min.y.min(point.y);
                max.x = max.x.max(point.x);
                max.y = max.y.max(point.y);
            }
            let min = Point::new(min.x as f32, min.y as f32);
            let max = Point::new(max.x as f32, max.y as f32);
            (min, max)
        }
    }
    pub(crate) fn replace_prim(&mut self, one: i32, two: i32) {
        if self.prims_len == 0 {
            return;
        }
        unsafe{
            let prims = std::slice::from_raw_parts_mut(*self.prims_ref, self.prims_len as usize);
            prims.iter_mut().for_each(|prim| {
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
    }
    pub(super) fn prims_retain_safe<F>(&mut self, mut f: F, journal: &mut UndoManager)
    where
        F: FnMut(&[i32; 3]) -> bool
    {
        if self.prims_len == 0 {
            return;
        }
        unsafe {
            let mut prims = Vec::from_raw_parts(*self.prims_ref, self.prims_len as usize, self.prims_len as usize);
            prims
                .iter()
                .enumerate()
                .rev()
                .for_each(|(placement, x)| {
                    if !f(&x.p) {
                        journal.deleted_prim(placement, x.p.clone())
                    }
                });
            prims.retain(|x| {
                f(&x.p)
            });
            self.prims_len = prims.len() as i32;
        }
    }
}
