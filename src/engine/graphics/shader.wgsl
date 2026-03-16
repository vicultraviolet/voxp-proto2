struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>
}

@vertex
fn vs_main(
    @builtin(vertex_index) i_VertexIndex: u32,
) -> VertexOutput {
    var o: VertexOutput;

    let x = f32(1 - i32(i_VertexIndex)) * 0.5;
    let y = f32(i32(i_VertexIndex & 1u) * 2 - 1) * 0.5;

    o.clip_position = vec4<f32>(x, y, 0.0, 1.0);

    o.color[i_VertexIndex] = 1.0;

    return o;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color.x, in.color.y, in.color.z, 1.0);
}
