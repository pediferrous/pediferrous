use crate::types::{
    constants,
    hierarchy::primitives::{name::OwnedName, obj_id::ObjId, object::Object, rectangle::Position},
};

use super::{image::ImageTransform, stream::Stream};

pub(crate) enum Operation<'a> {
    DrawImage {
        name: &'a OwnedName,
        transform: ImageTransform,
    },
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ContentStream {
    stream: Stream,
}

impl ContentStream {
    pub fn new(id: ObjId) -> Self {
        Self {
            stream: Stream::new(id),
        }
    }

    pub(crate) fn add_content(&mut self, operation: Operation) {
        match operation {
            Operation::DrawImage { name, transform } => self.draw_image(name, transform),
        }
    }

    fn draw_image(&mut self, name: &OwnedName, transform: ImageTransform) {
        let Position {
            x: width,
            y: height,
        } = transform.scale;

        let Position { x, y } = transform.position;

        // Save graphics state
        self.stream.push_bytes(b"q");
        self.stream.push_bytes(constants::NL_MARKER);

        // apply transform 🤯
        // width 0 0 height x y cm - Translate to (x, y) and scale to width x height
        self.stream
            .push_bytes(format!("{width} 0 0 {height} {x} {y} cm").as_bytes());
        self.stream.push_bytes(constants::NL_MARKER);

        // /ImgName Do - Paint image
        self.stream.push_bytes(name.as_bytes());
        self.stream.push_bytes(b"Do");
        self.stream.push_bytes(constants::NL_MARKER);

        // Restore graphics state
        self.stream.push_bytes(b"Q");
    }

    pub fn is_empty(&self) -> bool {
        self.stream.is_empty()
    }
}

impl Object for ContentStream {
    fn write(&self, writer: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        self.stream.write(writer)
    }

    fn obj_ref(&self) -> &ObjId {
        &self.stream.id
    }
}
