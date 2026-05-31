use iced::Point;
use libloading::Library;
use crate::foreign_functions::*;
use crate::VecRed;

use crate::model::borrow_types::*;

/// Impl function to assign values to fields by their number.
macro_rules! impl_field_setter {
    ($struct_name:ident, $($rest:tt)*) => {
        impl $struct_name {
            pub fn set_field_by_index(&mut self, index: usize, value: &str) -> Result<(), String> {
                let mut _counter: usize = 0;
                impl_field_setter!(@step self, index, value, _counter, $($rest)*);
                Err(format!("Field index {} out of range", index))
            }
        }
    };

    (@step $self:ident, $idx:ident, $val:ident, $counter:ident,
        $field:ident: [$type:ty; $len:literal], $($rest:tt)*) => {
        for _elem in 0..$len {
            if $idx == $counter {
                $self.$field[_elem] = $val.parse::<$type>()
                    .map_err(|_| format!("Failed to parse '{}' for {}[{}]", $val, stringify!($field), _elem))?;
                return Ok(());
            }
            $counter += 1;
        }
        impl_field_setter!(@step $self, $idx, $val, $counter, $($rest)*)
    };

    (@step $self:ident, $idx:ident, $val:ident, $counter:ident,
        $field:ident: $type:ty, $($rest:tt)*) => {
        if $idx == $counter {
            $self.$field = $val.parse::<$type>()
                .map_err(|_| format!("Failed to parse '{}' for field '{}'", $val, stringify!($field)))?;
            return Ok(());
        }
        $counter += 1;
        impl_field_setter!(@step $self, $idx, $val, $counter, $($rest)*)
    };

    (@step $self:ident, $idx:ident, $val:ident, $counter:ident $(,)?) => {};
}

impl VecRed {
    #[inline(always)]
    pub(super) fn open_math_core(&mut self) {
        if self.lib.is_some() {
            println!("Already opened!");
            return;
        }
        self.lib = unsafe {
            let temp_lib = Library::new("/home/alexe//Documents/FLib.dll");
            if temp_lib.is_err() {
                println!("No library");
                return;
            }
            Some(
                std::rc::Rc::new(
                    temp_lib.unwrap()
                ))
        };
        let lib = self.lib.as_ref().unwrap();
        f_init_model(lib.clone());
        for i in 0..self.model.points_len() {
            f_create_point(lib.clone(), (self.model.points(i), self.model.points_r(i)));
        }
        for j in 0..self.model.prims_len() {
            f_create_prim(lib.clone(), self.model.prims(j));
        }

        let points_ref = get_points_ref(lib.clone());
        let prims_ref = get_prims_ref(lib.clone());
        let nodes_ref = get_nodes_ref(lib.clone());
        let elems_ref = get_elems_ref(lib.clone());
        let library = self.lib.as_ref().unwrap().clone();
        self.model.make_borrow(library, points_ref, prims_ref, nodes_ref, elems_ref);
        
        impl_field_setter![TBPoint, x: f64, y: f64, r: f64, TypPoint: u8, Vp: f64, Ip: f64, NNode: i32, ];
        impl_field_setter![TPrimitive, p: [i32; 3], TypPrim: u8, IsFront: bool, Vp: f64, Ip: f64, ];
        //impl_field_setter![TNode, x: f64, y: f64, VP: f64, TypNode: u8, PNode: i32, KolSW: i32, NSW: *mut i32, KolSI: i32, NSI: *mut i32, NSK: *mut i8, vKolSWMemo: i32, vKolSIMemo: i32, F: f64, Yp: f64, ];
        impl_field_setter![TElement, m: [i32; 3], IZP: i16, Px: f64, Py: f64, XNJU: f64, A1: f64, Delta: f64, xs: f64, ys: f64, S: [f64; 6], A: [f64; 3], B: [f64; 3], C: [f64; 3], ];
        println!("Finished");
        self.state.redraw();
    }
    
    #[inline(always)]
    pub(super) fn create_region(&mut self, point: Point) {
        if let Some(lib) = self.lib.as_ref() {
            let out = f_create_region(lib.clone(), &point);
            println!("Region is {out}");
        }
    }
    
    #[inline(always)]
    pub(super) fn create_triangle(&mut self) {
        if let Some(lib) = self.lib.as_ref() {
            let out = f_build_fm(lib.clone());
            println!("Triangle is {}", out);
            self.model.sync_everything();
            self.state.redraw()
        }
    }
}
