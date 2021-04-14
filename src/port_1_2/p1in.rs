#[doc = "Register `P1IN` reader"]
pub struct R(crate::R<P1IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P1IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P1IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P1IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P1IN` writer"]
pub struct W(crate::W<P1IN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P1IN_SPEC>;
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
impl From<crate::W<P1IN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P1IN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1IN0` reader - P1IN0"]
pub struct P1IN0_R(crate::FieldReader<bool, bool>);
impl P1IN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IN0` writer - P1IN0"]
pub struct P1IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IN0_W<'a> {
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
#[doc = "Field `P1IN1` reader - P1IN1"]
pub struct P1IN1_R(crate::FieldReader<bool, bool>);
impl P1IN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IN1` writer - P1IN1"]
pub struct P1IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IN1_W<'a> {
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
#[doc = "Field `P1IN2` reader - P1IN2"]
pub struct P1IN2_R(crate::FieldReader<bool, bool>);
impl P1IN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IN2` writer - P1IN2"]
pub struct P1IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IN2_W<'a> {
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
#[doc = "Field `P1IN3` reader - P1IN3"]
pub struct P1IN3_R(crate::FieldReader<bool, bool>);
impl P1IN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IN3` writer - P1IN3"]
pub struct P1IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IN3_W<'a> {
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
#[doc = "Field `P1IN4` reader - P1IN4"]
pub struct P1IN4_R(crate::FieldReader<bool, bool>);
impl P1IN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IN4` writer - P1IN4"]
pub struct P1IN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IN4_W<'a> {
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
#[doc = "Field `P1IN5` reader - P1IN5"]
pub struct P1IN5_R(crate::FieldReader<bool, bool>);
impl P1IN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IN5` writer - P1IN5"]
pub struct P1IN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IN5_W<'a> {
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
#[doc = "Field `P1IN6` reader - P1IN6"]
pub struct P1IN6_R(crate::FieldReader<bool, bool>);
impl P1IN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IN6` writer - P1IN6"]
pub struct P1IN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IN6_W<'a> {
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
#[doc = "Field `P1IN7` reader - P1IN7"]
pub struct P1IN7_R(crate::FieldReader<bool, bool>);
impl P1IN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IN7` writer - P1IN7"]
pub struct P1IN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IN7_W<'a> {
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
    #[doc = "Bit 0 - P1IN0"]
    #[inline(always)]
    pub fn p1in0(&self) -> P1IN0_R {
        P1IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P1IN1"]
    #[inline(always)]
    pub fn p1in1(&self) -> P1IN1_R {
        P1IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P1IN2"]
    #[inline(always)]
    pub fn p1in2(&self) -> P1IN2_R {
        P1IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P1IN3"]
    #[inline(always)]
    pub fn p1in3(&self) -> P1IN3_R {
        P1IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P1IN4"]
    #[inline(always)]
    pub fn p1in4(&self) -> P1IN4_R {
        P1IN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P1IN5"]
    #[inline(always)]
    pub fn p1in5(&self) -> P1IN5_R {
        P1IN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P1IN6"]
    #[inline(always)]
    pub fn p1in6(&self) -> P1IN6_R {
        P1IN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P1IN7"]
    #[inline(always)]
    pub fn p1in7(&self) -> P1IN7_R {
        P1IN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1IN0"]
    #[inline(always)]
    pub fn p1in0(&mut self) -> P1IN0_W {
        P1IN0_W { w: self }
    }
    #[doc = "Bit 1 - P1IN1"]
    #[inline(always)]
    pub fn p1in1(&mut self) -> P1IN1_W {
        P1IN1_W { w: self }
    }
    #[doc = "Bit 2 - P1IN2"]
    #[inline(always)]
    pub fn p1in2(&mut self) -> P1IN2_W {
        P1IN2_W { w: self }
    }
    #[doc = "Bit 3 - P1IN3"]
    #[inline(always)]
    pub fn p1in3(&mut self) -> P1IN3_W {
        P1IN3_W { w: self }
    }
    #[doc = "Bit 4 - P1IN4"]
    #[inline(always)]
    pub fn p1in4(&mut self) -> P1IN4_W {
        P1IN4_W { w: self }
    }
    #[doc = "Bit 5 - P1IN5"]
    #[inline(always)]
    pub fn p1in5(&mut self) -> P1IN5_W {
        P1IN5_W { w: self }
    }
    #[doc = "Bit 6 - P1IN6"]
    #[inline(always)]
    pub fn p1in6(&mut self) -> P1IN6_W {
        P1IN6_W { w: self }
    }
    #[doc = "Bit 7 - P1IN7"]
    #[inline(always)]
    pub fn p1in7(&mut self) -> P1IN7_W {
        P1IN7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 1 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1in](index.html) module"]
pub struct P1IN_SPEC;
impl crate::RegisterSpec for P1IN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p1in::R](R) reader structure"]
impl crate::Readable for P1IN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p1in::W](W) writer structure"]
impl crate::Writable for P1IN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P1IN to value 0"]
impl crate::Resettable for P1IN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
