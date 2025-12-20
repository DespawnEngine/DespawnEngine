use crate::content::world;
use crate::content::world::chunks::chunk::{self, BLOCKS_IN_CHUNK, CHUNK_SIZE, Chunk, ChunkCoords};
use crate::content::world::chunks::chunk_mesh::{self, ChunkMesh};
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
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;
use vulkano::buffer::allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo};
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer, subbuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
use vulkano::descriptor_set::DescriptorSet;
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::layout::DescriptorSetLayout;
use vulkano::device::DeviceOwned;
use vulkano::image::sampler::Sampler;
use vulkano::image::view::ImageView;
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::pipeline::graphics::input_assembly::PrimitiveTopology;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::{DeviceSize, pipeline};

pub struct GameScene {
    pub world: Option<World>,
    pub chunk_meshes: RapidHashMap<ChunkCoords, Option<ChunkMesh>>,
    chunk_vertex_buffers: RapidHashMap<ChunkCoords, Subbuffer<[BlockVertex]>>,

    chunk_vertex_buffer_allocator: Option<SubbufferAllocator>,
    max_buffer_size: Option<DeviceSize>,

    block_uvs: RapidHashMap<String, AtlasUV>,
    current_chunk_pos: Option<ChunkCoords>,
}

impl Scene for GameScene {
    fn start(&mut self) {
        println!("Started Game Scene");
    }

