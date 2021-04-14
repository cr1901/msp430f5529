#[doc = "Register `P8SEL` reader"]
pub struct R(crate::R<P8SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P8SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P8SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P8SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P8SEL` writer"]
pub struct W(crate::W<P8SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P8SEL_SPEC>;
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
impl From<crate::W<P8SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P8SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P8SEL0` reader - P8SEL0"]
pub struct P8SEL0_R(crate::FieldReader<bool, bool>);
impl P8SEL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8SEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8SEL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8SEL0` writer - P8SEL0"]
pub struct P8SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> P8SEL0_W<'a> {
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
#[doc = "Field `P8SEL1` reader - P8SEL1"]
pub struct P8SEL1_R(crate::FieldReader<bool, bool>);
impl P8SEL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8SEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8SEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8SEL1` writer - P8SEL1"]
pub struct P8SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> P8SEL1_W<'a> {
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
#[doc = "Field `P8SEL2` reader - P8SEL2"]
pub struct P8SEL2_R(crate::FieldReader<bool, bool>);
impl P8SEL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8SEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8SEL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8SEL2` writer - P8SEL2"]
pub struct P8SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> P8SEL2_W<'a> {
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
#[doc = "Field `P8SEL3` reader - P8SEL3"]
pub struct P8SEL3_R(crate::FieldReader<bool, bool>);
impl P8SEL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8SEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8SEL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8SEL3` writer - P8SEL3"]
pub struct P8SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> P8SEL3_W<'a> {
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
#[doc = "Field `P8SEL4` reader - P8SEL4"]
pub struct P8SEL4_R(crate::FieldReader<bool, bool>);
impl P8SEL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8SEL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8SEL4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8SEL4` writer - P8SEL4"]
pub struct P8SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> P8SEL4_W<'a> {
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
#[doc = "Field `P8SEL5` reader - P8SEL5"]
pub struct P8SEL5_R(crate::FieldReader<bool, bool>);
impl P8SEL5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8SEL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8SEL5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8SEL5` writer - P8SEL5"]
pub struct P8SEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> P8SEL5_W<'a> {
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
#[doc = "Field `P8SEL6` reader - P8SEL6"]
pub struct P8SEL6_R(crate::FieldReader<bool, bool>);
impl P8SEL6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8SEL6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8SEL6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8SEL6` writer - P8SEL6"]
pub struct P8SEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> P8SEL6_W<'a> {
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
#[doc = "Field `P8SEL7` reader - P8SEL7"]
pub struct P8SEL7_R(crate::FieldReader<bool, bool>);
impl P8SEL7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8SEL7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8SEL7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8SEL7` writer - P8SEL7"]
pub struct P8SEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> P8SEL7_W<'a> {
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
    #[doc = "Bit 0 - P8SEL0"]
    #[inline(always)]
    pub fn p8sel0(&self) -> P8SEL0_R {
        P8SEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P8SEL1"]
    #[inline(always)]
    pub fn p8sel1(&self) -> P8SEL1_R {
        P8SEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P8SEL2"]
    #[inline(always)]
    pub fn p8sel2(&self) -> P8SEL2_R {
        P8SEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P8SEL3"]
    #[inline(always)]
    pub fn p8sel3(&self) -> P8SEL3_R {
        P8SEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P8SEL4"]
    #[inline(always)]
    pub fn p8sel4(&self) -> P8SEL4_R {
        P8SEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P8SEL5"]
    #[inline(always)]
    pub fn p8sel5(&self) -> P8SEL5_R {
        P8SEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P8SEL6"]
    #[inline(always)]
    pub fn p8sel6(&self) -> P8SEL6_R {
        P8SEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P8SEL7"]
    #[inline(always)]
    pub fn p8sel7(&self) -> P8SEL7_R {
        P8SEL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P8SEL0"]
    #[inline(always)]
    pub fn p8sel0(&mut self) -> P8SEL0_W {
        P8SEL0_W { w: self }
    }
    #[doc = "Bit 1 - P8SEL1"]
    #[inline(always)]
    pub fn p8sel1(&mut self) -> P8SEL1_W {
        P8SEL1_W { w: self }
    }
    #[doc = "Bit 2 - P8SEL2"]
    #[inline(always)]
    pub fn p8sel2(&mut self) -> P8SEL2_W {
        P8SEL2_W { w: self }
    }
    #[doc = "Bit 3 - P8SEL3"]
    #[inline(always)]
    pub fn p8sel3(&mut self) -> P8SEL3_W {
        P8SEL3_W { w: self }
    }
    #[doc = "Bit 4 - P8SEL4"]
    #[inline(always)]
    pub fn p8sel4(&mut self) -> P8SEL4_W {
        P8SEL4_W { w: self }
    }
    #[doc = "Bit 5 - P8SEL5"]
    #[inline(always)]
    pub fn p8sel5(&mut self) -> P8SEL5_W {
        P8SEL5_W { w: self }
    }
    #[doc = "Bit 6 - P8SEL6"]
    #[inline(always)]
    pub fn p8sel6(&mut self) -> P8SEL6_W {
        P8SEL6_W { w: self }
    }
    #[doc = "Bit 7 - P8SEL7"]
    #[inline(always)]
    pub fn p8sel7(&mut self) -> P8SEL7_W {
        P8SEL7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 8 Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p8sel](index.html) module"]
pub struct P8SEL_SPEC;
impl crate::RegisterSpec for P8SEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p8sel::R](R) reader structure"]
impl crate::Readable for P8SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p8sel::W](W) writer structure"]
impl crate::Writable for P8SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P8SEL to value 0"]
impl crate::Resettable for P8SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
