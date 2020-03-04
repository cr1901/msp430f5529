#[doc = "Reader of register UCSCTL6"]
pub type R = crate::R<u16, super::UCSCTL6>;
#[doc = "Writer for register UCSCTL6"]
pub type W = crate::W<u16, super::UCSCTL6>;
#[doc = "Register UCSCTL6 `reset()`'s with value 0"]
impl crate::ResetValue for super::UCSCTL6 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XT1OFF`"]
pub type XT1OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XT1OFF`"]
pub struct XT1OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> XT1OFF_W<'a> {
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
#[doc = "Reader of field `SMCLKOFF`"]
pub type SMCLKOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMCLKOFF`"]
pub struct SMCLKOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SMCLKOFF_W<'a> {
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
#[doc = "XIN/XOUT Cap Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XCAP_A {
    #[doc = "0: XIN/XOUT Cap 0"]
    XCAP_0 = 0,
    #[doc = "1: XIN/XOUT Cap 1"]
    XCAP_1 = 1,
    #[doc = "2: XIN/XOUT Cap 2"]
    XCAP_2 = 2,
    #[doc = "3: XIN/XOUT Cap 3"]
    XCAP_3 = 3,
}
impl From<XCAP_A> for u8 {
    #[inline(always)]
    fn from(variant: XCAP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `XCAP`"]
pub type XCAP_R = crate::R<u8, XCAP_A>;
impl XCAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XCAP_A {
        match self.bits {
            0 => XCAP_A::XCAP_0,
            1 => XCAP_A::XCAP_1,
            2 => XCAP_A::XCAP_2,
            3 => XCAP_A::XCAP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XCAP_0`"]
    #[inline(always)]
    pub fn is_xcap_0(&self) -> bool {
        *self == XCAP_A::XCAP_0
    }
    #[doc = "Checks if the value of the field is `XCAP_1`"]
    #[inline(always)]
    pub fn is_xcap_1(&self) -> bool {
        *self == XCAP_A::XCAP_1
    }
    #[doc = "Checks if the value of the field is `XCAP_2`"]
    #[inline(always)]
    pub fn is_xcap_2(&self) -> bool {
        *self == XCAP_A::XCAP_2
    }
    #[doc = "Checks if the value of the field is `XCAP_3`"]
    #[inline(always)]
    pub fn is_xcap_3(&self) -> bool {
        *self == XCAP_A::XCAP_3
    }
}
#[doc = "Write proxy for field `XCAP`"]
pub struct XCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> XCAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XCAP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "XIN/XOUT Cap 0"]
    #[inline(always)]
    pub fn xcap_0(self) -> &'a mut W {
        self.variant(XCAP_A::XCAP_0)
    }
    #[doc = "XIN/XOUT Cap 1"]
    #[inline(always)]
    pub fn xcap_1(self) -> &'a mut W {
        self.variant(XCAP_A::XCAP_1)
    }
    #[doc = "XIN/XOUT Cap 2"]
    #[inline(always)]
    pub fn xcap_2(self) -> &'a mut W {
        self.variant(XCAP_A::XCAP_2)
    }
    #[doc = "XIN/XOUT Cap 3"]
    #[inline(always)]
    pub fn xcap_3(self) -> &'a mut W {
        self.variant(XCAP_A::XCAP_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `XT1BYPASS`"]
pub type XT1BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XT1BYPASS`"]
pub struct XT1BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> XT1BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `XTS`"]
pub type XTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTS`"]
pub struct XTS_W<'a> {
    w: &'a mut W,
}
impl<'a> XTS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "XT1 Drive Level mode Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XT1DRIVE_A {
    #[doc = "0: XT1 Drive Level mode: 0"]
    XT1DRIVE_0 = 0,
    #[doc = "1: XT1 Drive Level mode: 1"]
    XT1DRIVE_1 = 1,
    #[doc = "2: XT1 Drive Level mode: 2"]
    XT1DRIVE_2 = 2,
    #[doc = "3: XT1 Drive Level mode: 3"]
    XT1DRIVE_3 = 3,
}
impl From<XT1DRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: XT1DRIVE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `XT1DRIVE`"]
pub type XT1DRIVE_R = crate::R<u8, XT1DRIVE_A>;
impl XT1DRIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XT1DRIVE_A {
        match self.bits {
            0 => XT1DRIVE_A::XT1DRIVE_0,
            1 => XT1DRIVE_A::XT1DRIVE_1,
            2 => XT1DRIVE_A::XT1DRIVE_2,
            3 => XT1DRIVE_A::XT1DRIVE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XT1DRIVE_0`"]
    #[inline(always)]
    pub fn is_xt1drive_0(&self) -> bool {
        *self == XT1DRIVE_A::XT1DRIVE_0
    }
    #[doc = "Checks if the value of the field is `XT1DRIVE_1`"]
    #[inline(always)]
    pub fn is_xt1drive_1(&self) -> bool {
        *self == XT1DRIVE_A::XT1DRIVE_1
    }
    #[doc = "Checks if the value of the field is `XT1DRIVE_2`"]
    #[inline(always)]
    pub fn is_xt1drive_2(&self) -> bool {
        *self == XT1DRIVE_A::XT1DRIVE_2
    }
    #[doc = "Checks if the value of the field is `XT1DRIVE_3`"]
    #[inline(always)]
    pub fn is_xt1drive_3(&self) -> bool {
        *self == XT1DRIVE_A::XT1DRIVE_3
    }
}
#[doc = "Write proxy for field `XT1DRIVE`"]
pub struct XT1DRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> XT1DRIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XT1DRIVE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "XT1 Drive Level mode: 0"]
    #[inline(always)]
    pub fn xt1drive_0(self) -> &'a mut W {
        self.variant(XT1DRIVE_A::XT1DRIVE_0)
    }
    #[doc = "XT1 Drive Level mode: 1"]
    #[inline(always)]
    pub fn xt1drive_1(self) -> &'a mut W {
        self.variant(XT1DRIVE_A::XT1DRIVE_1)
    }
    #[doc = "XT1 Drive Level mode: 2"]
    #[inline(always)]
    pub fn xt1drive_2(self) -> &'a mut W {
        self.variant(XT1DRIVE_A::XT1DRIVE_2)
    }
    #[doc = "XT1 Drive Level mode: 3"]
    #[inline(always)]
    pub fn xt1drive_3(self) -> &'a mut W {
        self.variant(XT1DRIVE_A::XT1DRIVE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `XT2OFF`"]
pub type XT2OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XT2OFF`"]
pub struct XT2OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> XT2OFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `XT2BYPASS`"]
pub type XT2BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XT2BYPASS`"]
pub struct XT2BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> XT2BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "XT2 Drive Level mode Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XT2DRIVE_A {
    #[doc = "0: XT2 Drive Level mode: 0"]
    XT2DRIVE_0 = 0,
    #[doc = "1: XT2 Drive Level mode: 1"]
    XT2DRIVE_1 = 1,
    #[doc = "2: XT2 Drive Level mode: 2"]
    XT2DRIVE_2 = 2,
    #[doc = "3: XT2 Drive Level mode: 3"]
    XT2DRIVE_3 = 3,
}
impl From<XT2DRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: XT2DRIVE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `XT2DRIVE`"]
pub type XT2DRIVE_R = crate::R<u8, XT2DRIVE_A>;
impl XT2DRIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XT2DRIVE_A {
        match self.bits {
            0 => XT2DRIVE_A::XT2DRIVE_0,
            1 => XT2DRIVE_A::XT2DRIVE_1,
            2 => XT2DRIVE_A::XT2DRIVE_2,
            3 => XT2DRIVE_A::XT2DRIVE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XT2DRIVE_0`"]
    #[inline(always)]
    pub fn is_xt2drive_0(&self) -> bool {
        *self == XT2DRIVE_A::XT2DRIVE_0
    }
    #[doc = "Checks if the value of the field is `XT2DRIVE_1`"]
    #[inline(always)]
    pub fn is_xt2drive_1(&self) -> bool {
        *self == XT2DRIVE_A::XT2DRIVE_1
    }
    #[doc = "Checks if the value of the field is `XT2DRIVE_2`"]
    #[inline(always)]
    pub fn is_xt2drive_2(&self) -> bool {
        *self == XT2DRIVE_A::XT2DRIVE_2
    }
    #[doc = "Checks if the value of the field is `XT2DRIVE_3`"]
    #[inline(always)]
    pub fn is_xt2drive_3(&self) -> bool {
        *self == XT2DRIVE_A::XT2DRIVE_3
    }
}
#[doc = "Write proxy for field `XT2DRIVE`"]
pub struct XT2DRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> XT2DRIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XT2DRIVE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "XT2 Drive Level mode: 0"]
    #[inline(always)]
    pub fn xt2drive_0(self) -> &'a mut W {
        self.variant(XT2DRIVE_A::XT2DRIVE_0)
    }
    #[doc = "XT2 Drive Level mode: 1"]
    #[inline(always)]
    pub fn xt2drive_1(self) -> &'a mut W {
        self.variant(XT2DRIVE_A::XT2DRIVE_1)
    }
    #[doc = "XT2 Drive Level mode: 2"]
    #[inline(always)]
    pub fn xt2drive_2(self) -> &'a mut W {
        self.variant(XT2DRIVE_A::XT2DRIVE_2)
    }
    #[doc = "XT2 Drive Level mode: 3"]
    #[inline(always)]
    pub fn xt2drive_3(self) -> &'a mut W {
        self.variant(XT2DRIVE_A::XT2DRIVE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u16) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - High Frequency Oscillator 1 (XT1) disable"]
    #[inline(always)]
    pub fn xt1off(&self) -> XT1OFF_R {
        XT1OFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SMCLK Off"]
    #[inline(always)]
    pub fn smclkoff(&self) -> SMCLKOFF_R {
        SMCLKOFF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - XIN/XOUT Cap Bit: 0"]
    #[inline(always)]
    pub fn xcap(&self) -> XCAP_R {
        XCAP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - XT1 bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn xt1bypass(&self) -> XT1BYPASS_R {
        XT1BYPASS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 1: Selects high-freq. oscillator"]
    #[inline(always)]
    pub fn xts(&self) -> XTS_R {
        XTS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - XT1 Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn xt1drive(&self) -> XT1DRIVE_R {
        XT1DRIVE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - High Frequency Oscillator 2 (XT2) disable"]
    #[inline(always)]
    pub fn xt2off(&self) -> XT2OFF_R {
        XT2OFF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - XT2 bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn xt2bypass(&self) -> XT2BYPASS_R {
        XT2BYPASS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - XT2 Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn xt2drive(&self) -> XT2DRIVE_R {
        XT2DRIVE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - High Frequency Oscillator 1 (XT1) disable"]
    #[inline(always)]
    pub fn xt1off(&mut self) -> XT1OFF_W {
        XT1OFF_W { w: self }
    }
    #[doc = "Bit 1 - SMCLK Off"]
    #[inline(always)]
    pub fn smclkoff(&mut self) -> SMCLKOFF_W {
        SMCLKOFF_W { w: self }
    }
    #[doc = "Bits 2:3 - XIN/XOUT Cap Bit: 0"]
    #[inline(always)]
    pub fn xcap(&mut self) -> XCAP_W {
        XCAP_W { w: self }
    }
    #[doc = "Bit 4 - XT1 bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn xt1bypass(&mut self) -> XT1BYPASS_W {
        XT1BYPASS_W { w: self }
    }
    #[doc = "Bit 5 - 1: Selects high-freq. oscillator"]
    #[inline(always)]
    pub fn xts(&mut self) -> XTS_W {
        XTS_W { w: self }
    }
    #[doc = "Bits 6:7 - XT1 Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn xt1drive(&mut self) -> XT1DRIVE_W {
        XT1DRIVE_W { w: self }
    }
    #[doc = "Bit 8 - High Frequency Oscillator 2 (XT2) disable"]
    #[inline(always)]
    pub fn xt2off(&mut self) -> XT2OFF_W {
        XT2OFF_W { w: self }
    }
    #[doc = "Bit 12 - XT2 bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn xt2bypass(&mut self) -> XT2BYPASS_W {
        XT2BYPASS_W { w: self }
    }
    #[doc = "Bits 14:15 - XT2 Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn xt2drive(&mut self) -> XT2DRIVE_W {
        XT2DRIVE_W { w: self }
    }
}
