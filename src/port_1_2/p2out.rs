#[doc = "Register `P2OUT` reader"]
pub struct R(crate::R<P2OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P2OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P2OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P2OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P2OUT` writer"]
pub struct W(crate::W<P2OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P2OUT_SPEC>;
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
impl From<crate::W<P2OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P2OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P2OUT0` reader - P2OUT0"]
pub struct P2OUT0_R(crate::FieldReader<bool, bool>);
impl P2OUT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2OUT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2OUT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2OUT0` writer - P2OUT0"]
pub struct P2OUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> P2OUT0_W<'a> {
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
#[doc = "Field `P2OUT1` reader - P2OUT1"]
pub struct P2OUT1_R(crate::FieldReader<bool, bool>);
impl P2OUT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2OUT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2OUT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2OUT1` writer - P2OUT1"]
pub struct P2OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> P2OUT1_W<'a> {
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
#[doc = "Field `P2OUT2` reader - P2OUT2"]
pub struct P2OUT2_R(crate::FieldReader<bool, bool>);
impl P2OUT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2OUT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2OUT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2OUT2` writer - P2OUT2"]
pub struct P2OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2OUT2_W<'a> {
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
#[doc = "Field `P2OUT3` reader - P2OUT3"]
pub struct P2OUT3_R(crate::FieldReader<bool, bool>);
impl P2OUT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2OUT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2OUT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2OUT3` writer - P2OUT3"]
pub struct P2OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> P2OUT3_W<'a> {
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
#[doc = "Field `P2OUT4` reader - P2OUT4"]
pub struct P2OUT4_R(crate::FieldReader<bool, bool>);
impl P2OUT4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2OUT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2OUT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2OUT4` writer - P2OUT4"]
pub struct P2OUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> P2OUT4_W<'a> {
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
#[doc = "Field `P2OUT5` reader - P2OUT5"]
pub struct P2OUT5_R(crate::FieldReader<bool, bool>);
impl P2OUT5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2OUT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2OUT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2OUT5` writer - P2OUT5"]
pub struct P2OUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> P2OUT5_W<'a> {
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
#[doc = "Field `P2OUT6` reader - P2OUT6"]
pub struct P2OUT6_R(crate::FieldReader<bool, bool>);
impl P2OUT6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2OUT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2OUT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2OUT6` writer - P2OUT6"]
pub struct P2OUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> P2OUT6_W<'a> {
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
#[doc = "Field `P2OUT7` reader - P2OUT7"]
pub struct P2OUT7_R(crate::FieldReader<bool, bool>);
impl P2OUT7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2OUT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2OUT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2OUT7` writer - P2OUT7"]
pub struct P2OUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> P2OUT7_W<'a> {
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
    #[doc = "Bit 0 - P2OUT0"]
    #[inline(always)]
    pub fn p2out0(&self) -> P2OUT0_R {
        P2OUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P2OUT1"]
    #[inline(always)]
    pub fn p2out1(&self) -> P2OUT1_R {
        P2OUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P2OUT2"]
    #[inline(always)]
    pub fn p2out2(&self) -> P2OUT2_R {
        P2OUT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P2OUT3"]
    #[inline(always)]
    pub fn p2out3(&self) -> P2OUT3_R {
        P2OUT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P2OUT4"]
    #[inline(always)]
    pub fn p2out4(&self) -> P2OUT4_R {
        P2OUT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P2OUT5"]
    #[inline(always)]
    pub fn p2out5(&self) -> P2OUT5_R {
        P2OUT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P2OUT6"]
    #[inline(always)]
    pub fn p2out6(&self) -> P2OUT6_R {
        P2OUT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P2OUT7"]
    #[inline(always)]
    pub fn p2out7(&self) -> P2OUT7_R {
        P2OUT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2OUT0"]
    #[inline(always)]
    pub fn p2out0(&mut self) -> P2OUT0_W {
        P2OUT0_W { w: self }
    }
    #[doc = "Bit 1 - P2OUT1"]
    #[inline(always)]
    pub fn p2out1(&mut self) -> P2OUT1_W {
        P2OUT1_W { w: self }
    }
    #[doc = "Bit 2 - P2OUT2"]
    #[inline(always)]
    pub fn p2out2(&mut self) -> P2OUT2_W {
        P2OUT2_W { w: self }
    }
    #[doc = "Bit 3 - P2OUT3"]
    #[inline(always)]
    pub fn p2out3(&mut self) -> P2OUT3_W {
        P2OUT3_W { w: self }
    }
    #[doc = "Bit 4 - P2OUT4"]
    #[inline(always)]
    pub fn p2out4(&mut self) -> P2OUT4_W {
        P2OUT4_W { w: self }
    }
    #[doc = "Bit 5 - P2OUT5"]
    #[inline(always)]
    pub fn p2out5(&mut self) -> P2OUT5_W {
        P2OUT5_W { w: self }
    }
    #[doc = "Bit 6 - P2OUT6"]
    #[inline(always)]
    pub fn p2out6(&mut self) -> P2OUT6_W {
        P2OUT6_W { w: self }
    }
    #[doc = "Bit 7 - P2OUT7"]
    #[inline(always)]
    pub fn p2out7(&mut self) -> P2OUT7_W {
        P2OUT7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 2 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2out](index.html) module"]
pub struct P2OUT_SPEC;
impl crate::RegisterSpec for P2OUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p2out::R](R) reader structure"]
impl crate::Readable for P2OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p2out::W](W) writer structure"]
impl crate::Writable for P2OUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P2OUT to value 0"]
impl crate::Resettable for P2OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
