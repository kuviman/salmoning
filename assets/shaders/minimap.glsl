#ifdef VERTEX_SHADER

attribute vec3 a_pos;

uniform mat4 u_model_matrix;
uniform mat4 u_view_matrix;
uniform mat4 u_projection_matrix;

void main() {
    vec4 pos = u_model_matrix * vec4(a_pos, 1.0);
    gl_Position = u_projection_matrix * u_view_matrix * pos;
}
#endif

#ifdef FRAGMENT_SHADER

uniform sampler2D u_texture;
uniform vec4 u_color;

void main() {
    gl_FragColor = u_color;
}
#endif
