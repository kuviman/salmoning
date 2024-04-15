uniform float u_time;
varying vec2 v_pos;

uniform mat4 u_view_matrix;
uniform mat4 u_projection_matrix;

#ifdef VERTEX_SHADER

attribute vec2 a_pos;

void main() {
    v_pos = a_pos;
    gl_Position = vec4(a_pos, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER

void main() {
    vec4 low_color = vec4(0.0, 0.0, 1.0, 1.0);
    vec4 high_color = vec4(0.0, 1.0, 1.0, 1.0);

    mat4 inv = inverse(u_projection_matrix * u_view_matrix);
    vec4 pos_a = inv * vec4(v_pos, 0.0, 1.0);
    vec4 pos_b = inv * vec4(v_pos, 1.0, 1.0);
    vec3 dir = pos_b.xyz / pos_b.w - pos_a.xyz / pos_a.w;

    float t = atan(dir.z / length(dir.xy)) / 3.14 + 0.5;
    gl_FragColor = low_color + (high_color - low_color) * t;
}
#endif
