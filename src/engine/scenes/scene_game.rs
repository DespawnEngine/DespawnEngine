use crate::content::world;
use crate::content::world::chunks::chunk::{self, CHUNK_SIZE, Chunk};
use crate::content::world::chunks::chunk_mesh;
use crate::content::world::world::World;
use crate::engine::core::content_loader::GameContent;
use crate::engine::core::input::InputState;
use crate::engine::rendering::camera::Camera;
use crate::engine::rendering::descriptor_helpers::make_mvp_descriptor_set;
use crate::engine::rendering::texture_atlas::AtlasUV;
use crate::engine::rendering::vertex::BlockVertex;
use crate::engine::scenes::handling::scene_trait::{Scene, SceneResources};
use std::collections::HashMap;
use std::io::Error;
use std::sync::Arc;
use vulkano::buffer::Subbuffer;
use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
use vulkano::descriptor_set::DescriptorSet;
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::layout::DescriptorSetLayout;
use vulkano::image::sampler::Sampler;
use vulkano::image::view::ImageView;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::pipeline::graphics::viewport::Viewport;

pub struct GameScene {
    pub world: Option<World>,
    pub chunk_meshes: HashMap<[i32; 3], Option<Subbuffer<[BlockVertex]>>>,
    block_uvs: HashMap<String, AtlasUV>,
}

impl Scene for GameScene {
    fn start(&mut self) {
        println!("Started Game Scene");
    }

    fn update(&mut self, delta_time: f32, input_state: &mut InputState, camera: &mut Camera) {
        camera.update(delta_time, input_state);
        let camera_pos = camera.position;
        let current_chunk_pos: [i32; 3] = [
            (camera_pos[0] as i32).div_euclid(CHUNK_SIZE as i32),
            (camera_pos[1] as i32).div_euclid(CHUNK_SIZE as i32),
            (camera_pos[2] as i32).div_euclid(CHUNK_SIZE as i32),
        ];
        self.world.as_mut().unwrap().load_chunk(current_chunk_pos);
        if !self.chunk_meshes.contains_key(&current_chunk_pos) {
            let block_uvs = self.block_uvs.clone();
            let chunk = self
                .world
                .as_mut()
                .unwrap()
                .get_chunk(current_chunk_pos, &GameContent::get())
                .unwrap();
            let allocator = self
                .world
                .as_ref()
                .unwrap()
                .memory_allocator
                .clone()
                .unwrap();
            let out_chunk_mesh = chunk_mesh::build_chunk_mesh(allocator, &chunk, &block_uvs);
            self.chunk_meshes.insert(current_chunk_pos, out_chunk_mesh);
        };
    }

    fn fixed_update(
        &mut self,
        _delta_time: f32,
        _input_state: &mut InputState,
        _camera: &mut Camera,
    ) {
    }

    fn late_update(
        &mut self,
        _delta_time: f32,
        _input_state: &mut InputState,
        _camera: &mut Camera,
    ) {
    }

    fn draw(
        &self,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        _viewport: &Viewport,
        _allocator: &StandardMemoryAllocator,
        resources: &SceneResources,
    ) {
        let Some(_world) = &self.world else {
            return;
        };
        let pipeline = &resources.default_pipeline;

        for (_pos, mesh) in &self.chunk_meshes {
            if let Some(mesh) = mesh {
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
        make_mvp_descriptor_set(
            memory_allocator,
            descriptor_set_allocator,
            layout,
            camera,
            texture_view,
            sampler,
        )
    }

    fn inject_resources(&mut self, res: &SceneResources) {
        if self.world.is_none() {
            let mut world = World::new();
            world.set_allocator(res.memory_allocator.clone());

            self.block_uvs = res.block_uvs.clone().unwrap();
            self.world = Some(world);
        }
        self.init_world(&self.block_uvs.clone());
    }
}

impl GameScene {
    pub fn new() -> Self {
        Self {
            world: None,
            chunk_meshes: HashMap::new(),
            block_uvs: HashMap::new(), // is overwritten instead of added to
        }
    }

    pub fn init_world(&mut self, block_uvs: &HashMap<String, AtlasUV>) {
        self.build_all_loaded_chunks(block_uvs);
    }

    pub fn build_all_loaded_chunks(&mut self, block_uvs: &HashMap<String, AtlasUV>) {
        self.world.as_mut().unwrap().init();
        let allocator = self
            .world
            .as_ref()
            .unwrap()
            .memory_allocator
            .clone()
            .unwrap();
        for (chunk_pos, chunk) in &self.world.as_ref().unwrap().loaded_chunks {
            let out_chunk_mesh =
                chunk_mesh::build_chunk_mesh(allocator.clone(), &chunk, &block_uvs);
            self.chunk_meshes.insert(*chunk_pos, out_chunk_mesh);
        }
    }

    fn load_chunk_mesh(&mut self, res: &SceneResources, chunk: Arc<Chunk>) {
        let world = self.world.as_mut().unwrap();
        let allocator = world.memory_allocator.clone().unwrap();

        let out_chunk_mesh = chunk_mesh::build_chunk_mesh(allocator, &chunk, &self.block_uvs);
        self.chunk_meshes.insert(chunk.position, out_chunk_mesh);
    }
}
