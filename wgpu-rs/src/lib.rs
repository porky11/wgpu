extern crate arrayvec;
extern crate wgpu_native as wgn;

use arrayvec::ArrayVec;

use std::ffi::CString;
use std::ops::Range;
use std::ptr;

pub use wgn::{
    AdapterDescriptor, Attachment, BindGroupLayoutBinding, BindingType, BlendStateDescriptor,
    Color, ColorWriteFlags, CommandBufferDescriptor, DepthStencilStateDescriptor, DeviceDescriptor,
    Extensions, Extent3d, LoadOp, Origin3d, PowerPreference, PrimitiveTopology,
    RenderPassColorAttachmentDescriptor, RenderPassDepthStencilAttachmentDescriptor,
    ShaderModuleDescriptor, ShaderStage, ShaderStageFlags, StoreOp,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsageFlags, TextureViewDescriptor,
    BufferDescriptor, SwapChainDescriptor,
};

pub struct Instance {
    id: wgn::InstanceId,
}

pub struct Adapter {
    id: wgn::AdapterId,
}

pub struct Device {
    id: wgn::DeviceId,
}

pub struct Buffer {
    id: wgn::BufferId,
}

pub struct Texture {
    id: wgn::TextureId,
}

pub struct TextureView {
    id: wgn::TextureViewId,
}

pub struct Surface {
    id: wgn::SurfaceId,
}

pub struct SwapChain {
    id: wgn::SwapChainId,
}

pub struct BindGroupLayout {
    id: wgn::BindGroupLayoutId,
}

pub struct BindGroup {
    id: wgn::BindGroupId,
}

pub struct ShaderModule {
    id: wgn::ShaderModuleId,
}

pub struct PipelineLayout {
    id: wgn::PipelineLayoutId,
}

pub struct BlendState {
    id: wgn::BlendStateId,
}

pub struct DepthStencilState {
    id: wgn::DepthStencilStateId,
}

pub struct RenderPipeline {
    id: wgn::RenderPipelineId,
}

pub struct ComputePipeline {
    id: wgn::ComputePipelineId,
}

pub struct CommandBuffer {
    id: wgn::CommandBufferId,
}

pub struct RenderPass<'a> {
    id: wgn::RenderPassId,
    parent: &'a mut CommandBuffer,
}

pub struct ComputePass<'a> {
    id: wgn::ComputePassId,
    parent: &'a mut CommandBuffer,
}

pub struct Queue {
    id: wgn::QueueId,
}

pub struct BindGroupLayoutDescriptor<'a> {
    pub bindings: &'a [BindGroupLayoutBinding],
}

pub struct PipelineLayoutDescriptor<'a> {
    pub bind_group_layouts: &'a [&'a BindGroupLayout],
}

pub struct PipelineStageDescriptor<'a> {
    pub module: &'a ShaderModule,
    pub stage: ShaderStage,
    pub entry_point: &'a str,
}

pub struct AttachmentsState<'a> {
    pub color_attachments: &'a [Attachment],
    pub depth_stencil_attachment: Option<Attachment>,
}

pub struct RenderPipelineDescriptor<'a> {
    pub layout: &'a PipelineLayout,
    pub stages: &'a [PipelineStageDescriptor<'a>],
    pub primitive_topology: PrimitiveTopology,
    pub attachments_state: AttachmentsState<'a>,
    pub blend_states: &'a [&'a BlendState],
    pub depth_stencil_state: &'a DepthStencilState,
}

pub struct RenderPassDescriptor<'a> {
    pub color_attachments: &'a [RenderPassColorAttachmentDescriptor<&'a TextureView>],
    pub depth_stencil_attachment: Option<RenderPassDepthStencilAttachmentDescriptor<&'a TextureView>>,
}

impl Instance {
    pub fn new() -> Self {
        Instance {
            id: wgn::wgpu_create_instance(),
        }
    }

    pub fn get_adapter(&self, desc: &AdapterDescriptor) -> Adapter {
        Adapter {
            id: wgn::wgpu_instance_get_adapter(self.id, desc),
        }
    }

    #[cfg(feature = "winit")]
    pub fn create_surface(&self, window: &wgn::winit::Window) -> Surface {
        Surface {
            id: wgn::wgpu_instance_create_surface_from_winit(self.id, window)
        }
    }
}

impl Adapter {
    pub fn create_device(&self, desc: &DeviceDescriptor) -> Device {
        Device {
            id: wgn::wgpu_adapter_create_device(self.id, desc),
        }
    }
}

