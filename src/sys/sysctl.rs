#[doc = "Register `SYSCTL` reader"]
pub struct R(crate::R<SYSCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCTL` writer"]
pub struct W(crate::W<SYSCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCTL_SPEC>;
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
impl From<crate::W<SYSCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSRIVECT` reader - SYS - RAM based interrupt vectors"]
pub struct SYSRIVECT_R(crate::FieldReader<bool, bool>);
impl SYSRIVECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSRIVECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSRIVECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSRIVECT` writer - SYS - RAM based interrupt vectors"]
pub struct SYSRIVECT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRIVECT_W<'a> {
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
#[doc = "Field `SYSPMMPE` reader - SYS - PMM access protect"]
pub struct SYSPMMPE_R(crate::FieldReader<bool, bool>);
impl SYSPMMPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSPMMPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSPMMPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSPMMPE` writer - SYS - PMM access protect"]
pub struct SYSPMMPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSPMMPE_W<'a> {
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
#[doc = "Field `SYSBSLIND` reader - SYS - TCK/RST indication detected"]
pub struct SYSBSLIND_R(crate::FieldReader<bool, bool>);
impl SYSBSLIND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSBSLIND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSBSLIND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSBSLIND` writer - SYS - TCK/RST indication detected"]
pub struct SYSBSLIND_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSBSLIND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `SYSJTAGPIN` reader - SYS - Dedicated JTAG pins enabled"]
pub struct SYSJTAGPIN_R(crate::FieldReader<bool, bool>);
impl SYSJTAGPIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSJTAGPIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSJTAGPIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSJTAGPIN` writer - SYS - Dedicated JTAG pins enabled"]
pub struct SYSJTAGPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSJTAGPIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SYS - RAM based interrupt vectors"]
    #[inline(always)]
    pub fn sysrivect(&self) -> SYSRIVECT_R {
        SYSRIVECT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - SYS - PMM access protect"]
    #[inline(always)]
    pub fn syspmmpe(&self) -> SYSPMMPE_R {
        SYSPMMPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SYS - TCK/RST indication detected"]
    #[inline(always)]
    pub fn sysbslind(&self) -> SYSBSLIND_R {
        SYSBSLIND_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SYS - Dedicated JTAG pins enabled"]
    #[inline(always)]
    pub fn sysjtagpin(&self) -> SYSJTAGPIN_R {
        SYSJTAGPIN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYS - RAM based interrupt vectors"]
    #[inline(always)]
    pub fn sysrivect(&mut self) -> SYSRIVECT_W {
        SYSRIVECT_W { w: self }
    }
    #[doc = "Bit 2 - SYS - PMM access protect"]
    #[inline(always)]
    pub fn syspmmpe(&mut self) -> SYSPMMPE_W {
        SYSPMMPE_W { w: self }
    }
    #[doc = "Bit 4 - SYS - TCK/RST indication detected"]
    #[inline(always)]
    pub fn sysbslind(&mut self) -> SYSBSLIND_W {
        SYSBSLIND_W { w: self }
    }
    #[doc = "Bit 5 - SYS - Dedicated JTAG pins enabled"]
    #[inline(always)]
    pub fn sysjtagpin(&mut self) -> SYSJTAGPIN_W {
        SYSJTAGPIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctl](index.html) module"]
pub struct SYSCTL_SPEC;
impl crate::RegisterSpec for SYSCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sysctl::R](R) reader structure"]
impl crate::Readable for SYSCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysctl::W](W) writer structure"]
impl crate::Writable for SYSCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCTL to value 0"]
impl crate::Resettable for SYSCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
