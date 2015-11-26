#version 120

attribute vec3 a_Pos;
attribute vec4 a_Color;
varying vec4 v_Color;

uniform mat4 u_Transform;

void main() {
    v_Color = a_Color;
    gl_Position = u_Transform * vec4(a_Pos, 1.0);
}
