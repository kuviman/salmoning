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
uniform sampler2D u_scribbles_texture;

void main() {
    mat4 inv = inverse(u_projection_matrix * u_view_matrix);
    vec4 pos_a = inv * vec4(v_pos, 0.0, 1.0);
    vec4 pos_b = inv * vec4(v_pos, 1.0, 1.0);
    vec3 dir = pos_b.xyz / pos_b.w - pos_a.xyz / pos_a.w;

    float y = atan(dir.z / length(dir.xy)) / 3.14 + 0.5;
    float x = atan(dir.x, dir.y) / 6.28 + 0.5;

    vec4 gradient_color = texture2D(u_gradient_texture, vec2(0.0, y));
    float repeat = 4.0;
    vec4 scribble_color = texture2D(u_scribbles_texture, vec2(x, y) * repeat);

    vec3 color = gradient_color.rgb * (1.0 - scribble_color.a) + scribble_color.rgb * scribble_color.a;
    gl_FragColor = vec4(color, 1.0);
}
#endif
