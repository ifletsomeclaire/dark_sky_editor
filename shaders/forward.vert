#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;
// layout(location = 3) in vec4 Vertex_Color;
// layout(location = 4) in float Vertex_Texture;

layout(location = 0) out vec3 v_Position;
layout(location = 1) out vec3 v_Normal;
layout(location = 2) out vec2 v_Uv;
// layout(location = 3) out vec4 v_Color;
// layout(location = 4) out float v_Texture;
layout(location = 5) out float v_Lod;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 3, binding = 1) uniform MeshMaterial_distance {
    float distance;
};

layout(set = 2, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    v_Normal = (Model * vec4(Vertex_Normal, 1.0)).xyz;
    v_Normal = mat3(Model) * Vertex_Normal;
    v_Position = (Model * vec4(Vertex_Position, 1.0)).xyz;
    v_Uv = Vertex_Uv;
    // v_Color = Vertex_Color;
    // v_Texture = Vertex_Texture;
    v_Lod = distance;
    gl_Position = ViewProj * vec4(v_Position, 1.0);
}
