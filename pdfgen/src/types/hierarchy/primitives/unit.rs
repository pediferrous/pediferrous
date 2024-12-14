use std::fmt::Display;

/// Unit used for various options in PDF where the default user space unit is required.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Unit {
    /// Size that is equivalent to default user space unit converted into milimeters.
    Mm(f32),

    /// Size that is equivalent to default user space unit converted into centimeters.
    Cm(f32),

    /// Size that is equivalent to default user space unit converted into inches.
    In(f32),
}

impl Unit {
    pub const fn from_mm(mm: f32) -> Self {
        Self::Mm(mm)
    }

    pub const fn from_cm(cm: f32) -> Self {
        Self::Cm(cm)
    }

    pub const fn from_inch(inch: f32) -> Self {
        Self::In(inch)
    }

    pub const fn into_user_unit(self) -> f32 {
        match self {
            Unit::Mm(_) => self.into_inch().into_user_unit(),
            Unit::Cm(_) => self.into_inch().into_user_unit(),
            // by default 1 user space unit is 1/72th of an inch
            Unit::In(inch) => inch * 72.0,
        }
    }

    const fn into_inch(self) -> Self {
        match self {
            Unit::Mm(mm) => Self::In(mm / 25.4),
            Unit::Cm(cm) => Self::Mm(cm * 10_f32).into_inch(),
            Unit::In(_) => self,
        }
    }

    pub const fn from_unit(unit: f32) -> Unit {
        Self::In(unit / 72.0)
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let unit = self.into_user_unit();
        f.write_fmt(format_args!("{unit}"))
    }
}
