#version 140
in vec2 position;
uniform float aspectRatio;
uniform float pointSize;
void main() {
    gl_Position = vec4(position.x * aspectRatio, position.y, 0.0, 1.0);
    gl_PointSize = pointSize;
}