#[doc = "Register `USBPHYCTL` reader"]
pub struct R(crate::R<USBPHYCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPHYCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPHYCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPHYCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPHYCTL` writer"]
pub struct W(crate::W<USBPHYCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPHYCTL_SPEC>;
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
impl From<crate::W<USBPHYCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPHYCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PUOUT0` reader - USB - USB Port Output Signal Bit 0"]
pub struct PUOUT0_R(crate::FieldReader<bool, bool>);
impl PUOUT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PUOUT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUOUT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUOUT0` writer - USB - USB Port Output Signal Bit 0"]
pub struct PUOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PUOUT0_W<'a> {
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
#[doc = "Field `PUOUT1` reader - USB - USB Port Output Signal Bit 1"]
pub struct PUOUT1_R(crate::FieldReader<bool, bool>);
impl PUOUT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PUOUT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUOUT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUOUT1` writer - USB - USB Port Output Signal Bit 1"]
pub struct PUOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PUOUT1_W<'a> {
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
#[doc = "Field `PUIN0` reader - USB - PU0/DP Input Data"]
pub struct PUIN0_R(crate::FieldReader<bool, bool>);
impl PUIN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PUIN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUIN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUIN0` writer - USB - PU0/DP Input Data"]
pub struct PUIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PUIN0_W<'a> {
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
#[doc = "Field `PUIN1` reader - USB - PU1/DM Input Data"]
pub struct PUIN1_R(crate::FieldReader<bool, bool>);
impl PUIN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PUIN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUIN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUIN1` writer - USB - PU1/DM Input Data"]
pub struct PUIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PUIN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `PUOPE` reader - USB - USB Port Output Enable"]
pub struct PUOPE_R(crate::FieldReader<bool, bool>);
impl PUOPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PUOPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUOPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUOPE` writer - USB - USB Port Output Enable"]
pub struct PUOPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PUOPE_W<'a> {
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
#[doc = "Field `PUSEL` reader - USB - USB Port Function Select"]
pub struct PUSEL_R(crate::FieldReader<bool, bool>);
impl PUSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PUSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUSEL` writer - USB - USB Port Function Select"]
pub struct PUSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PUSEL_W<'a> {
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
#[doc = "Field `PUIPE` reader - USB - PHY Single Ended Input enable"]
pub struct PUIPE_R(crate::FieldReader<bool, bool>);
impl PUIPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PUIPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUIPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUIPE` writer - USB - PHY Single Ended Input enable"]
pub struct PUIPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PUIPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB - USB Port Output Signal Bit 0"]
    #[inline(always)]
    pub fn puout0(&self) -> PUOUT0_R {
        PUOUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB - USB Port Output Signal Bit 1"]
    #[inline(always)]
    pub fn puout1(&self) -> PUOUT1_R {
        PUOUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB - PU0/DP Input Data"]
    #[inline(always)]
    pub fn puin0(&self) -> PUIN0_R {
        PUIN0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB - PU1/DM Input Data"]
    #[inline(always)]
    pub fn puin1(&self) -> PUIN1_R {
        PUIN1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB - USB Port Output Enable"]
    #[inline(always)]
    pub fn puope(&self) -> PUOPE_R {
        PUOPE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB - USB Port Function Select"]
    #[inline(always)]
    pub fn pusel(&self) -> PUSEL_R {
        PUSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USB - PHY Single Ended Input enable"]
    #[inline(always)]
    pub fn puipe(&self) -> PUIPE_R {
        PUIPE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - USB Port Output Signal Bit 0"]
    #[inline(always)]
    pub fn puout0(&mut self) -> PUOUT0_W {
        PUOUT0_W { w: self }
    }
    #[doc = "Bit 1 - USB - USB Port Output Signal Bit 1"]
    #[inline(always)]
    pub fn puout1(&mut self) -> PUOUT1_W {
        PUOUT1_W { w: self }
    }
    #[doc = "Bit 2 - USB - PU0/DP Input Data"]
    #[inline(always)]
    pub fn puin0(&mut self) -> PUIN0_W {
        PUIN0_W { w: self }
    }
    #[doc = "Bit 3 - USB - PU1/DM Input Data"]
    #[inline(always)]
    pub fn puin1(&mut self) -> PUIN1_W {
        PUIN1_W { w: self }
    }
    #[doc = "Bit 5 - USB - USB Port Output Enable"]
    #[inline(always)]
    pub fn puope(&mut self) -> PUOPE_W {
        PUOPE_W { w: self }
    }
    #[doc = "Bit 7 - USB - USB Port Function Select"]
    #[inline(always)]
    pub fn pusel(&mut self) -> PUSEL_W {
        PUSEL_W { w: self }
    }
    #[doc = "Bit 8 - USB - PHY Single Ended Input enable"]
    #[inline(always)]
    pub fn puipe(&mut self) -> PUIPE_W {
        PUIPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbphyctl](index.html) module"]
pub struct USBPHYCTL_SPEC;
impl crate::RegisterSpec for USBPHYCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbphyctl::R](R) reader structure"]
impl crate::Readable for USBPHYCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbphyctl::W](W) writer structure"]
impl crate::Writable for USBPHYCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBPHYCTL to value 0"]
impl crate::Resettable for USBPHYCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
