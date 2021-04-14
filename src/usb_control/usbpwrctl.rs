#[doc = "Register `USBPWRCTL` reader"]
pub struct R(crate::R<USBPWRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPWRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPWRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPWRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPWRCTL` writer"]
pub struct W(crate::W<USBPWRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPWRCTL_SPEC>;
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
impl From<crate::W<USBPWRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPWRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VUOVLIFG` reader - USB - VUSB Overload Interrupt Flag"]
pub struct VUOVLIFG_R(crate::FieldReader<bool, bool>);
impl VUOVLIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VUOVLIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VUOVLIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VUOVLIFG` writer - USB - VUSB Overload Interrupt Flag"]
pub struct VUOVLIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> VUOVLIFG_W<'a> {
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
#[doc = "Field `VBONIFG` reader - USB - VBUS \"Coming ON\" Interrupt Flag"]
pub struct VBONIFG_R(crate::FieldReader<bool, bool>);
impl VBONIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBONIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBONIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBONIFG` writer - USB - VBUS \"Coming ON\" Interrupt Flag"]
pub struct VBONIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> VBONIFG_W<'a> {
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
#[doc = "Field `VBOFFIFG` reader - USB - VBUS \"Going OFF\" Interrupt Flag"]
pub struct VBOFFIFG_R(crate::FieldReader<bool, bool>);
impl VBOFFIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBOFFIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBOFFIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBOFFIFG` writer - USB - VBUS \"Going OFF\" Interrupt Flag"]
pub struct VBOFFIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> VBOFFIFG_W<'a> {
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
#[doc = "Field `USBBGVBV` reader - USB - USB Bandgap and VBUS valid"]
pub struct USBBGVBV_R(crate::FieldReader<bool, bool>);
impl USBBGVBV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBBGVBV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBBGVBV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBBGVBV` writer - USB - USB Bandgap and VBUS valid"]
pub struct USBBGVBV_W<'a> {
    w: &'a mut W,
}
impl<'a> USBBGVBV_W<'a> {
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
#[doc = "Field `USBDETEN` reader - USB - VBUS on/off events enable"]
pub struct USBDETEN_R(crate::FieldReader<bool, bool>);
impl USBDETEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBDETEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBDETEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBDETEN` writer - USB - VBUS on/off events enable"]
pub struct USBDETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDETEN_W<'a> {
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
#[doc = "Field `OVLAOFF` reader - USB - LDO overload auto off enable"]
pub struct OVLAOFF_R(crate::FieldReader<bool, bool>);
impl OVLAOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVLAOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVLAOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVLAOFF` writer - USB - LDO overload auto off enable"]
pub struct OVLAOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVLAOFF_W<'a> {
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
#[doc = "Field `SLDOAON` reader - USB - Secondary LDO auto on enable"]
pub struct SLDOAON_R(crate::FieldReader<bool, bool>);
impl SLDOAON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLDOAON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLDOAON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLDOAON` writer - USB - Secondary LDO auto on enable"]
pub struct SLDOAON_W<'a> {
    w: &'a mut W,
}
impl<'a> SLDOAON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `VUOVLIE` reader - USB - Overload indication Interrupt Enable"]
pub struct VUOVLIE_R(crate::FieldReader<bool, bool>);
impl VUOVLIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VUOVLIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VUOVLIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VUOVLIE` writer - USB - Overload indication Interrupt Enable"]
pub struct VUOVLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VUOVLIE_W<'a> {
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
#[doc = "Field `VBONIE` reader - USB - VBUS \"Coming ON\" Interrupt Enable"]
pub struct VBONIE_R(crate::FieldReader<bool, bool>);
impl VBONIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBONIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBONIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBONIE` writer - USB - VBUS \"Coming ON\" Interrupt Enable"]
pub struct VBONIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBONIE_W<'a> {
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
#[doc = "Field `VBOFFIE` reader - USB - VBUS \"Going OFF\" Interrupt Enable"]
pub struct VBOFFIE_R(crate::FieldReader<bool, bool>);
impl VBOFFIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBOFFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBOFFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBOFFIE` writer - USB - VBUS \"Going OFF\" Interrupt Enable"]
pub struct VBOFFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBOFFIE_W<'a> {
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
#[doc = "Field `VUSBEN` reader - USB - LDO Enable (3.3V)"]
pub struct VUSBEN_R(crate::FieldReader<bool, bool>);
impl VUSBEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VUSBEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VUSBEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VUSBEN` writer - USB - LDO Enable (3.3V)"]
pub struct VUSBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VUSBEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u16 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `SLDOEN` reader - USB - Secondary LDO Enable (1.8V)"]
pub struct SLDOEN_R(crate::FieldReader<bool, bool>);
impl SLDOEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLDOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLDOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLDOEN` writer - USB - Secondary LDO Enable (1.8V)"]
pub struct SLDOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLDOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u16 & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB - VUSB Overload Interrupt Flag"]
    #[inline(always)]
    pub fn vuovlifg(&self) -> VUOVLIFG_R {
        VUOVLIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB - VBUS \"Coming ON\" Interrupt Flag"]
    #[inline(always)]
    pub fn vbonifg(&self) -> VBONIFG_R {
        VBONIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB - VBUS \"Going OFF\" Interrupt Flag"]
    #[inline(always)]
    pub fn vboffifg(&self) -> VBOFFIFG_R {
        VBOFFIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB - USB Bandgap and VBUS valid"]
    #[inline(always)]
    pub fn usbbgvbv(&self) -> USBBGVBV_R {
        USBBGVBV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USB - VBUS on/off events enable"]
    #[inline(always)]
    pub fn usbdeten(&self) -> USBDETEN_R {
        USBDETEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB - LDO overload auto off enable"]
    #[inline(always)]
    pub fn ovlaoff(&self) -> OVLAOFF_R {
        OVLAOFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USB - Secondary LDO auto on enable"]
    #[inline(always)]
    pub fn sldoaon(&self) -> SLDOAON_R {
        SLDOAON_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USB - Overload indication Interrupt Enable"]
    #[inline(always)]
    pub fn vuovlie(&self) -> VUOVLIE_R {
        VUOVLIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - USB - VBUS \"Coming ON\" Interrupt Enable"]
    #[inline(always)]
    pub fn vbonie(&self) -> VBONIE_R {
        VBONIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USB - VBUS \"Going OFF\" Interrupt Enable"]
    #[inline(always)]
    pub fn vboffie(&self) -> VBOFFIE_R {
        VBOFFIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USB - LDO Enable (3.3V)"]
    #[inline(always)]
    pub fn vusben(&self) -> VUSBEN_R {
        VUSBEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - USB - Secondary LDO Enable (1.8V)"]
    #[inline(always)]
    pub fn sldoen(&self) -> SLDOEN_R {
        SLDOEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - VUSB Overload Interrupt Flag"]
    #[inline(always)]
    pub fn vuovlifg(&mut self) -> VUOVLIFG_W {
        VUOVLIFG_W { w: self }
    }
    #[doc = "Bit 1 - USB - VBUS \"Coming ON\" Interrupt Flag"]
    #[inline(always)]
    pub fn vbonifg(&mut self) -> VBONIFG_W {
        VBONIFG_W { w: self }
    }
    #[doc = "Bit 2 - USB - VBUS \"Going OFF\" Interrupt Flag"]
    #[inline(always)]
    pub fn vboffifg(&mut self) -> VBOFFIFG_W {
        VBOFFIFG_W { w: self }
    }
    #[doc = "Bit 3 - USB - USB Bandgap and VBUS valid"]
    #[inline(always)]
    pub fn usbbgvbv(&mut self) -> USBBGVBV_W {
        USBBGVBV_W { w: self }
    }
    #[doc = "Bit 4 - USB - VBUS on/off events enable"]
    #[inline(always)]
    pub fn usbdeten(&mut self) -> USBDETEN_W {
        USBDETEN_W { w: self }
    }
    #[doc = "Bit 5 - USB - LDO overload auto off enable"]
    #[inline(always)]
    pub fn ovlaoff(&mut self) -> OVLAOFF_W {
        OVLAOFF_W { w: self }
    }
    #[doc = "Bit 6 - USB - Secondary LDO auto on enable"]
    #[inline(always)]
    pub fn sldoaon(&mut self) -> SLDOAON_W {
        SLDOAON_W { w: self }
    }
    #[doc = "Bit 8 - USB - Overload indication Interrupt Enable"]
    #[inline(always)]
    pub fn vuovlie(&mut self) -> VUOVLIE_W {
        VUOVLIE_W { w: self }
    }
    #[doc = "Bit 9 - USB - VBUS \"Coming ON\" Interrupt Enable"]
    #[inline(always)]
    pub fn vbonie(&mut self) -> VBONIE_W {
        VBONIE_W { w: self }
    }
    #[doc = "Bit 10 - USB - VBUS \"Going OFF\" Interrupt Enable"]
    #[inline(always)]
    pub fn vboffie(&mut self) -> VBOFFIE_W {
        VBOFFIE_W { w: self }
    }
    #[doc = "Bit 11 - USB - LDO Enable (3.3V)"]
    #[inline(always)]
    pub fn vusben(&mut self) -> VUSBEN_W {
        VUSBEN_W { w: self }
    }
    #[doc = "Bit 12 - USB - Secondary LDO Enable (1.8V)"]
    #[inline(always)]
    pub fn sldoen(&mut self) -> SLDOEN_W {
        SLDOEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpwrctl](index.html) module"]
pub struct USBPWRCTL_SPEC;
impl crate::RegisterSpec for USBPWRCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbpwrctl::R](R) reader structure"]
impl crate::Readable for USBPWRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbpwrctl::W](W) writer structure"]
impl crate::Writable for USBPWRCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBPWRCTL to value 0"]
impl crate::Resettable for USBPWRCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
