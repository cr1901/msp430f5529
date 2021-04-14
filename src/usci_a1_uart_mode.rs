#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI A1 Control Register 1"]
    pub uca1ctl1: crate::Reg<uca1ctl1::UCA1CTL1_SPEC>,
    #[doc = "0x01 - USCI A1 Control Register 0"]
    pub uca1ctl0: crate::Reg<uca1ctl0::UCA1CTL0_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x06 - USCI A1 Baud Rate 0"]
    pub uca1br0: crate::Reg<uca1br0::UCA1BR0_SPEC>,
    #[doc = "0x07 - USCI A1 Baud Rate 1"]
    pub uca1br1: crate::Reg<uca1br1::UCA1BR1_SPEC>,
    #[doc = "0x08 - USCI A1 Modulation Control"]
    pub uca1mctl: crate::Reg<uca1mctl::UCA1MCTL_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x0a - USCI A1 Status Register"]
    pub uca1stat: crate::Reg<uca1stat::UCA1STAT_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x0c - USCI A1 Receive Buffer"]
    pub uca1rxbuf: crate::Reg<uca1rxbuf::UCA1RXBUF_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x0e - USCI A1 Transmit Buffer"]
    pub uca1txbuf: crate::Reg<uca1txbuf::UCA1TXBUF_SPEC>,
    _reserved8: [u8; 0x01],
    #[doc = "0x10 - USCI A1 LIN Control"]
    pub uca1abctl: crate::Reg<uca1abctl::UCA1ABCTL_SPEC>,
    _reserved9: [u8; 0x01],
    #[doc = "0x12 - USCI A1 IrDA Transmit Control"]
    pub uca1irtctl: crate::Reg<uca1irtctl::UCA1IRTCTL_SPEC>,
    #[doc = "0x13 - USCI A1 IrDA Receive Control"]
    pub uca1irrctl: crate::Reg<uca1irrctl::UCA1IRRCTL_SPEC>,
    _reserved11: [u8; 0x08],
    #[doc = "0x1c - USCI A1 Interrupt Enable Register"]
    pub uca1ie: crate::Reg<uca1ie::UCA1IE_SPEC>,
    #[doc = "0x1d - USCI A1 Interrupt Flags Register"]
    pub uca1ifg: crate::Reg<uca1ifg::UCA1IFG_SPEC>,
    #[doc = "0x1e - USCI A1 Interrupt Vector Register"]
    pub uca1iv: crate::Reg<uca1iv::UCA1IV_SPEC>,
}
#[doc = "UCA1CTL1 register accessor: an alias for `Reg<UCA1CTL1_SPEC>`"]
pub type UCA1CTL1 = crate::Reg<uca1ctl1::UCA1CTL1_SPEC>;
#[doc = "USCI A1 Control Register 1"]
pub mod uca1ctl1;
#[doc = "UCA1CTL0 register accessor: an alias for `Reg<UCA1CTL0_SPEC>`"]
pub type UCA1CTL0 = crate::Reg<uca1ctl0::UCA1CTL0_SPEC>;
#[doc = "USCI A1 Control Register 0"]
pub mod uca1ctl0;
#[doc = "UCA1BR0 register accessor: an alias for `Reg<UCA1BR0_SPEC>`"]
pub type UCA1BR0 = crate::Reg<uca1br0::UCA1BR0_SPEC>;
#[doc = "USCI A1 Baud Rate 0"]
pub mod uca1br0;
#[doc = "UCA1BR1 register accessor: an alias for `Reg<UCA1BR1_SPEC>`"]
pub type UCA1BR1 = crate::Reg<uca1br1::UCA1BR1_SPEC>;
#[doc = "USCI A1 Baud Rate 1"]
pub mod uca1br1;
#[doc = "UCA1MCTL register accessor: an alias for `Reg<UCA1MCTL_SPEC>`"]
pub type UCA1MCTL = crate::Reg<uca1mctl::UCA1MCTL_SPEC>;
#[doc = "USCI A1 Modulation Control"]
pub mod uca1mctl;
#[doc = "UCA1STAT register accessor: an alias for `Reg<UCA1STAT_SPEC>`"]
pub type UCA1STAT = crate::Reg<uca1stat::UCA1STAT_SPEC>;
#[doc = "USCI A1 Status Register"]
pub mod uca1stat;
#[doc = "UCA1RXBUF register accessor: an alias for `Reg<UCA1RXBUF_SPEC>`"]
pub type UCA1RXBUF = crate::Reg<uca1rxbuf::UCA1RXBUF_SPEC>;
#[doc = "USCI A1 Receive Buffer"]
pub mod uca1rxbuf;
#[doc = "UCA1TXBUF register accessor: an alias for `Reg<UCA1TXBUF_SPEC>`"]
pub type UCA1TXBUF = crate::Reg<uca1txbuf::UCA1TXBUF_SPEC>;
#[doc = "USCI A1 Transmit Buffer"]
pub mod uca1txbuf;
#[doc = "UCA1ABCTL register accessor: an alias for `Reg<UCA1ABCTL_SPEC>`"]
pub type UCA1ABCTL = crate::Reg<uca1abctl::UCA1ABCTL_SPEC>;
#[doc = "USCI A1 LIN Control"]
pub mod uca1abctl;
#[doc = "UCA1IRTCTL register accessor: an alias for `Reg<UCA1IRTCTL_SPEC>`"]
pub type UCA1IRTCTL = crate::Reg<uca1irtctl::UCA1IRTCTL_SPEC>;
#[doc = "USCI A1 IrDA Transmit Control"]
pub mod uca1irtctl;
#[doc = "UCA1IRRCTL register accessor: an alias for `Reg<UCA1IRRCTL_SPEC>`"]
pub type UCA1IRRCTL = crate::Reg<uca1irrctl::UCA1IRRCTL_SPEC>;
#[doc = "USCI A1 IrDA Receive Control"]
pub mod uca1irrctl;
#[doc = "UCA1IE register accessor: an alias for `Reg<UCA1IE_SPEC>`"]
pub type UCA1IE = crate::Reg<uca1ie::UCA1IE_SPEC>;
#[doc = "USCI A1 Interrupt Enable Register"]
pub mod uca1ie;
#[doc = "UCA1IFG register accessor: an alias for `Reg<UCA1IFG_SPEC>`"]
pub type UCA1IFG = crate::Reg<uca1ifg::UCA1IFG_SPEC>;
#[doc = "USCI A1 Interrupt Flags Register"]
pub mod uca1ifg;
#[doc = "UCA1IV register accessor: an alias for `Reg<UCA1IV_SPEC>`"]
pub type UCA1IV = crate::Reg<uca1iv::UCA1IV_SPEC>;
#[doc = "USCI A1 Interrupt Vector Register"]
pub mod uca1iv;
