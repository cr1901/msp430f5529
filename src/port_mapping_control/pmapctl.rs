#[doc = "Register `PMAPCTL` reader"]
pub struct R(crate::R<PMAPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMAPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMAPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMAPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMAPCTL` writer"]
pub struct W(crate::W<PMAPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMAPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PMAPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMAPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMAPLOCKED` reader - Port Mapping Lock bit. Read only"]
pub struct PMAPLOCKED_R(crate::FieldReader<bool, bool>);
impl PMAPLOCKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMAPLOCKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMAPLOCKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMAPLOCKED` writer - Port Mapping Lock bit. Read only"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `PMAPRECFG` reader - Port Mapping re-configuration control bit"]
pub struct PMAPRECFG_R(crate::FieldReader<bool, bool>);
impl PMAPRECFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMAPRECFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMAPRECFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMAPRECFG` writer - Port Mapping re-configuration control bit"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Mapping control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmapctl](index.html) module"]
pub struct PMAPCTL_SPEC;
impl crate::RegisterSpec for PMAPCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pmapctl::R](R) reader structure"]
impl crate::Readable for PMAPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmapctl::W](W) writer structure"]
impl crate::Writable for PMAPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMAPCTL to value 0"]
impl crate::Resettable for PMAPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
