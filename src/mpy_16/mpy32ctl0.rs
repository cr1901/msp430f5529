#[doc = "Reader of register MPY32CTL0"]
pub type R = crate::R<u16, super::MPY32CTL0>;
#[doc = "Writer for register MPY32CTL0"]
pub type W = crate::W<u16, super::MPY32CTL0>;
#[doc = "Register MPY32CTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPY32CTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MPYC`"]
pub type MPYC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPYC`"]
pub struct MPYC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYC_W<'a> {
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
#[doc = "Reader of field `MPYFRAC`"]
pub type MPYFRAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPYFRAC`"]
pub struct MPYFRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYFRAC_W<'a> {
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
#[doc = "Reader of field `MPYSAT`"]
pub type MPYSAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPYSAT`"]
pub struct MPYSAT_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYSAT_W<'a> {
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
#[doc = "Multiplier mode Bit:0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MPYM_A {
    #[doc = "0: Multiplier mode: MPY"]
    MPYM_0 = 0,
    #[doc = "1: Multiplier mode: MPYS"]
    MPYM_1 = 1,
    #[doc = "2: Multiplier mode: MAC"]
    MPYM_2 = 2,
    #[doc = "3: Multiplier mode: MACS"]
    MPYM_3 = 3,
}
impl From<MPYM_A> for u8 {
    #[inline(always)]
    fn from(variant: MPYM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MPYM`"]
pub type MPYM_R = crate::R<u8, MPYM_A>;
impl MPYM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPYM_A {
        match self.bits {
            0 => MPYM_A::MPYM_0,
            1 => MPYM_A::MPYM_1,
            2 => MPYM_A::MPYM_2,
            3 => MPYM_A::MPYM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MPYM_0`"]
    #[inline(always)]
    pub fn is_mpym_0(&self) -> bool {
        *self == MPYM_A::MPYM_0
    }
    #[doc = "Checks if the value of the field is `MPYM_1`"]
    #[inline(always)]
    pub fn is_mpym_1(&self) -> bool {
        *self == MPYM_A::MPYM_1
    }
    #[doc = "Checks if the value of the field is `MPYM_2`"]
    #[inline(always)]
    pub fn is_mpym_2(&self) -> bool {
        *self == MPYM_A::MPYM_2
    }
    #[doc = "Checks if the value of the field is `MPYM_3`"]
    #[inline(always)]
    pub fn is_mpym_3(&self) -> bool {
        *self == MPYM_A::MPYM_3
    }
}
#[doc = "Write proxy for field `MPYM`"]
pub struct MPYM_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPYM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Multiplier mode: MPY"]
    #[inline(always)]
    pub fn mpym_0(self) -> &'a mut W {
        self.variant(MPYM_A::MPYM_0)
    }
    #[doc = "Multiplier mode: MPYS"]
    #[inline(always)]
    pub fn mpym_1(self) -> &'a mut W {
        self.variant(MPYM_A::MPYM_1)
    }
    #[doc = "Multiplier mode: MAC"]
    #[inline(always)]
    pub fn mpym_2(self) -> &'a mut W {
        self.variant(MPYM_A::MPYM_2)
    }
    #[doc = "Multiplier mode: MACS"]
    #[inline(always)]
    pub fn mpym_3(self) -> &'a mut W {
        self.variant(MPYM_A::MPYM_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `OP1_32`"]
pub type OP1_32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OP1_32`"]
pub struct OP1_32_W<'a> {
    w: &'a mut W,
}
impl<'a> OP1_32_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `OP2_32`"]
pub type OP2_32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OP2_32`"]
pub struct OP2_32_W<'a> {
    w: &'a mut W,
}
impl<'a> OP2_32_W<'a> {
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
#[doc = "Reader of field `MPYDLYWRTEN`"]
pub type MPYDLYWRTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPYDLYWRTEN`"]
pub struct MPYDLYWRTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYDLYWRTEN_W<'a> {
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
#[doc = "Reader of field `MPYDLY32`"]
pub type MPYDLY32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPYDLY32`"]
pub struct MPYDLY32_W<'a> {
    w: &'a mut W,
}
impl<'a> MPYDLY32_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Carry of the multiplier"]
    #[inline(always)]
    pub fn mpyc(&self) -> MPYC_R {
        MPYC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fractional mode"]
    #[inline(always)]
    pub fn mpyfrac(&self) -> MPYFRAC_R {
        MPYFRAC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Saturation mode"]
    #[inline(always)]
    pub fn mpysat(&self) -> MPYSAT_R {
        MPYSAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Multiplier mode Bit:0"]
    #[inline(always)]
    pub fn mpym(&self) -> MPYM_R {
        MPYM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Bit-width of operand 1 0:16Bit / 1:32Bit"]
    #[inline(always)]
    pub fn op1_32(&self) -> OP1_32_R {
        OP1_32_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bit-width of operand 2 0:16Bit / 1:32Bit"]
    #[inline(always)]
    pub fn op2_32(&self) -> OP2_32_R {
        OP2_32_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Delayed write enable"]
    #[inline(always)]
    pub fn mpydlywrten(&self) -> MPYDLYWRTEN_R {
        MPYDLYWRTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Delayed write mode"]
    #[inline(always)]
    pub fn mpydly32(&self) -> MPYDLY32_R {
        MPYDLY32_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Carry of the multiplier"]
    #[inline(always)]
    pub fn mpyc(&mut self) -> MPYC_W {
        MPYC_W { w: self }
    }
    #[doc = "Bit 2 - Fractional mode"]
    #[inline(always)]
    pub fn mpyfrac(&mut self) -> MPYFRAC_W {
        MPYFRAC_W { w: self }
    }
    #[doc = "Bit 3 - Saturation mode"]
    #[inline(always)]
    pub fn mpysat(&mut self) -> MPYSAT_W {
        MPYSAT_W { w: self }
    }
    #[doc = "Bits 4:5 - Multiplier mode Bit:0"]
    #[inline(always)]
    pub fn mpym(&mut self) -> MPYM_W {
        MPYM_W { w: self }
    }
    #[doc = "Bit 6 - Bit-width of operand 1 0:16Bit / 1:32Bit"]
    #[inline(always)]
    pub fn op1_32(&mut self) -> OP1_32_W {
        OP1_32_W { w: self }
    }
    #[doc = "Bit 7 - Bit-width of operand 2 0:16Bit / 1:32Bit"]
    #[inline(always)]
    pub fn op2_32(&mut self) -> OP2_32_W {
        OP2_32_W { w: self }
    }
    #[doc = "Bit 8 - Delayed write enable"]
    #[inline(always)]
    pub fn mpydlywrten(&mut self) -> MPYDLYWRTEN_W {
        MPYDLYWRTEN_W { w: self }
    }
    #[doc = "Bit 9 - Delayed write mode"]
    #[inline(always)]
    pub fn mpydly32(&mut self) -> MPYDLY32_W {
        MPYDLY32_W { w: self }
    }
}
