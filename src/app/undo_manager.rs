use iced::Point;
use crate::model::Model;

///Contains functions to undo actions
pub struct UndoManager {
    undo_stack: Vec<Box<dyn FnOnce(&mut Model) + Send>>,
    pub max_len: usize,
}

impl UndoManager {
    pub fn clear(&mut self) {
        self.undo_stack.clear()
    }
    pub fn deleted_point(&mut self, point: (Point, f32), num: usize) {
        let func: Box<dyn FnOnce(&mut Model) + Send> = Box::new(move |model: &mut Model| {
            let len = model.points_len();
            model.points_push(point.0, point.1);
            model.points_swap(len, num);
            model.replace_prim(num as i32, len as i32)
        });
        self.push(func);
    }
    pub fn deleted_prim(&mut self, placement: usize, line: [i32; 3]) {
        let func: Box<dyn FnOnce(&mut Model) + Send> = Box::new(move |model: &mut Model| {
            model.prims_insert(placement, line);
        });
        self.push(func)
    }
    pub fn pushed_point(&mut self) {
        let func: Box<dyn FnOnce(&mut Model) + Send> = Box::new(move |model: &mut Model| {
            model.points_pop();
        });
        self.push(func);
    }
    pub fn pushed_prim(&mut self) {
        let func: Box<dyn FnOnce(&mut Model) + Send> = Box::new(move |model: &mut Model| {
            model.prims_pop();
        });
        self.push(func);
    }
    pub fn changed_point(&mut self, old: (Point, f32), num: usize) {
        let func: Box<dyn FnOnce(&mut Model) + Send> = Box::new(move |model: &mut Model| {
            model.point_set(num, old);
        });
        self.undo_stack.push(func);
    }
    fn push(&mut self, f: Box<dyn FnOnce(&mut Model) + Send>) {
        self.undo_stack.push(f);
        if self.undo_stack.len() >= self.max_len {
            let _ = self.undo_stack.remove(0);
        }
    }
    pub fn undo(&mut self) -> Box<dyn FnOnce(&mut Model) + Send> {
        if let Some(func) = self.undo_stack.pop() {
            func
        }
        else {
            Box::new(|_|{})
        }
    }
}

impl Default for UndoManager {
    fn default() -> Self {
        Self {
            undo_stack: vec![],
            max_len: 25
        }
    }
}
