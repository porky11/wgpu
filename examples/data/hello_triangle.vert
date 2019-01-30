#version 450

out gl_PerVertex {
    vec4 gl_Position;
};


layout(binding = 0) uniform VertexData {
    vec2 positions[3];
} vertex_data;

void main() {
    gl_Position = vec4(vertex_data.positions[gl_VertexIndex], 0.0, 1.0);
}
