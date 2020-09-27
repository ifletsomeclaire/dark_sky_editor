#version 450

in vec4 gl_FragCoord;

const int MAX_LIGHTS = 10;

struct Light {
    mat4 proj;
    vec4 pos;
    vec4 color;
};

layout(location = 0) in vec3 v_Position;
layout(location = 1) in vec3 v_Normal;
layout(location = 2) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Lights {
    uvec4 NumLights;
    Light SceneLights[MAX_LIGHTS];
};

layout(set = 3, binding = 0) uniform MegaMeshMaterial_basecolor {
    vec4 basecolor;
};

# ifdef MEGAMESHMATERIAL_TEXTURE
layout(set = 3, binding = 1) uniform texture2D MegaMeshMaterial_texture;
layout(set = 3, binding = 2) uniform sampler MegaMeshMaterial_texture_sampler;
# endif


void main() {
    vec4 output_color = basecolor;

    # ifdef MEGAMESHMATERIAL_TEXTURE
        output_color = texture(
            sampler2D(MegaMeshMaterial_texture, MegaMeshMaterial_texture_sampler),
            v_Uv);
    # endif

    o_Target = output_color;

}
