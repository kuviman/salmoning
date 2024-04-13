varying vec2 v_uv;

#ifdef VERTEX_SHADER

attribute vec3 a_pos;
attribute vec2 a_uv;

uniform mat4 u_model_matrix;
uniform mat4 u_view_matrix;
uniform mat4 u_projection_matrix;

void main() {
    v_uv = a_uv;
    gl_Position = u_projection_matrix * u_view_matrix * u_model_matrix * vec4(a_pos, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER

uniform sampler2D u_texture;
uniform vec4 u_match_color;
uniform vec4 u_replace_color;

void main() {
    gl_FragColor = texture2D(u_texture, v_uv);
    if (gl_FragColor == u_match_color) {
        gl_FragColor = u_replace_color;
    }
    if (gl_FragColor.a < 0.5) {
        discard;
    }
}
#endif