impl Device {
    pub fn create_shader_module(&self, spv: &[u8]) -> ShaderModule {
        let desc = wgn::ShaderModuleDescriptor {
            code: wgn::ByteArray {
                bytes: spv.as_ptr(),
                length: spv.len(),
            },
        };
        ShaderModule {
            id: wgn::wgpu_device_create_shader_module(self.id, &desc),
        }
    }

    //TODO: borrow instead of new object?
    pub fn get_queue(&self) -> Queue {
        Queue {
            id: wgn::wgpu_device_get_queue(self.id),
        }
    }

    pub fn create_command_buffer(&self, desc: &CommandBufferDescriptor) -> CommandBuffer {
        CommandBuffer {
            id: wgn::wgpu_device_create_command_buffer(self.id, desc),
        }
    }

    pub fn create_bind_group_layout(&self, desc: &BindGroupLayoutDescriptor) -> BindGroupLayout {
        BindGroupLayout {
            id: wgn::wgpu_device_create_bind_group_layout(
                self.id,
                &wgn::BindGroupLayoutDescriptor {
                    bindings: desc.bindings.as_ptr(),
                    bindings_length: desc.bindings.len(),
                },
            ),
        }
    }

    pub fn create_pipeline_layout(&self, desc: &PipelineLayoutDescriptor) -> PipelineLayout {
        //TODO: avoid allocation here
        let temp_layouts = desc
            .bind_group_layouts
            .iter()
            .map(|bgl| bgl.id)
            .collect::<Vec<_>>();
        PipelineLayout {
            id: wgn::wgpu_device_create_pipeline_layout(
                self.id,
                &wgn::PipelineLayoutDescriptor {
                    bind_group_layouts: temp_layouts.as_ptr(),
                    bind_group_layouts_length: temp_layouts.len(),
                },
            ),
        }
    }

    pub fn create_blend_state(&self, desc: &BlendStateDescriptor) -> BlendState {
        BlendState {
            id: wgn::wgpu_device_create_blend_state(self.id, desc),
        }
    }

    pub fn create_depth_stencil_state(
        &self,
        desc: &DepthStencilStateDescriptor,
    ) -> DepthStencilState {
        DepthStencilState {
            id: wgn::wgpu_device_create_depth_stencil_state(self.id, desc),
        }
    }

    pub fn create_render_pipeline(&self, desc: &RenderPipelineDescriptor) -> RenderPipeline {
        let entry_points = desc
            .stages
            .iter()
            .map(|ps| CString::new(ps.entry_point).unwrap())
            .collect::<ArrayVec<[_; 2]>>();
        let stages = desc
            .stages
            .iter()
            .zip(&entry_points)
            .map(|(ps, ep_name)| wgn::PipelineStageDescriptor {
                module: ps.module.id,
                stage: ps.stage,
                entry_point: ep_name.as_ptr(),
            })
            .collect::<ArrayVec<[_; 2]>>();

        let temp_blend_states = desc.blend_states.iter().map(|bs| bs.id).collect::<Vec<_>>();

        RenderPipeline {
            id: wgn::wgpu_device_create_render_pipeline(
                self.id,
                &wgn::RenderPipelineDescriptor {
                    layout: desc.layout.id,
                    stages: stages.as_ptr(),
                    stages_length: stages.len(),
                    primitive_topology: desc.primitive_topology,
                    attachments_state: wgn::AttachmentsState {
                        color_attachments: desc.attachments_state.color_attachments.as_ptr(),
                        color_attachments_length: desc.attachments_state.color_attachments.len(),
                        depth_stencil_attachment: desc
                            .attachments_state
                            .depth_stencil_attachment
                            .as_ref()
                            .map(|at| at as *const _)
                            .unwrap_or(ptr::null()),
                    },
                    blend_states: temp_blend_states.as_ptr(),
                    blend_states_length: temp_blend_states.len(),
                    depth_stencil_state: desc.depth_stencil_state.id,
                },
            ),
        }
    }

    pub fn create_buffer(&self, desc: &BufferDescriptor) -> Buffer {
        Buffer {
            id: wgn::wgpu_device_create_buffer(self.id, desc),
        }
    }

    pub fn create_texture(&self, desc: &TextureDescriptor) -> Texture {
        Texture {
            id: wgn::wgpu_device_create_texture(self.id, desc),
        }
    }

    pub fn create_swap_chain(&self, surface: &Surface, desc: &SwapChainDescriptor) -> SwapChain {
        SwapChain {
            id: wgn::wgpu_device_create_swap_chain(self.id, surface.id, desc),
        }
    }
}

impl Buffer {
    pub fn set_buffer_data(&self, offset: u32, data: &[u8]) {
        wgn::wgpu_buffer_set_sub_data(self.id, offset, data.len() as u32, data.as_ptr());
    }
}

