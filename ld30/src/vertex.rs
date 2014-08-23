
use internal::{Position, UV};

#[vertex_format]
pub struct Vertex {
    pos: [f32, ..3],
}

impl Vertex {
    pub fn new(pos: Position) -> Vertex {
        Vertex {
            pos: pos,
        }
    }
}
