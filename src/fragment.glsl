#version 140
out vec4 color;

void main() {
    if (length(gl_PointCoord - vec2(0.5)) > 0.5) {
        discard;
    }
    
    color = vec4(1.0, 0.0, 0.0, 1.0);
}