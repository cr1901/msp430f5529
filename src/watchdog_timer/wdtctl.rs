#[doc = "Reader of register WDTCTL"]
pub type R = crate::R<u16, super::WDTCTL>;
#[doc = "Writer for register WDTCTL"]
pub type W = crate::W<u16, super::WDTCTL>;
#[doc = "Register WDTCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::WDTCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "WDT - Timer Interval Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDTIS_A {
    #[doc = "0: WDT - Timer Interval Select: /2G"]
    WDTIS_0 = 0,
    #[doc = "1: WDT - Timer Interval Select: /128M"]
    WDTIS_1 = 1,
    #[doc = "2: WDT - Timer Interval Select: /8192k"]
    WDTIS_2 = 2,
    #[doc = "3: WDT - Timer Interval Select: /512k"]
    WDTIS_3 = 3,
    #[doc = "4: WDT - Timer Interval Select: /32k"]
    WDTIS_4 = 4,
    #[doc = "5: WDT - Timer Interval Select: /8192"]
    WDTIS_5 = 5,
    #[doc = "6: WDT - Timer Interval Select: /512"]
    WDTIS_6 = 6,
    #[doc = "7: WDT - Timer Interval Select: /64"]
    WDTIS_7 = 7,
}
impl From<WDTIS_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTIS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WDTIS`"]
pub type WDTIS_R = crate::R<u8, WDTIS_A>;
impl WDTIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTIS_A {
        match self.bits {
            0 => WDTIS_A::WDTIS_0,
            1 => WDTIS_A::WDTIS_1,
            2 => WDTIS_A::WDTIS_2,
            3 => WDTIS_A::WDTIS_3,
            4 => WDTIS_A::WDTIS_4,
            5 => WDTIS_A::WDTIS_5,
            6 => WDTIS_A::WDTIS_6,
            7 => WDTIS_A::WDTIS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WDTIS_0`"]
    #[inline(always)]
    pub fn is_wdtis_0(&self) -> bool {
        *self == WDTIS_A::WDTIS_0
    }
    #[doc = "Checks if the value of the field is `WDTIS_1`"]
    #[inline(always)]
    pub fn is_wdtis_1(&self) -> bool {
        *self == WDTIS_A::WDTIS_1
    }
    #[doc = "Checks if the value of the field is `WDTIS_2`"]
    #[inline(always)]
    pub fn is_wdtis_2(&self) -> bool {
        *self == WDTIS_A::WDTIS_2
    }
    #[doc = "Checks if the value of the field is `WDTIS_3`"]
    #[inline(always)]
    pub fn is_wdtis_3(&self) -> bool {
        *self == WDTIS_A::WDTIS_3
    }
    #[doc = "Checks if the value of the field is `WDTIS_4`"]
    #[inline(always)]
    pub fn is_wdtis_4(&self) -> bool {
        *self == WDTIS_A::WDTIS_4
    }
    #[doc = "Checks if the value of the field is `WDTIS_5`"]
    #[inline(always)]
    pub fn is_wdtis_5(&self) -> bool {
        *self == WDTIS_A::WDTIS_5
    }
    #[doc = "Checks if the value of the field is `WDTIS_6`"]
    #[inline(always)]
    pub fn is_wdtis_6(&self) -> bool {
        *self == WDTIS_A::WDTIS_6
    }
    #[doc = "Checks if the value of the field is `WDTIS_7`"]
    #[inline(always)]
    pub fn is_wdtis_7(&self) -> bool {
        *self == WDTIS_A::WDTIS_7
    }
}
#[doc = "Write proxy for field `WDTIS`"]
pub struct WDTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTIS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "WDT - Timer Interval Select: /2G"]
    #[inline(always)]
    pub fn wdtis_0(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_0)
    }
    #[doc = "WDT - Timer Interval Select: /128M"]
    #[inline(always)]
    pub fn wdtis_1(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_1)
    }
    #[doc = "WDT - Timer Interval Select: /8192k"]
    #[inline(always)]
    pub fn wdtis_2(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_2)
    }
    #[doc = "WDT - Timer Interval Select: /512k"]
    #[inline(always)]
    pub fn wdtis_3(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_3)
    }
    #[doc = "WDT - Timer Interval Select: /32k"]
    #[inline(always)]
    pub fn wdtis_4(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_4)
    }
    #[doc = "WDT - Timer Interval Select: /8192"]
    #[inline(always)]
    pub fn wdtis_5(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_5)
    }
    #[doc = "WDT - Timer Interval Select: /512"]
    #[inline(always)]
    pub fn wdtis_6(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_6)
    }
    #[doc = "WDT - Timer Interval Select: /64"]
    #[inline(always)]
    pub fn wdtis_7(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `WDTCNTCL`"]
pub type WDTCNTCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTCNTCL`"]
pub struct WDTCNTCL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTCNTCL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `WDTTMSEL`"]
pub type WDTTMSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTTMSEL`"]
pub struct WDTTMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTTMSEL_W<'a> {
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
#[doc = "WDT - Timer Clock Source Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDTSSEL_A {
    #[doc = "0: WDT - Timer Clock Source Select: SMCLK"]
    WDTSSEL_0 = 0,
    #[doc = "1: WDT - Timer Clock Source Select: ACLK"]
    WDTSSEL_1 = 1,
    #[doc = "2: WDT - Timer Clock Source Select: VLO_CLK"]
    WDTSSEL_2 = 2,
    #[doc = "3: WDT - Timer Clock Source Select: reserved"]
    WDTSSEL_3 = 3,
}
impl From<WDTSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WDTSSEL`"]
pub type WDTSSEL_R = crate::R<u8, WDTSSEL_A>;
impl WDTSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTSSEL_A {
        match self.bits {
            0 => WDTSSEL_A::WDTSSEL_0,
            1 => WDTSSEL_A::WDTSSEL_1,
            2 => WDTSSEL_A::WDTSSEL_2,
            3 => WDTSSEL_A::WDTSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WDTSSEL_0`"]
    #[inline(always)]
    pub fn is_wdtssel_0(&self) -> bool {
        *self == WDTSSEL_A::WDTSSEL_0
    }
    #[doc = "Checks if the value of the field is `WDTSSEL_1`"]
    #[inline(always)]
    pub fn is_wdtssel_1(&self) -> bool {
        *self == WDTSSEL_A::WDTSSEL_1
    }
    #[doc = "Checks if the value of the field is `WDTSSEL_2`"]
    #[inline(always)]
    pub fn is_wdtssel_2(&self) -> bool {
        *self == WDTSSEL_A::WDTSSEL_2
    }
    #[doc = "Checks if the value of the field is `WDTSSEL_3`"]
    #[inline(always)]
    pub fn is_wdtssel_3(&self) -> bool {
        *self == WDTSSEL_A::WDTSSEL_3
    }
}
#[doc = "Write proxy for field `WDTSSEL`"]
pub struct WDTSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "WDT - Timer Clock Source Select: SMCLK"]
    #[inline(always)]
    pub fn wdtssel_0(self) -> &'a mut W {
        self.variant(WDTSSEL_A::WDTSSEL_0)
    }
    #[doc = "WDT - Timer Clock Source Select: ACLK"]
    #[inline(always)]
    pub fn wdtssel_1(self) -> &'a mut W {
        self.variant(WDTSSEL_A::WDTSSEL_1)
    }
    #[doc = "WDT - Timer Clock Source Select: VLO_CLK"]
    #[inline(always)]
    pub fn wdtssel_2(self) -> &'a mut W {
        self.variant(WDTSSEL_A::WDTSSEL_2)
    }
    #[doc = "WDT - Timer Clock Source Select: reserved"]
    #[inline(always)]
    pub fn wdtssel_3(self) -> &'a mut W {
        self.variant(WDTSSEL_A::WDTSSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u16) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `WDTHOLD`"]
