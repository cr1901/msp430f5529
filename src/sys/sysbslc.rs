#[doc = "Register `SYSBSLC` reader"]
pub struct R(crate::R<SYSBSLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSBSLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSBSLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSBSLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSBSLC` writer"]
pub struct W(crate::W<SYSBSLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSBSLC_SPEC>;
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
impl From<crate::W<SYSBSLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSBSLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSBSLSIZE0` reader - SYS - BSL Protection Size 0"]
pub struct SYSBSLSIZE0_R(crate::FieldReader<bool, bool>);
impl SYSBSLSIZE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSBSLSIZE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSBSLSIZE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSBSLSIZE0` writer - SYS - BSL Protection Size 0"]
pub struct SYSBSLSIZE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSBSLSIZE0_W<'a> {
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
#[doc = "Field `SYSBSLSIZE1` reader - SYS - BSL Protection Size 1"]
pub struct SYSBSLSIZE1_R(crate::FieldReader<bool, bool>);
impl SYSBSLSIZE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSBSLSIZE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSBSLSIZE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSBSLSIZE1` writer - SYS - BSL Protection Size 1"]
pub struct SYSBSLSIZE1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSBSLSIZE1_W<'a> {
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
#[doc = "Field `SYSBSLR` reader - SYS - RAM assigned to BSL"]
pub struct SYSBSLR_R(crate::FieldReader<bool, bool>);
impl SYSBSLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSBSLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSBSLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSBSLR` writer - SYS - RAM assigned to BSL"]
pub struct SYSBSLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSBSLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SYSBSLOFF` reader - SYS - BSL Memory disabled"]
pub struct SYSBSLOFF_R(crate::FieldReader<bool, bool>);
impl SYSBSLOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSBSLOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSBSLOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSBSLOFF` writer - SYS - BSL Memory disabled"]
pub struct SYSBSLOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSBSLOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u16 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `SYSBSLPE` reader - SYS - BSL Memory protection enabled"]
pub struct SYSBSLPE_R(crate::FieldReader<bool, bool>);
impl SYSBSLPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSBSLPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSBSLPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSBSLPE` writer - SYS - BSL Memory protection enabled"]
pub struct SYSBSLPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSBSLPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u16 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SYS - BSL Protection Size 0"]
    #[inline(always)]
    pub fn sysbslsize0(&self) -> SYSBSLSIZE0_R {
        SYSBSLSIZE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SYS - BSL Protection Size 1"]
    #[inline(always)]
    pub fn sysbslsize1(&self) -> SYSBSLSIZE1_R {
        SYSBSLSIZE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SYS - RAM assigned to BSL"]
    #[inline(always)]
    pub fn sysbslr(&self) -> SYSBSLR_R {
        SYSBSLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SYS - BSL Memory disabled"]
    #[inline(always)]
    pub fn sysbsloff(&self) -> SYSBSLOFF_R {
        SYSBSLOFF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SYS - BSL Memory protection enabled"]
    #[inline(always)]
    pub fn sysbslpe(&self) -> SYSBSLPE_R {
        SYSBSLPE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYS - BSL Protection Size 0"]
    #[inline(always)]
    pub fn sysbslsize0(&mut self) -> SYSBSLSIZE0_W {
        SYSBSLSIZE0_W { w: self }
    }
    #[doc = "Bit 1 - SYS - BSL Protection Size 1"]
    #[inline(always)]
    pub fn sysbslsize1(&mut self) -> SYSBSLSIZE1_W {
        SYSBSLSIZE1_W { w: self }
    }
    #[doc = "Bit 2 - SYS - RAM assigned to BSL"]
    #[inline(always)]
    pub fn sysbslr(&mut self) -> SYSBSLR_W {
        SYSBSLR_W { w: self }
    }
    #[doc = "Bit 14 - SYS - BSL Memory disabled"]
    #[inline(always)]
    pub fn sysbsloff(&mut self) -> SYSBSLOFF_W {
        SYSBSLOFF_W { w: self }
    }
    #[doc = "Bit 15 - SYS - BSL Memory protection enabled"]
    #[inline(always)]
    pub fn sysbslpe(&mut self) -> SYSBSLPE_W {
        SYSBSLPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Boot strap configuration area\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysbslc](index.html) module"]
pub struct SYSBSLC_SPEC;
impl crate::RegisterSpec for SYSBSLC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sysbslc::R](R) reader structure"]
impl crate::Readable for SYSBSLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysbslc::W](W) writer structure"]
impl crate::Writable for SYSBSLC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSBSLC to value 0"]
impl crate::Resettable for SYSBSLC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
