#[doc = "Reader of register RCCTL0"]
pub type R = crate::R<u16, super::RCCTL0>;
#[doc = "Writer for register RCCTL0"]
pub type W = crate::W<u16, super::RCCTL0>;
#[doc = "Register RCCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RCCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RCRS0OFF`"]
pub type RCRS0OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCRS0OFF`"]
pub struct RCRS0OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRS0OFF_W<'a> {
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
#[doc = "Reader of field `RCRS1OFF`"]
pub type RCRS1OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCRS1OFF`"]
pub struct RCRS1OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRS1OFF_W<'a> {
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
#[doc = "Reader of field `RCRS2OFF`"]
pub type RCRS2OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCRS2OFF`"]
pub struct RCRS2OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRS2OFF_W<'a> {
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
#[doc = "Reader of field `RCRS3OFF`"]
pub type RCRS3OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCRS3OFF`"]
pub struct RCRS3OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRS3OFF_W<'a> {
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
#[doc = "Reader of field `RCRS7OFF`"]
pub type RCRS7OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCRS7OFF`"]
pub struct RCRS7OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRS7OFF_W<'a> {
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
    #[doc = "Bit 0 - RAM Controller RAM Sector 0 Off"]
    #[inline(always)]
    pub fn rcrs0off(&self) -> RCRS0OFF_R {
        RCRS0OFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RAM Controller RAM Sector 1 Off"]
    #[inline(always)]
    pub fn rcrs1off(&self) -> RCRS1OFF_R {
        RCRS1OFF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RAM Controller RAM Sector 2 Off"]
    #[inline(always)]
    pub fn rcrs2off(&self) -> RCRS2OFF_R {
        RCRS2OFF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RAM Controller RAM Sector 3 Off"]
    #[inline(always)]
    pub fn rcrs3off(&self) -> RCRS3OFF_R {
        RCRS3OFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RAM Controller RAM Sector 7 (USB) Off"]
    #[inline(always)]
    pub fn rcrs7off(&self) -> RCRS7OFF_R {
        RCRS7OFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM Controller RAM Sector 0 Off"]
    #[inline(always)]
    pub fn rcrs0off(&mut self) -> RCRS0OFF_W {
        RCRS0OFF_W { w: self }
    }
    #[doc = "Bit 1 - RAM Controller RAM Sector 1 Off"]
    #[inline(always)]
    pub fn rcrs1off(&mut self) -> RCRS1OFF_W {
        RCRS1OFF_W { w: self }
    }
    #[doc = "Bit 2 - RAM Controller RAM Sector 2 Off"]
    #[inline(always)]
    pub fn rcrs2off(&mut self) -> RCRS2OFF_W {
        RCRS2OFF_W { w: self }
    }
    #[doc = "Bit 3 - RAM Controller RAM Sector 3 Off"]
    #[inline(always)]
    pub fn rcrs3off(&mut self) -> RCRS3OFF_W {
        RCRS3OFF_W { w: self }
    }
    #[doc = "Bit 7 - RAM Controller RAM Sector 7 (USB) Off"]
    #[inline(always)]
    pub fn rcrs7off(&mut self) -> RCRS7OFF_W {
        RCRS7OFF_W { w: self }
    }
}
