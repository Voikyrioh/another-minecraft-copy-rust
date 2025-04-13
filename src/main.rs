mod game;
mod cube;
mod camera;

use std::sync::Arc;
use wgpu::{Device, Queue, RenderPipeline, Surface};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use crate::cube::Cube;
use image::GenericImageView;

pub struct Memory {
    cubes: Vec<cube::Cube>,
}

pub struct GameState<'a> {
    _instance: wgpu::Instance,
    surface: Surface<'a>,
    device: Device,
    render_pipeline: RenderPipeline,
    queue: Queue,
    config: wgpu::SurfaceConfiguration,
    memory: Memory,
}

impl<'a> GameState<'a> {
    async fn new(window: Arc<Window>) -> GameState<'a> {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
        let size = Arc::clone(&window).inner_size();
        let adapter = &instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }).await.unwrap();

        let (device, queue) = adapter.request_device(
                &wgpu::DeviceDescriptor::default(),
                None,
            ).await.unwrap();

        let surface_caps = &surface.get_capabilities(&adapter);
        let format = surface_caps.formats[0];

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 0,
            alpha_mode:surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // Load the shaders from disk
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Option::from("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Option::from("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        /*
                let diffuse_bytes = include_bytes!("/textures/grass.png");
                let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
                let diffuse_rgba = diffuse_image.to_rgba8();

                let dimensions = diffuse_image.dimensions();
                let texture_size = wgpu::Extent3d {
                    width: dimensions.0,
                    height: dimensions.1,
                    depth_or_array_layers: 1,
                };
                let diffuse_texture = device.create_texture(
                    &wgpu::TextureDescriptor {
                        size: texture_size,
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format: wgpu::TextureFormat::Rgba8UnormSrgb,
                        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::RENDER_ATTACHMENT,
                        label: Some("/textures/grass.png"),

                        view_formats: &[],
                    }
                );*/

        Self {
            _instance: instance,
            surface,
            device,
            render_pipeline,
            queue,
            config,
            memory: Memory { cubes: vec![Cube::new(vec![0,0,0])] },
        }
    }

    fn reconfigure_size(&mut self, size: PhysicalSize<u32>) {
        self.config.height = size.height;
        self.config.width = size.width;
        self.surface.configure(&self.device, &self.config);
    }
}

/*
pub async fn run_game(event_loop: ActiveEventLoop, window: &Window) {

    self.window = Some(event_loop.create_window(Window::default_attributes().with_title("test")).unwrap());
    let size = self.window.as_ref().unwrap().inner_size();
    let instance = wgpu::util::new_instance_with_webgpu_detection(&wgpu::InstanceDescriptor::default());
    let surface = instance.create_surface(&self.window.as_ref()).unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to find an appropriate adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor::default(),
            None,
        )
        .await
        .expect("Failed to create device");

    let surface_caps = surface.get_capabilities(&adapter);
    let format = surface_caps.formats[0];

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        desired_maximum_frame_latency: 0,
        alpha_mode:surface_caps.alpha_modes[0],
        view_formats: vec![],
    };
    surface.configure(&device, &config);

    // Load the shaders from disk
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Option::from("vs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Option::from("fs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });



    event_loop.run_app(&mut move |event, _, mut control_flow: ControlFlow| {
        let _ = (&instance, &adapter, &shader, &pipeline_layout);
        control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Recreate the surface with the new size
                instance.poll_all(true);
                config.width = size.width;
                config.height = size.height;
                surface.configure(&device, &config);
            }
            Event::WindowEvent(_) => {

            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => control_flow = ControlFlow::Wait,
            _ => {}
        }
    }).expect("TODO: panic message");
}
*/

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Arc::new(event_loop.create_window(Window::default_attributes().with_title("test")).unwrap());
            self.window = Some(window.clone());

            let state = pollster::block_on(GameState::new(window.clone()));
            self.state = Some(state);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::Resized(size) => {
                if let Some(state) = self.state.as_mut() {
                    state.reconfigure_size(size);
                }
            }
            WindowEvent::RedrawRequested => {
                let frame = self.state.as_ref().unwrap().surface.get_current_texture().unwrap();
                let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = self.state.as_ref().unwrap().device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.05, g: 0.062, b: 0.08, a: 1.0 }),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });
                    rpass.set_pipeline(&self.state.as_ref().unwrap().render_pipeline);
                    rpass.draw(0..3, 0..1);
                }

                self.state.as_ref().unwrap().queue.submit(Some(encoder.finish()));
                frame.present();

                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

#[derive(Default)]
struct App<'a> {
    window: Option<Arc<Window>>,
    state: Option<GameState<'a>>
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
