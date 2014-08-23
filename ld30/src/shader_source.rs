use gfx;

pub static VERTEX_SRC: gfx::ShaderSource = shaders! {
    GLSL_150: b"
#version 150 core
in vec3 pos;
out vec2 v_uv;
uniform mat4 model_view_projection;

void main() {
    gl_Position = model_view_projection * vec4(pos, 1.0);
}
"
};

pub static FRAGMENT_SRC: gfx::ShaderSource = shaders! {
    GLSL_150: b"
#version 150 core
out vec4 out_color;
uniform vec4 color;

void main() {
    out_color = color;
}
"
};

