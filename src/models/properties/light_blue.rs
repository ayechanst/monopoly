pub enum LightBlueProperty {
    OrientalAve { houses: HouseCount },
    VermontAve { houses: HouseCount },
    ConnecticutAve { houses: HouseCount },
}

impl LightBlueProperty {
    pub fn rent_price(&self) -> i32 {
        // TODO: if a person has all properties, rent is doubled
        match self {
            OrientalAve { houses } | VermontAve { houses } => {
                let rent = 6;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => rent * 15,
                    Three => rent * 45,
                    Four => 400,
                    Hotel => 550,
                }
            }
            ConnecticutAve { houses } => {
                let rent = 8;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => 100,
                    Three => 300,
                    Four => 450,
                    Hotel => 600,
                }
            }
        }
    }
}

// Pink

pub enum YellowProperty {
    Atlantic { houses: i32 },
    Marvin { houses: i32 },
    Ventnor { houses: i32 },
}

impl YellowProperty {
    pub fn rent_price(&self) -> i32 {
        match self {
            Atlantic { houses } => {
                let rent = 22;
                match houses {
                    0 => rent,
                    1 => rent * 5,
                    2 => todo!(),
                    3 => todo!(),
                    4 => todo!(),
                    5 => todo!(),
                }
            }
            Brown(prop) => prop.rent_price(),
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
