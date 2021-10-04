use bytemuck::{Pod, Zeroable};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;
use ultraviolet::{Mat4, Vec4};
use wgpu_engine::*;

mod surface_manager;
pub use surface_manager::SurfaceManager;

mod display;
pub use display::Display;

mod surface;
pub use surface::*;

mod screen_task;
use screen_task::*;

mod shaders;
mod prepare_descriptors;

#[cfg(test)]
mod tests;

