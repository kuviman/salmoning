varying vec4 v_color;

#ifdef VERTEX_SHADER

attribute vec3 a_pos;

attribute vec4 i_color;
attribute mat4 i_model_matrix;

uniform mat4 u_view_matrix;
uniform mat4 u_projection_matrix;

void main() {
    v_color = i_color;
    vec4 pos = i_model_matrix * vec4(a_pos, 1.0);
    gl_Position = u_projection_matrix * u_view_matrix * pos;
}
#endif

#ifdef FRAGMENT_SHADER

void main() {
    gl_FragColor = v_color;
}
#endif