    fn update(&mut self, delta_time: f32, input_state: &mut InputState, camera: &mut Camera) {
        camera.update(delta_time, input_state);
        let camera_pos = camera.position;
        let current_chunk_pos: ChunkCoords = [
            (camera_pos[0] as i32).div_euclid(CHUNK_SIZE as i32),
            (camera_pos[1] as i32).div_euclid(CHUNK_SIZE as i32),
            (camera_pos[2] as i32).div_euclid(CHUNK_SIZE as i32),
        ];

        // self.current_chunk_pos is unupdated at this point, so really its "last_chunk_pos"
        if Some(current_chunk_pos) != self.current_chunk_pos {
            let visible_chunks = GameScene::get_all_chunk_pos_in_render(camera);
            let visible_set: HashSet<ChunkCoords> = visible_chunks.iter().cloned().collect();

            let world = self.world.as_mut().unwrap();
            let allocator = world.memory_allocator.clone().unwrap();

            for &chunk_pos in &visible_chunks {
                if !world.loaded_chunks.contains_key(&chunk_pos) {
                    world.load_chunk(chunk_pos);

                    let chunk = world.loaded_chunks.get(&chunk_pos).unwrap();
                    let mesh =
                        chunk_mesh::build_chunk_mesh(allocator.clone(), chunk, &self.block_uvs);
                    self.chunk_meshes.insert(chunk_pos, mesh);
                }
            }

            let existing_loaded: Vec<ChunkCoords> = world.loaded_chunks.keys().cloned().collect();
            for chunk_pos in existing_loaded {
                if !visible_set.contains(&chunk_pos) {
                    world.unload_chunk(chunk_pos);
                    self.chunk_meshes.remove(&chunk_pos);
                }
            }

            self.current_chunk_pos = Some(current_chunk_pos);

            self.update_chunk_vertex_buffers();
        }
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

        let max_vertex_input_bindings = pipeline
            .device()
            .physical_device()
            .properties()
            .max_vertex_input_bindings;

        let vtx_buffers: Vec<Subbuffer<[BlockVertex]>> =
            self.chunk_vertex_buffers.values().cloned().collect();

        println!(
            "there are {} bufs, chunked into chunks of {max_vertex_input_bindings}",
            vtx_buffers.iter().len()
        );

        if !vtx_buffers.is_empty() {
            for buffer in vtx_buffers {
                let buffer_vtx_count = buffer.len();

                builder
                    .bind_pipeline_graphics(pipeline.clone())
                    .unwrap()
                    .bind_vertex_buffers(0, buffer)
                    .unwrap();

                unsafe {
                    builder.draw(buffer_vtx_count as u32, 1, 0, 0).unwrap();
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
        println!("injecting resources into game scene");

        if self.chunk_vertex_buffer_allocator.is_none() {
            self.chunk_vertex_buffer_allocator = Some(SubbufferAllocator::new(
                res.memory_allocator.clone(),
                SubbufferAllocatorCreateInfo {
                    buffer_usage: BufferUsage::VERTEX_BUFFER,
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                        | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
            ));
        }
        if self.max_buffer_size.is_none() {
            self.max_buffer_size = res
                .memory_allocator
                .clone()
                .device()
                .physical_device()
                .properties()
                .max_buffer_size;
        }

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
            chunk_vertex_buffer_allocator: None,
            max_buffer_size: None,
            chunk_meshes: RapidHashMap::default(),
            block_uvs: RapidHashMap::default(), // is overwritten instead of added to
            chunk_vertex_buffers: RapidHashMap::default(),
            current_chunk_pos: None,
        }
    }

    fn update_chunk_vertex_buffers(&mut self) {
        // let horizontal_render_distance = UserSettings::instance().horizontal_render_distance;
        // let vertical_render_distance = UserSettings::instance().vertical_render_distance;

        self.chunk_vertex_buffers.clear();

        let subbuffer_allocator = self.chunk_vertex_buffer_allocator.as_mut().unwrap();

        let chunk_meshes = self
            .chunk_meshes
            .iter()
            .filter(|(_chunk_pos, chunk_mesh)| chunk_mesh.is_some());

        let start_time = Instant::now();

        let current_chunk_pos = self
            .current_chunk_pos
            .expect("Not currently in a chunk? What!");

        let mut temp_final_chunk_pos = None;

        let mut verticies: Vec<BlockVertex> = vec![];

        // dumb asf that i gotta put an `if let` inside but eh
        for (chunk_pos, chunk_mesh) in chunk_meshes {
            if let Some(chunk_mesh) = chunk_mesh {
                temp_final_chunk_pos = Some(chunk_pos);
                let mut chunk_verticies: Vec<BlockVertex> = vec![];

                if current_chunk_pos[0] >= chunk_pos[0] {
                    chunk_verticies.append(&mut chunk_mesh.x_pos.clone());
                }
                if current_chunk_pos[0] <= chunk_pos[0] {
                    chunk_verticies.append(&mut chunk_mesh.x_neg.clone());
                }

                if current_chunk_pos[1] >= chunk_pos[1] {
                    chunk_verticies.append(&mut chunk_mesh.y_pos.clone());
                }
                if current_chunk_pos[1] >= chunk_pos[1] {
                    chunk_verticies.append(&mut chunk_mesh.y_neg.clone());
                }

                if current_chunk_pos[2] >= chunk_pos[2] {
                    chunk_verticies.append(&mut chunk_mesh.z_pos.clone());
                }
                if current_chunk_pos[2] >= chunk_pos[2] {
                    chunk_verticies.append(&mut chunk_mesh.z_neg.clone());
                }

                // println!(
                //     "current size of {}, max of {}",
                //     (chunk_verticies.len() + verticies.len()) * 3 * size_of::<BlockVertex>(),
                //     self.max_buffer_size.unwrap()
                // );

                if ((chunk_verticies.len() + verticies.len()) * 300 * size_of::<BlockVertex>())
                    < self.max_buffer_size.unwrap() as usize
                {
                    verticies.append(&mut chunk_verticies);
                } else {
                    let buff_size = (verticies.len() * size_of::<BlockVertex>()) as u64;

                    let subbuffer: Subbuffer<[BlockVertex]> =
                        subbuffer_allocator.allocate_slice(buff_size).unwrap();

                    // copied
                    // https://docs.rs/vulkano/latest/src/vulkano/buffer/mod.rs.html#268-296
                    // because man this is giving me a headache
                    let mut write_guard = subbuffer.write().unwrap();

                    for (o, i) in write_guard.iter_mut().zip(verticies) {
                        *o = i;
                    }
                    verticies = chunk_verticies;

                    self.chunk_vertex_buffers
                        .insert(*chunk_pos, subbuffer.clone());
                }
            }
        }
        if let Some(last_chunk_pos) = temp_final_chunk_pos {
            let buff_size = (verticies.len() * size_of::<BlockVertex>()) as u64;

            let subbuffer: Subbuffer<[BlockVertex]> =
                subbuffer_allocator.allocate_slice(buff_size).unwrap();

            // copied
            // https://docs.rs/vulkano/latest/src/vulkano/buffer/mod.rs.html#268-296
            // because man this is giving me a headache
            let mut write_guard = subbuffer.write().unwrap();

            for (o, i) in write_guard.iter_mut().zip(verticies) {
                *o = i;
            }

            self.chunk_vertex_buffers
                .insert(*last_chunk_pos, subbuffer.clone());
        }
        println!(
            "It took {:?} to update the vtx buffers",
            start_time.elapsed()
        );
    }

    // there has got to be a better way to find if a point on a grid is within a radius of
    // another point on a grid (hori)
    fn get_all_chunk_pos_in_render(camera: &Camera) -> Vec<ChunkCoords> {
        let start_time = Instant::now();
        let settings = UserSettings::instance();
        let vert_dist = settings.vertical_render_distance;
        let hori_dist = settings.horizontal_render_distance;

        let camera_pos = camera.position;
        let current_chunk_pos: ChunkCoords = [
            (camera_pos[0] as i32).div_euclid(CHUNK_SIZE as i32),
            (camera_pos[1] as i32).div_euclid(CHUNK_SIZE as i32),
            (camera_pos[2] as i32).div_euclid(CHUNK_SIZE as i32),
        ];

        let is_in_vert = |chunk_pos: ChunkCoords| -> bool {
            (chunk_pos[1]).abs_diff(current_chunk_pos[1]) <= vert_dist
        };
        let is_in_hori = |chunk_pos: ChunkCoords| -> bool {
            let abs_x_diff = (chunk_pos[0]).abs_diff(current_chunk_pos[0]);
            let abs_z_diff = (chunk_pos[2]).abs_diff(current_chunk_pos[2]);

            (abs_x_diff.pow(2) + abs_z_diff.pow(2)) <= hori_dist.pow(2)
        };

        let is_in_render =
            |chunk_pos: ChunkCoords| -> bool { is_in_vert(chunk_pos) && is_in_hori(chunk_pos) };

        let max_chunk_x = current_chunk_pos[0] + hori_dist as i32;
        let min_chunk_x = current_chunk_pos[0] - hori_dist as i32;

        let max_chunk_z = current_chunk_pos[2] + hori_dist as i32;
        let min_chunk_z = current_chunk_pos[2] - hori_dist as i32;

        let max_chunk_y = current_chunk_pos[1] + vert_dist as i32;
        let min_chunk_y = current_chunk_pos[1] - vert_dist as i32;

        let mut out_chunk_pos: Vec<ChunkCoords> = vec![];
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
        let _world = self.world.as_mut().unwrap();

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
            let out_chunk_mesh = chunk_mesh::build_chunk_mesh(allocator.clone(), chunk, block_uvs);
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
    pub fn amount_of_cached_chunk_meshes(&self) -> usize {
        self.chunk_meshes.values().flatten().count()
    }
}
