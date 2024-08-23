pub fn map(x: u16, y: u16) -> f64 {
    let mut result: u32 = 0;
    set_bit(&mut result, 0, true);
    let mut x_index = 0;
    let mut y_index = 0;
    for i in 0..32 {
        if i % 2 == 0 {
            set_bit(&mut result, i, ((y >> y_index) & 1) == 1);
            y_index += 1;
        } else {
            set_bit(&mut result, i, ((x >> x_index) & 1) == 1);
            x_index += 1;
        }
    }
    f64::from(result) / f64::from(u32::MAX)
}

fn set_bit(n: &mut u32, pos: u32, val: bool) {
    if val {
        *n |= 1 << pos; // met le bit à 1 à la position pos
    } else {
        *n &= !(1 << pos); // met le bit à 0 à la position pos
    }
}
