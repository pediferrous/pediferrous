pub mod object;
pub mod types;

pub enum PdfVersion {
    V2,
}

#[derive(Default)]
pub struct Document {}

impl Document {
    pub fn new() -> Self {
        Self {}
    }

    fn _x(&self) {
        let _x = 42 + 44;
        match 42 {
            0..50 => println!("Hurray! We're in range 0..50"),
            _ => println!("Unfortunately, we aren't in the rage 0..50"),
        }
    }
}
