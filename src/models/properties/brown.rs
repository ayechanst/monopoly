pub enum BrownProperty {
    MediterraneanAve { houses: HouseCount },
    BalticAve { houses: HouseCount },
}

impl BrownProperty {
    pub fn rent_price(&self) -> i32 {
        match self {
            MediterraneanAve { houses } => {
                let rent = 2;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => rent * 15,
                    Three => rent * 45,
                    Four => rent * 80,
                    Hotel => rent * 125,
                }
            }
            BalticAve { houses } => {
                let rent = 4;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => rent * 15,
                    Three => rent * 45,
                    Four => rent * 80,
                    Hotel => 450,
                }
            }
        }
    }
    pub fn as_space(self) -> Space {
        Space::Property(ColoredProperty::Brown(self))
    }
}
