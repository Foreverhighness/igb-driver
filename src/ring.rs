use core::time::Duration;

use dma_api::{DVec, Direction};

use crate::regs::{RXDCTL, TXDCTL};
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

    pub fn init_rx(&mut self) {
        const RDBAL: u32 = 0x0C000;
        const RDBAH: u32 = 0x0C004;
        const RDLEN: u32 = 0x0C008;
        const RDH: u32 = 0x0C010;
        const RDT: u32 = 0x0C018;

        let bus_addr = self.descriptors.bus_addr();
        let len = self.descriptors.len();

        self.reg.write_32(RDBAL, bus_addr as u32);
        self.reg.write_32(RDBAH, (bus_addr >> 32) as u32);
        self.reg.write_32(RDLEN, len as u32);
        self.reg.write_32(RDH, 0);
        self.reg.write_32(RDT, 0);

        self.reg.modify_reg(|reg: RXDCTL| RXDCTL::ENABLE | reg);

        self.reg
            .wait_for(
                |reg: RXDCTL| Ok(reg.contains(RXDCTL::ENABLE)),
                Duration::from_millis(1),
                Some(1000),
            )
            .unwrap();
    }

    pub(crate) fn init_tx(&self) {
        const TDBAL: u32 = 0x0E000;
        const TDBAH: u32 = 0x0E004;
        const TDLEN: u32 = 0x0E008;
        const TDH: u32 = 0x0E010;
        const TDT: u32 = 0x0E018;

        let bus_addr = self.descriptors.bus_addr();
        let len = self.descriptors.len();

        self.reg.write_32(TDBAL, bus_addr as u32);
        self.reg.write_32(TDBAH, (bus_addr >> 32) as u32);
        self.reg.write_32(TDLEN, len as u32);
        self.reg.write_32(TDH, 0);
        self.reg.write_32(TDT, 0);

        self.reg.modify_reg(|reg: TXDCTL| TXDCTL::ENABLE | reg);

        self.reg
            .wait_for(
                |reg: TXDCTL| Ok(reg.contains(TXDCTL::ENABLE)),
                Duration::from_millis(1),
                Some(1000),
            )
            .unwrap();
    }
}
