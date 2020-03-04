#[doc = "Reader of register PMMCTL0"]
pub type R = crate::R<u16, super::PMMCTL0>;
#[doc = "Writer for register PMMCTL0"]
pub type W = crate::W<u16, super::PMMCTL0>;
#[doc = "Register PMMCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PMMCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PMM Core Voltage Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PMMCOREV_A {
    #[doc = "0: PMM Core Voltage 0 (1.35V)"]
    PMMCOREV_0 = 0,
    #[doc = "1: PMM Core Voltage 1 (1.55V)"]
    PMMCOREV_1 = 1,
    #[doc = "2: PMM Core Voltage 2 (1.75V)"]
    PMMCOREV_2 = 2,
    #[doc = "3: PMM Core Voltage 3 (1.85V)"]
    PMMCOREV_3 = 3,
}
impl From<PMMCOREV_A> for u8 {
    #[inline(always)]
    fn from(variant: PMMCOREV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PMMCOREV`"]
pub type PMMCOREV_R = crate::R<u8, PMMCOREV_A>;
impl PMMCOREV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMMCOREV_A {
        match self.bits {
            0 => PMMCOREV_A::PMMCOREV_0,
            1 => PMMCOREV_A::PMMCOREV_1,
            2 => PMMCOREV_A::PMMCOREV_2,
            3 => PMMCOREV_A::PMMCOREV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PMMCOREV_0`"]
    #[inline(always)]
    pub fn is_pmmcorev_0(&self) -> bool {
        *self == PMMCOREV_A::PMMCOREV_0
    }
    #[doc = "Checks if the value of the field is `PMMCOREV_1`"]
    #[inline(always)]
    pub fn is_pmmcorev_1(&self) -> bool {
        *self == PMMCOREV_A::PMMCOREV_1
    }
    #[doc = "Checks if the value of the field is `PMMCOREV_2`"]
    #[inline(always)]
    pub fn is_pmmcorev_2(&self) -> bool {
        *self == PMMCOREV_A::PMMCOREV_2
    }
    #[doc = "Checks if the value of the field is `PMMCOREV_3`"]
    #[inline(always)]
    pub fn is_pmmcorev_3(&self) -> bool {
        *self == PMMCOREV_A::PMMCOREV_3
    }
}
#[doc = "Write proxy for field `PMMCOREV`"]
pub struct PMMCOREV_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMCOREV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMMCOREV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PMM Core Voltage 0 (1.35V)"]
    #[inline(always)]
    pub fn pmmcorev_0(self) -> &'a mut W {
        self.variant(PMMCOREV_A::PMMCOREV_0)
    }
    #[doc = "PMM Core Voltage 1 (1.55V)"]
    #[inline(always)]
    pub fn pmmcorev_1(self) -> &'a mut W {
        self.variant(PMMCOREV_A::PMMCOREV_1)
    }
    #[doc = "PMM Core Voltage 2 (1.75V)"]
    #[inline(always)]
    pub fn pmmcorev_2(self) -> &'a mut W {
        self.variant(PMMCOREV_A::PMMCOREV_2)
    }
    #[doc = "PMM Core Voltage 3 (1.85V)"]
    #[inline(always)]
    pub fn pmmcorev_3(self) -> &'a mut W {
        self.variant(PMMCOREV_A::PMMCOREV_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PMMSWBOR`"]
pub type PMMSWBOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMMSWBOR`"]
pub struct PMMSWBOR_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMSWBOR_W<'a> {
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
#[doc = "Reader of field `PMMSWPOR`"]
pub type PMMSWPOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMMSWPOR`"]
pub struct PMMSWPOR_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMSWPOR_W<'a> {
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
#[doc = "Reader of field `PMMREGOFF`"]
pub type PMMREGOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMMREGOFF`"]
pub struct PMMREGOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMREGOFF_W<'a> {
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
#[doc = "Reader of field `PMMHPMRE`"]
pub type PMMHPMRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMMHPMRE`"]
pub struct PMMHPMRE_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMHPMRE_W<'a> {
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
    #[doc = "Bits 0:1 - PMM Core Voltage Bit: 0"]
    #[inline(always)]
    pub fn pmmcorev(&self) -> PMMCOREV_R {
        PMMCOREV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - PMM Software BOR"]
    #[inline(always)]
    pub fn pmmswbor(&self) -> PMMSWBOR_R {
        PMMSWBOR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PMM Software POR"]
    #[inline(always)]
    pub fn pmmswpor(&self) -> PMMSWPOR_R {
        PMMSWPOR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PMM Turn Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&self) -> PMMREGOFF_R {
        PMMREGOFF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PMM Global High Power Module Request Enable"]
    #[inline(always)]
    pub fn pmmhpmre(&self) -> PMMHPMRE_R {
        PMMHPMRE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PMM Core Voltage Bit: 0"]
    #[inline(always)]
    pub fn pmmcorev(&mut self) -> PMMCOREV_W {
        PMMCOREV_W { w: self }
    }
    #[doc = "Bit 2 - PMM Software BOR"]
    #[inline(always)]
    pub fn pmmswbor(&mut self) -> PMMSWBOR_W {
        PMMSWBOR_W { w: self }
    }
    #[doc = "Bit 3 - PMM Software POR"]
    #[inline(always)]
    pub fn pmmswpor(&mut self) -> PMMSWPOR_W {
        PMMSWPOR_W { w: self }
    }
    #[doc = "Bit 4 - PMM Turn Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&mut self) -> PMMREGOFF_W {
        PMMREGOFF_W { w: self }
    }
    #[doc = "Bit 7 - PMM Global High Power Module Request Enable"]
    #[inline(always)]
    pub fn pmmhpmre(&mut self) -> PMMHPMRE_W {
        PMMHPMRE_W { w: self }
    }
}
