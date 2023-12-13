#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ucb1ctl1: UCB1CTL1,
    ucb1ctl0: UCB1CTL0,
    _reserved2: [u8; 0x04],
    ucb1br0: UCB1BR0,
    ucb1br1: UCB1BR1,
    _reserved4: [u8; 0x02],
    ucb1stat: UCB1STAT,
    _reserved5: [u8; 0x01],
    ucb1rxbuf: UCB1RXBUF,
    _reserved6: [u8; 0x01],
    ucb1txbuf: UCB1TXBUF,
    _reserved7: [u8; 0x01],
    ucb1i2coa: UCB1I2COA,
    ucb1i2csa: UCB1I2CSA,
    _reserved9: [u8; 0x08],
    ucb1ie: UCB1IE,
    ucb1ifg: UCB1IFG,
    ucb1iv: UCB1IV,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI B1 Control Register 1"]
    #[inline(always)]
    pub const fn ucb1ctl1(&self) -> &UCB1CTL1 {
        &self.ucb1ctl1
    }
    #[doc = "0x01 - USCI B1 Control Register 0"]
    #[inline(always)]
    pub const fn ucb1ctl0(&self) -> &UCB1CTL0 {
        &self.ucb1ctl0
    }
    #[doc = "0x06 - USCI B1 Baud Rate 0"]
    #[inline(always)]
    pub const fn ucb1br0(&self) -> &UCB1BR0 {
        &self.ucb1br0
    }
    #[doc = "0x07 - USCI B1 Baud Rate 1"]
    #[inline(always)]
    pub const fn ucb1br1(&self) -> &UCB1BR1 {
        &self.ucb1br1
    }
    #[doc = "0x0a - USCI B1 Status Register"]
    #[inline(always)]
    pub const fn ucb1stat(&self) -> &UCB1STAT {
        &self.ucb1stat
    }
    #[doc = "0x0c - USCI B1 Receive Buffer"]
    #[inline(always)]
    pub const fn ucb1rxbuf(&self) -> &UCB1RXBUF {
        &self.ucb1rxbuf
    }
    #[doc = "0x0e - USCI B1 Transmit Buffer"]
    #[inline(always)]
    pub const fn ucb1txbuf(&self) -> &UCB1TXBUF {
        &self.ucb1txbuf
    }
    #[doc = "0x10 - USCI B1 I2C Own Address"]
    #[inline(always)]
    pub const fn ucb1i2coa(&self) -> &UCB1I2COA {
        &self.ucb1i2coa
    }
    #[doc = "0x12 - USCI B1 I2C Slave Address"]
    #[inline(always)]
    pub const fn ucb1i2csa(&self) -> &UCB1I2CSA {
        &self.ucb1i2csa
    }
    #[doc = "0x1c - USCI B1 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucb1ie(&self) -> &UCB1IE {
        &self.ucb1ie
    }
    #[doc = "0x1d - USCI B1 Interrupt Flags Register"]
    #[inline(always)]
    pub const fn ucb1ifg(&self) -> &UCB1IFG {
        &self.ucb1ifg
    }
    #[doc = "0x1e - USCI B1 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn ucb1iv(&self) -> &UCB1IV {
        &self.ucb1iv
    }
}
#[doc = "UCB1CTL1 (rw) register accessor: USCI B1 Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1ctl1`]
module"]
pub type UCB1CTL1 = crate::Reg<ucb1ctl1::UCB1CTL1_SPEC>;
#[doc = "USCI B1 Control Register 1"]
pub mod ucb1ctl1;
#[doc = "UCB1CTL0 (rw) register accessor: USCI B1 Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1ctl0`]
module"]
pub type UCB1CTL0 = crate::Reg<ucb1ctl0::UCB1CTL0_SPEC>;
#[doc = "USCI B1 Control Register 0"]
pub mod ucb1ctl0;
#[doc = "UCB1BR0 (rw) register accessor: USCI B1 Baud Rate 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1br0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1br0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1br0`]
module"]
pub type UCB1BR0 = crate::Reg<ucb1br0::UCB1BR0_SPEC>;
#[doc = "USCI B1 Baud Rate 0"]
pub mod ucb1br0;
#[doc = "UCB1BR1 (rw) register accessor: USCI B1 Baud Rate 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1br1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1br1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1br1`]
module"]
pub type UCB1BR1 = crate::Reg<ucb1br1::UCB1BR1_SPEC>;
#[doc = "USCI B1 Baud Rate 1"]
pub mod ucb1br1;
#[doc = "UCB1STAT (rw) register accessor: USCI B1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1stat`]
module"]
pub type UCB1STAT = crate::Reg<ucb1stat::UCB1STAT_SPEC>;
#[doc = "USCI B1 Status Register"]
pub mod ucb1stat;
#[doc = "UCB1RXBUF (rw) register accessor: USCI B1 Receive Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1rxbuf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1rxbuf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1rxbuf`]
module"]
pub type UCB1RXBUF = crate::Reg<ucb1rxbuf::UCB1RXBUF_SPEC>;
#[doc = "USCI B1 Receive Buffer"]
pub mod ucb1rxbuf;
#[doc = "UCB1TXBUF (rw) register accessor: USCI B1 Transmit Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1txbuf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1txbuf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1txbuf`]
module"]
pub type UCB1TXBUF = crate::Reg<ucb1txbuf::UCB1TXBUF_SPEC>;
#[doc = "USCI B1 Transmit Buffer"]
pub mod ucb1txbuf;
#[doc = "UCB1IE (rw) register accessor: USCI B1 Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1ie`]
module"]
pub type UCB1IE = crate::Reg<ucb1ie::UCB1IE_SPEC>;
#[doc = "USCI B1 Interrupt Enable Register"]
pub mod ucb1ie;
#[doc = "UCB1IFG (rw) register accessor: USCI B1 Interrupt Flags Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1ifg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1ifg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1ifg`]
module"]
pub type UCB1IFG = crate::Reg<ucb1ifg::UCB1IFG_SPEC>;
#[doc = "USCI B1 Interrupt Flags Register"]
pub mod ucb1ifg;
#[doc = "UCB1I2COA (rw) register accessor: USCI B1 I2C Own Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1i2coa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1i2coa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1i2coa`]
module"]
pub type UCB1I2COA = crate::Reg<ucb1i2coa::UCB1I2COA_SPEC>;
#[doc = "USCI B1 I2C Own Address"]
pub mod ucb1i2coa;
#[doc = "UCB1I2CSA (rw) register accessor: USCI B1 I2C Slave Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1i2csa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1i2csa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1i2csa`]
module"]
pub type UCB1I2CSA = crate::Reg<ucb1i2csa::UCB1I2CSA_SPEC>;
#[doc = "USCI B1 I2C Slave Address"]
pub mod ucb1i2csa;
#[doc = "UCB1IV (rw) register accessor: USCI B1 Interrupt Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1iv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1iv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1iv`]
module"]
pub type UCB1IV = crate::Reg<ucb1iv::UCB1IV_SPEC>;
#[doc = "USCI B1 Interrupt Vector Register"]
pub mod ucb1iv;
