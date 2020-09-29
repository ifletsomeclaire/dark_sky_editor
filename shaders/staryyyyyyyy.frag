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

layout(set = 3, binding = 0) uniform SkyboxMaterial_basecolor {
    vec4 basecolor;
};


void main() {
    vec4 output_color = basecolor;

    vec3 normal = normalize(v_Normal);
    vec3 ambient = vec3(0.05, 0.05, 0.05);
    vec3 color = ambient;
    for (int i=0; i<int(NumLights.x) && i<MAX_LIGHTS; ++i) {
        Light light = SceneLights[i];
        // compute Lambertian diffuse term
        vec3 light_dir = normalize(light.pos.xyz - v_Position);
        float diffuse = max(0.0, dot(normal, light_dir));
        // add light contribution
        color += diffuse * light.color.xyz;
    }



    float res_x = 720. * 3.;
    float res_y = 480. * 3.;

    vec2 screen_pos = vec2(gl_FragCoord.x, gl_FragCoord.y);
    vec2 resolution = vec2(res_x, res_y);
    vec2 uv = (screen_pos-.5*resolution.xy)/resolution.y;

    uv *= 3.;

    
    vec3 col = vec3(0);

    float d = length(uv);
    float m = .06/d;

    col += m;
    float rays = max(0., 1.-abs(uv.x*uv.y*(res_y/2.)));

    col += rays;
    // float m = smoothstep(.2, .05, d);



    
    // output_color.xyz *= color;

    
    // output_color[3] += gl_FragCoord.y / 2000.;
    // // output_color[2] *= gl_FragCoord.x;
    // o_Target = output_color;

    vec4 out_color =  vec4(col, 1.0);

    o_Target = out_color;
    
}
