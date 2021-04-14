#[doc = "Register `FCTL4` reader"]
pub struct R(crate::R<FCTL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCTL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCTL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCTL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCTL4` writer"]
pub struct W(crate::W<FCTL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCTL4_SPEC>;
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
impl From<crate::W<FCTL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCTL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VPE` reader - Voltage Changed during Program Error Flag"]
pub struct VPE_R(crate::FieldReader<bool, bool>);
impl VPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VPE` writer - Voltage Changed during Program Error Flag"]
pub struct VPE_W<'a> {
    w: &'a mut W,
}
impl<'a> VPE_W<'a> {
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
#[doc = "Field `MGR0` reader - Marginal read 0 mode."]
pub struct MGR0_R(crate::FieldReader<bool, bool>);
impl MGR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MGR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MGR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MGR0` writer - Marginal read 0 mode."]
pub struct MGR0_W<'a> {
    w: &'a mut W,
}
impl<'a> MGR0_W<'a> {
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
#[doc = "Field `MGR1` reader - Marginal read 1 mode."]
pub struct MGR1_R(crate::FieldReader<bool, bool>);
impl MGR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MGR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MGR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MGR1` writer - Marginal read 1 mode."]
pub struct MGR1_W<'a> {
    w: &'a mut W,
}
impl<'a> MGR1_W<'a> {
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
#[doc = "Field `LOCKINFO` reader - Lock INFO Memory bit: read = 1 - Segment is locked (read only)"]
pub struct LOCKINFO_R(crate::FieldReader<bool, bool>);
impl LOCKINFO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCKINFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKINFO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKINFO` writer - Lock INFO Memory bit: read = 1 - Segment is locked (read only)"]
pub struct LOCKINFO_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKINFO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Voltage Changed during Program Error Flag"]
    #[inline(always)]
    pub fn vpe(&self) -> VPE_R {
        VPE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Marginal read 0 mode."]
    #[inline(always)]
    pub fn mgr0(&self) -> MGR0_R {
        MGR0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Marginal read 1 mode."]
    #[inline(always)]
    pub fn mgr1(&self) -> MGR1_R {
        MGR1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Lock INFO Memory bit: read = 1 - Segment is locked (read only)"]
    #[inline(always)]
    pub fn lockinfo(&self) -> LOCKINFO_R {
        LOCKINFO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Changed during Program Error Flag"]
    #[inline(always)]
    pub fn vpe(&mut self) -> VPE_W {
        VPE_W { w: self }
    }
    #[doc = "Bit 4 - Marginal read 0 mode."]
    #[inline(always)]
    pub fn mgr0(&mut self) -> MGR0_W {
        MGR0_W { w: self }
    }
    #[doc = "Bit 5 - Marginal read 1 mode."]
    #[inline(always)]
    pub fn mgr1(&mut self) -> MGR1_W {
        MGR1_W { w: self }
    }
    #[doc = "Bit 7 - Lock INFO Memory bit: read = 1 - Segment is locked (read only)"]
    #[inline(always)]
    pub fn lockinfo(&mut self) -> LOCKINFO_W {
        LOCKINFO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH Control 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctl4](index.html) module"]
pub struct FCTL4_SPEC;
impl crate::RegisterSpec for FCTL4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fctl4::R](R) reader structure"]
impl crate::Readable for FCTL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fctl4::W](W) writer structure"]
impl crate::Writable for FCTL4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCTL4 to value 0"]
impl crate::Resettable for FCTL4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