impl Texture {
    pub fn create_texture_view(&self, desc: &TextureViewDescriptor) -> TextureView {
        TextureView {
            id: wgn::wgpu_texture_create_texture_view(self.id, desc),
        }
    }

    pub fn create_default_texture_view(&self) -> TextureView {
        TextureView {
            id: wgn::wgpu_texture_create_default_texture_view(self.id),
        }
    }
}

impl CommandBuffer {
    pub fn begin_render_pass(&mut self, desc: &RenderPassDescriptor) -> RenderPass {
        let colors = desc
            .color_attachments
            .iter()
            .map(|ca| RenderPassColorAttachmentDescriptor {
                attachment: ca.attachment.id,
                load_op: ca.load_op,
                store_op: ca.store_op,
                clear_color: ca.clear_color,
            })
            .collect::<ArrayVec<[_; 4]>>();

        let depth_stencil = desc.depth_stencil_attachment.as_ref().map(|dsa| {
            RenderPassDepthStencilAttachmentDescriptor {
                attachment: dsa.attachment.id,
                depth_load_op: dsa.depth_load_op,
                depth_store_op: dsa.depth_store_op,
                clear_depth: dsa.clear_depth,
                stencil_load_op: dsa.stencil_load_op,
                stencil_store_op: dsa.stencil_store_op,
                clear_stencil: dsa.clear_stencil,
            }
        });

        RenderPass {
            id: wgn::wgpu_command_buffer_begin_render_pass(
                self.id,
                wgn::RenderPassDescriptor {
                    color_attachments: colors.as_ptr(),
                    color_attachments_length: colors.len(),
                    depth_stencil_attachment: depth_stencil
                        .as_ref()
                        .map(|at| at as *const _)
                        .unwrap_or(ptr::null()),
                },
            ),
            parent: self,
        }
    }

    pub fn begin_compute_pass(&mut self) -> ComputePass {
        ComputePass {
            id: wgn::wgpu_command_buffer_begin_compute_pass(self.id),
            parent: self,
        }
    }
}

impl<'a> RenderPass<'a> {
    pub fn end_pass(self) -> &'a mut CommandBuffer {
        wgn::wgpu_render_pass_end_pass(self.id);
        self.parent
    }

    pub fn set_bind_group(&mut self, index: u32, bind_group: &BindGroup) {
        wgn::wgpu_render_pass_set_bind_group(self.id, index, bind_group.id);
    }

    pub fn set_pipeline(&mut self, pipeline: &RenderPipeline) {
        wgn::wgpu_render_pass_set_pipeline(self.id, pipeline.id);
    }

    pub fn draw(
        &mut self, vertices: Range<u32>, instances: Range<u32>
    ) {
        wgn::wgpu_render_pass_draw(
            self.id,
            vertices.end - vertices.start,
            instances.end - instances.start,
            vertices.start,
            instances.start,
        );
    }

    pub fn draw_indexed(
        &mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>
    ) {
        wgn::wgpu_render_pass_draw_indexed(
            self.id,
            indices.end - indices.start,
            instances.end - instances.start,
            indices.start,
            base_vertex,
            instances.start,
        );
    }
}

impl<'a> ComputePass<'a> {
    pub fn end_pass(self) -> &'a mut CommandBuffer {
        wgn::wgpu_compute_pass_end_pass(self.id);
        self.parent
    }

    pub fn set_bind_group(&mut self, index: u32, bind_group: &BindGroup) {
        wgn::wgpu_compute_pass_set_bind_group(self.id, index, bind_group.id);
    }

    pub fn set_pipeline(&mut self, pipeline: &ComputePipeline) {
        wgn::wgpu_compute_pass_set_pipeline(self.id, pipeline.id);
    }

    pub fn dispatch(&mut self, x: u32, y: u32, z: u32) {
        wgn::wgpu_compute_pass_dispatch(self.id, x, y, z);
    }
}

impl Queue {
    pub fn submit(&self, command_buffers: &[CommandBuffer]) {
        wgn::wgpu_queue_submit(
            self.id,
            command_buffers.as_ptr() as *const _,
            command_buffers.len(),
        );
    }
}

impl SwapChain {
    //TODO: borrow instead of new object?
    pub fn get_next_texture(&self) -> (Texture, TextureView) {
        let output = wgn::wgpu_swap_chain_get_next_texture(self.id);
        (Texture { id: output.texture_id} , TextureView { id: output.view_id })
    }

    pub fn present(&self) {
        wgn::wgpu_swap_chain_present(self.id);
    }
}
