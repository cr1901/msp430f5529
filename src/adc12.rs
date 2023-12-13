#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    adc12ctl0: ADC12CTL0,
    adc12ctl1: ADC12CTL1,
    adc12ctl2: ADC12CTL2,
    _reserved3: [u8; 0x04],
    adc12ifg: ADC12IFG,
    adc12ie: ADC12IE,
    adc12iv: ADC12IV,
    adc12mctl0: ADC12MCTL0,
    adc12mctl1: ADC12MCTL1,
    adc12mctl2: ADC12MCTL2,
    adc12mctl3: ADC12MCTL3,
    adc12mctl4: ADC12MCTL4,
    adc12mctl5: ADC12MCTL5,
    adc12mctl6: ADC12MCTL6,
    adc12mctl7: ADC12MCTL7,
    adc12mctl8: ADC12MCTL8,
    adc12mctl9: ADC12MCTL9,
    adc12mctl10: ADC12MCTL10,
    adc12mctl11: ADC12MCTL11,
    adc12mctl12: ADC12MCTL12,
    adc12mctl13: ADC12MCTL13,
    adc12mctl14: ADC12MCTL14,
    adc12mctl15: ADC12MCTL15,
    adc12mem0: ADC12MEM0,
    adc12mem1: ADC12MEM1,
    adc12mem2: ADC12MEM2,
    adc12mem3: ADC12MEM3,
    adc12mem4: ADC12MEM4,
    adc12mem5: ADC12MEM5,
    adc12mem6: ADC12MEM6,
    adc12mem7: ADC12MEM7,
    adc12mem8: ADC12MEM8,
    adc12mem9: ADC12MEM9,
    adc12mem10: ADC12MEM10,
    adc12mem11: ADC12MEM11,
    adc12mem12: ADC12MEM12,
    adc12mem13: ADC12MEM13,
    adc12mem14: ADC12MEM14,
    adc12mem15: ADC12MEM15,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC12+ Control 0"]
    #[inline(always)]
    pub const fn adc12ctl0(&self) -> &ADC12CTL0 {
        &self.adc12ctl0
    }
    #[doc = "0x02 - ADC12+ Control 1"]
    #[inline(always)]
    pub const fn adc12ctl1(&self) -> &ADC12CTL1 {
        &self.adc12ctl1
    }
    #[doc = "0x04 - ADC12+ Control 2"]
    #[inline(always)]
    pub const fn adc12ctl2(&self) -> &ADC12CTL2 {
        &self.adc12ctl2
    }
    #[doc = "0x0a - ADC12+ Interrupt Flag"]
    #[inline(always)]
    pub const fn adc12ifg(&self) -> &ADC12IFG {
        &self.adc12ifg
    }
    #[doc = "0x0c - ADC12+ Interrupt Enable"]
    #[inline(always)]
    pub const fn adc12ie(&self) -> &ADC12IE {
        &self.adc12ie
    }
    #[doc = "0x0e - ADC12+ Interrupt Vector Word"]
    #[inline(always)]
    pub const fn adc12iv(&self) -> &ADC12IV {
        &self.adc12iv
    }
    #[doc = "0x10 - ADC12 Memory Control 0"]
    #[inline(always)]
    pub const fn adc12mctl0(&self) -> &ADC12MCTL0 {
        &self.adc12mctl0
    }
    #[doc = "0x11 - ADC12 Memory Control 1"]
    #[inline(always)]
    pub const fn adc12mctl1(&self) -> &ADC12MCTL1 {
        &self.adc12mctl1
    }
    #[doc = "0x12 - ADC12 Memory Control 2"]
    #[inline(always)]
    pub const fn adc12mctl2(&self) -> &ADC12MCTL2 {
        &self.adc12mctl2
    }
    #[doc = "0x13 - ADC12 Memory Control 3"]
    #[inline(always)]
    pub const fn adc12mctl3(&self) -> &ADC12MCTL3 {
        &self.adc12mctl3
    }
    #[doc = "0x14 - ADC12 Memory Control 4"]
    #[inline(always)]
    pub const fn adc12mctl4(&self) -> &ADC12MCTL4 {
        &self.adc12mctl4
    }
    #[doc = "0x15 - ADC12 Memory Control 5"]
    #[inline(always)]
    pub const fn adc12mctl5(&self) -> &ADC12MCTL5 {
        &self.adc12mctl5
    }
    #[doc = "0x16 - ADC12 Memory Control 6"]
    #[inline(always)]
    pub const fn adc12mctl6(&self) -> &ADC12MCTL6 {
        &self.adc12mctl6
    }
    #[doc = "0x17 - ADC12 Memory Control 7"]
    #[inline(always)]
    pub const fn adc12mctl7(&self) -> &ADC12MCTL7 {
        &self.adc12mctl7
    }
    #[doc = "0x18 - ADC12 Memory Control 8"]
    #[inline(always)]
    pub const fn adc12mctl8(&self) -> &ADC12MCTL8 {
        &self.adc12mctl8
    }
    #[doc = "0x19 - ADC12 Memory Control 9"]
    #[inline(always)]
    pub const fn adc12mctl9(&self) -> &ADC12MCTL9 {
        &self.adc12mctl9
    }
    #[doc = "0x1a - ADC12 Memory Control 10"]
    #[inline(always)]
    pub const fn adc12mctl10(&self) -> &ADC12MCTL10 {
        &self.adc12mctl10
    }
    #[doc = "0x1b - ADC12 Memory Control 11"]
    #[inline(always)]
    pub const fn adc12mctl11(&self) -> &ADC12MCTL11 {
        &self.adc12mctl11
    }
    #[doc = "0x1c - ADC12 Memory Control 12"]
    #[inline(always)]
    pub const fn adc12mctl12(&self) -> &ADC12MCTL12 {
        &self.adc12mctl12
    }
    #[doc = "0x1d - ADC12 Memory Control 13"]
    #[inline(always)]
    pub const fn adc12mctl13(&self) -> &ADC12MCTL13 {
        &self.adc12mctl13
    }
    #[doc = "0x1e - ADC12 Memory Control 14"]
    #[inline(always)]
    pub const fn adc12mctl14(&self) -> &ADC12MCTL14 {
        &self.adc12mctl14
    }
    #[doc = "0x1f - ADC12 Memory Control 15"]
    #[inline(always)]
    pub const fn adc12mctl15(&self) -> &ADC12MCTL15 {
        &self.adc12mctl15
    }
    #[doc = "0x20 - ADC12 Conversion Memory 0"]
    #[inline(always)]
    pub const fn adc12mem0(&self) -> &ADC12MEM0 {
        &self.adc12mem0
    }
    #[doc = "0x22 - ADC12 Conversion Memory 1"]
    #[inline(always)]
    pub const fn adc12mem1(&self) -> &ADC12MEM1 {
        &self.adc12mem1
    }
    #[doc = "0x24 - ADC12 Conversion Memory 2"]
    #[inline(always)]
    pub const fn adc12mem2(&self) -> &ADC12MEM2 {
        &self.adc12mem2
    }
    #[doc = "0x26 - ADC12 Conversion Memory 3"]
    #[inline(always)]
    pub const fn adc12mem3(&self) -> &ADC12MEM3 {
        &self.adc12mem3
    }
    #[doc = "0x28 - ADC12 Conversion Memory 4"]
    #[inline(always)]
    pub const fn adc12mem4(&self) -> &ADC12MEM4 {
        &self.adc12mem4
    }
    #[doc = "0x2a - ADC12 Conversion Memory 5"]
    #[inline(always)]
    pub const fn adc12mem5(&self) -> &ADC12MEM5 {
        &self.adc12mem5
    }
    #[doc = "0x2c - ADC12 Conversion Memory 6"]
    #[inline(always)]
    pub const fn adc12mem6(&self) -> &ADC12MEM6 {
        &self.adc12mem6
    }
    #[doc = "0x2e - ADC12 Conversion Memory 7"]
    #[inline(always)]
    pub const fn adc12mem7(&self) -> &ADC12MEM7 {
        &self.adc12mem7
    }
    #[doc = "0x30 - ADC12 Conversion Memory 8"]
    #[inline(always)]
    pub const fn adc12mem8(&self) -> &ADC12MEM8 {
        &self.adc12mem8
    }
    #[doc = "0x32 - ADC12 Conversion Memory 9"]
    #[inline(always)]
    pub const fn adc12mem9(&self) -> &ADC12MEM9 {
        &self.adc12mem9
    }
    #[doc = "0x34 - ADC12 Conversion Memory 10"]
    #[inline(always)]
    pub const fn adc12mem10(&self) -> &ADC12MEM10 {
        &self.adc12mem10
    }
    #[doc = "0x36 - ADC12 Conversion Memory 11"]
    #[inline(always)]
    pub const fn adc12mem11(&self) -> &ADC12MEM11 {
        &self.adc12mem11
    }
    #[doc = "0x38 - ADC12 Conversion Memory 12"]
    #[inline(always)]
    pub const fn adc12mem12(&self) -> &ADC12MEM12 {
        &self.adc12mem12
    }
    #[doc = "0x3a - ADC12 Conversion Memory 13"]
    #[inline(always)]
    pub const fn adc12mem13(&self) -> &ADC12MEM13 {
        &self.adc12mem13
    }
    #[doc = "0x3c - ADC12 Conversion Memory 14"]
    #[inline(always)]
    pub const fn adc12mem14(&self) -> &ADC12MEM14 {
        &self.adc12mem14
    }
    #[doc = "0x3e - ADC12 Conversion Memory 15"]
    #[inline(always)]
    pub const fn adc12mem15(&self) -> &ADC12MEM15 {
        &self.adc12mem15
    }
}
#[doc = "ADC12MCTL0 (rw) register accessor: ADC12 Memory Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl0`]
module"]
pub type ADC12MCTL0 = crate::Reg<adc12mctl0::ADC12MCTL0_SPEC>;
#[doc = "ADC12 Memory Control 0"]
pub mod adc12mctl0;
#[doc = "ADC12MCTL1 (rw) register accessor: ADC12 Memory Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl1`]
module"]
pub type ADC12MCTL1 = crate::Reg<adc12mctl1::ADC12MCTL1_SPEC>;
#[doc = "ADC12 Memory Control 1"]
pub mod adc12mctl1;
#[doc = "ADC12MCTL2 (rw) register accessor: ADC12 Memory Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl2`]
module"]
pub type ADC12MCTL2 = crate::Reg<adc12mctl2::ADC12MCTL2_SPEC>;
#[doc = "ADC12 Memory Control 2"]
pub mod adc12mctl2;
#[doc = "ADC12MCTL3 (rw) register accessor: ADC12 Memory Control 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl3`]
module"]
pub type ADC12MCTL3 = crate::Reg<adc12mctl3::ADC12MCTL3_SPEC>;
#[doc = "ADC12 Memory Control 3"]
pub mod adc12mctl3;
#[doc = "ADC12MCTL4 (rw) register accessor: ADC12 Memory Control 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl4`]
module"]
pub type ADC12MCTL4 = crate::Reg<adc12mctl4::ADC12MCTL4_SPEC>;
#[doc = "ADC12 Memory Control 4"]
pub mod adc12mctl4;
#[doc = "ADC12MCTL5 (rw) register accessor: ADC12 Memory Control 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl5`]
module"]
pub type ADC12MCTL5 = crate::Reg<adc12mctl5::ADC12MCTL5_SPEC>;
#[doc = "ADC12 Memory Control 5"]
pub mod adc12mctl5;
#[doc = "ADC12MCTL6 (rw) register accessor: ADC12 Memory Control 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mctl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mctl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl6`]
module"]
pub type ADC12MCTL6 = crate::Reg<adc12mctl6::ADC12MCTL6_SPEC>;
#[doc = "ADC12 Memory Control 6"]
pub mod adc12mctl6;
#[doc = "ADC12MCTL7 (rw) register accessor: ADC12 Memory Control 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mctl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mctl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl7`]
module"]
pub type ADC12MCTL7 = crate::Reg<adc12mctl7::ADC12MCTL7_SPEC>;
#[doc = "ADC12 Memory Control 7"]
pub mod adc12mctl7;
#[doc = "ADC12MCTL8 (rw) register accessor: ADC12 Memory Control 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mctl8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mctl8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl8`]
module"]
pub type ADC12MCTL8 = crate::Reg<adc12mctl8::ADC12MCTL8_SPEC>;
#[doc = "ADC12 Memory Control 8"]
pub mod adc12mctl8;
#[doc = "ADC12MCTL9 (rw) register accessor: ADC12 Memory Control 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mctl9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mctl9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl9`]
module"]
pub type ADC12MCTL9 = crate::Reg<adc12mctl9::ADC12MCTL9_SPEC>;
#[doc = "ADC12 Memory Control 9"]
pub mod adc12mctl9;
#[doc = "ADC12MCTL10 (rw) register accessor: ADC12 Memory Control 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mctl10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mctl10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl10`]
module"]
pub type ADC12MCTL10 = crate::Reg<adc12mctl10::ADC12MCTL10_SPEC>;
#[doc = "ADC12 Memory Control 10"]
pub mod adc12mctl10;
#[doc = "ADC12MCTL11 (rw) register accessor: ADC12 Memory Control 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mctl11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mctl11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl11`]
module"]
pub type ADC12MCTL11 = crate::Reg<adc12mctl11::ADC12MCTL11_SPEC>;
#[doc = "ADC12 Memory Control 11"]
pub mod adc12mctl11;
#[doc = "ADC12MCTL12 (rw) register accessor: ADC12 Memory Control 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mctl12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mctl12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl12`]
module"]
pub type ADC12MCTL12 = crate::Reg<adc12mctl12::ADC12MCTL12_SPEC>;
#[doc = "ADC12 Memory Control 12"]
pub mod adc12mctl12;
#[doc = "ADC12MCTL13 (rw) register accessor: ADC12 Memory Control 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mctl13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mctl13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl13`]
module"]
pub type ADC12MCTL13 = crate::Reg<adc12mctl13::ADC12MCTL13_SPEC>;
#[doc = "ADC12 Memory Control 13"]
pub mod adc12mctl13;
#[doc = "ADC12MCTL14 (rw) register accessor: ADC12 Memory Control 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mctl14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mctl14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl14`]
module"]
pub type ADC12MCTL14 = crate::Reg<adc12mctl14::ADC12MCTL14_SPEC>;
#[doc = "ADC12 Memory Control 14"]
pub mod adc12mctl14;
#[doc = "ADC12MCTL15 (rw) register accessor: ADC12 Memory Control 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mctl15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mctl15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mctl15`]
module"]
pub type ADC12MCTL15 = crate::Reg<adc12mctl15::ADC12MCTL15_SPEC>;
#[doc = "ADC12 Memory Control 15"]
pub mod adc12mctl15;
#[doc = "ADC12CTL0 (rw) register accessor: ADC12+ Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12ctl0`]
module"]
pub type ADC12CTL0 = crate::Reg<adc12ctl0::ADC12CTL0_SPEC>;
#[doc = "ADC12+ Control 0"]
pub mod adc12ctl0;
#[doc = "ADC12CTL1 (rw) register accessor: ADC12+ Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12ctl1`]
module"]
pub type ADC12CTL1 = crate::Reg<adc12ctl1::ADC12CTL1_SPEC>;
#[doc = "ADC12+ Control 1"]
pub mod adc12ctl1;
#[doc = "ADC12CTL2 (rw) register accessor: ADC12+ Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12ctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12ctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12ctl2`]
module"]
pub type ADC12CTL2 = crate::Reg<adc12ctl2::ADC12CTL2_SPEC>;
#[doc = "ADC12+ Control 2"]
pub mod adc12ctl2;
#[doc = "ADC12IFG (rw) register accessor: ADC12+ Interrupt Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12ifg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12ifg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12ifg`]
module"]
pub type ADC12IFG = crate::Reg<adc12ifg::ADC12IFG_SPEC>;
#[doc = "ADC12+ Interrupt Flag"]
pub mod adc12ifg;
#[doc = "ADC12IE (rw) register accessor: ADC12+ Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12ie`]
module"]
pub type ADC12IE = crate::Reg<adc12ie::ADC12IE_SPEC>;
#[doc = "ADC12+ Interrupt Enable"]
pub mod adc12ie;
#[doc = "ADC12IV (rw) register accessor: ADC12+ Interrupt Vector Word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12iv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12iv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12iv`]
module"]
pub type ADC12IV = crate::Reg<adc12iv::ADC12IV_SPEC>;
#[doc = "ADC12+ Interrupt Vector Word"]
pub mod adc12iv;
#[doc = "ADC12MEM0 (rw) register accessor: ADC12 Conversion Memory 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mem0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mem0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem0`]
module"]
pub type ADC12MEM0 = crate::Reg<adc12mem0::ADC12MEM0_SPEC>;
#[doc = "ADC12 Conversion Memory 0"]
pub mod adc12mem0;
#[doc = "ADC12MEM1 (rw) register accessor: ADC12 Conversion Memory 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mem1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mem1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem1`]
module"]
pub type ADC12MEM1 = crate::Reg<adc12mem1::ADC12MEM1_SPEC>;
#[doc = "ADC12 Conversion Memory 1"]
pub mod adc12mem1;
#[doc = "ADC12MEM2 (rw) register accessor: ADC12 Conversion Memory 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mem2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mem2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem2`]
module"]
pub type ADC12MEM2 = crate::Reg<adc12mem2::ADC12MEM2_SPEC>;
#[doc = "ADC12 Conversion Memory 2"]
pub mod adc12mem2;
#[doc = "ADC12MEM3 (rw) register accessor: ADC12 Conversion Memory 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mem3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mem3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem3`]
module"]
pub type ADC12MEM3 = crate::Reg<adc12mem3::ADC12MEM3_SPEC>;
#[doc = "ADC12 Conversion Memory 3"]
pub mod adc12mem3;
#[doc = "ADC12MEM4 (rw) register accessor: ADC12 Conversion Memory 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mem4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mem4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem4`]
module"]
pub type ADC12MEM4 = crate::Reg<adc12mem4::ADC12MEM4_SPEC>;
#[doc = "ADC12 Conversion Memory 4"]
pub mod adc12mem4;
#[doc = "ADC12MEM5 (rw) register accessor: ADC12 Conversion Memory 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mem5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mem5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem5`]
module"]
pub type ADC12MEM5 = crate::Reg<adc12mem5::ADC12MEM5_SPEC>;
#[doc = "ADC12 Conversion Memory 5"]
pub mod adc12mem5;
#[doc = "ADC12MEM6 (rw) register accessor: ADC12 Conversion Memory 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mem6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mem6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem6`]
module"]
pub type ADC12MEM6 = crate::Reg<adc12mem6::ADC12MEM6_SPEC>;
#[doc = "ADC12 Conversion Memory 6"]
pub mod adc12mem6;
#[doc = "ADC12MEM7 (rw) register accessor: ADC12 Conversion Memory 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mem7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mem7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem7`]
module"]
pub type ADC12MEM7 = crate::Reg<adc12mem7::ADC12MEM7_SPEC>;
#[doc = "ADC12 Conversion Memory 7"]
pub mod adc12mem7;
#[doc = "ADC12MEM8 (rw) register accessor: ADC12 Conversion Memory 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mem8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mem8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem8`]
module"]
pub type ADC12MEM8 = crate::Reg<adc12mem8::ADC12MEM8_SPEC>;
#[doc = "ADC12 Conversion Memory 8"]
pub mod adc12mem8;
#[doc = "ADC12MEM9 (rw) register accessor: ADC12 Conversion Memory 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mem9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mem9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem9`]
module"]
pub type ADC12MEM9 = crate::Reg<adc12mem9::ADC12MEM9_SPEC>;
#[doc = "ADC12 Conversion Memory 9"]
pub mod adc12mem9;
#[doc = "ADC12MEM10 (rw) register accessor: ADC12 Conversion Memory 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mem10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mem10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem10`]
module"]
pub type ADC12MEM10 = crate::Reg<adc12mem10::ADC12MEM10_SPEC>;
#[doc = "ADC12 Conversion Memory 10"]
pub mod adc12mem10;
#[doc = "ADC12MEM11 (rw) register accessor: ADC12 Conversion Memory 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mem11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mem11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem11`]
module"]
pub type ADC12MEM11 = crate::Reg<adc12mem11::ADC12MEM11_SPEC>;
#[doc = "ADC12 Conversion Memory 11"]
pub mod adc12mem11;
#[doc = "ADC12MEM12 (rw) register accessor: ADC12 Conversion Memory 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mem12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mem12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem12`]
module"]
pub type ADC12MEM12 = crate::Reg<adc12mem12::ADC12MEM12_SPEC>;
#[doc = "ADC12 Conversion Memory 12"]
pub mod adc12mem12;
#[doc = "ADC12MEM13 (rw) register accessor: ADC12 Conversion Memory 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mem13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mem13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem13`]
module"]
pub type ADC12MEM13 = crate::Reg<adc12mem13::ADC12MEM13_SPEC>;
#[doc = "ADC12 Conversion Memory 13"]
pub mod adc12mem13;
#[doc = "ADC12MEM14 (rw) register accessor: ADC12 Conversion Memory 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mem14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mem14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem14`]
module"]
pub type ADC12MEM14 = crate::Reg<adc12mem14::ADC12MEM14_SPEC>;
#[doc = "ADC12 Conversion Memory 14"]
pub mod adc12mem14;
#[doc = "ADC12MEM15 (rw) register accessor: ADC12 Conversion Memory 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mem15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mem15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12mem15`]
module"]
pub type ADC12MEM15 = crate::Reg<adc12mem15::ADC12MEM15_SPEC>;
#[doc = "ADC12 Conversion Memory 15"]
pub mod adc12mem15;
