use iced::Point;
use libloading::{Library, Symbol};
use crate::model::model_main::Model;

pub fn f_init_model(lib: &Library) {
    unsafe {
        let func: Symbol<unsafe fn()> = lib.get(b"FInitModel").unwrap();
        func()
    }
}

pub fn f_create_point(lib: &Library, point: &(Point, f32)) -> i32 {
    unsafe {
        let func: Symbol<unsafe fn(&f64, &f64, &f64, &u8) -> i32> = lib.get(b"FCreatePoint").expect("No create point");
        let byte = 0u8;
        let res = func(&(point.0.x as f64), &(point.0.y as f64), &(point.1 as f64), &byte);
        res
    }
}

#[allow(non_snake_case)]
pub fn f_create_prim(lib: &Library, prim: &(i32, i32, i32)) -> i32 {
    unsafe {
        let func: Symbol<unsafe fn(&i32, &i32, &i32, &u8, &f64) -> i32> = lib.get(b"FCreatePrim").expect("no create prim");
        let TPrim = 1u8;
        let VP = 1f64;
        func(&prim.0, &prim.1, &prim.2, &TPrim, &VP)
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

/// взять количество узлов в модели
fn f_get_nnode(lib: &Library) -> i32 {
    unsafe {
        let func: Symbol<unsafe fn() -> i32> = lib.get(b"FGetNNode").expect("No get n node");
        func()
    }
}
/// взять координату X узла N
fn f_get_xnode(lib: &Library, num: i32) -> f32 {
    unsafe {
        let func: Symbol<unsafe fn(i32) -> f64> = lib.get(b"FGetXNode").expect("No get x node");
        func(num) as f32
    }
}
/// взять координату Y узла N
fn f_get_ynode(lib: &Library, num: i32) -> f32 {
    unsafe {
        let func: Symbol<unsafe fn(i32) -> f64> = lib.get(b"FGetYNode").expect("No get y node");
        func(num) as f32
    }
}
/// взять количество элементов в модели
fn f_get_nelem(lib: &Library) -> i32 {
    unsafe {
        let func: Symbol<unsafe fn() -> i32> = lib.get(b"FGetNElem").expect("No get n elem");
        func()
    }
}
/// взять глобальный номер узла с локальным номером num в элементе с номером i
fn f_get_nnode_in_elem(lib: &Library, i: i32, num: i32) -> i32 {
    unsafe {
        let func: Symbol<unsafe fn(i32, i32) -> i32> = lib.get(b"FGetNNodeInElem").expect("No get n node in elem");
        func(i, num)
    }
}

fn f_get_x_point(lib: &Library, i: i32) -> f64 {
    unsafe {
        let func: Symbol<unsafe fn (i32) -> f64> = lib.get(b"FGetXPoint").expect("No get x point");
        func(i)
    }
}

fn f_get_y_point(lib: &Library, i: i32) -> f64 {
    unsafe {
        let func: Symbol<unsafe fn (i32) -> f64> = lib.get(b"FGetYPoint").expect("No get y point");
        func(i)
    }
}

fn f_get_r_point(lib: &Library, i: i32) -> f64 {
    unsafe {
        let func: Symbol<unsafe fn (i32) -> f64> = lib.get(b"FGetRPoint").expect("No get r point");
        func(i)
    }
}

fn f_get_n_point(lib: &Library) -> i32 {
    unsafe {
        let func: Symbol<unsafe fn() -> i32> = lib.get(b"FGetNPoint").expect("No num of points");
        func()
    }
}

fn f_get_n_prim(lib: &Library) -> i32 {
    unsafe {
        let func: Symbol<unsafe fn() -> i32> = lib.get(b"FGetNPrim").expect("No num of prims");
        func()
    }
}
fn get_points_prims(lib: &Library) -> (Vec<(Point, f32)>, Vec<(i32, i32, i32)>) {
    let points_num = f_get_n_point(lib);
    let prims_num = f_get_n_prim(lib);

    let points = Vec::with_capacity(points_num as usize);
    let prims = Vec::with_capacity(prims_num as usize);

    (points, prims)
}

pub fn get_nodes_full(lib: &Library) -> (Vec<Point>, Vec<(i32, i32, i32)>) {
    let node_num = f_get_nnode(lib);
    let triangle_num = f_get_nelem(lib);
    let mut node_dots = Vec::with_capacity(node_num as usize);
    let mut node_lines = Vec::with_capacity(triangle_num as usize);

    for i in 0..node_num {
        let mut p = Point::default();
        p.x = f_get_xnode(lib, i);
        p.y = f_get_ynode(lib, i);
        node_dots.push(p);
    }

    for i in 0..triangle_num {
        let mut line = (0, 0, 0);
        line.0 = f_get_nnode_in_elem(lib, i, 1);
        line.1 = f_get_nnode_in_elem(lib, i, 2);
        line.2 = f_get_nnode_in_elem(lib, i, 3);
        node_lines.push(line)
    }

    (node_dots, node_lines)
}

pub fn get_full_model(lib: &Library, model: &mut Model) {
    let (node_points, node_lines) = get_nodes_full(lib);
    get_points_prims(lib);
    
    model.node_points = node_points;
    model.node_lines = node_lines;
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
