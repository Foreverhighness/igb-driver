use axdriver_net::DevResult;

const RECV_BATCH_SIZE: usize = 64;
const RX_BUFFER_SIZE: usize = 1024;
const MEM_POOL: usize = 4096;
const MEM_POOL_ENTRY_SIZE: usize = 2048;

pub struct IgbDevice {}

impl IgbDevice {
    /// Creates a net igb NIC instance and initialize, or returns a error if
    /// any step fails.
    pub fn init(base: usize, len: usize) -> DevResult<Self> {
        Ok(Self {})
    }
}
