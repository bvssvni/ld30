
use internal::{Position, UV};

#[vertex_format]
pub struct Vertex {
    pos: [f32, ..3],
}

impl Clone for Vertex {
    fn clone(&self) -> Vertex {
        *self
    }
}

impl Vertex {
    pub fn new(pos: Position) -> Vertex {
        Vertex {
            pos: pos,
        }
    }
}
