use std::mem::size_of;

use super::{
    shaders::{Shader, Shaders},
    vertex::Vertex,
    DEPTH_FORMAT,
};

#[derive(Debug)]
pub struct Pipelines {
    pub model: Pipeline,
    pub mesh: Pipeline,
}

impl Pipelines {
    pub fn new(
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[bind_group_layout],
                push_constant_ranges: &[],
            });

        let shaders = Shaders::new(device);

        let model = Pipeline::new(
            device,
            &pipeline_layout,
            shaders.model(),
            wgpu::PolygonMode::Fill,
        );
        let mesh = Pipeline::new(
            device,
            &pipeline_layout,
            shaders.mesh(),
            wgpu::PolygonMode::Line,
        );

        Self { model, mesh }
    }
}

#[derive(Debug)]
pub struct Pipeline(pub wgpu::RenderPipeline);

impl Pipeline {
    fn new(
        device: &wgpu::Device,
        pipeline_layout: &wgpu::PipelineLayout,
        shader: Shader,
        polygon_mode: wgpu::PolygonMode,
    ) -> Self {
        let pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(pipeline_layout),
                vertex: wgpu::VertexState {
                    module: shader.module,
                    entry_point: "vertex",
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: size_of::<Vertex>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![
                            0 => Float32x3,
                            1 => Float32x3,
                            2 => Float32x4,
                        ],
                    }],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    clamp_depth: false,
                    polygon_mode: polygon_mode,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::LessEqual,
                    stencil: wgpu::StencilState {
                        front: wgpu::StencilFaceState::IGNORE,
                        back: wgpu::StencilFaceState::IGNORE,
                        read_mask: 0,
                        write_mask: 0,
                    },
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment: Some(wgpu::FragmentState {
                    module: shader.module,
                    entry_point: shader.frag_entry,
                    targets: &[wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        // TASK: Enable alpha blending.
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    }],
                }),
            });

        Self(pipeline)
    }
}
