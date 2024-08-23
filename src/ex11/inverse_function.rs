pub fn reverse_map(n: f64) -> (u16, u16) {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut x_pos_index = 1;
    let mut y_pos_index = 0;
    let ret: u32 = ((u32::MAX as f64) * n) as u32;

    for i in 0..16 {
        set_bit(&mut x, i, ((ret >> x_pos_index) & 1) == 1);
        set_bit(&mut y, i, ((ret >> y_pos_index) & 1) == 1);
        x_pos_index += 2;
        y_pos_index += 2;
    }

    (x, y)
}

fn set_bit(n: &mut u16, pos: u32, val: bool) {
    if val {
        *n |= 1 << pos; // met le bit à 1 à la position pos
    } else {
        *n &= !(1 << pos); // met le bit à 0 à la position pos
    }
}
