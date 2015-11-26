#version 120

varying vec2 v_TexCoord;
uniform vec4 u_Color;

void main() {
    float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
    gl_FragColor = mix(u_Color, vec4(0.0,0.0,0.0,0.0), blend*1.0);
}
