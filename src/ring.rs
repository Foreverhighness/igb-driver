use dma_api::{DVec, Direction};

use crate::{descriptor::Descriptor, err::IgbError, regs::Reg};

pub const DEFAULT_RING_SIZE: usize = 256;

pub struct Ring<D: Descriptor> {
    pub descriptors: DVec<D>,
    reg: Reg,
}

impl<D: Descriptor> Ring<D> {
    pub fn new(reg: Reg, size: usize) -> Result<Self, IgbError> {
        let descriptors =
            DVec::zeros(size, 4096, Direction::Bidirectional).ok_or(IgbError::NoMemory)?;

        Ok(Self { descriptors, reg })
    }


    pub fn init(&mut self){
        
    }
}
