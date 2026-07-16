use iced::widget::{row, Row, text, text_input, Column};
use crate::app::core::VecRed;
use crate::Message;

/// Creates [TextInput] for entering values from keyboard.
#[inline(always)]
fn create_text_input(value: String, what: &'static str, num: usize, order: usize, descr: &'static str) -> Row<'static, Message> {
    let mut result = row![];
    result = result.push(text(descr)); 
    result = result.push(text_input(&value, &value).on_input(move |text| Message::ChangeParams(what, num, text, order)));
    result
}

impl VecRed {
    /// Borrowed point's fields.
    pub(super) fn full_point(&self, num: usize) -> Column<'_, Message> {
        let mut column = iced::widget::column![];
        if let Some(point) = self.model.tb_point_ref(num) {
            let typ_point = point.TypPoint.to_string();
            let typ_point = create_text_input(typ_point, "point", num, 3, "TypPoint");

            let vp = point.Vp.to_string();
            let vp = create_text_input(vp, "point", num, 4, "Vp");

            let ip = point.Ip.to_string();
            let ip = create_text_input(ip, "point", num, 5, "Ip");

            let n_node = point.NNode.to_string();
            let n_node = text(format!("NNode {}", n_node));

            column = iced::widget::column![typ_point, vp, ip, n_node];
        }

        column
    }

    pub(super) fn full_prim(&self, index: i32) -> Column<'_, Message>{
        let mut column = iced::widget::column![];
        if index == -1 {
            return column
        }
        let index = index as usize;
        if let Some(prim) = self.model.t_primitive_ref(index) {
            let intro = text(format!("Prim number: {}", index));

            let p1 = prim.p[0].to_string();
            let p1 = create_text_input(p1, "prim", index, 0, "p1");
            
            let p2 = prim.p[1].to_string();
            let p2 = create_text_input(p2, "prim", index, 1, "p2");
            
            let p3 = prim.p[2].to_string();
            let p3 = create_text_input(p3, "prim", index, 2, "p3");
            
            let typ_prim = prim.TypPrim.to_string();
            let typ_prim = create_text_input(typ_prim, "prim", index, 3, "TypPrim");
            
            let is_front = prim.IsFront.to_string();
            let is_front = create_text_input(is_front, "prim", index, 4, "IsFront");
            
            let vp = prim.Vp.to_string();
            let vp = create_text_input(vp, "prim", index, 5, "Vp");
            
            let ip = prim.Ip.to_string();
            let ip = create_text_input(ip, "prim", index, 6, "Ip");
            
            column = iced::widget::column![intro, p1, p2, p3, typ_prim, is_front, vp, ip];
        }
        column
    }
    
    pub(super) fn full_node(&self, index: i32) -> Column<'_, Message> {
        let mut column = iced::widget::column![];
        if index == -1 {
            return column
        }
        let index = index as usize;
        if let Some(node) = self.model.t_node_ref(index) {
            let intro = text(format!("Node number: {}", index));
            
            let x = text(format!("x: {}", node.x));
            let y = text(format!("y: {}", node.y));
            
            column = iced::widget::column![intro, x, y];
        }
        column
    }
    
    pub(super) fn full_region(&self, index: i32) -> Column<'_, Message> {
        let mut column = iced::widget::column![];
        if index == -1 {
            return column
        }
        let index = index as usize;
        if let Some(region) = self.model.t_region_ref(index) {
            let intro = text(format!("Region number: {}", index));
            
            let triw = region.TriW.to_string();
            let triw = create_text_input(triw, "region", index, 0, "TriW");
            
            let cnu = region.CNu.to_string();
            let cnu = create_text_input(cnu, "region", index,  1, "CNu");
            
            column = iced::widget::column![intro, triw, cnu];
        }
        column
    }
}

/*
    pub p: [i32; 3],
    pub TypPrim: u8,
    pub IsFront: bool,
    pub Vp: f64,
    pub Ip: f64
 */