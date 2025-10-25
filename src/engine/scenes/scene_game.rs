use crate::content::world;
use crate::content::world::chunks::chunk::{self, CHUNK_SIZE, Chunk};
use crate::content::world::chunks::chunk_mesh;
use crate::content::world::world::World;
use crate::engine::core::content_loader::GameContent;
use crate::engine::core::input::InputState;
use crate::engine::core::user_settings::UserSettings;
use crate::engine::rendering::camera::Camera;
use crate::engine::rendering::descriptor_helpers::make_mvp_descriptor_set;
use crate::engine::rendering::texture_atlas::AtlasUV;
use crate::engine::rendering::vertex::BlockVertex;
use crate::engine::scenes::handling::scene_trait::{Scene, SceneResources};
use rapidhash::RapidHashMap;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
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
    pub chunk_meshes: RapidHashMap<[i32; 3], Option<Subbuffer<[BlockVertex]>>>,
    block_uvs: RapidHashMap<String, AtlasUV>,
    last_chunk_pos: Option<[i32; 3]>,
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

        if Some(current_chunk_pos) != self.last_chunk_pos {
            for chunk_pos in GameScene::get_all_chunk_pos_in_render(camera) {
                self.world.as_mut().unwrap().load_chunk(chunk_pos);
            }
        }

        self.last_chunk_pos = Some(current_chunk_pos);

        // println!(
        //     "{} chunks with meshes",
        // );

        self.build_all_unbuild_loaded_chunks(&self.block_uvs.clone());
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

        for mesh in self.chunk_meshes.values().flatten() {
            // using .flatten() removes all "None" from iter
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
            chunk_meshes: RapidHashMap::default(),
            block_uvs: RapidHashMap::default(), // is overwritten instead of added to
            last_chunk_pos: None,
        }
    }

    // there has got to be a better way to find if a point on a grid is within a radius of
    // another point on a grid (hori)
    fn get_all_chunk_pos_in_render(camera: &Camera) -> Vec<[i32; 3]> {
        let start_time = Instant::now();
        let settings = UserSettings::instance();
        let vert_dist = settings.vertical_render_distance;
        let hori_dist = settings.horizontal_render_distance;

        let camera_pos = camera.position;
        let current_chunk_pos: [i32; 3] = [
            (camera_pos[0] as i32).div_euclid(CHUNK_SIZE as i32),
            (camera_pos[1] as i32).div_euclid(CHUNK_SIZE as i32),
            (camera_pos[2] as i32).div_euclid(CHUNK_SIZE as i32),
        ];

        let is_in_vert = |chunk_pos: [i32; 3]| -> bool {
            (chunk_pos[1]).abs_diff(current_chunk_pos[1]) <= vert_dist
        };
        let is_in_hori = |chunk_pos: [i32; 3]| -> bool {
            let abs_x_diff = (chunk_pos[0]).abs_diff(current_chunk_pos[0]);
            let abs_z_diff = (chunk_pos[2]).abs_diff(current_chunk_pos[2]);

            (abs_x_diff.pow(2) + abs_z_diff.pow(2)) <= hori_dist.pow(2)
        };

        let is_in_render =
            |chunk_pos: [i32; 3]| -> bool { is_in_vert(chunk_pos) && is_in_hori(chunk_pos) };

        let max_chunk_x = current_chunk_pos[0] + hori_dist as i32;
        let min_chunk_x = current_chunk_pos[0] - hori_dist as i32;

        let max_chunk_z = current_chunk_pos[2] + hori_dist as i32;
        let min_chunk_z = current_chunk_pos[2] - hori_dist as i32;

        let max_chunk_y = current_chunk_pos[1] + vert_dist as i32;
        let min_chunk_y = current_chunk_pos[1] - vert_dist as i32;

        let mut out_chunk_pos: Vec<[i32; 3]> = vec![];
        for chunk_x_pos in min_chunk_x - 1..max_chunk_x + 1 {
            for chunk_y_pos in min_chunk_y - 1..max_chunk_y + 1 {
                for chunk_z_pos in min_chunk_z - 1..max_chunk_z + 1 {
                    let chunk_pos = [chunk_x_pos, chunk_y_pos, chunk_z_pos];
                    if is_in_render(chunk_pos) {
                        out_chunk_pos.push(chunk_pos);
                    }
                }
            }
        }
        println!(
            "took {:?} to get all chunks within radii {}, {}",
            start_time.elapsed(),
            hori_dist,
            vert_dist
        );
        out_chunk_pos
    }

    pub fn init_world(&mut self, block_uvs: &RapidHashMap<String, AtlasUV>) {
        let world = self.world.as_mut().unwrap();

        self.build_all_loaded_chunks(block_uvs);
    }

    pub fn build_all_loaded_chunks(&mut self, block_uvs: &RapidHashMap<String, AtlasUV>) {
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

    pub fn build_all_unbuild_loaded_chunks(&mut self, block_uvs: &RapidHashMap<String, AtlasUV>) {
        let allocator = self
            .world
            .as_ref()
            .unwrap()
            .memory_allocator
            .clone()
            .unwrap();
        for (chunk_pos, chunk) in &self.world.as_ref().unwrap().loaded_chunks {
            if self.chunk_meshes.contains_key(chunk_pos) {
                continue;
            }

            let out_chunk_mesh = chunk_mesh::build_chunk_mesh(allocator.clone(), chunk, block_uvs);
            self.chunk_meshes.insert(*chunk_pos, out_chunk_mesh);
        }
    }

    fn load_chunk_mesh(&mut self, _res: &SceneResources, chunk: Arc<Chunk>) {
        let world = self.world.as_mut().unwrap();
        let allocator = world.memory_allocator.clone().unwrap();

        let out_chunk_mesh = chunk_mesh::build_chunk_mesh(allocator, &chunk, &self.block_uvs);
        self.chunk_meshes.insert(chunk.position, out_chunk_mesh);
    }

    pub fn amount_of_chunk_meshes(&self) -> usize {
        self.chunk_meshes.values().flatten().count()
    }
}