pub type WDTHOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTHOLD`"]
pub struct WDTHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTHOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - WDT - Timer Interval Select 0"]
    #[inline(always)]
    pub fn wdtis(&self) -> WDTIS_R {
        WDTIS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - WDT - Timer Clear"]
    #[inline(always)]
    pub fn wdtcntcl(&self) -> WDTCNTCL_R {
        WDTCNTCL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WDT - Timer Mode Select"]
    #[inline(always)]
    pub fn wdttmsel(&self) -> WDTTMSEL_R {
        WDTTMSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - WDT - Timer Clock Source Select 0"]
    #[inline(always)]
    pub fn wdtssel(&self) -> WDTSSEL_R {
        WDTSSEL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - WDT - Timer hold"]
    #[inline(always)]
    pub fn wdthold(&self) -> WDTHOLD_R {
        WDTHOLD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - WDT - Timer Interval Select 0"]
    #[inline(always)]
    pub fn wdtis(&mut self) -> WDTIS_W {
        WDTIS_W { w: self }
    }
    #[doc = "Bit 3 - WDT - Timer Clear"]
    #[inline(always)]
    pub fn wdtcntcl(&mut self) -> WDTCNTCL_W {
        WDTCNTCL_W { w: self }
    }
    #[doc = "Bit 4 - WDT - Timer Mode Select"]
    #[inline(always)]
    pub fn wdttmsel(&mut self) -> WDTTMSEL_W {
        WDTTMSEL_W { w: self }
    }
    #[doc = "Bits 5:6 - WDT - Timer Clock Source Select 0"]
    #[inline(always)]
    pub fn wdtssel(&mut self) -> WDTSSEL_W {
        WDTSSEL_W { w: self }
    }
    #[doc = "Bit 7 - WDT - Timer hold"]
    #[inline(always)]
    pub fn wdthold(&mut self) -> WDTHOLD_W {
        WDTHOLD_W { w: self }
    }
}
