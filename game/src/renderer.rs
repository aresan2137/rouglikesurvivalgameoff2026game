#[cfg(not(target_arch = "wasm32"))]
use std::fs;

use bevy_ecs::component::Component;
use glam::*;
use smq_engine::{
    *,
};
use wgpu::*;

use crate::{
    gen_bake::{GenAssets, Meshes, Textures}};

#[allow(unused)]
pub struct Renderer {
    gen_assets: GenAssets,
    mat1: RenderMaterial,
    bind_group0: BindGroupS,
    bind_group1: BindGroupS,
    bind_group2_lin: Vec<BindGroupS>
}

fn new_texture_binding(context: &mut Context<Renderer>, gen_assets: &GenAssets) -> Vec<BindGroupS> {
    let mut vev = Vec::new();

    for texture in gen_assets.textures.iter() {
        vev.push(BindGroupS::new(
            context,
            &[
                gen_assets.dubodata2.get_binding(ShaderStages::VERTEX),
                texture.get_texture_binding(ShaderStages::FRAGMENT),
                texture.get_sampler_binding(ShaderStages::FRAGMENT),
            ],
            None
        ));
    }   

    return vev;
}

#[allow(unused)]
pub fn load_assets(context: &mut Context<Renderer>) {
    #[cfg(not(target_arch = "wasm32"))]
    let ssf_text_vec = fs::read("smq_proj/build/game.ssf").unwrap();
    #[cfg(not(target_arch = "wasm32"))]
    let ssf_text = ssf_text_vec.as_slice();

    #[cfg(target_arch = "wasm32")]
    let ssf_text = include_bytes!("../../smq_proj/build/game.ssf");

    let ssf_data = load_ssf(ssf_text).unwrap();

    let mut gen_assets = GenAssets::init_gen_assets(context, &ssf_data);

    let aspect = 16.0/9.0;
    gen_assets.ubodata0.data.proj = camera::rh::proj::directx::orthographic(-10.0 * aspect, 10.0 * aspect, -10.0, 10.0, 1.0, -1.0);

    gen_assets.ubodata0.upload_data(context);
    gen_assets.dubodata2.upload_data(context);

    let bind_group0 = BindGroupS::new(
        context,
        &[gen_assets.ubodata0.get_binding(ShaderStages::VERTEX)],
        None,
    );

    let bind_group1 = BindGroupS::new(
        context,
        &[gen_assets.ubodata1.get_binding(ShaderStages::VERTEX)],
        None,
    );

    let bind_group2_lin = new_texture_binding(context, &gen_assets);

    let holding = context.holding.as_mut().unwrap();

    let mat1layout = holding
        .device
        .create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                Some(&bind_group0.bind_group_layout),
                Some(&bind_group1.bind_group_layout),
                Some(&bind_group2_lin[0].bind_group_layout),
                None,
            ],
            immediate_size: 0,
        });


    let mat1module = holding.device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
            "../../smq_proj/assets/renderer/shaders/textured.wgsl"
        ))),
    });

    let mat1 = RenderMaterial::new(
        context,
        None,
        None,
        &gen_assets.renderer_main_renderTexture,
        &mat1module,
        None,
        get_default_vert_layout(),
        Some(CompareFunction::Less),
        &mat1layout,
        None,
        Some(true)
    );

    context.data = Some(Renderer {
        gen_assets,
        mat1,
        bind_group0,
        bind_group1,
        bind_group2_lin
    });
}

#[derive(Component)]
pub struct DrawedObject {
    pub position: Vec3,
    pub rotation: Quat,
    pub size: Vec3,
    pub meshid: Meshes,
    pub textureid: Textures,
    pub is_gui: bool,
    pub enabled: bool
}

#[allow(unused)]
pub fn render(context: &mut Context<Renderer>, full_output: ::egui::FullOutput) {
    let mut data = context.data.take().unwrap();

    if let Some(mut frame) = context.start_frame() {
        
    let mut camera_query = context.world.query::<&FreeCamera>();
        if let Some(cam) = camera_query.iter(&context.world).next() {
            let q_yaw = Quat::from_axis_angle(Vec3::Y, cam.yaw.to_radians());
            let q_pitch = Quat::from_axis_angle(Vec3::X, cam.pitch.to_radians());
            let rotation = (q_yaw * q_pitch).normalize();

            let forward = rotation * Vec3::new(0.0, 0.0, 1.0);
            let up = rotation * Vec3::new(0.0, 1.0, 0.0);

            let view = glam::camera::lh::view::look_to_mat4(cam.position, forward, up);

            data.gen_assets.ubodata1.data.view = view;
            data.gen_assets.ubodata1.upload_data(context);
        } else {
            println!("NO CAMERA!!!");
        }

        let mut objects_query = context.world.query::<&DrawedObject>();
        let mut objects_to_draw = Vec::new();
        for (i, pos) in objects_query.iter(&context.world).enumerate() {
            let model_matrix = Mat4::from_scale_rotation_translation(pos.size, pos.rotation, pos.position);

            data.gen_assets.dubodata2.data[i].model = model_matrix;
            data.gen_assets.dubodata2.data[i].is_gui = pos.is_gui as u32;

            objects_to_draw.push((pos.meshid, pos.textureid, pos.enabled));
        }

        data.gen_assets.dubodata2.upload_data(&context);

        {
            let mut render_pass = context.get_render_pass(&mut frame, &data.gen_assets.renderer_main_renderTexture);

            render_pass.set_pipeline(&mut data.mat1.pipeline);

            render_pass.set_bind_group(0, &data.bind_group0.bind_group, &[]);
            render_pass.set_bind_group(1, &data.bind_group1.bind_group, &[]);

            for i in 0..objects_to_draw.len()  {
                let obj = objects_to_draw[i];

                if obj.2 {
                    let gutid = obj.0 as usize;
                    
                    render_pass.set_bind_group(2, &data.bind_group2_lin[obj.1 as usize].bind_group, &[i as u32 * 256]);

                    render_pass.set_vertex_buffer(0, data.gen_assets.meshes[gutid].buffer.slice(..));
                    render_pass.draw(0..data.gen_assets.meshes[gutid].vertex_count, 0..1);
                }
            }
        }

        context.blit_screen(
            &mut frame,
            &data.gen_assets.blitinfo,
            &data.gen_assets.blit_group,
        );

        context.draw_egui(&mut frame, full_output);

        context.end_frame(frame);
    }

    context.data = Some(data);
}
