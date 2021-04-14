#[doc = "Register `SYSJMBC` reader"]
pub struct R(crate::R<SYSJMBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSJMBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSJMBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSJMBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSJMBC` writer"]
pub struct W(crate::W<SYSJMBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSJMBC_SPEC>;
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
impl From<crate::W<SYSJMBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSJMBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JMBIN0FG` reader - SYS - Incoming JTAG Mailbox 0 Flag"]
pub struct JMBIN0FG_R(crate::FieldReader<bool, bool>);
impl JMBIN0FG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JMBIN0FG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JMBIN0FG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JMBIN0FG` writer - SYS - Incoming JTAG Mailbox 0 Flag"]
pub struct JMBIN0FG_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBIN0FG_W<'a> {
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
#[doc = "Field `JMBIN1FG` reader - SYS - Incoming JTAG Mailbox 1 Flag"]
pub struct JMBIN1FG_R(crate::FieldReader<bool, bool>);
impl JMBIN1FG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JMBIN1FG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JMBIN1FG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JMBIN1FG` writer - SYS - Incoming JTAG Mailbox 1 Flag"]
pub struct JMBIN1FG_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBIN1FG_W<'a> {
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
#[doc = "Field `JMBOUT0FG` reader - SYS - Outgoing JTAG Mailbox 0 Flag"]
pub struct JMBOUT0FG_R(crate::FieldReader<bool, bool>);
impl JMBOUT0FG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JMBOUT0FG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JMBOUT0FG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JMBOUT0FG` writer - SYS - Outgoing JTAG Mailbox 0 Flag"]
pub struct JMBOUT0FG_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBOUT0FG_W<'a> {
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
#[doc = "Field `JMBOUT1FG` reader - SYS - Outgoing JTAG Mailbox 1 Flag"]
pub struct JMBOUT1FG_R(crate::FieldReader<bool, bool>);
impl JMBOUT1FG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JMBOUT1FG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JMBOUT1FG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JMBOUT1FG` writer - SYS - Outgoing JTAG Mailbox 1 Flag"]
pub struct JMBOUT1FG_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBOUT1FG_W<'a> {
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
#[doc = "Field `JMBMODE` reader - SYS - JMB 16/32 Bit Mode"]
pub struct JMBMODE_R(crate::FieldReader<bool, bool>);
impl JMBMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JMBMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JMBMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JMBMODE` writer - SYS - JMB 16/32 Bit Mode"]
pub struct JMBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBMODE_W<'a> {
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
#[doc = "Field `JMBCLR0OFF` reader - SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe"]
pub struct JMBCLR0OFF_R(crate::FieldReader<bool, bool>);
impl JMBCLR0OFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JMBCLR0OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JMBCLR0OFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JMBCLR0OFF` writer - SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe"]
pub struct JMBCLR0OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBCLR0OFF_W<'a> {
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
#[doc = "Field `JMBCLR1OFF` reader - SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe"]
pub struct JMBCLR1OFF_R(crate::FieldReader<bool, bool>);
impl JMBCLR1OFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JMBCLR1OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JMBCLR1OFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JMBCLR1OFF` writer - SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe"]
pub struct JMBCLR1OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBCLR1OFF_W<'a> {
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
    #[doc = "Bit 0 - SYS - Incoming JTAG Mailbox 0 Flag"]
    #[inline(always)]
    pub fn jmbin0fg(&self) -> JMBIN0FG_R {
        JMBIN0FG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SYS - Incoming JTAG Mailbox 1 Flag"]
    #[inline(always)]
    pub fn jmbin1fg(&self) -> JMBIN1FG_R {
        JMBIN1FG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SYS - Outgoing JTAG Mailbox 0 Flag"]
    #[inline(always)]
    pub fn jmbout0fg(&self) -> JMBOUT0FG_R {
        JMBOUT0FG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SYS - Outgoing JTAG Mailbox 1 Flag"]
    #[inline(always)]
    pub fn jmbout1fg(&self) -> JMBOUT1FG_R {
        JMBOUT1FG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SYS - JMB 16/32 Bit Mode"]
    #[inline(always)]
    pub fn jmbmode(&self) -> JMBMODE_R {
        JMBMODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe"]
    #[inline(always)]
    pub fn jmbclr0off(&self) -> JMBCLR0OFF_R {
        JMBCLR0OFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe"]
    #[inline(always)]
    pub fn jmbclr1off(&self) -> JMBCLR1OFF_R {
        JMBCLR1OFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYS - Incoming JTAG Mailbox 0 Flag"]
    #[inline(always)]
    pub fn jmbin0fg(&mut self) -> JMBIN0FG_W {
        JMBIN0FG_W { w: self }
    }
    #[doc = "Bit 1 - SYS - Incoming JTAG Mailbox 1 Flag"]
    #[inline(always)]
    pub fn jmbin1fg(&mut self) -> JMBIN1FG_W {
        JMBIN1FG_W { w: self }
    }
    #[doc = "Bit 2 - SYS - Outgoing JTAG Mailbox 0 Flag"]
    #[inline(always)]
    pub fn jmbout0fg(&mut self) -> JMBOUT0FG_W {
        JMBOUT0FG_W { w: self }
    }
    #[doc = "Bit 3 - SYS - Outgoing JTAG Mailbox 1 Flag"]
    #[inline(always)]
    pub fn jmbout1fg(&mut self) -> JMBOUT1FG_W {
        JMBOUT1FG_W { w: self }
    }
    #[doc = "Bit 4 - SYS - JMB 16/32 Bit Mode"]
    #[inline(always)]
    pub fn jmbmode(&mut self) -> JMBMODE_W {
        JMBMODE_W { w: self }
    }
    #[doc = "Bit 6 - SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe"]
    #[inline(always)]
    pub fn jmbclr0off(&mut self) -> JMBCLR0OFF_W {
        JMBCLR0OFF_W { w: self }
    }
    #[doc = "Bit 7 - SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe"]
    #[inline(always)]
    pub fn jmbclr1off(&mut self) -> JMBCLR1OFF_W {
        JMBCLR1OFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JTAG mailbox control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysjmbc](index.html) module"]
pub struct SYSJMBC_SPEC;
impl crate::RegisterSpec for SYSJMBC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sysjmbc::R](R) reader structure"]
impl crate::Readable for SYSJMBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysjmbc::W](W) writer structure"]
impl crate::Writable for SYSJMBC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSJMBC to value 0"]
impl crate::Resettable for SYSJMBC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
