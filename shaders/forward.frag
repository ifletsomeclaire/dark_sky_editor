#version 450

const int MAX_LIGHTS = 10;

struct Light {
    mat4 proj;
    vec4 pos;
    vec4 color;
};

layout(location = 0) in vec3 v_Position;
layout(location = 1) in vec3 v_Normal;
layout(location = 2) in vec2 v_Uv;
// layout(location = 3) in vec4 v_Color;
// layout(location = 4) in float v_Texture;
layout(location = 5) in float v_Lod;

layout(location = 0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Lights {
    uvec4 NumLights;
    Light SceneLights[MAX_LIGHTS];
};

layout(set = 3, binding = 0) uniform MeshMaterial_basecolor {
    vec4 Albedo;
};

# ifdef MESHMATERIAL_TEXTURE1
layout(set = 3, binding = 2) uniform texture2D MeshMaterial_texture1;
layout(set = 3, binding = 3) uniform sampler MeshMaterial_texture1_sampler;
# endif
# ifdef MESHMATERIAL_TEXTURE2
layout(set = 3, binding = 4) uniform texture2D MeshMaterial_texture2;
layout(set = 3, binding = 5) uniform sampler MeshMaterial_texture2_sampler;
# endif
# ifdef MESHMATERIAL_TEXTURE3
layout(set = 3, binding = 6) uniform texture2D MeshMaterial_texture3;
layout(set = 3, binding = 7) uniform sampler MeshMaterial_texture3_sampler;
# endif
# ifdef MESHMATERIAL_TEXTURE4
layout(set = 3, binding = 8) uniform texture2D MeshMaterial_texture4;
layout(set = 3, binding = 9) uniform sampler MeshMaterial_texture4_sampler;
# endif
# ifdef MESHMATERIAL_TEXTURE5
layout(set = 3, binding = 10) uniform texture2D MeshMaterial_texture5;
layout(set = 3, binding = 11) uniform sampler MeshMaterial_texture5_sampler;
# endif

void main() {
    vec4 output_color = Albedo;
if (v_Lod > 2000.0) {
    # ifdef MESHMATERIAL_TEXTURE2
        output_color *= texture(
            sampler2D(MeshMaterial_texture2, MeshMaterial_texture2_sampler),
            v_Uv);
    # endif
} else if (v_Lod > 4000.0) {
    # ifdef MESHMATERIAL_TEXTURE3
        output_color *= texture(
            sampler2D(MeshMaterial_texture3, MeshMaterial_texture3_sampler),
            v_Uv);
    # endif
} else if (v_Lod > 8000.0) {
    # ifdef MESHMATERIAL_TEXTURE4
        output_color *= texture(
            sampler2D(MeshMaterial_texture4, MeshMaterial_texture4_sampler),
            v_Uv);
    # endif
} else if (v_Lod > 10000.0) {
    # ifdef MESHMATERIAL_TEXTURE5
        output_color *= texture(
            sampler2D(MeshMaterial_texture5, MeshMaterial_texture5_sampler),
            v_Uv);
    # endif
} else {
    # ifdef MESHMATERIAL_TEXTURE1
        output_color *= texture(
            sampler2D(MeshMaterial_texture1, MeshMaterial_texture1_sampler),
            v_Uv);
    # endif
}

# ifdef MESHMATERIAL_SHADED
    vec3 normal = normalize(v_Normal);
    vec3 ambient = vec3(0.05, 0.05, 0.05);
    // accumulate color
    vec3 color = ambient;
    for (int i=0; i<int(NumLights.x) && i<MAX_LIGHTS; ++i) {
        Light light = SceneLights[i];
        // compute Lambertian diffuse term
        vec3 light_dir = normalize(light.pos.xyz - v_Position);
        float diffuse = max(0.0, dot(normal, light_dir));
        // add light contribution
        color += diffuse * light.color.xyz;
    }
    output_color.xyz *= color;
# endif
    // vec4 blue = vec4(0.0, 0.0, 1.0, 1.0);
    // if (output_color.a > 0.0) {
        o_Target = output_color;
    // } else {
        // o_Target = vec4(blue[0], blue[1], blue[2], v_Uv[0]);  
    // }
    // multiply the light by material color
}
