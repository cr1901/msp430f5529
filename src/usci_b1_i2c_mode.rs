#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI B1 Control Register 1"]
    pub ucb1ctl1: crate::Reg<ucb1ctl1::UCB1CTL1_SPEC>,
    #[doc = "0x01 - USCI B1 Control Register 0"]
    pub ucb1ctl0: crate::Reg<ucb1ctl0::UCB1CTL0_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x06 - USCI B1 Baud Rate 0"]
    pub ucb1br0: crate::Reg<ucb1br0::UCB1BR0_SPEC>,
    #[doc = "0x07 - USCI B1 Baud Rate 1"]
    pub ucb1br1: crate::Reg<ucb1br1::UCB1BR1_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x0a - USCI B1 Status Register"]
    pub ucb1stat: crate::Reg<ucb1stat::UCB1STAT_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x0c - USCI B1 Receive Buffer"]
    pub ucb1rxbuf: crate::Reg<ucb1rxbuf::UCB1RXBUF_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x0e - USCI B1 Transmit Buffer"]
    pub ucb1txbuf: crate::Reg<ucb1txbuf::UCB1TXBUF_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x10 - USCI B1 I2C Own Address"]
    pub ucb1i2coa: crate::Reg<ucb1i2coa::UCB1I2COA_SPEC>,
    #[doc = "0x12 - USCI B1 I2C Slave Address"]
    pub ucb1i2csa: crate::Reg<ucb1i2csa::UCB1I2CSA_SPEC>,
    _reserved9: [u8; 0x08],
    #[doc = "0x1c - USCI B1 Interrupt Enable Register"]
    pub ucb1ie: crate::Reg<ucb1ie::UCB1IE_SPEC>,
    #[doc = "0x1d - USCI B1 Interrupt Flags Register"]
    pub ucb1ifg: crate::Reg<ucb1ifg::UCB1IFG_SPEC>,
    #[doc = "0x1e - USCI B1 Interrupt Vector Register"]
    pub ucb1iv: crate::Reg<ucb1iv::UCB1IV_SPEC>,
}
#[doc = "UCB1CTL1 register accessor: an alias for `Reg<UCB1CTL1_SPEC>`"]
pub type UCB1CTL1 = crate::Reg<ucb1ctl1::UCB1CTL1_SPEC>;
#[doc = "USCI B1 Control Register 1"]
pub mod ucb1ctl1;
#[doc = "UCB1CTL0 register accessor: an alias for `Reg<UCB1CTL0_SPEC>`"]
pub type UCB1CTL0 = crate::Reg<ucb1ctl0::UCB1CTL0_SPEC>;
#[doc = "USCI B1 Control Register 0"]
pub mod ucb1ctl0;
#[doc = "UCB1BR0 register accessor: an alias for `Reg<UCB1BR0_SPEC>`"]
pub type UCB1BR0 = crate::Reg<ucb1br0::UCB1BR0_SPEC>;
#[doc = "USCI B1 Baud Rate 0"]
pub mod ucb1br0;
#[doc = "UCB1BR1 register accessor: an alias for `Reg<UCB1BR1_SPEC>`"]
pub type UCB1BR1 = crate::Reg<ucb1br1::UCB1BR1_SPEC>;
#[doc = "USCI B1 Baud Rate 1"]
pub mod ucb1br1;
#[doc = "UCB1STAT register accessor: an alias for `Reg<UCB1STAT_SPEC>`"]
pub type UCB1STAT = crate::Reg<ucb1stat::UCB1STAT_SPEC>;
#[doc = "USCI B1 Status Register"]
pub mod ucb1stat;
#[doc = "UCB1RXBUF register accessor: an alias for `Reg<UCB1RXBUF_SPEC>`"]
pub type UCB1RXBUF = crate::Reg<ucb1rxbuf::UCB1RXBUF_SPEC>;
#[doc = "USCI B1 Receive Buffer"]
pub mod ucb1rxbuf;
#[doc = "UCB1TXBUF register accessor: an alias for `Reg<UCB1TXBUF_SPEC>`"]
pub type UCB1TXBUF = crate::Reg<ucb1txbuf::UCB1TXBUF_SPEC>;
#[doc = "USCI B1 Transmit Buffer"]
pub mod ucb1txbuf;
#[doc = "UCB1IE register accessor: an alias for `Reg<UCB1IE_SPEC>`"]
pub type UCB1IE = crate::Reg<ucb1ie::UCB1IE_SPEC>;
#[doc = "USCI B1 Interrupt Enable Register"]
pub mod ucb1ie;
#[doc = "UCB1IFG register accessor: an alias for `Reg<UCB1IFG_SPEC>`"]
pub type UCB1IFG = crate::Reg<ucb1ifg::UCB1IFG_SPEC>;
#[doc = "USCI B1 Interrupt Flags Register"]
pub mod ucb1ifg;
#[doc = "UCB1I2COA register accessor: an alias for `Reg<UCB1I2COA_SPEC>`"]
pub type UCB1I2COA = crate::Reg<ucb1i2coa::UCB1I2COA_SPEC>;
#[doc = "USCI B1 I2C Own Address"]
pub mod ucb1i2coa;
#[doc = "UCB1I2CSA register accessor: an alias for `Reg<UCB1I2CSA_SPEC>`"]
pub type UCB1I2CSA = crate::Reg<ucb1i2csa::UCB1I2CSA_SPEC>;
#[doc = "USCI B1 I2C Slave Address"]
pub mod ucb1i2csa;
#[doc = "UCB1IV register accessor: an alias for `Reg<UCB1IV_SPEC>`"]
pub type UCB1IV = crate::Reg<ucb1iv::UCB1IV_SPEC>;
#[doc = "USCI B1 Interrupt Vector Register"]
pub mod ucb1iv;
