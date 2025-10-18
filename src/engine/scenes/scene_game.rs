use crate::engine::scenes::handling::scene_trait::{Scene, SceneResources};
use crate::engine::core::input::InputState;
use crate::engine::rendering::camera::Camera;
use crate::engine::rendering::descriptor_helpers::make_mvp_descriptor_set;
use vulkano::buffer::Subbuffer;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::descriptor_set::{DescriptorSet};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::layout::DescriptorSetLayout;
use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
use vulkano::image::sampler::Sampler;
use vulkano::image::view::ImageView;
use vulkano::pipeline::graphics::viewport::Viewport;
use crate::engine::rendering::vertex::MyVertex;
use crate::engine::core::content_loader::GameContent;
use crate::content::world::world::World;
use crate::content::world::chunks::chunk_mesh::build_chunk_mesh;
use std::collections::HashMap;
use std::sync::Arc;

pub struct GameScene {
    pub world: Option<World>,
    pub chunk_meshes: HashMap<[i32; 3], Subbuffer<[MyVertex]>>,
}

impl Scene for GameScene {
    fn start(&mut self) {
        println!("Started Game Scene");
    }

    fn update(&mut self, delta_time: f32, input_state: &mut InputState, camera: &mut Camera) {
        camera.update(delta_time, input_state);
    }

    fn fixed_update(&mut self, _delta_time: f32, _input_state: &mut InputState, _camera: &mut Camera) {}

    fn late_update(&mut self, _delta_time: f32, _input_state: &mut InputState, _camera: &mut Camera) {}

    fn draw(
        &self,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        _viewport: &Viewport,
        _allocator: &StandardMemoryAllocator,
        resources: &SceneResources,
    ) {
        let Some(world) = &self.world else { return; };
        let pipeline = &resources.default_pipeline;

        for (pos, mesh) in &self.chunk_meshes {
            // Bind pipeline and vertex buffer
            builder
                .bind_pipeline_graphics(pipeline.clone())
                .unwrap()
                .bind_vertex_buffers(0, mesh.clone())
                .unwrap();

            unsafe {
                builder.draw(mesh.len() as u32, 1, 0, 0).unwrap();
            }
        }
    }

    fn create_mvp_descriptor_set(
        &self,
        memory_allocator: &Arc<StandardMemoryAllocator>,
        descriptor_set_allocator: &Arc<StandardDescriptorSetAllocator>,
        layout: &Arc<DescriptorSetLayout>,
        camera: &Camera,
        texture_view: &Arc<ImageView>,
        sampler: &Arc<Sampler>,
    ) -> Option<Arc<DescriptorSet>> {
        make_mvp_descriptor_set(memory_allocator, descriptor_set_allocator, layout, camera, texture_view, sampler)
    }

    fn inject_resources(&mut self, res: &SceneResources) {
        if self.world.is_none() {
            let mut world = World::new();
            world.set_allocator(res.memory_allocator.clone());

            // Load initial chunk
            let content = GameContent::get();
            let chunk = world.load_chunk([0, 0, 0], &content);
            let allocator = world.memory_allocator.clone().unwrap();
            let chunk_mesh = build_chunk_mesh(allocator, &chunk);

            self.chunk_meshes.insert([0, 0, 0], chunk_mesh);
            self.world = Some(world);
        }
    }
}

impl GameScene {
    pub fn new() -> Self {
        Self {
            world: None,
            chunk_meshes: HashMap::new(),
        }
    }

    /// Initialize world and build meshes for all loaded chunks
    pub fn init_world(&mut self) {
        let content: Arc<GameContent> = GameContent::get();
        let mut world = World::new();

        let chunk = world.load_chunk([0, 0, 0], &content);
        let allocator = world.memory_allocator.clone().unwrap_or_else(|| {
            panic!("World missing allocator, pass StandardMemoryAllocator when creating world.");
        });

        let chunk_mesh = build_chunk_mesh(allocator, &chunk);

        self.chunk_meshes.insert([0, 0, 0], chunk_mesh);
        self.world = Some(world);
    }
}