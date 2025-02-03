pub mod config;
mod error;
mod model;
mod operation;

use cubecl::prelude::*;
use cubecl::reduce::Reduce;
use cubecl::server::Handle;
use cubecl::{CubeElement, Runtime};
use ndarray::{Data, Dimension};

#[derive(Debug, Clone)]
pub struct MetaData {
    pub stride: Box<[usize]>,
    pub shape: Box<[usize]>,
}

impl MetaData {
    pub fn build(stride: Box<[usize]>, shape: Box<[usize]>) -> MetaData {
        MetaData { stride, shape }
    }
    pub fn handle_empty<R: Runtime>(&self) -> (&Self, Handle) {
        let client = R::client(&Default::default());
        let size_f32 = std::mem::size_of::<f32>();
        let size = &self.shape.iter().product::<usize>() * size_f32;
        let handle = client.empty(size);
        (self, handle)
    }
    pub fn handle_from_vec<R: Runtime>(&self, input: Vec<f32>) -> (&Self, Handle) {
        let client = R::client(&Default::default());
        let put = input.as_slice();
        let handle = client.create(f32::as_bytes(put));
        (self, handle)
    }
}
