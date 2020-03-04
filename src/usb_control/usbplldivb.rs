#[doc = "Reader of register USBPLLDIVB"]
pub type R = crate::R<u16, super::USBPLLDIVB>;
#[doc = "Writer for register USBPLLDIVB"]
pub type W = crate::W<u16, super::USBPLLDIVB>;
#[doc = "Register USBPLLDIVB `reset()`'s with value 0"]
impl crate::ResetValue for super::USBPLLDIVB {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UPMB0`"]
pub type UPMB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPMB0`"]
pub struct UPMB0_W<'a> {
    w: &'a mut W,
}
impl<'a> UPMB0_W<'a> {
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
#[doc = "Reader of field `UPMB1`"]
pub type UPMB1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPMB1`"]
pub struct UPMB1_W<'a> {
    w: &'a mut W,
}
impl<'a> UPMB1_W<'a> {
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
#[doc = "Reader of field `UPMB2`"]
pub type UPMB2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPMB2`"]
pub struct UPMB2_W<'a> {
    w: &'a mut W,
}
impl<'a> UPMB2_W<'a> {
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
#[doc = "Reader of field `UPMB3`"]
pub type UPMB3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPMB3`"]
pub struct UPMB3_W<'a> {
    w: &'a mut W,
}
impl<'a> UPMB3_W<'a> {
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
#[doc = "Reader of field `UPMB4`"]
pub type UPMB4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPMB4`"]
pub struct UPMB4_W<'a> {
    w: &'a mut W,
}
impl<'a> UPMB4_W<'a> {
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
#[doc = "Reader of field `UPMB5`"]
pub type UPMB5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPMB5`"]
pub struct UPMB5_W<'a> {
    w: &'a mut W,
}
impl<'a> UPMB5_W<'a> {
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
#[doc = "Reader of field `UPQB0`"]
pub type UPQB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPQB0`"]
pub struct UPQB0_W<'a> {
    w: &'a mut W,
}
impl<'a> UPQB0_W<'a> {
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
#[doc = "Reader of field `UPQB1`"]
pub type UPQB1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPQB1`"]
pub struct UPQB1_W<'a> {
    w: &'a mut W,
}
impl<'a> UPQB1_W<'a> {
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
#[doc = "Reader of field `UPQB2`"]
pub type UPQB2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPQB2`"]
pub struct UPQB2_W<'a> {
    w: &'a mut W,
}
impl<'a> UPQB2_W<'a> {
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
impl R {
    #[doc = "Bit 0 - USB - PLL feedback divider buffer Bit 0"]
    #[inline(always)]
    pub fn upmb0(&self) -> UPMB0_R {
        UPMB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB - PLL feedback divider buffer Bit 1"]
    #[inline(always)]
    pub fn upmb1(&self) -> UPMB1_R {
        UPMB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB - PLL feedback divider buffer Bit 2"]
    #[inline(always)]
    pub fn upmb2(&self) -> UPMB2_R {
        UPMB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB - PLL feedback divider buffer Bit 3"]
    #[inline(always)]
    pub fn upmb3(&self) -> UPMB3_R {
        UPMB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USB - PLL feedback divider buffer Bit 4"]
    #[inline(always)]
    pub fn upmb4(&self) -> UPMB4_R {
        UPMB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB - PLL feedback divider buffer Bit 5"]
    #[inline(always)]
    pub fn upmb5(&self) -> UPMB5_R {
        UPMB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USB - PLL prescale divider buffer Bit 0"]
    #[inline(always)]
    pub fn upqb0(&self) -> UPQB0_R {
        UPQB0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - USB - PLL prescale divider buffer Bit 1"]
    #[inline(always)]
    pub fn upqb1(&self) -> UPQB1_R {
        UPQB1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USB - PLL prescale divider buffer Bit 2"]
    #[inline(always)]
    pub fn upqb2(&self) -> UPQB2_R {
        UPQB2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - PLL feedback divider buffer Bit 0"]
    #[inline(always)]
    pub fn upmb0(&mut self) -> UPMB0_W {
        UPMB0_W { w: self }
    }
    #[doc = "Bit 1 - USB - PLL feedback divider buffer Bit 1"]
    #[inline(always)]
    pub fn upmb1(&mut self) -> UPMB1_W {
        UPMB1_W { w: self }
    }
    #[doc = "Bit 2 - USB - PLL feedback divider buffer Bit 2"]
    #[inline(always)]
    pub fn upmb2(&mut self) -> UPMB2_W {
        UPMB2_W { w: self }
    }
    #[doc = "Bit 3 - USB - PLL feedback divider buffer Bit 3"]
    #[inline(always)]
    pub fn upmb3(&mut self) -> UPMB3_W {
        UPMB3_W { w: self }
    }
    #[doc = "Bit 4 - USB - PLL feedback divider buffer Bit 4"]
    #[inline(always)]
    pub fn upmb4(&mut self) -> UPMB4_W {
        UPMB4_W { w: self }
    }
    #[doc = "Bit 5 - USB - PLL feedback divider buffer Bit 5"]
    #[inline(always)]
    pub fn upmb5(&mut self) -> UPMB5_W {
        UPMB5_W { w: self }
    }
    #[doc = "Bit 8 - USB - PLL prescale divider buffer Bit 0"]
    #[inline(always)]
    pub fn upqb0(&mut self) -> UPQB0_W {
        UPQB0_W { w: self }
    }
    #[doc = "Bit 9 - USB - PLL prescale divider buffer Bit 1"]
    #[inline(always)]
    pub fn upqb1(&mut self) -> UPQB1_W {
        UPQB1_W { w: self }
    }
    #[doc = "Bit 10 - USB - PLL prescale divider buffer Bit 2"]
    #[inline(always)]
    pub fn upqb2(&mut self) -> UPQB2_W {
        UPQB2_W { w: self }
    }
}
