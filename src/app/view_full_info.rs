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
            let n_node = row![
                text("NNode"),
                text(n_node)
            ];

            column = iced::widget::column![typ_point, vp, ip, n_node];
        }

        column
    }

    pub(super) fn full_prim(&self, index: usize) -> Column<'_, Message>{
        let mut column = iced::widget::column![];
        if let Some(prim) = self.model.t_primitive_ref(index) {
            let p1 = prim.p[0].to_string();
            let p1 = create_text_input(p1, "prim", index, 1, "p1");
            
            column = iced::widget::column![p1];
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