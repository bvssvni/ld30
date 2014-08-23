
use internal::{Position, UV};

#[vertex_format]
pub struct Vertex {
    pos: [f32, ..3],
    uv: [f32, ..2]
}

impl Vertex {
    pub fn new(pos: Position, uv: UV) -> Vertex {
        Vertex {
            pos: pos,
            uv: uv
        }
    }
}
