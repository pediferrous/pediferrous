#[derive(Default, Debug, Clone)]
pub struct PdfGen {
    obj_ids: Vec<usize>,
}

impl PdfGen {
    pub fn insert_obj(&mut self, obj_id: usize) {
        self.obj_ids.push(obj_id);
    }
}

pub fn add(a: usize, b: usize) -> usize {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 3);
        assert_eq!(result, 5);
    }
}
