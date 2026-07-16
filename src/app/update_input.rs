use crate::VecRed;

impl VecRed {
    #[inline(always)]
    pub(super) fn move_point_apply(&mut self) {
        if let Some((chosen_p, chosen_r, chosen_num)) = self.chosen_point.as_mut() {
            chosen_p.x = self.point_string[0].parse::<f32>().unwrap();
            chosen_p.y = self.point_string[1].parse::<f32>().unwrap();
            *chosen_r = self.point_string[2].parse::<f32>().unwrap();

            if self.model.points(*chosen_num) != *chosen_p || self.model.points_r(*chosen_num) != *chosen_r {
                self.journal.changed_point((self.model.points(*chosen_num), self.model.points_r(*chosen_num)), *chosen_num);
                self.model.point_set(*chosen_num, *chosen_p, *chosen_r);
                self.state.redraw();
            }
        }
    }

    #[inline(always)]
    fn change_point(&mut self, order: usize) {
        if let Ok(new_value) = self.point_string[order].trim().parse::<f32>() {

            match order {
                0 => {
                    self.chosen_point.as_mut().unwrap().0.x = new_value;
                }
                1 => {
                    self.chosen_point.as_mut().unwrap().0.y = new_value;
                }
                2 => {
                    self.chosen_point.as_mut().unwrap().1 = new_value;
                }
                _ => {}
            }
        }
    }
    
    #[inline(always)]
    pub(super) fn change_params(&mut self, what: &'static str, index: usize, new_value: String, order: usize) {
        if what == "point" && order < 3 {
            self.point_string[order] = new_value;
            self.change_point(order)
        }
        else {
            match what {
                "point" => {
                    let point = self.model.tb_point_ref(index).unwrap();
                    let _ = point.set_field_by_index(order, &new_value);
                },
                "prim" => {
                    let prim = self.model.t_primitive_ref(index).unwrap();
                    let _ = prim.set_field_by_index(order, &new_value);
                },
                "region" => {
                    let region = self.model.t_region_ref(index).unwrap();
                    let _ = region.set_field_by_index(order, &new_value);
                }
                _ => panic!("No such thing to change!")
            }
        }
    }
}
