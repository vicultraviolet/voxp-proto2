use futures::executor::block_on;
use wgpu::{Adapter, Backends, DeviceDescriptor, ExperimentalFeatures, Features, Instance, InstanceDescriptor, Limits, MemoryHints, PowerPreference, Queue, RequestAdapterOptions, Trace};

use crate::engine::window::Window;

pub struct Device {
    instance: Instance,
    adapter: Adapter,
    device: wgpu::Device,
    queue: Queue
}

impl Device {
    pub fn new(window: &Window) -> Self {
        let instance_descriptor = InstanceDescriptor {
            backends: Backends::VULKAN,
            ..Default::default()
        };
        let instance = Instance::new(&instance_descriptor);

        let surface = instance
            .create_surface(window.glfw_window())
            .expect("Failed to create surface!");

        let adapter_options = RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };

        let adapter = instance.request_adapter(&adapter_options);
        let adapter = block_on(adapter).expect("Failed to initialize adapter!");

        let device_descriptor = DeviceDescriptor {
            label: None,
            required_features: Features::empty(),
            experimental_features: ExperimentalFeatures::disabled(),
            required_limits: Limits::default(),
            memory_hints: MemoryHints::Performance,
            trace: Trace::Off,
        };

        let device = adapter.request_device(&device_descriptor);
        let (device, queue) = block_on(device).expect("Failed to create device!");

        Self{
            instance,
            adapter,
            device,
            queue
        }
    }

    pub(super) fn instance(&self) -> &Instance { &self.instance }
    pub(super) fn adapter(&self) -> &Adapter { &self.adapter }
    pub(super) fn device(&self) -> &wgpu::Device { &self.device }
    pub(super) fn queue(&self) -> &Queue { &self.queue }
}
