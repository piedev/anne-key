use stm32l151::USB;
use super::pma::PMA;
use usb::usb_ext::UsbEpExt;

pub struct UsbHid {
    pub report: [u8; 5],
    pub up: bool,
}

impl UsbHid {
    pub fn new() -> UsbHid {
        UsbHid {
            report: [0x01, 0x00, 0x00, 0x00, 0x00],
            up: false,
        }
    }

    pub fn ctr(&mut self, usb: &mut USB, pma: &mut PMA) {
        if !usb.istr.read().dir().bit_is_set() {
            pma.write_buffer_u8(0x100, &self.report);
            pma.pma_area.set_u16(10, self.report.len() as u16);
            usb.usb_ep1r.toggle_tx_out();
            self.up = true;
        } else {
            panic!()
        }
    }
}
