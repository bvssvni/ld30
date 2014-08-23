use gfx;

pub static VERTEX_SRC: gfx::ShaderSource = shaders! {
    GLSL_150: b"
#version 150 core
in vec3 pos;
in vec2 uv;
out vec2 v_uv;
uniform mat4 model_view_projection;

main() {
    v_uv = uv;
    gl_Position = model_view_projection * vec4(pos, 1.0);
}
"
};

pub static FRAGMENT_SRC: gfx::ShaderSource = shaders! {
    GLSL_150: b"
#version 150 core
in vec2 v_uv;
out vec4 out_color;
uniform sampler2D texture;

main() {
    vec4 tex_color = texture(texture, v_uv);
    out_color = tex_color;
}
"
};

