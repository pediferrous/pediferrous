//! Default user space unit primitive used for positioning and sizing elements on the page.

use std::fmt::Display;

/// Internal representation options for the [`Unit`] type. By default, the default user space unit
/// is 1/72th of an inch. `Inner` allows us to use other measurement units for the value and to
/// convert between them.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Inner {
    /// Size that is equivalent to default user space unit converted into milimeters.
    Mm(f32),

    /// Size that is equivalent to default user space unit converted into centimeters.
    Cm(f32),

    /// Size that is equivalent to default user space unit converted into inches.
    In(f32),
}

impl Inner {
    /// Converts the `Unit` into default user space unit to be specified in a PDF document,
    /// regardless of how this `Unit` is currently internally represented.
    ///
    /// # Example
    ///
    /// ```
    /// # use pdfgen::types::hierarchy::primitives::unit::Unit;
    /// let unit = Unit::from_inch(1.0);
    /// assert_eq!(unit.into_user_unit(), 72.0);
    /// ```
    pub const fn into_user_unit(self) -> f32 {
        match self {
            Inner::Mm(_) => self.into_inch().into_user_unit(),
            Inner::Cm(_) => self.into_inch().into_user_unit(),
            // by default 1 user space unit is 1/72th of an inch
            Inner::In(inch) => inch * 72.0,
        }
    }

    /// Converts the internal representation of the `Unit` into inch representation. This is
    /// useful, becuase the default user space unit is 1/72th of an inch, and we intend to keep the
    /// default representation.
    const fn into_inch(self) -> Self {
        match self {
            Inner::Mm(mm) => Self::In(mm / 25.4),
            Inner::Cm(cm) => Self::Mm(cm * 10_f32).into_inch(),
            Inner::In(_) => self,
        }
    }
}

impl Display for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let unit = self.into_user_unit();
        f.write_fmt(format_args!("{unit}"))
    }
}

/// `Unit` represents a value that is used for various options in PDF where the default user space
/// unit is required.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Unit {
    inner: Inner,
}

impl Unit {
    /// Creates a new `Unit` from the specified number of milimeters.
    ///
    /// # Example
    ///
    /// ```
    /// # use pdfgen::types::hierarchy::primitives::unit::Unit;
    /// let unit = Unit::from_mm(1.0);
    /// assert_eq!(unit.into_user_unit().floor(), 2.0);
    /// ```
    pub const fn from_mm(mm: f32) -> Self {
        Self {
            inner: Inner::Mm(mm),
        }
    }

    /// Creates a new `Unit` from the specified number of centimeters.
    ///
    /// # Example
    ///
    /// ```
    /// # use pdfgen::types::hierarchy::primitives::unit::Unit;
    /// let unit = Unit::from_cm(1.0);
    /// assert_eq!(unit.into_user_unit().floor(), 28.0);
    /// ```
    pub const fn from_cm(cm: f32) -> Self {
        Self {
            inner: Inner::Cm(cm),
        }
    }

    /// Creates a new `Unit` from the specified number of inches.
    ///
    /// # Example
    ///
    /// ```
    /// # use pdfgen::types::hierarchy::primitives::unit::Unit;
    /// let unit = Unit::from_inch(1.0);
    /// assert_eq!(unit.into_user_unit(), 72.0);
    /// ```
    pub const fn from_inch(inch: f32) -> Self {
        Self {
            inner: Inner::In(inch),
        }
    }

    /// Creates a new `Unit` from the specified number of default user space units.
    pub const fn from_unit(unit: f32) -> Unit {
        Self {
            inner: Inner::In(unit / 72.0),
        }
    }

    /// Converts the `Unit` into default user space unit to be specified in a PDF document,
    /// regardless of how this `Unit` is currently internally represented.
    ///
    /// # Example
    ///
    /// ```
    /// # use pdfgen::types::hierarchy::primitives::unit::Unit;
    /// let unit = Unit::from_inch(1.0);
    /// assert_eq!(unit.into_user_unit(), 72.0);
    /// ```
    pub const fn into_user_unit(self) -> f32 {
        match self.inner {
            Inner::Mm(_) => self.into_inch().into_user_unit(),
            Inner::Cm(_) => self.into_inch().into_user_unit(),
            // by default 1 user space unit is 1/72th of an inch
            Inner::In(inch) => inch * 72.0,
        }
    }

    /// Converts the internal representation of the `Unit` into inch representation. This is
    /// useful, becuase the default user space unit is 1/72th of an inch, and we intend to keep the
    /// default representation.
    const fn into_inch(mut self) -> Self {
        self.inner = match self.inner {
            Inner::Mm(mm) => Inner::In(mm / 25.4),
            Inner::Cm(cm) => Inner::Mm(cm * 10_f32).into_inch(),
            Inner::In(_) => self.inner,
        };

        self
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
