#[doc = "Register `USBPLLDIVB` reader"]
pub struct R(crate::R<USBPLLDIVB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPLLDIVB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPLLDIVB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPLLDIVB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPLLDIVB` writer"]
pub struct W(crate::W<USBPLLDIVB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPLLDIVB_SPEC>;
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
impl From<crate::W<USBPLLDIVB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPLLDIVB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPMB0` reader - USB - PLL feedback divider buffer Bit 0"]
pub struct UPMB0_R(crate::FieldReader<bool, bool>);
impl UPMB0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPMB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPMB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPMB0` writer - USB - PLL feedback divider buffer Bit 0"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `UPMB1` reader - USB - PLL feedback divider buffer Bit 1"]
pub struct UPMB1_R(crate::FieldReader<bool, bool>);
impl UPMB1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPMB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPMB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPMB1` writer - USB - PLL feedback divider buffer Bit 1"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `UPMB2` reader - USB - PLL feedback divider buffer Bit 2"]
pub struct UPMB2_R(crate::FieldReader<bool, bool>);
impl UPMB2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPMB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPMB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPMB2` writer - USB - PLL feedback divider buffer Bit 2"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `UPMB3` reader - USB - PLL feedback divider buffer Bit 3"]
pub struct UPMB3_R(crate::FieldReader<bool, bool>);
impl UPMB3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPMB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPMB3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPMB3` writer - USB - PLL feedback divider buffer Bit 3"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `UPMB4` reader - USB - PLL feedback divider buffer Bit 4"]
pub struct UPMB4_R(crate::FieldReader<bool, bool>);
impl UPMB4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPMB4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPMB4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPMB4` writer - USB - PLL feedback divider buffer Bit 4"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `UPMB5` reader - USB - PLL feedback divider buffer Bit 5"]
pub struct UPMB5_R(crate::FieldReader<bool, bool>);
impl UPMB5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPMB5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPMB5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPMB5` writer - USB - PLL feedback divider buffer Bit 5"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `UPQB0` reader - USB - PLL prescale divider buffer Bit 0"]
pub struct UPQB0_R(crate::FieldReader<bool, bool>);
impl UPQB0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPQB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPQB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPQB0` writer - USB - PLL prescale divider buffer Bit 0"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `UPQB1` reader - USB - PLL prescale divider buffer Bit 1"]
pub struct UPQB1_R(crate::FieldReader<bool, bool>);
impl UPQB1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPQB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPQB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPQB1` writer - USB - PLL prescale divider buffer Bit 1"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `UPQB2` reader - USB - PLL prescale divider buffer Bit 2"]
pub struct UPQB2_R(crate::FieldReader<bool, bool>);
impl UPQB2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPQB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPQB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPQB2` writer - USB - PLL prescale divider buffer Bit 2"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PLL Clock Divider Buffer control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbplldivb](index.html) module"]
pub struct USBPLLDIVB_SPEC;
impl crate::RegisterSpec for USBPLLDIVB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbplldivb::R](R) reader structure"]
impl crate::Readable for USBPLLDIVB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbplldivb::W](W) writer structure"]
impl crate::Writable for USBPLLDIVB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBPLLDIVB to value 0"]
impl crate::Resettable for USBPLLDIVB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
