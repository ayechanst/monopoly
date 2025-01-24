pub struct BoardMsg {
    pub msg: String,
    pub player_position: (f32, f32),
    // pub pay_rent: PayRent,
}

pub struct PayRent {
    pub rent: u32,
}
