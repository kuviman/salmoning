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
    
    mat4 model_view = u_view_matrix * u_model_matrix;
    model_view[0][0] = 0.4; 
    model_view[0][1] = 0.0; 
    model_view[0][2] = 0.0; 
    model_view[1][0] = 0.0; 
    model_view[1][1] = 0.4; 
    model_view[1][2] = 0.0; 
    model_view[2][0] = 0.0; 
    model_view[2][1] = 0.0; 
    model_view[2][2] = 0.4; 
    vec4 pos = model_view * vec4(a_pos, 1.0);
    gl_Position = u_projection_matrix * pos;
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
