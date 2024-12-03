pub enum YellowProperty {
    Atlantic { houses: HouseCount },
    Marvin { houses: HouseCount },
    Ventnor { houses: HouseCount },
}

impl YellowProperty {
    pub fn rent_price(&self) -> i32 {
        match self {
            AtlanticAve { houses } | VentnorAve => {
                let rent = 22;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => 330,
                    Three => 800,
                    Four => 975,
                    Hotel => 1150,
                }
            }
            MarvinGardens => {
                let rent = 24;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => 360,
                    Three => 850,
                    Four => 1025,
                    Hotel => 1200,
                }
            }
        }
    }

    pub fn as_space(self) -> Space {
        // instance methods
        Space::Property(ColoredProperty::Yellow(self))
    }

    pub fn atlantic() -> Self {
        // class method
        YellowProperty::Atlantic { houses: 0 }
    }
    // YellowPropery.atlantic().as_space()
}
