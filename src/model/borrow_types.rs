#[allow(non_snake_case, dead_code)]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub(crate) struct TBPoint {
    pub x: f64,
    pub y: f64,
    pub r: f64,
    pub TypPoint: u8,
    pub Vp: f64,
    pub Ip: f64,
    pub NNode: i32,
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Default)]
#[repr(C)]
pub(crate) struct TPrimitive {
    pub p: [i32; 3],
    pub TypPrim: u8,
    pub IsFront: bool,
    pub Vp: f64,
    pub Ip: f64
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug)]
#[repr(C)]
pub(crate) struct TNode {
    pub x: f64,
    pub y: f64,
    pub VP: f64,
    pub TypNode: u8,
    pub PNode: i32,
    pub KolSW: i32,
    pub NSW: *mut i32, //array of integer <- это
    pub KolSI: i32,
    pub NSI: *mut i32, //array of integer
    pub NSK: *mut i8, //array of shortint; 
    pub vKolSWMemo: i32,
    pub vKolSIMemo: i32,
    pub F: f64,
    pub Yp: f64
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug)]
#[repr(C)]
pub(crate) struct TElement {
    pub m: [i32; 3],
    pub IZP: i16,
    //================================
    pub Px: f64,
    pub Py: f64,
    pub XNJU: f64,
    pub A1: f64,
    pub Delta: f64,
    pub xs: f64,
    pub ys: f64,
    pub S: [f64; 6], //array[1..6] of double;
    pub A: [f64; 3],
    pub B: [f64; 3],
    pub C: [f64; 3]
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug)]
#[repr(C)]
pub(crate) struct TRegion {
    x: f64,
    y: f64,
    R: f64,
    TriW: bool,
    CNu: f64,
    PrMag: u8,
    Px: f64,
    Py: f64,
    W: f64,
    Ip: f64,
    ST: f64,
    PLOT: f64,
    JbyIW: bool,
    MatCharName: [u8; 40], //TMatNameString,
    //------------------------------
    KGran: i32,
    Gran: *mut TGran,
    vGranMemo: i32,
    NMatChar: i32,
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug)]
#[repr(C)]
pub(crate) struct TGran {
    Node: i32,
    UgUzl: f64
}
