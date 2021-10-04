use inline_spirv::inline_spirv;

pub const VERTEX_SHADER_CODE: &[u32] = inline_spirv!(
    r#"
#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 size;
layout(location = 2) in uint index;

layout(push_constant) uniform PushConstants {
    mat4 projection_matrix;
};

layout(location = 0) out vec3 fragment_pos;
layout(location = 1) flat out uint out_index;

void main() {
    switch (gl_VertexIndex) {
        case 0:{
            vec3 vertex = position;

            vec4 projected_vertex = projection_matrix * vec4(vertex,1.0);
            gl_Position = vec4(projected_vertex.xy,0.0,1.0);
            fragment_pos = vec3(0.0,0.0,projected_vertex.z);
            break;
        }
        case 1:{
            vec3 vertex = vec3(position.x,position.y+size.y,position.z);

            vec4 projected_vertex = projection_matrix * vec4(vertex,1.0);
            gl_Position = vec4(projected_vertex.xy,0.0,1.0);
            fragment_pos = vec3(0.0,1.0,projected_vertex.z);
            break;
        }
        case 2:{
            vec3 vertex = vec3(position.x+size.x,position.y,position.z);

            vec4 projected_vertex = projection_matrix * vec4(vertex,1.0);
            gl_Position = vec4(projected_vertex.xy,0.0,1.0);
            fragment_pos = vec3(1.0,0.0,projected_vertex.z);
            break;
        }
        case 3:{
            vec3 vertex = vec3(position.x+size.x,position.y+size.y,position.z);

            vec4 projected_vertex = projection_matrix * vec4(vertex,1.0);
            gl_Position = vec4(projected_vertex.xy,0.0,1.0);
            fragment_pos = vec3(1.0,1.0,projected_vertex.z);
            break;
        }
        default:{
            vec3 vertex = vec3(2.0,2.0,2.0);

            gl_Position = vec4(vertex,1.0);
            fragment_pos = vec3(0.0,0.0,0.0);
        }
    }

    out_index = index;
}
"#,
    vert
);

pub const FRAGMENT_SHADER_CODE: &[u32] = inline_spirv!(
    r#"
#version 450

#extension GL_EXT_nonuniform_qualifier : require

layout(location = 0) in vec3 fragment_position;
layout(location = 1) nonuniformEXT flat in uint index;  // dynamically non-uniform
layout(location = 0) out vec4 fragment_color;

layout(set = 0, binding = 0) uniform sampler samp;
layout(set = 0, binding = 1) uniform texture2D textures[];


void main() {
    vec4 color = vec4(texture(sampler2D(textures[index], samp), fragment_position.xy));
    if(color.w == 0.0) {discard;}
    else{fragment_color = color;}

    gl_FragDepth = fragment_position.z;
}
"#,
    frag
);
