use crate::model::model_main::Model;
use iced::Point;
use csv::{WriterBuilder, ReaderBuilder};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Csv {
    p1: f32,
    p2: f32,
    p3: f32,
    l1: Option<i32>,
    l2: Option<i32>,
    l3: Option<i32>,
    n_d1: f32,
    n_d2: f32,
    n_l1: Option<i32>,
    n_l2: Option<i32>
}

impl Csv {
    fn new(dots: [f32; 3], lines: Option<[i32; 3]>, node_dots: [f32; 2], node_lines: Option<[i32; 2]>) -> Self {
        let real_lines = match lines {
            None => [None, None, None],
            _ => {
                let l = lines.as_ref().unwrap();
                [Some(l[0]), Some(l[1]), Some(l[2])]
            }
        };
        
        let real_node_lines = match node_lines {
            None => [None, None],
            _ => {
                let l = node_lines.as_ref().unwrap();
                [Some(l[0]), Some(l[1])]
            }
        };
        
        Self {
            p1: dots[0],
            p2: dots[1],
            p3: dots[2],
            l1: real_lines[0],
            l2: real_lines[1],
            l3: real_lines[2],
            n_d1: node_dots[0],
            n_d2: node_dots[1],
            n_l1: real_node_lines[0],
            n_l2: real_node_lines[1]
        }
    }
}

pub fn import_model(path: String, model: &mut Model) -> bool {
    if path.len() >= 4 {
        match path.trim().get((path.len() - 4)..=(path.len() - 2)) {
            Some("csv") => import_csv_model(path, model),
            _ => false
        }
    }
    else {
        false
    }
}

fn import_csv_model (path: String, model: &mut Model) -> bool {
    if let Ok(mut reader) = ReaderBuilder::new().delimiter(b'\t').from_path(path.trim()) {
        let records = reader.deserialize::<Csv>();
        let mut dots: Vec<(Point, f32)> = Vec::new();
        let mut lines: Vec<(i32, i32, i32)> = Vec::new();
        let mut node_dots: Vec<Point> = Vec::new();
        let mut node_lines: Vec<(i32, i32)> = Vec::new();

        for i in records {
            if let Ok(rec) = i.as_ref() {
                let p = [&rec.p1, &rec.p2, &rec.p3];
                if p.iter().all(|a| !a.is_nan()) {
                    dots.push((Point::new(*p[0], *p[1]), *p[2]))
                }

                let l = [&rec.l1, &rec.l2, &rec.l3];
                if l.iter().all(|a| a.is_some()) {
                    lines.push((l[0].unwrap(), l[1].unwrap(), l[2].unwrap()))
                }

                let np = [&rec.n_d1, &rec.n_d2];
                if np.iter().all(|a| !a.is_nan()) {
                    node_dots.push(Point::new(*np[0], *np[1]))
                }

                let nl = [&rec.n_l1, &rec.n_l2];
                if nl.iter().all(|a| a.is_some()) {
                    node_lines.push((l[0].unwrap(), l[1].unwrap()))
                }
                
            }

        }


        model.dots = dots;
        model.lines = lines;
        model.node_dots = node_dots;
        model.node_lines = node_lines;
        return true
    }

    false
}

pub fn export_model(path: String, model: &Model) -> bool {
    if path.len() >= 4 {
        match path.get((path.len() - 4)..=(path.len() - 2)) {
            Some("csv") => export_csv_model(path, model),
            _ => false
        }
    }
    else {
        false
    }
}

fn export_csv_model(path: String, model: &Model) -> bool {
    if let Ok(mut writer) = WriterBuilder::new().delimiter(b'\t').from_path(path.trim()) {
        let records = model.export();
        let max_len = usize::max(records.0.len(), records.1.len());

        for i in 0..max_len {
            let rec = Csv::new(
                records.0.get(i).copied().unwrap_or([f32::NAN; 3]),
                records.1.get(i).copied(),
                records.2.get(i).copied().unwrap_or([f32::NAN; 2]),
                records.3.get(i).copied()
            );

            writer.serialize(rec).expect("No write");
        }
        writer.flush().expect("No flush");
        return true
    }
    false
}