use educe::Educe;
use std::ops::Deref;
use wgpu::util::DeviceExt;

#[derive(Debug, Educe)]
#[educe(Deref, DerefMut)]
pub struct SimpleBuffer {
    #[educe(Deref, DerefMut)]
    pub buffer: wgpu::Buffer,
    size: u64,
    usage: wgpu::BufferUsages,
}

impl SimpleBuffer {
    pub fn new<D>(device: D, size: u64, usage: wgpu::BufferUsages) -> Self
    where
        D: Deref<Target = wgpu::Device>,
    {
        let usage = usage;
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage,
            mapped_at_creation: false,
        });

        Self {
            buffer,
            size,
            usage,
        }
    }

    pub fn from_bytes<B, D>(device: D, bytes: B, usage: wgpu::BufferUsages) -> Self
    where
        B: Deref<Target = [u8]>,
        D: Deref<Target = wgpu::Device>,
    {
        let usage = usage;
        let size = bytes.len() as u64;
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &bytes,
            usage,
        });
        Self {
            buffer,
            size,
            usage,
        }
    }

    pub fn upload<B, Q>(&self, queue: Q, data: B, offset: wgpu::BufferAddress)
    where
        B: Deref<Target = [u8]>,
        Q: Deref<Target = wgpu::Queue>,
    {
        queue.write_buffer(&self.buffer, offset, &data)
    }

    pub fn size(&self) -> u64 {
        self.size
    }
    pub fn usage(&self) -> wgpu::BufferUsages {
        self.usage
    }
}
