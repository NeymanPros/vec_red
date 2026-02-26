use iced::Point;
use libloading::{Library, Symbol};
use crate::model::borrow_model::*;

pub fn f_init_model(lib: &Library) {
    unsafe {
        let func: Symbol<unsafe fn()> = lib.get(b"FInitModel").unwrap();
        func()
    }
}

pub fn f_create_point(lib: &Library, point: (Point, f32)) -> i32 {
    unsafe {
        let func: Symbol<unsafe fn(&f64, &f64, &f64, &u8) -> i32> = lib.get(b"FCreatePoint").expect("No create point");
        let byte = 0u8;
        let res = func(&(point.0.x as f64), &(point.0.y as f64), &(point.1 as f64), &byte);
        res
    }
}

#[allow(non_snake_case)]
pub fn f_create_prim(lib: &Library, prim: &[i32; 3]) -> i32 {
    unsafe {
        let func: Symbol<unsafe fn(&i32, &i32, &i32, &u8, &f64) -> i32> = lib.get(b"FCreatePrim").expect("no create prim");
        let TPrim = 1u8;
        let VP = 1f64;
        func(&prim[0], &prim[1], &prim[2], &TPrim, &VP)
    }
}

#[allow(non_snake_case)]
pub fn f_create_region(lib: &Library, point: &Point) -> i32 {
    unsafe {
        let func: Symbol<unsafe fn(&f64, &f64, &f64, &bool, &u8,
                                   &String, &f64, &f64, &f64, &f64, &f64, &f64,
                                   &bool) -> i32> = lib.get(b"FCreateRegion").expect("No create region");
        let xp= point.x as f64;
        let yp = point.y as f64;
        let Rp = 20f64;
        let pTriW = true;
        let pPrMag = 3u8;
        let pMatCharName = String::from("2013");
        let pMu = 1f64;
        let pPx = 1f64;
        let pPy = 1f64;
        let pW = 1f64;
        let pIp = 1f64;
        let pPlot = 1.1f64;
        let pJbyIW = true;
        func(&xp, &yp, &Rp, &pTriW, &pPrMag, &pMatCharName, &pMu, &pPx, &pPy, &pW, &pIp, &pPlot, &pJbyIW)
    }
}

pub fn f_build_fm(lib: &Library) -> bool {
    unsafe {
        let func: Symbol<unsafe fn() -> bool> = lib.get(b"FBuildFM").expect("No build fm");
        func()
    }
}

/// Получить магнитную индукцию в треугольном элементе
fn f_get_bx_by_bm(lib: &Library, i: i32, bx: &mut f64, by: &mut f64, bm: &mut f64) {
    unsafe {
        let func: Symbol<fn (i32, &mut f64, &mut f64, &mut f64)> = lib.get(b"FGetBxByBm").expect("No bm");
        func(i, bx, by, bm)
    }
}

pub fn get_bm_only(lib: &Library, i: i32) -> f32 {
    let mut bx = 0.;
    let mut by = 0.;
    let mut bm = 0.;
    f_get_bx_by_bm(lib, i, &mut bx, &mut by, &mut bm);
    bm as f32
}

pub fn get_points_ref(lib: &Library) -> (*mut *mut TBPoint, *mut i32) {
    unsafe {
        let func1: Symbol<fn () -> *mut *mut TBPoint> = lib.get(b"FGetRef").expect("No get ref");
        let func2: Symbol<fn () -> *mut i32> = lib.get(b"FGetLen").expect("No get len");
        (func1(), func2())
    }
}

pub fn f_open_dat(lib: &Library, path: &String) -> bool {
    let mut path_vec: Vec<&str> = path.split('/').collect();
    if path_vec.len() <= 1 {
        path_vec  = path.split('\\').collect();
    }
    if path_vec.len() <= 1 {
        return false
    }
    let file_name = path_vec.pop().unwrap().to_string();
    let file_dir: String = path_vec.into_iter().map(|a| {
        a.to_owned() + "/"
    }).collect();
    
    let mut arr_name: [u8; 256] = [0; 256];
    for (i, byte) in file_name.bytes().enumerate() {
        arr_name[i] = byte;
    }
    arr_name[file_name.len()] = b'\0';

    let mut arr_dir: [u8; 256] = [0; 256];
    for (i, byte) in file_dir.bytes().enumerate() {
        arr_dir[i] = byte;
    }
    arr_dir[file_dir.len()] = b'\0';
    unsafe {
        let func: Symbol<unsafe fn(&[u8; 256], &[u8; 256]) -> bool> = lib.get(b"FOpenDat").expect("No FOpenDat");
        func(&arr_dir, &arr_name)
    }
}

pub fn f_save_dat(lib: &Library, path: String) -> bool {
    let mut path_vec: Vec<&str> = path.split('/').collect();
    if path_vec.len() <= 1 {
        path_vec  = path.split('\\').collect();
    }
    if path_vec.len() <= 1 {
        return false
    }
    let file_name = path_vec.pop().unwrap().to_string();
    let file_dir: String = path_vec.into_iter().map(|a| {
        a.to_owned() + "/"
    }).collect();

    let mut arr_name: [u8; 256] = [0; 256];
    for (i, byte) in file_name.bytes().enumerate() {
        arr_name[i] = byte;
    }
    arr_name[file_name.len()] = b'\0';

    let mut arr_dir: [u8; 256] = [0; 256];
    for (i, byte) in file_dir.bytes().enumerate() {
        arr_dir[i] = byte;
    }
    arr_dir[file_dir.len()] = b'\0';
    
    unsafe {
        let func: Symbol<unsafe fn(&[u8; 256], &[u8; 256]) -> bool> = lib.get(b"FSaveDat").expect("No FSaveDat");
        func(&arr_dir, &arr_name)
    }
}
