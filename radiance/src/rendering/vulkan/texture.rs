use super::{
    adhoc_command_runner::AdhocCommandRunner, buffer::Buffer, device::Device, image::Image,
    image_view::ImageView, sampler::Sampler,
};
use crate::rendering::texture::{Texture, TextureDef};
use ash::vk;
use lru::LruCache;
use std::error::Error;
use std::rc::Rc;

pub struct VulkanTexture {
    image: Image,
    image_view: ImageView,
    sampler: Sampler,
}

impl Texture for VulkanTexture {
    fn width(&self) -> u32 {
        self.image.width()
    }

    fn height(&self) -> u32 {
        self.image.height()
    }
}

impl VulkanTexture {
    pub fn new(
        def: &TextureDef,
        device: &Rc<Device>,
        allocator: &Rc<vk_mem::Allocator>,
        command_runner: &Rc<AdhocCommandRunner>,
    ) -> Result<Self, Box<dyn Error>> {
        let texture_missing =
            image::load_from_memory(radiance_assets::TEXTURE_MISSING_TEXTURE_FILE)
                .unwrap()
                .to_rgba8();
        let rgba_image = def.image().unwrap_or_else(|| &texture_missing);

        let buffer = Buffer::new_staging_buffer_with_data(allocator, &rgba_image)?;
        let format = vk::Format::R8G8B8A8_UNORM;
        let mut image = Image::new_color_image(allocator, rgba_image.width(), rgba_image.height())?;
        image.transit_layout(
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &command_runner,
        )?;
        image.copy_from(&buffer, 0, &command_runner)?;
        image.transit_layout(
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            &command_runner,
        )?;

        let image_view = ImageView::new_color_image_view(device.clone(), image.vk_image(), format)?;
        let sampler = Sampler::new(device.clone())?;

        Ok(Self {
            image,
            image_view,
            sampler,
        })
    }

    pub fn from_buffer(
        image_buffer: &[u8],
        row_length: u32,
        width: u32,
        height: u32,
        device: &Rc<Device>,
        allocator: &Rc<vk_mem::Allocator>,
        command_runner: &Rc<AdhocCommandRunner>,
    ) -> Result<Self, Box<dyn Error>> {
        let buffer = Buffer::new_staging_buffer_with_data(allocator, &image_buffer)?;
        let format = vk::Format::R8G8B8A8_UNORM;
        let mut image = Image::new_color_image(allocator, width, height)?;
        image.transit_layout(
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &command_runner,
        )?;
        image.copy_from(&buffer, row_length, &command_runner)?;
        image.transit_layout(
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            &command_runner,
        )?;

        let image_view = ImageView::new_color_image_view(device.clone(), image.vk_image(), format)?;
        let sampler = Sampler::new(device.clone())?;

        Ok(Self {
            image,
            image_view,
            sampler,
        })
    }

    pub fn image(&self) -> &Image {
        &self.image
    }

    pub fn image_view(&self) -> &ImageView {
        &self.image_view
    }

    pub fn sampler(&self) -> &Sampler {
        &self.sampler
    }
}

pub struct VulkanTextureStore {
    store: LruCache<String, Rc<VulkanTexture>>,
}

impl VulkanTextureStore {
    pub fn new() -> Self {
        Self {
            store: LruCache::new(10000),
        }
    }

    pub fn get_or_update(
        &mut self,
        name: &str,
        update: impl FnOnce() -> VulkanTexture,
    ) -> Rc<VulkanTexture> {
        if let Some(t) = self.store.get(name) {
            t.clone()
        } else {
            let t = Rc::new(update());
            self.store.put(name.to_string(), t.clone());
            t
        }
    }
}
