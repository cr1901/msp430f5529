#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    uca0ctl1_spi: UCA0CTL1_SPI,
    uca0ctl0_spi: UCA0CTL0_SPI,
    _reserved2: [u8; 0x04],
    uca0br0_spi: UCA0BR0_SPI,
    uca0br1_spi: UCA0BR1_SPI,
    uca0mctl_spi: UCA0MCTL_SPI,
    _reserved5: [u8; 0x01],
    uca0stat_spi: UCA0STAT_SPI,
    _reserved6: [u8; 0x01],
    uca0rxbuf_spi: UCA0RXBUF_SPI,
    _reserved7: [u8; 0x01],
    uca0txbuf_spi: UCA0TXBUF_SPI,
    _reserved8: [u8; 0x0d],
    uca0ie_spi: UCA0IE_SPI,
    uca0ifg_spi: UCA0IFG_SPI,
    uca0iv_spi: UCA0IV_SPI,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI A0 Control Register 1"]
    #[inline(always)]
    pub const fn uca0ctl1_spi(&self) -> &UCA0CTL1_SPI {
        &self.uca0ctl1_spi
    }
    #[doc = "0x01 - USCI A0 Control Register 0"]
    #[inline(always)]
    pub const fn uca0ctl0_spi(&self) -> &UCA0CTL0_SPI {
        &self.uca0ctl0_spi
    }
    #[doc = "0x06 - USCI A0 Baud Rate 0"]
    #[inline(always)]
    pub const fn uca0br0_spi(&self) -> &UCA0BR0_SPI {
        &self.uca0br0_spi
    }
    #[doc = "0x07 - USCI A0 Baud Rate 1"]
    #[inline(always)]
    pub const fn uca0br1_spi(&self) -> &UCA0BR1_SPI {
        &self.uca0br1_spi
    }
    #[doc = "0x08 - USCI A0 Modulation Control"]
    #[inline(always)]
    pub const fn uca0mctl_spi(&self) -> &UCA0MCTL_SPI {
        &self.uca0mctl_spi
    }
    #[doc = "0x0a - USCI A0 Status Register"]
    #[inline(always)]
    pub const fn uca0stat_spi(&self) -> &UCA0STAT_SPI {
        &self.uca0stat_spi
    }
    #[doc = "0x0c - USCI A0 Receive Buffer"]
    #[inline(always)]
    pub const fn uca0rxbuf_spi(&self) -> &UCA0RXBUF_SPI {
        &self.uca0rxbuf_spi
    }
    #[doc = "0x0e - USCI A0 Transmit Buffer"]
    #[inline(always)]
    pub const fn uca0txbuf_spi(&self) -> &UCA0TXBUF_SPI {
        &self.uca0txbuf_spi
    }
    #[doc = "0x1c - USCI A0 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn uca0ie_spi(&self) -> &UCA0IE_SPI {
        &self.uca0ie_spi
    }
    #[doc = "0x1d - USCI A0 Interrupt Flags Register"]
    #[inline(always)]
    pub const fn uca0ifg_spi(&self) -> &UCA0IFG_SPI {
        &self.uca0ifg_spi
    }
    #[doc = "0x1e - USCI A0 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn uca0iv_spi(&self) -> &UCA0IV_SPI {
        &self.uca0iv_spi
    }
}
#[doc = "UCA0CTL1_SPI (rw) register accessor: USCI A0 Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0ctl1_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0ctl1_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ctl1_spi`]
module"]
pub type UCA0CTL1_SPI = crate::Reg<uca0ctl1_spi::UCA0CTL1_SPI_SPEC>;
#[doc = "USCI A0 Control Register 1"]
pub mod uca0ctl1_spi;
#[doc = "UCA0CTL0_SPI (rw) register accessor: USCI A0 Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0ctl0_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0ctl0_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ctl0_spi`]
module"]
pub type UCA0CTL0_SPI = crate::Reg<uca0ctl0_spi::UCA0CTL0_SPI_SPEC>;
#[doc = "USCI A0 Control Register 0"]
pub mod uca0ctl0_spi;
#[doc = "UCA0BR0_SPI (rw) register accessor: USCI A0 Baud Rate 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0br0_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0br0_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0br0_spi`]
module"]
pub type UCA0BR0_SPI = crate::Reg<uca0br0_spi::UCA0BR0_SPI_SPEC>;
#[doc = "USCI A0 Baud Rate 0"]
pub mod uca0br0_spi;
#[doc = "UCA0BR1_SPI (rw) register accessor: USCI A0 Baud Rate 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0br1_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0br1_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0br1_spi`]
module"]
pub type UCA0BR1_SPI = crate::Reg<uca0br1_spi::UCA0BR1_SPI_SPEC>;
#[doc = "USCI A0 Baud Rate 1"]
pub mod uca0br1_spi;
#[doc = "UCA0MCTL_SPI (rw) register accessor: USCI A0 Modulation Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0mctl_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0mctl_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0mctl_spi`]
module"]
pub type UCA0MCTL_SPI = crate::Reg<uca0mctl_spi::UCA0MCTL_SPI_SPEC>;
#[doc = "USCI A0 Modulation Control"]
pub mod uca0mctl_spi;
#[doc = "UCA0STAT_SPI (rw) register accessor: USCI A0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0stat_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0stat_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0stat_spi`]
module"]
pub type UCA0STAT_SPI = crate::Reg<uca0stat_spi::UCA0STAT_SPI_SPEC>;
#[doc = "USCI A0 Status Register"]
pub mod uca0stat_spi;
#[doc = "UCA0RXBUF_SPI (rw) register accessor: USCI A0 Receive Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0rxbuf_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0rxbuf_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0rxbuf_spi`]
module"]
pub type UCA0RXBUF_SPI = crate::Reg<uca0rxbuf_spi::UCA0RXBUF_SPI_SPEC>;
#[doc = "USCI A0 Receive Buffer"]
pub mod uca0rxbuf_spi;
#[doc = "UCA0TXBUF_SPI (rw) register accessor: USCI A0 Transmit Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0txbuf_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0txbuf_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0txbuf_spi`]
module"]
pub type UCA0TXBUF_SPI = crate::Reg<uca0txbuf_spi::UCA0TXBUF_SPI_SPEC>;
#[doc = "USCI A0 Transmit Buffer"]
pub mod uca0txbuf_spi;
#[doc = "UCA0IE_SPI (rw) register accessor: USCI A0 Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0ie_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0ie_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ie_spi`]
module"]
pub type UCA0IE_SPI = crate::Reg<uca0ie_spi::UCA0IE_SPI_SPEC>;
#[doc = "USCI A0 Interrupt Enable Register"]
pub mod uca0ie_spi;
#[doc = "UCA0IFG_SPI (rw) register accessor: USCI A0 Interrupt Flags Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0ifg_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0ifg_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ifg_spi`]
module"]
pub type UCA0IFG_SPI = crate::Reg<uca0ifg_spi::UCA0IFG_SPI_SPEC>;
#[doc = "USCI A0 Interrupt Flags Register"]
pub mod uca0ifg_spi;
#[doc = "UCA0IV_SPI (rw) register accessor: USCI A0 Interrupt Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0iv_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0iv_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0iv_spi`]
module"]
pub type UCA0IV_SPI = crate::Reg<uca0iv_spi::UCA0IV_SPI_SPEC>;
#[doc = "USCI A0 Interrupt Vector Register"]
pub mod uca0iv_spi;
