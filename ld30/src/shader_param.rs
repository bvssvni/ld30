
#[shader_param(Program)]
pub struct ShaderParam {
    pub model_view_projection: [[f32, ..4], ..4],
    pub color: [f32, ..4],
}

