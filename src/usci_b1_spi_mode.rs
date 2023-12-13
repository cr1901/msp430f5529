#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ucb1ctl1_spi: UCB1CTL1_SPI,
    ucb1ctl0_spi: UCB1CTL0_SPI,
    _reserved2: [u8; 0x04],
    ucb1br0_spi: UCB1BR0_SPI,
    ucb1br1_spi: UCB1BR1_SPI,
    _reserved4: [u8; 0x02],
    ucb1stat_spi: UCB1STAT_SPI,
    _reserved5: [u8; 0x01],
    ucb1rxbuf_spi: UCB1RXBUF_SPI,
    _reserved6: [u8; 0x01],
    ucb1txbuf_spi: UCB1TXBUF_SPI,
    _reserved7: [u8; 0x0d],
    ucb1ie_spi: UCB1IE_SPI,
    ucb1ifg_spi: UCB1IFG_SPI,
    ucb1iv_spi: UCB1IV_SPI,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI B1 Control Register 1"]
    #[inline(always)]
    pub const fn ucb1ctl1_spi(&self) -> &UCB1CTL1_SPI {
        &self.ucb1ctl1_spi
    }
    #[doc = "0x01 - USCI B1 Control Register 0"]
    #[inline(always)]
    pub const fn ucb1ctl0_spi(&self) -> &UCB1CTL0_SPI {
        &self.ucb1ctl0_spi
    }
    #[doc = "0x06 - USCI B1 Baud Rate 0"]
    #[inline(always)]
    pub const fn ucb1br0_spi(&self) -> &UCB1BR0_SPI {
        &self.ucb1br0_spi
    }
    #[doc = "0x07 - USCI B1 Baud Rate 1"]
    #[inline(always)]
    pub const fn ucb1br1_spi(&self) -> &UCB1BR1_SPI {
        &self.ucb1br1_spi
    }
    #[doc = "0x0a - USCI B1 Status Register"]
    #[inline(always)]
    pub const fn ucb1stat_spi(&self) -> &UCB1STAT_SPI {
        &self.ucb1stat_spi
    }
    #[doc = "0x0c - USCI B1 Receive Buffer"]
    #[inline(always)]
    pub const fn ucb1rxbuf_spi(&self) -> &UCB1RXBUF_SPI {
        &self.ucb1rxbuf_spi
    }
    #[doc = "0x0e - USCI B1 Transmit Buffer"]
    #[inline(always)]
    pub const fn ucb1txbuf_spi(&self) -> &UCB1TXBUF_SPI {
        &self.ucb1txbuf_spi
    }
    #[doc = "0x1c - USCI B1 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucb1ie_spi(&self) -> &UCB1IE_SPI {
        &self.ucb1ie_spi
    }
    #[doc = "0x1d - USCI B1 Interrupt Flags Register"]
    #[inline(always)]
    pub const fn ucb1ifg_spi(&self) -> &UCB1IFG_SPI {
        &self.ucb1ifg_spi
    }
    #[doc = "0x1e - USCI B1 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn ucb1iv_spi(&self) -> &UCB1IV_SPI {
        &self.ucb1iv_spi
    }
}
#[doc = "UCB1CTL1_SPI (rw) register accessor: USCI B1 Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1ctl1_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1ctl1_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1ctl1_spi`]
module"]
pub type UCB1CTL1_SPI = crate::Reg<ucb1ctl1_spi::UCB1CTL1_SPI_SPEC>;
#[doc = "USCI B1 Control Register 1"]
pub mod ucb1ctl1_spi;
#[doc = "UCB1CTL0_SPI (rw) register accessor: USCI B1 Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1ctl0_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1ctl0_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1ctl0_spi`]
module"]
pub type UCB1CTL0_SPI = crate::Reg<ucb1ctl0_spi::UCB1CTL0_SPI_SPEC>;
#[doc = "USCI B1 Control Register 0"]
pub mod ucb1ctl0_spi;
#[doc = "UCB1BR0_SPI (rw) register accessor: USCI B1 Baud Rate 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1br0_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1br0_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1br0_spi`]
module"]
pub type UCB1BR0_SPI = crate::Reg<ucb1br0_spi::UCB1BR0_SPI_SPEC>;
#[doc = "USCI B1 Baud Rate 0"]
pub mod ucb1br0_spi;
#[doc = "UCB1BR1_SPI (rw) register accessor: USCI B1 Baud Rate 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1br1_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1br1_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1br1_spi`]
module"]
pub type UCB1BR1_SPI = crate::Reg<ucb1br1_spi::UCB1BR1_SPI_SPEC>;
#[doc = "USCI B1 Baud Rate 1"]
pub mod ucb1br1_spi;
#[doc = "UCB1STAT_SPI (rw) register accessor: USCI B1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1stat_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1stat_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1stat_spi`]
module"]
pub type UCB1STAT_SPI = crate::Reg<ucb1stat_spi::UCB1STAT_SPI_SPEC>;
#[doc = "USCI B1 Status Register"]
pub mod ucb1stat_spi;
#[doc = "UCB1RXBUF_SPI (rw) register accessor: USCI B1 Receive Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1rxbuf_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1rxbuf_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1rxbuf_spi`]
module"]
pub type UCB1RXBUF_SPI = crate::Reg<ucb1rxbuf_spi::UCB1RXBUF_SPI_SPEC>;
#[doc = "USCI B1 Receive Buffer"]
pub mod ucb1rxbuf_spi;
#[doc = "UCB1TXBUF_SPI (rw) register accessor: USCI B1 Transmit Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1txbuf_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1txbuf_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1txbuf_spi`]
module"]
pub type UCB1TXBUF_SPI = crate::Reg<ucb1txbuf_spi::UCB1TXBUF_SPI_SPEC>;
#[doc = "USCI B1 Transmit Buffer"]
pub mod ucb1txbuf_spi;
#[doc = "UCB1IE_SPI (rw) register accessor: USCI B1 Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1ie_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1ie_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1ie_spi`]
module"]
pub type UCB1IE_SPI = crate::Reg<ucb1ie_spi::UCB1IE_SPI_SPEC>;
#[doc = "USCI B1 Interrupt Enable Register"]
pub mod ucb1ie_spi;
#[doc = "UCB1IFG_SPI (rw) register accessor: USCI B1 Interrupt Flags Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1ifg_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1ifg_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1ifg_spi`]
module"]
pub type UCB1IFG_SPI = crate::Reg<ucb1ifg_spi::UCB1IFG_SPI_SPEC>;
#[doc = "USCI B1 Interrupt Flags Register"]
pub mod ucb1ifg_spi;
#[doc = "UCB1IV_SPI (rw) register accessor: USCI B1 Interrupt Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1iv_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1iv_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1iv_spi`]
module"]
pub type UCB1IV_SPI = crate::Reg<ucb1iv_spi::UCB1IV_SPI_SPEC>;
#[doc = "USCI B1 Interrupt Vector Register"]
pub mod ucb1iv_spi;
