#[doc = "Register `P3IN` reader"]
pub struct R(crate::R<P3IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P3IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P3IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P3IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P3IN` writer"]
pub struct W(crate::W<P3IN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P3IN_SPEC>;
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
impl From<crate::W<P3IN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P3IN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P3IN0` reader - P3IN0"]
pub struct P3IN0_R(crate::FieldReader<bool, bool>);
impl P3IN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P3IN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IN0` writer - P3IN0"]
pub struct P3IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IN0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `P3IN1` reader - P3IN1"]
pub struct P3IN1_R(crate::FieldReader<bool, bool>);
impl P3IN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P3IN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IN1` writer - P3IN1"]
pub struct P3IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `P3IN2` reader - P3IN2"]
pub struct P3IN2_R(crate::FieldReader<bool, bool>);
impl P3IN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P3IN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IN2` writer - P3IN2"]
pub struct P3IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `P3IN3` reader - P3IN3"]
pub struct P3IN3_R(crate::FieldReader<bool, bool>);
impl P3IN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P3IN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IN3` writer - P3IN3"]
pub struct P3IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IN3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `P3IN4` reader - P3IN4"]
pub struct P3IN4_R(crate::FieldReader<bool, bool>);
impl P3IN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P3IN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IN4` writer - P3IN4"]
pub struct P3IN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IN4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `P3IN5` reader - P3IN5"]
pub struct P3IN5_R(crate::FieldReader<bool, bool>);
impl P3IN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P3IN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IN5` writer - P3IN5"]
pub struct P3IN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IN5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `P3IN6` reader - P3IN6"]
pub struct P3IN6_R(crate::FieldReader<bool, bool>);
impl P3IN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P3IN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IN6` writer - P3IN6"]
pub struct P3IN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IN6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `P3IN7` reader - P3IN7"]
pub struct P3IN7_R(crate::FieldReader<bool, bool>);
impl P3IN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P3IN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IN7` writer - P3IN7"]
pub struct P3IN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IN7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - P3IN0"]
    #[inline(always)]
    pub fn p3in0(&self) -> P3IN0_R {
        P3IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P3IN1"]
    #[inline(always)]
    pub fn p3in1(&self) -> P3IN1_R {
        P3IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P3IN2"]
    #[inline(always)]
    pub fn p3in2(&self) -> P3IN2_R {
        P3IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P3IN3"]
    #[inline(always)]
    pub fn p3in3(&self) -> P3IN3_R {
        P3IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P3IN4"]
    #[inline(always)]
    pub fn p3in4(&self) -> P3IN4_R {
        P3IN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P3IN5"]
    #[inline(always)]
    pub fn p3in5(&self) -> P3IN5_R {
        P3IN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P3IN6"]
    #[inline(always)]
    pub fn p3in6(&self) -> P3IN6_R {
        P3IN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P3IN7"]
    #[inline(always)]
    pub fn p3in7(&self) -> P3IN7_R {
        P3IN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3IN0"]
    #[inline(always)]
    pub fn p3in0(&mut self) -> P3IN0_W {
        P3IN0_W { w: self }
    }
    #[doc = "Bit 1 - P3IN1"]
    #[inline(always)]
    pub fn p3in1(&mut self) -> P3IN1_W {
        P3IN1_W { w: self }
    }
    #[doc = "Bit 2 - P3IN2"]
    #[inline(always)]
    pub fn p3in2(&mut self) -> P3IN2_W {
        P3IN2_W { w: self }
    }
    #[doc = "Bit 3 - P3IN3"]
    #[inline(always)]
    pub fn p3in3(&mut self) -> P3IN3_W {
        P3IN3_W { w: self }
    }
    #[doc = "Bit 4 - P3IN4"]
    #[inline(always)]
    pub fn p3in4(&mut self) -> P3IN4_W {
        P3IN4_W { w: self }
    }
    #[doc = "Bit 5 - P3IN5"]
    #[inline(always)]
    pub fn p3in5(&mut self) -> P3IN5_W {
        P3IN5_W { w: self }
    }
    #[doc = "Bit 6 - P3IN6"]
    #[inline(always)]
    pub fn p3in6(&mut self) -> P3IN6_W {
        P3IN6_W { w: self }
    }
    #[doc = "Bit 7 - P3IN7"]
    #[inline(always)]
    pub fn p3in7(&mut self) -> P3IN7_W {
        P3IN7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 3 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3in](index.html) module"]
pub struct P3IN_SPEC;
impl crate::RegisterSpec for P3IN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p3in::R](R) reader structure"]
impl crate::Readable for P3IN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p3in::W](W) writer structure"]
impl crate::Writable for P3IN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P3IN to value 0"]
impl crate::Resettable for P3IN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
