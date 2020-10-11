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

layout(set = 3, binding = 0) uniform DebugMaterial_basecolor {
    vec4 basecolor;
};

# ifdef DEBUGMATERIAL_TEXTURE
layout(set = 3, binding = 1) uniform texture2D DebugMaterial_texture;
layout(set = 3, binding = 2) uniform sampler DebugMaterial_texture_sampler;
# endif

vec4 gen_color(vec2 uv) {
    if (uv.x + uv.y > 1.99) {
        return vec4(1.0, 1.0, 1.0, 1.0); //white 1,1
    } else if (uv.x + uv.y < 0.01) {
        return vec4(0.0, 0.0, 1.0, 1.0); //blue 0,0
    } else if (uv.x > 0.99) {
        return vec4(1.0, 0.0, 0.0, 1.0); //red 1,0
    } else if (uv.y > 0.99) {
        return vec4(0.0, 1.0, 0.0, 1.0); //green 0,1
    } else if (uv.x + uv.y > 0.99) {
        return vec4(1.0, 0.0, 1.0, 1.0); //pink .5,.5
    } else if (uv.x > 0.49) {
        return vec4(1.0, 1.0, 0.0, 1.0); //yellow .5,0
    } else if (uv.y > 0.49) {
        return vec4(0.0, 1.0, 1.0, 1.0); //teal 0,.5
    } else {
        return vec4(0.0, 0.0, 0.0, 1.0); //black 0,0
    }
}

void main() {
    // vec4 output_color = basecolor;

    vec4 output_color = gen_color(v_Uv);

    # ifdef DEBUGMATERIAL_TEXTURE
        output_color = texture(
            sampler2D(DebugMaterial_texture, DebugMaterial_texture_sampler),
            v_Uv);
    # endif

    // APPLY LIGHTING
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
    // END APPLY LIGHTING

    o_Target = output_color;
}
