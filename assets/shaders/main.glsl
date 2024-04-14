uniform float u_time;
uniform float u_wiggle;

varying vec4 v_color;
varying vec2 v_uv;

#ifdef VERTEX_SHADER

attribute vec3 a_pos;
attribute vec4 a_color;
attribute vec2 a_uv;

uniform mat4 u_model_matrix;
uniform mat4 u_view_matrix;
uniform mat4 u_projection_matrix;

void main() {
    v_uv = a_uv;
    v_color = a_color;
    vec4 pos = u_model_matrix * vec4(a_pos, 1.0);
    float t = u_time * 3.0 + pos.x + pos.y;
    vec3 wiggle_pos = a_pos * 0.5 + 0.5;
    pos += vec4(vec3(sin(t), sin(t), cos(t)) * wiggle_pos, 0.0) * 0.1 * u_wiggle * wiggle_pos.z;
    gl_Position = u_projection_matrix * u_view_matrix * pos;
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
    gl_FragColor *= v_color;
    if (gl_FragColor.a < 0.5) {
        discard;
    }
}
#endif
