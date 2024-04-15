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

uniform sampler2D u_gradient_texture;
uniform sampler2D u_scribles_texture;

void main() {
    mat4 inv = inverse(u_projection_matrix * u_view_matrix);
    vec4 pos_a = inv * vec4(v_pos, 0.0, 1.0);
    vec4 pos_b = inv * vec4(v_pos, 1.0, 1.0);
    vec3 dir = pos_b.xyz / pos_b.w - pos_a.xyz / pos_a.w;

    float t = atan(dir.z / length(dir.xy)) / 3.14 + 0.5;
    vec4 gradient_color = texture2D(u_gradient_texture, vec2(0.0, t));

    gl_FragColor = gradient_color;
}
#endif
