#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::forward_io::FragmentOutput

struct StatusBarMaterial {
    foreground_color: vec4<f32>,
    background_color: vec4<f32>,
    percent: f32,
};


@group(1) @binding(0) var<uniform> material: StatusBarMaterial;

@fragment
fn fragment(mesh: VertexOutput) -> FragmentOutput {

    var out: FragmentOutput;

    if (mesh.uv.x <= material.percent) {
        out.color = material.foreground_color;
    } else {
        out.color = material.background_color;
    }

    return out;
}