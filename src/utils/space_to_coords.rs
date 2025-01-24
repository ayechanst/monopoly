pub fn space_to_coords(usize_space: usize) -> (f32, f32) {
    let go = (5.0, -5.0);
    let space = usize_space as f32;
    if space >= 0.0 && space <= 10.0 {
        (-(space - 5.0), -5.0)
    } else if space >= 11.0 && space <= 20.0 {
        (-5.0, (space - 5.0) - 10.0)
    } else if space >= 21.0 && space <= 30.0 {
        ((space - 5.0) - 20.0, 5.0)
    } else {
        (5.0, -((space - 5.0) - 30.0))
    }
}
