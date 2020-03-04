#[doc = "Reader of register ADC12IFG"]
pub type R = crate::R<u16, super::ADC12IFG>;
#[doc = "Writer for register ADC12IFG"]
pub type W = crate::W<u16, super::ADC12IFG>;
#[doc = "Register ADC12IFG `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12IFG {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC12IFG0`"]
pub type ADC12IFG0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG0`"]
pub struct ADC12IFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG0_W<'a> {
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
#[doc = "Reader of field `ADC12IFG1`"]
pub type ADC12IFG1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG1`"]
pub struct ADC12IFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG1_W<'a> {
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
#[doc = "Reader of field `ADC12IFG2`"]
pub type ADC12IFG2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG2`"]
pub struct ADC12IFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG2_W<'a> {
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
#[doc = "Reader of field `ADC12IFG3`"]
pub type ADC12IFG3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG3`"]
pub struct ADC12IFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG3_W<'a> {
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
#[doc = "Reader of field `ADC12IFG4`"]
pub type ADC12IFG4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG4`"]
pub struct ADC12IFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG4_W<'a> {
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
#[doc = "Reader of field `ADC12IFG5`"]
pub type ADC12IFG5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG5`"]
pub struct ADC12IFG5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG5_W<'a> {
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
#[doc = "Reader of field `ADC12IFG6`"]
pub type ADC12IFG6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG6`"]
pub struct ADC12IFG6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG6_W<'a> {
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
#[doc = "Reader of field `ADC12IFG7`"]
pub type ADC12IFG7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG7`"]
pub struct ADC12IFG7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG7_W<'a> {
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
#[doc = "Reader of field `ADC12IFG8`"]
pub type ADC12IFG8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG8`"]
pub struct ADC12IFG8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG8_W<'a> {
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
#[doc = "Reader of field `ADC12IFG9`"]
pub type ADC12IFG9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG9`"]
pub struct ADC12IFG9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG9_W<'a> {
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
#[doc = "Reader of field `ADC12IFG10`"]
pub type ADC12IFG10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG10`"]
pub struct ADC12IFG10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `ADC12IFG11`"]
pub type ADC12IFG11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG11`"]
pub struct ADC12IFG11_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `ADC12IFG12`"]
pub type ADC12IFG12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG12`"]
pub struct ADC12IFG12_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG12_W<'a> {
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
#[doc = "Reader of field `ADC12IFG13`"]
pub type ADC12IFG13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG13`"]
pub struct ADC12IFG13_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `ADC12IFG14`"]
pub type ADC12IFG14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG14`"]
pub struct ADC12IFG14_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `ADC12IFG15`"]
pub type ADC12IFG15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG15`"]
pub struct ADC12IFG15_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADC12 Memory 0 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg0(&self) -> ADC12IFG0_R {
        ADC12IFG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC12 Memory 1 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg1(&self) -> ADC12IFG1_R {
        ADC12IFG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC12 Memory 2 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg2(&self) -> ADC12IFG2_R {
        ADC12IFG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC12 Memory 3 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg3(&self) -> ADC12IFG3_R {
        ADC12IFG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC12 Memory 4 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg4(&self) -> ADC12IFG4_R {
        ADC12IFG4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC12 Memory 5 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg5(&self) -> ADC12IFG5_R {
        ADC12IFG5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC12 Memory 6 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg6(&self) -> ADC12IFG6_R {
        ADC12IFG6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC12 Memory 7 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg7(&self) -> ADC12IFG7_R {
        ADC12IFG7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC12 Memory 8 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg8(&self) -> ADC12IFG8_R {
        ADC12IFG8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC12 Memory 9 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg9(&self) -> ADC12IFG9_R {
        ADC12IFG9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADC12 Memory 10 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg10(&self) -> ADC12IFG10_R {
        ADC12IFG10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADC12 Memory 11 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg11(&self) -> ADC12IFG11_R {
        ADC12IFG11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ADC12 Memory 12 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg12(&self) -> ADC12IFG12_R {
        ADC12IFG12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ADC12 Memory 13 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg13(&self) -> ADC12IFG13_R {
        ADC12IFG13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ADC12 Memory 14 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg14(&self) -> ADC12IFG14_R {
        ADC12IFG14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADC12 Memory 15 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg15(&self) -> ADC12IFG15_R {
        ADC12IFG15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12 Memory 0 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg0(&mut self) -> ADC12IFG0_W {
        ADC12IFG0_W { w: self }
    }
    #[doc = "Bit 1 - ADC12 Memory 1 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg1(&mut self) -> ADC12IFG1_W {
        ADC12IFG1_W { w: self }
    }
    #[doc = "Bit 2 - ADC12 Memory 2 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg2(&mut self) -> ADC12IFG2_W {
        ADC12IFG2_W { w: self }
    }
    #[doc = "Bit 3 - ADC12 Memory 3 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg3(&mut self) -> ADC12IFG3_W {
        ADC12IFG3_W { w: self }
    }
    #[doc = "Bit 4 - ADC12 Memory 4 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg4(&mut self) -> ADC12IFG4_W {
        ADC12IFG4_W { w: self }
    }
    #[doc = "Bit 5 - ADC12 Memory 5 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg5(&mut self) -> ADC12IFG5_W {
        ADC12IFG5_W { w: self }
    }
    #[doc = "Bit 6 - ADC12 Memory 6 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg6(&mut self) -> ADC12IFG6_W {
        ADC12IFG6_W { w: self }
    }
    #[doc = "Bit 7 - ADC12 Memory 7 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg7(&mut self) -> ADC12IFG7_W {
        ADC12IFG7_W { w: self }
    }
    #[doc = "Bit 8 - ADC12 Memory 8 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg8(&mut self) -> ADC12IFG8_W {
        ADC12IFG8_W { w: self }
    }
    #[doc = "Bit 9 - ADC12 Memory 9 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg9(&mut self) -> ADC12IFG9_W {
        ADC12IFG9_W { w: self }
    }
    #[doc = "Bit 10 - ADC12 Memory 10 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg10(&mut self) -> ADC12IFG10_W {
        ADC12IFG10_W { w: self }
    }
    #[doc = "Bit 11 - ADC12 Memory 11 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg11(&mut self) -> ADC12IFG11_W {
        ADC12IFG11_W { w: self }
    }
    #[doc = "Bit 12 - ADC12 Memory 12 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg12(&mut self) -> ADC12IFG12_W {
        ADC12IFG12_W { w: self }
    }
    #[doc = "Bit 13 - ADC12 Memory 13 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg13(&mut self) -> ADC12IFG13_W {
        ADC12IFG13_W { w: self }
    }
    #[doc = "Bit 14 - ADC12 Memory 14 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg14(&mut self) -> ADC12IFG14_W {
        ADC12IFG14_W { w: self }
    }
    #[doc = "Bit 15 - ADC12 Memory 15 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg15(&mut self) -> ADC12IFG15_W {
        ADC12IFG15_W { w: self }
    }
}
