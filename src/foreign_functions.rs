use iced::Point;
use libloading::{Library, Symbol};

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
fn fget_nnode(lib: &Library) -> i32 {
    unsafe {
        let func: Symbol<unsafe fn() -> i32> = lib.get(b"FGetNNode").expect("No get n node");
        func()
    }
}
/// взять координату X узла N
fn fget_xnode(lib: &Library, num: i32) -> f32 {
    unsafe {
        let func: Symbol<unsafe fn(i32) -> f64> = lib.get(b"FGetXNode").expect("No get x node");
        func(num) as f32
    }
}
/// взять координату Y узла N
fn fget_ynode(lib: &Library, num: i32) -> f32 {
    unsafe {
        let func: Symbol<unsafe fn(i32) -> f64> = lib.get(b"FGetYNode").expect("No get y node");
        func(num) as f32
    }
}
/// взять количество элементов в модели
fn fget_nelem(lib: &Library) -> i32 {
    unsafe {
        let func: Symbol<unsafe fn() -> i32> = lib.get(b"FGetNElem").expect("No get n elem");
        func()
    }
}
/// взять глобальный номер узла с локальным номером num в элементе с номером i
fn fget_nnode_in_elem(lib: &Library, i: i32, num: i32) -> i32 {
    unsafe {
        let func: Symbol<unsafe fn(i32, i32) -> i32> = lib.get(b"FGetNNodeInElem").expect("No get n node in elem");
        func(i, num)
    }
}

pub fn get_nodes_full(lib: &Library) -> (Vec<Point>, Vec<(i32, i32)>) {
    let node_num = fget_nnode(lib);
    let triangle_num = fget_nelem(lib);
    let mut node_dots = Vec::with_capacity(node_num as usize);
    let mut node_lines = Vec::with_capacity((triangle_num * 3) as usize);
    
    for i in 0..node_num {
        let mut p = Point::default();
        p.x = fget_xnode(lib, i);
        p.y = fget_ynode(lib, i);
        node_dots.push(p);
    }
    
    for i in 0..triangle_num {
        let mut line = (0, 0, 0);
        line.0 = fget_nnode_in_elem(lib, i, 1);
        line.1 = fget_nnode_in_elem(lib, i, 2);
        line.2 = fget_nnode_in_elem(lib, i, 3);

        node_lines.push((line.0, line.1));
        node_lines.push((line.0, line.2));
        node_lines.push((line.1, line.2));
    }

    (node_dots, node_lines)
}

pub fn fopen_dat() {
    todo!()
}
