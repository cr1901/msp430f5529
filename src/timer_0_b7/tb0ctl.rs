#[doc = "Reader of register TB0CTL"]
pub type R = crate::R<u16, super::TB0CTL>;
#[doc = "Writer for register TB0CTL"]
pub type W = crate::W<u16, super::TB0CTL>;
#[doc = "Register TB0CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::TB0CTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TBIFG`"]
pub type TBIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBIFG`"]
pub struct TBIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> TBIFG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `TBIE`"]
pub type TBIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBIE`"]
pub struct TBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TBCLR`"]
pub type TBCLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBCLR`"]
pub struct TBCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Timer0_B7 mode control 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MC_A {
    #[doc = "0: Timer A mode control: 0 - Stop"]
    MC_0 = 0,
    #[doc = "1: Timer A mode control: 1 - Up to CCR0"]
    MC_1 = 1,
    #[doc = "2: Timer A mode control: 2 - Continuous up"]
    MC_2 = 2,
    #[doc = "3: Timer A mode control: 3 - Up/Down"]
    MC_3 = 3,
}
impl From<MC_A> for u8 {
    #[inline(always)]
    fn from(variant: MC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MC`"]
pub type MC_R = crate::R<u8, MC_A>;
impl MC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MC_A {
        match self.bits {
            0 => MC_A::MC_0,
            1 => MC_A::MC_1,
            2 => MC_A::MC_2,
            3 => MC_A::MC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MC_0`"]
    #[inline(always)]
    pub fn is_mc_0(&self) -> bool {
        *self == MC_A::MC_0
    }
    #[doc = "Checks if the value of the field is `MC_1`"]
    #[inline(always)]
    pub fn is_mc_1(&self) -> bool {
        *self == MC_A::MC_1
    }
    #[doc = "Checks if the value of the field is `MC_2`"]
    #[inline(always)]
    pub fn is_mc_2(&self) -> bool {
        *self == MC_A::MC_2
    }
    #[doc = "Checks if the value of the field is `MC_3`"]
    #[inline(always)]
    pub fn is_mc_3(&self) -> bool {
        *self == MC_A::MC_3
    }
}
#[doc = "Write proxy for field `MC`"]
pub struct MC_W<'a> {
    w: &'a mut W,
}
impl<'a> MC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timer A mode control: 0 - Stop"]
    #[inline(always)]
    pub fn mc_0(self) -> &'a mut W {
        self.variant(MC_A::MC_0)
    }
    #[doc = "Timer A mode control: 1 - Up to CCR0"]
    #[inline(always)]
    pub fn mc_1(self) -> &'a mut W {
        self.variant(MC_A::MC_1)
    }
    #[doc = "Timer A mode control: 2 - Continuous up"]
    #[inline(always)]
    pub fn mc_2(self) -> &'a mut W {
        self.variant(MC_A::MC_2)
    }
    #[doc = "Timer A mode control: 3 - Up/Down"]
    #[inline(always)]
    pub fn mc_3(self) -> &'a mut W {
        self.variant(MC_A::MC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Timer0_B7 clock input divider 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ID_A {
    #[doc = "0: Timer A input divider: 0 - /1"]
    ID_0 = 0,
    #[doc = "1: Timer A input divider: 1 - /2"]
    ID_1 = 1,
    #[doc = "2: Timer A input divider: 2 - /4"]
    ID_2 = 2,
    #[doc = "3: Timer A input divider: 3 - /8"]
    ID_3 = 3,
}
impl From<ID_A> for u8 {
    #[inline(always)]
    fn from(variant: ID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u8, ID_A>;
impl ID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ID_A {
        match self.bits {
            0 => ID_A::ID_0,
            1 => ID_A::ID_1,
            2 => ID_A::ID_2,
            3 => ID_A::ID_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ID_0`"]
    #[inline(always)]
    pub fn is_id_0(&self) -> bool {
        *self == ID_A::ID_0
    }
    #[doc = "Checks if the value of the field is `ID_1`"]
    #[inline(always)]
    pub fn is_id_1(&self) -> bool {
        *self == ID_A::ID_1
    }
    #[doc = "Checks if the value of the field is `ID_2`"]
    #[inline(always)]
    pub fn is_id_2(&self) -> bool {
        *self == ID_A::ID_2
    }
    #[doc = "Checks if the value of the field is `ID_3`"]
    #[inline(always)]
    pub fn is_id_3(&self) -> bool {
        *self == ID_A::ID_3
    }
}
#[doc = "Write proxy for field `ID`"]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ID_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timer A input divider: 0 - /1"]
    #[inline(always)]
    pub fn id_0(self) -> &'a mut W {
        self.variant(ID_A::ID_0)
    }
    #[doc = "Timer A input divider: 1 - /2"]
    #[inline(always)]
    pub fn id_1(self) -> &'a mut W {
        self.variant(ID_A::ID_1)
    }
    #[doc = "Timer A input divider: 2 - /4"]
    #[inline(always)]
    pub fn id_2(self) -> &'a mut W {
        self.variant(ID_A::ID_2)
    }
    #[doc = "Timer A input divider: 3 - /8"]
    #[inline(always)]
    pub fn id_3(self) -> &'a mut W {
        self.variant(ID_A::ID_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Clock source 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TBSSEL_A {
    #[doc = "0: Clock Source: TBCLK"]
    TBSSEL_0 = 0,
    #[doc = "1: Clock Source: ACLK"]
    TBSSEL_1 = 1,
    #[doc = "2: Clock Source: SMCLK"]
    TBSSEL_2 = 2,
    #[doc = "3: Clock Source: INCLK"]
    TBSSEL_3 = 3,
}
impl From<TBSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TBSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TBSSEL`"]
pub type TBSSEL_R = crate::R<u8, TBSSEL_A>;
impl TBSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBSSEL_A {
        match self.bits {
            0 => TBSSEL_A::TBSSEL_0,
            1 => TBSSEL_A::TBSSEL_1,
            2 => TBSSEL_A::TBSSEL_2,
            3 => TBSSEL_A::TBSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TBSSEL_0`"]
    #[inline(always)]
    pub fn is_tbssel_0(&self) -> bool {
        *self == TBSSEL_A::TBSSEL_0
    }
    #[doc = "Checks if the value of the field is `TBSSEL_1`"]
    #[inline(always)]
    pub fn is_tbssel_1(&self) -> bool {
        *self == TBSSEL_A::TBSSEL_1
    }
    #[doc = "Checks if the value of the field is `TBSSEL_2`"]
    #[inline(always)]
    pub fn is_tbssel_2(&self) -> bool {
        *self == TBSSEL_A::TBSSEL_2
    }
    #[doc = "Checks if the value of the field is `TBSSEL_3`"]
    #[inline(always)]
    pub fn is_tbssel_3(&self) -> bool {
        *self == TBSSEL_A::TBSSEL_3
    }
}
#[doc = "Write proxy for field `TBSSEL`"]
pub struct TBSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TBSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock Source: TBCLK"]
    #[inline(always)]
    pub fn tbssel_0(self) -> &'a mut W {
        self.variant(TBSSEL_A::TBSSEL_0)
    }
    #[doc = "Clock Source: ACLK"]
    #[inline(always)]
    pub fn tbssel_1(self) -> &'a mut W {
        self.variant(TBSSEL_A::TBSSEL_1)
    }
    #[doc = "Clock Source: SMCLK"]
    #[inline(always)]
    pub fn tbssel_2(self) -> &'a mut W {
        self.variant(TBSSEL_A::TBSSEL_2)
    }
    #[doc = "Clock Source: INCLK"]
    #[inline(always)]
    pub fn tbssel_3(self) -> &'a mut W {
        self.variant(TBSSEL_A::TBSSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Counter lenght 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CNTL_A {
    #[doc = "0: Counter lenght: 16 bit"]
    CNTL_0 = 0,
    #[doc = "1: Counter lenght: 12 bit"]
    CNTL_1 = 1,
    #[doc = "2: Counter lenght: 10 bit"]
    CNTL_2 = 2,
    #[doc = "3: Counter lenght:  8 bit"]
    CNTL_3 = 3,
}
impl From<CNTL_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CNTL`"]
pub type CNTL_R = crate::R<u8, CNTL_A>;
impl CNTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTL_A {
        match self.bits {
            0 => CNTL_A::CNTL_0,
            1 => CNTL_A::CNTL_1,
            2 => CNTL_A::CNTL_2,
            3 => CNTL_A::CNTL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CNTL_0`"]
    #[inline(always)]
    pub fn is_cntl_0(&self) -> bool {
        *self == CNTL_A::CNTL_0
    }
    #[doc = "Checks if the value of the field is `CNTL_1`"]
    #[inline(always)]
    pub fn is_cntl_1(&self) -> bool {
        *self == CNTL_A::CNTL_1
    }
    #[doc = "Checks if the value of the field is `CNTL_2`"]
    #[inline(always)]
    pub fn is_cntl_2(&self) -> bool {
        *self == CNTL_A::CNTL_2
    }
    #[doc = "Checks if the value of the field is `CNTL_3`"]
    #[inline(always)]
    pub fn is_cntl_3(&self) -> bool {
        *self == CNTL_A::CNTL_3
    }
}
#[doc = "Write proxy for field `CNTL`"]
pub struct CNTL_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Counter lenght: 16 bit"]
    #[inline(always)]
    pub fn cntl_0(self) -> &'a mut W {
        self.variant(CNTL_A::CNTL_0)
    }
    #[doc = "Counter lenght: 12 bit"]
    #[inline(always)]
    pub fn cntl_1(self) -> &'a mut W {
        self.variant(CNTL_A::CNTL_1)
    }
    #[doc = "Counter lenght: 10 bit"]
    #[inline(always)]
    pub fn cntl_2(self) -> &'a mut W {
        self.variant(CNTL_A::CNTL_2)
    }
    #[doc = "Counter lenght: 8 bit"]
    #[inline(always)]
    pub fn cntl_3(self) -> &'a mut W {
        self.variant(CNTL_A::CNTL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u16) & 0x03) << 11);
        self.w
    }
}
#[doc = "Timer0_B7 Compare latch load group 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TBCLGRP_A {
    #[doc = "0: Timer0_B7 Group: 0 - individually"]
    TBCLGRP_0 = 0,
    #[doc = "1: Timer0_B7 Group: 1 - 3 groups (1-2"]
    TBCLGRP_1 = 1,
    #[doc = "2: Timer0_B7 Group: 2 - 2 groups (1-3"]
    TBCLGRP_2 = 2,
    #[doc = "3: Timer0_B7 Group: 3 - 1 group (all)"]
    TBCLGRP_3 = 3,
}
impl From<TBCLGRP_A> for u8 {
    #[inline(always)]
    fn from(variant: TBCLGRP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TBCLGRP`"]
pub type TBCLGRP_R = crate::R<u8, TBCLGRP_A>;
impl TBCLGRP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBCLGRP_A {
        match self.bits {
            0 => TBCLGRP_A::TBCLGRP_0,
            1 => TBCLGRP_A::TBCLGRP_1,
            2 => TBCLGRP_A::TBCLGRP_2,
            3 => TBCLGRP_A::TBCLGRP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TBCLGRP_0`"]
    #[inline(always)]
    pub fn is_tbclgrp_0(&self) -> bool {
        *self == TBCLGRP_A::TBCLGRP_0
    }
    #[doc = "Checks if the value of the field is `TBCLGRP_1`"]
    #[inline(always)]
    pub fn is_tbclgrp_1(&self) -> bool {
        *self == TBCLGRP_A::TBCLGRP_1
    }
    #[doc = "Checks if the value of the field is `TBCLGRP_2`"]
    #[inline(always)]
    pub fn is_tbclgrp_2(&self) -> bool {
        *self == TBCLGRP_A::TBCLGRP_2
    }
    #[doc = "Checks if the value of the field is `TBCLGRP_3`"]
    #[inline(always)]
    pub fn is_tbclgrp_3(&self) -> bool {
        *self == TBCLGRP_A::TBCLGRP_3
    }
}
#[doc = "Write proxy for field `TBCLGRP`"]
pub struct TBCLGRP_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCLGRP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBCLGRP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timer0_B7 Group: 0 - individually"]
    #[inline(always)]
    pub fn tbclgrp_0(self) -> &'a mut W {
        self.variant(TBCLGRP_A::TBCLGRP_0)
    }
    #[doc = "Timer0_B7 Group: 1 - 3 groups (1-2"]
    #[inline(always)]
    pub fn tbclgrp_1(self) -> &'a mut W {
        self.variant(TBCLGRP_A::TBCLGRP_1)
    }
    #[doc = "Timer0_B7 Group: 2 - 2 groups (1-3"]
    #[inline(always)]
    pub fn tbclgrp_2(self) -> &'a mut W {
        self.variant(TBCLGRP_A::TBCLGRP_2)
    }
    #[doc = "Timer0_B7 Group: 3 - 1 group (all)"]
    #[inline(always)]
    pub fn tbclgrp_3(self) -> &'a mut W {
        self.variant(TBCLGRP_A::TBCLGRP_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u16) & 0x03) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Timer0_B7 interrupt flag"]
    #[inline(always)]
    pub fn tbifg(&self) -> TBIFG_R {
        TBIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer0_B7 interrupt enable"]
    #[inline(always)]
    pub fn tbie(&self) -> TBIE_R {
        TBIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer0_B7 counter clear"]
    #[inline(always)]
    pub fn tbclr(&self) -> TBCLR_R {
        TBCLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Timer0_B7 mode control 1"]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Timer0_B7 clock input divider 1"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Clock source 1"]
    #[inline(always)]
    pub fn tbssel(&self) -> TBSSEL_R {
        TBSSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - Counter lenght 1"]
    #[inline(always)]
    pub fn cntl(&self) -> CNTL_R {
        CNTL_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:14 - Timer0_B7 Compare latch load group 1"]
    #[inline(always)]
    pub fn tbclgrp(&self) -> TBCLGRP_R {
        TBCLGRP_R::new(((self.bits >> 13) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Timer0_B7 interrupt flag"]
    #[inline(always)]
    pub fn tbifg(&mut self) -> TBIFG_W {
        TBIFG_W { w: self }
    }
    #[doc = "Bit 1 - Timer0_B7 interrupt enable"]
    #[inline(always)]
    pub fn tbie(&mut self) -> TBIE_W {
        TBIE_W { w: self }
    }
    #[doc = "Bit 2 - Timer0_B7 counter clear"]
    #[inline(always)]
    pub fn tbclr(&mut self) -> TBCLR_W {
        TBCLR_W { w: self }
    }
    #[doc = "Bits 4:5 - Timer0_B7 mode control 1"]
    #[inline(always)]
    pub fn mc(&mut self) -> MC_W {
        MC_W { w: self }
    }
    #[doc = "Bits 6:7 - Timer0_B7 clock input divider 1"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    #[doc = "Bits 8:9 - Clock source 1"]
    #[inline(always)]
    pub fn tbssel(&mut self) -> TBSSEL_W {
        TBSSEL_W { w: self }
    }
    #[doc = "Bits 11:12 - Counter lenght 1"]
    #[inline(always)]
    pub fn cntl(&mut self) -> CNTL_W {
        CNTL_W { w: self }
    }
    #[doc = "Bits 13:14 - Timer0_B7 Compare latch load group 1"]
    #[inline(always)]
    pub fn tbclgrp(&mut self) -> TBCLGRP_W {
        TBCLGRP_W { w: self }
    }
}
