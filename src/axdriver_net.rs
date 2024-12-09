use axdriver_net::{
    BaseDriverOps, DevResult, DeviceType, EthernetAddress, NetBufPtr, NetDriverOps,
};

use crate::{IgbDevice, DEVICE_NAME};

impl BaseDriverOps for IgbDevice {
    fn device_name(&self) -> &str {
        DEVICE_NAME
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::Net
    }
}

impl NetDriverOps for IgbDevice {
    fn mac_address(&self) -> EthernetAddress {
        todo!()
    }

    fn can_transmit(&self) -> bool {
        todo!()
    }

    fn can_receive(&self) -> bool {
        todo!()
    }

    fn rx_queue_size(&self) -> usize {
        todo!()
    }

    fn tx_queue_size(&self) -> usize {
        todo!()
    }

    fn recycle_rx_buffer(&mut self, rx_buf: NetBufPtr) -> DevResult {
        todo!()
    }

    fn recycle_tx_buffers(&mut self) -> DevResult {
        todo!()
    }

    fn transmit(&mut self, tx_buf: NetBufPtr) -> DevResult {
        todo!()
    }

    fn receive(&mut self) -> DevResult<NetBufPtr> {
        todo!()
    }

    fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<NetBufPtr> {
        todo!()
    }
}
