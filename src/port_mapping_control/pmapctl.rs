#[doc = "Reader of register PMAPCTL"]
pub type R = crate::R<u16, super::PMAPCTL>;
#[doc = "Writer for register PMAPCTL"]
pub type W = crate::W<u16, super::PMAPCTL>;
#[doc = "Register PMAPCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::PMAPCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PMAPLOCKED`"]
pub type PMAPLOCKED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMAPLOCKED`"]
pub struct PMAPLOCKED_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAPLOCKED_W<'a> {
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
#[doc = "Reader of field `PMAPRECFG`"]
pub type PMAPRECFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMAPRECFG`"]
pub struct PMAPRECFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAPRECFG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Port Mapping Lock bit. Read only"]
    #[inline(always)]
    pub fn pmaplocked(&self) -> PMAPLOCKED_R {
        PMAPLOCKED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port Mapping re-configuration control bit"]
    #[inline(always)]
    pub fn pmaprecfg(&self) -> PMAPRECFG_R {
        PMAPRECFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Mapping Lock bit. Read only"]
    #[inline(always)]
    pub fn pmaplocked(&mut self) -> PMAPLOCKED_W {
        PMAPLOCKED_W { w: self }
    }
    #[doc = "Bit 1 - Port Mapping re-configuration control bit"]
    #[inline(always)]
    pub fn pmaprecfg(&mut self) -> PMAPRECFG_W {
        PMAPRECFG_W { w: self }
    }
}
