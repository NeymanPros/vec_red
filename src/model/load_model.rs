use crate::model::model_main::Model;
use iced::Point;
use csv::{WriterBuilder, ReaderBuilder};
use libloading::Library;
use crate::foreign_functions::fopen_dat;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Csv {
    p1: Option<f32>,
    p2: Option<f32>,
    p3: Option<f32>,
    l1: Option<i32>,
    l2: Option<i32>,
    l3: Option<i32>,
    n_p1: Option<f32>,
    n_p2: Option<f32>,
    n_l1: Option<i32>,
    n_l2: Option<i32>
}

impl Csv {
    fn new(dots: &Option<&(Point, f32)>, lines: &Option<&(i32, i32, i32)>, node_dots: &Option<&Point>, node_lines: &Option<&(i32, i32)>) -> Self {
        let (p1, p2, p3) = match dots {
            None => (None, None, None),
            _ => {
                let p = dots.as_ref().unwrap();
                (Some(p.0.x), Some(p.0.y), Some(p.1))
            }
        };
        let (l1, l2, l3) = match lines {
            None => (None, None, None),
            _ => {
                let l = lines.as_ref().unwrap();
                (Some(l.0), Some(l.1), Some(l.2))
            }
        };
        let (n_p1, n_p2) = match node_dots {
            None => (None, None),
            _ => {
                let p = dots.as_ref().unwrap();
                (Some(p.0.x), Some(p.0.y))
            }
        };
        let (n_l1, n_l2) = match node_lines {
            None => (None, None),
            _ => {
                let l = node_lines.as_ref().unwrap();
                (Some(l.0), Some(l.1))
            }
        };
        
        Self {
            p1, p2, p3,
            l1, l2, l3,
            n_p1, n_p2,
            n_l1, n_l2
        }
    }
}

pub fn import_model(lib: &Library, path: String, model: &mut Model) -> bool {
    let path = path.trim().to_string();
    if path.len() >= 3 {
        match path.get((path.len() - 3)..=(path.len() - 1)) {
            Some("csv") => import_csv_model(path, model),
            Some("mke") => import_mke_model(lib, path, model),
            _ => false
        }
    }
    else {
        false
    }
}

fn import_csv_model (path: String, model: &mut Model) -> bool {
    if let Ok(mut reader) = ReaderBuilder::new().delimiter(b'\t').from_path(path) {
        let records = reader.deserialize::<Csv>();
        let mut dots: Vec<(Point, f32)> = Vec::new();
        let mut lines: Vec<(i32, i32, i32)> = Vec::new();
        let mut node_dots: Vec<Point> = Vec::new();
        let mut node_lines: Vec<(i32, i32)> = Vec::new();

        for i in records {
            if let Ok(rec) = i.as_ref() {
                 if let (Some(p1), Some(p2), Some(p3)) = (rec.p1, rec.p2, rec.p3) {
                    dots.push((Point::new(p1, p2), p3))
                }

                if let (Some(l1), Some(l2), Some(l3)) = (rec.l1, rec.l2, rec.l3) {
                    lines.push((l1, l2, l3))
                }

                if let (Some(np1), Some(np2)) = (rec.n_p1, rec.n_p2) {
                    node_dots.push(Point::new(np1, np2));
                }

                if let(Some(n_l1), Some(n_l2)) = (rec.n_l1, rec.n_l2) {
                    node_lines.push((n_l1, n_l2))
                }
                
            }

        }
        
        model.points = dots;
        model.prims = lines;
        model.node_points = node_dots;
        model.node_lines = node_lines;
        return true
    }

    false
}

fn import_mke_model(lib: &Library, path: String, model: &mut Model) -> bool {
    if !fopen_dat(lib, &path) {
        return false
    }
    
    
    true
}

pub fn export_model(path: String, model: &Model) -> bool {
    let path = path.trim().to_string();
    if path.len() >= 4 {
        match path.get((path.len() - 3)..=(path.len() - 1)) {
            Some("csv") => export_csv_model(path, model),
            _ => false
        }
    }
    else {
        false
    }
}

fn export_csv_model(path: String, model: &Model) -> bool {
    if let Ok(mut writer) = WriterBuilder::new().delimiter(b'\t').from_path(path) {
        let max_len = usize::max(model.node_points.len(), model.points.len());

        for i in 0..max_len {
            let rec = Csv::new(
                &model.points.get(i),
                &model.prims.get(i),
                &model.node_points.get(i),
                &model.node_lines.get(i)
            );

            writer.serialize(rec).expect("No write");
        }
        writer.flush().expect("No flush");
        return true
    }
    false
}
