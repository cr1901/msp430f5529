#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ucb0ctl1: UCB0CTL1,
    ucb0ctl0: UCB0CTL0,
    _reserved2: [u8; 0x04],
    ucb0br0: UCB0BR0,
    ucb0br1: UCB0BR1,
    _reserved4: [u8; 0x02],
    ucb0stat: UCB0STAT,
    _reserved5: [u8; 0x01],
    ucb0rxbuf: UCB0RXBUF,
    _reserved6: [u8; 0x01],
    ucb0txbuf: UCB0TXBUF,
    _reserved7: [u8; 0x01],
    ucb0i2coa: UCB0I2COA,
    ucb0i2csa: UCB0I2CSA,
    _reserved9: [u8; 0x08],
    ucb0ie: UCB0IE,
    ucb0ifg: UCB0IFG,
    ucb0iv: UCB0IV,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI B0 Control Register 1"]
    #[inline(always)]
    pub const fn ucb0ctl1(&self) -> &UCB0CTL1 {
        &self.ucb0ctl1
    }
    #[doc = "0x01 - USCI B0 Control Register 0"]
    #[inline(always)]
    pub const fn ucb0ctl0(&self) -> &UCB0CTL0 {
        &self.ucb0ctl0
    }
    #[doc = "0x06 - USCI B0 Baud Rate 0"]
    #[inline(always)]
    pub const fn ucb0br0(&self) -> &UCB0BR0 {
        &self.ucb0br0
    }
    #[doc = "0x07 - USCI B0 Baud Rate 1"]
    #[inline(always)]
    pub const fn ucb0br1(&self) -> &UCB0BR1 {
        &self.ucb0br1
    }
    #[doc = "0x0a - USCI B0 Status Register"]
    #[inline(always)]
    pub const fn ucb0stat(&self) -> &UCB0STAT {
        &self.ucb0stat
    }
    #[doc = "0x0c - USCI B0 Receive Buffer"]
    #[inline(always)]
    pub const fn ucb0rxbuf(&self) -> &UCB0RXBUF {
        &self.ucb0rxbuf
    }
    #[doc = "0x0e - USCI B0 Transmit Buffer"]
    #[inline(always)]
    pub const fn ucb0txbuf(&self) -> &UCB0TXBUF {
        &self.ucb0txbuf
    }
    #[doc = "0x10 - USCI B0 I2C Own Address"]
    #[inline(always)]
    pub const fn ucb0i2coa(&self) -> &UCB0I2COA {
        &self.ucb0i2coa
    }
    #[doc = "0x12 - USCI B0 I2C Slave Address"]
    #[inline(always)]
    pub const fn ucb0i2csa(&self) -> &UCB0I2CSA {
        &self.ucb0i2csa
    }
    #[doc = "0x1c - USCI B0 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucb0ie(&self) -> &UCB0IE {
        &self.ucb0ie
    }
    #[doc = "0x1d - USCI B0 Interrupt Flags Register"]
    #[inline(always)]
    pub const fn ucb0ifg(&self) -> &UCB0IFG {
        &self.ucb0ifg
    }
    #[doc = "0x1e - USCI B0 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn ucb0iv(&self) -> &UCB0IV {
        &self.ucb0iv
    }
}
#[doc = "UCB0CTL1 (rw) register accessor: USCI B0 Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctl1`]
module"]
pub type UCB0CTL1 = crate::Reg<ucb0ctl1::UCB0CTL1_SPEC>;
#[doc = "USCI B0 Control Register 1"]
pub mod ucb0ctl1;
#[doc = "UCB0CTL0 (rw) register accessor: USCI B0 Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctl0`]
module"]
pub type UCB0CTL0 = crate::Reg<ucb0ctl0::UCB0CTL0_SPEC>;
#[doc = "USCI B0 Control Register 0"]
pub mod ucb0ctl0;
#[doc = "UCB0BR0 (rw) register accessor: USCI B0 Baud Rate 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0br0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0br0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0br0`]
module"]
pub type UCB0BR0 = crate::Reg<ucb0br0::UCB0BR0_SPEC>;
#[doc = "USCI B0 Baud Rate 0"]
pub mod ucb0br0;
#[doc = "UCB0BR1 (rw) register accessor: USCI B0 Baud Rate 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0br1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0br1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0br1`]
module"]
pub type UCB0BR1 = crate::Reg<ucb0br1::UCB0BR1_SPEC>;
#[doc = "USCI B0 Baud Rate 1"]
pub mod ucb0br1;
#[doc = "UCB0STAT (rw) register accessor: USCI B0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0stat`]
module"]
pub type UCB0STAT = crate::Reg<ucb0stat::UCB0STAT_SPEC>;
#[doc = "USCI B0 Status Register"]
pub mod ucb0stat;
#[doc = "UCB0RXBUF (rw) register accessor: USCI B0 Receive Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0rxbuf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0rxbuf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0rxbuf`]
module"]
pub type UCB0RXBUF = crate::Reg<ucb0rxbuf::UCB0RXBUF_SPEC>;
#[doc = "USCI B0 Receive Buffer"]
pub mod ucb0rxbuf;
#[doc = "UCB0TXBUF (rw) register accessor: USCI B0 Transmit Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0txbuf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0txbuf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0txbuf`]
module"]
pub type UCB0TXBUF = crate::Reg<ucb0txbuf::UCB0TXBUF_SPEC>;
#[doc = "USCI B0 Transmit Buffer"]
pub mod ucb0txbuf;
#[doc = "UCB0IE (rw) register accessor: USCI B0 Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ie`]
module"]
pub type UCB0IE = crate::Reg<ucb0ie::UCB0IE_SPEC>;
#[doc = "USCI B0 Interrupt Enable Register"]
pub mod ucb0ie;
#[doc = "UCB0IFG (rw) register accessor: USCI B0 Interrupt Flags Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0ifg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0ifg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ifg`]
module"]
pub type UCB0IFG = crate::Reg<ucb0ifg::UCB0IFG_SPEC>;
#[doc = "USCI B0 Interrupt Flags Register"]
pub mod ucb0ifg;
#[doc = "UCB0I2COA (rw) register accessor: USCI B0 I2C Own Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0i2coa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0i2coa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0i2coa`]
module"]
pub type UCB0I2COA = crate::Reg<ucb0i2coa::UCB0I2COA_SPEC>;
#[doc = "USCI B0 I2C Own Address"]
pub mod ucb0i2coa;
#[doc = "UCB0I2CSA (rw) register accessor: USCI B0 I2C Slave Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0i2csa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0i2csa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0i2csa`]
module"]
pub type UCB0I2CSA = crate::Reg<ucb0i2csa::UCB0I2CSA_SPEC>;
#[doc = "USCI B0 I2C Slave Address"]
pub mod ucb0i2csa;
#[doc = "UCB0IV (rw) register accessor: USCI B0 Interrupt Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0iv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0iv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0iv`]
module"]
pub type UCB0IV = crate::Reg<ucb0iv::UCB0IV_SPEC>;
#[doc = "USCI B0 Interrupt Vector Register"]
pub mod ucb0iv;
