use crate::{
    descriptor::{AdvRxDesc, AdvTxDesc},
    err::IgbError,
    phy::Phy,
    regs::{Reg, CTRL, CTRL_EXT, RCTL, RXPBS, STATUS, SWPBS, TCTL, TXPBS},
    ring::{Ring, DEFAULT_RING_SIZE},
};
use core::{ptr::NonNull, time::Duration};

type Result<T> = core::result::Result<T, IgbError>;

const RECV_BATCH_SIZE: usize = 64;
const RX_BUFFER_SIZE: usize = 1024;
const MEM_POOL: usize = 4096;
const MEM_POOL_ENTRY_SIZE: usize = 2048;

pub struct IgbDevice {
    reg: Reg,
    tx_ring: Ring<AdvTxDesc>,
    rx_ring: Ring<AdvRxDesc>,
    phy: Phy,
}

unsafe impl Send for IgbDevice {}
unsafe impl Sync for IgbDevice {}

impl IgbDevice {
    pub fn new(bar0: NonNull<u8>) -> Result<Self> {
        let reg = Reg::new(bar0);
        let tx_ring = Ring::new(reg, DEFAULT_RING_SIZE)?;
        let rx_ring = Ring::new(reg, DEFAULT_RING_SIZE)?;

        Ok(Self {
            reg,
            tx_ring,
            rx_ring,
            phy: Phy::new(reg),
        })
    }

    // 4.5.3
    pub fn open(&mut self) -> Result<()> {
        // 4.5.4 Interrupts During Initialization
        self.reg.disable_interrupts();
        self.reg.write_reg(CTRL::RST);
        self.reg.wait_for(
            |reg: CTRL| Ok(!reg.contains(CTRL::RST)),
            Duration::from_millis(1),
            Some(1000),
        )?;
        // 4.5.4 disable interrupts again after reset
        self.reg.disable_interrupts();

        self.reg
            .modify_reg(|reg: CTRL_EXT| CTRL_EXT::DRV_LOAD | reg);

        log::info!("Mac address: {:02X?}", self.reg.read_mac());

        // 4.5.5 Global Reset and General Configuration
        self.global_reset_and_general_configuration()?;

        // 4.5.6 Flow Control Setup (skipped)
        // 4.5.7 Link Setup Mechanisms and Control/Status Bit Summary
        self.setup_phy_and_the_link()?;

        log::info!("status: {:?}", self.status());

        self.init_stat();

        self.init_rx();

        self.init_tx();

        self.enable_interrupts();

        self.reg
            .write_reg(CTRL::SLU | CTRL::FD | CTRL::SPD_1000 | CTRL::FRCDPX | CTRL::FRCSPD);

        Ok(())
    }

    /// 4.5.8 and 8.19
    fn init_stat(&mut self) {
        // CRC Error Count - CRCERRS (0x04000; RC)
        const STAT_REG_FIRST: u32 = 0x04000;
        // Switch Drop Packet Count - SDPC (0x41A4; RC)
        const STAT_REG_LAST: u32 = 0x041A4;

        log::debug!("Initialization of Statistics");
        for reg in (STAT_REG_FIRST..=STAT_REG_LAST).step_by(4) {
            self.reg.read_32(reg);
        }
    }

    /// 4.5.9 Receive Initialization
    fn init_rx(&mut self) {
        // disable rx when configing.
        self.reg.write_reg(RCTL::empty());

        self.rx_ring.init();

        self.reg.write_reg(RCTL::RXEN | RCTL::SZ_4096);
    }

    fn init_tx(&mut self) {
        self.reg.write_reg(TCTL::empty());

        self.tx_ring.init();

        self.reg.write_reg(TCTL::EN);
    }

    fn setup_phy_and_the_link(&mut self) -> Result<()> {
        log::debug!("Setup the PHY and the link");
        self.phy.power_up()?;
        Ok(())
    }

    pub fn mac(&self) -> [u8; 6] {
        self.reg.read_mac()
    }

    fn enable_interrupts(&self) {
        //TODO
    }

    pub fn status(&self) -> IgbStatus {
        let raw = self.reg.read_reg::<STATUS>();
        let speed_raw = (raw.bits() >> 6) & 0b11;

        IgbStatus {
            link_up: raw.contains(STATUS::LU),
            speed: match speed_raw {
                0 => Speed::Mb10,
                1 => Speed::Mb100,
                0b10 => Speed::Mb1000,
                _ => Speed::Mb1000,
            },
            full_duplex: raw.contains(STATUS::FD),
            phy_reset_asserted: raw.contains(STATUS::PHYRA),
        }
    }

    /// 4.5.5
    fn global_reset_and_general_configuration(&mut self) -> Result<()> {
        log::debug!("Global Reset and General Configuration");

        log::trace!("Set rst with 1, and ILOS with 0");

        log::trace!(
            "Set the packet buffer allocation for transmit and receive flows with default value"
        );
        self.reg.write_32(RXPBS, 64);
        self.reg.write_32(TXPBS, 40);
        self.reg.write_32(SWPBS, 20);

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct IgbStatus {
    pub full_duplex: bool,
    pub link_up: bool,
    pub speed: Speed,
    pub phy_reset_asserted: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Speed {
    Mb10,
    Mb100,
    Mb1000,
}
