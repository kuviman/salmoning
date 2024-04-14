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

void main() {
    gl_FragColor = vec4(1.0, 1.0, 0.0, 1.0 - v_uv.y);
}
#endif