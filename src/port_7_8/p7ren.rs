#[doc = "Register `P7REN` reader"]
pub struct R(crate::R<P7REN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P7REN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P7REN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P7REN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P7REN` writer"]
pub struct W(crate::W<P7REN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P7REN_SPEC>;
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
impl From<crate::W<P7REN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P7REN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P7REN0` reader - P7REN0"]
pub struct P7REN0_R(crate::FieldReader<bool, bool>);
impl P7REN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7REN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7REN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7REN0` writer - P7REN0"]
pub struct P7REN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P7REN0_W<'a> {
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
#[doc = "Field `P7REN1` reader - P7REN1"]
pub struct P7REN1_R(crate::FieldReader<bool, bool>);
impl P7REN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7REN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7REN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7REN1` writer - P7REN1"]
pub struct P7REN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P7REN1_W<'a> {
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
#[doc = "Field `P7REN2` reader - P7REN2"]
pub struct P7REN2_R(crate::FieldReader<bool, bool>);
impl P7REN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7REN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7REN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7REN2` writer - P7REN2"]
pub struct P7REN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P7REN2_W<'a> {
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
#[doc = "Field `P7REN3` reader - P7REN3"]
pub struct P7REN3_R(crate::FieldReader<bool, bool>);
impl P7REN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7REN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7REN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7REN3` writer - P7REN3"]
pub struct P7REN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P7REN3_W<'a> {
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
#[doc = "Field `P7REN4` reader - P7REN4"]
pub struct P7REN4_R(crate::FieldReader<bool, bool>);
impl P7REN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7REN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7REN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7REN4` writer - P7REN4"]
pub struct P7REN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P7REN4_W<'a> {
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
#[doc = "Field `P7REN5` reader - P7REN5"]
pub struct P7REN5_R(crate::FieldReader<bool, bool>);
impl P7REN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7REN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7REN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7REN5` writer - P7REN5"]
pub struct P7REN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P7REN5_W<'a> {
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
#[doc = "Field `P7REN6` reader - P7REN6"]
pub struct P7REN6_R(crate::FieldReader<bool, bool>);
impl P7REN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7REN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7REN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7REN6` writer - P7REN6"]
pub struct P7REN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P7REN6_W<'a> {
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
#[doc = "Field `P7REN7` reader - P7REN7"]
pub struct P7REN7_R(crate::FieldReader<bool, bool>);
impl P7REN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7REN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7REN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7REN7` writer - P7REN7"]
pub struct P7REN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P7REN7_W<'a> {
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
    #[doc = "Bit 0 - P7REN0"]
    #[inline(always)]
    pub fn p7ren0(&self) -> P7REN0_R {
        P7REN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P7REN1"]
    #[inline(always)]
    pub fn p7ren1(&self) -> P7REN1_R {
        P7REN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P7REN2"]
    #[inline(always)]
    pub fn p7ren2(&self) -> P7REN2_R {
        P7REN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P7REN3"]
    #[inline(always)]
    pub fn p7ren3(&self) -> P7REN3_R {
        P7REN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P7REN4"]
    #[inline(always)]
    pub fn p7ren4(&self) -> P7REN4_R {
        P7REN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P7REN5"]
    #[inline(always)]
    pub fn p7ren5(&self) -> P7REN5_R {
        P7REN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P7REN6"]
    #[inline(always)]
    pub fn p7ren6(&self) -> P7REN6_R {
        P7REN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P7REN7"]
    #[inline(always)]
    pub fn p7ren7(&self) -> P7REN7_R {
        P7REN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P7REN0"]
    #[inline(always)]
    pub fn p7ren0(&mut self) -> P7REN0_W {
        P7REN0_W { w: self }
    }
    #[doc = "Bit 1 - P7REN1"]
    #[inline(always)]
    pub fn p7ren1(&mut self) -> P7REN1_W {
        P7REN1_W { w: self }
    }
    #[doc = "Bit 2 - P7REN2"]
    #[inline(always)]
    pub fn p7ren2(&mut self) -> P7REN2_W {
        P7REN2_W { w: self }
    }
    #[doc = "Bit 3 - P7REN3"]
    #[inline(always)]
    pub fn p7ren3(&mut self) -> P7REN3_W {
        P7REN3_W { w: self }
    }
    #[doc = "Bit 4 - P7REN4"]
    #[inline(always)]
    pub fn p7ren4(&mut self) -> P7REN4_W {
        P7REN4_W { w: self }
    }
    #[doc = "Bit 5 - P7REN5"]
    #[inline(always)]
    pub fn p7ren5(&mut self) -> P7REN5_W {
        P7REN5_W { w: self }
    }
    #[doc = "Bit 6 - P7REN6"]
    #[inline(always)]
    pub fn p7ren6(&mut self) -> P7REN6_W {
        P7REN6_W { w: self }
    }
    #[doc = "Bit 7 - P7REN7"]
    #[inline(always)]
    pub fn p7ren7(&mut self) -> P7REN7_W {
        P7REN7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 7 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7ren](index.html) module"]
pub struct P7REN_SPEC;
impl crate::RegisterSpec for P7REN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p7ren::R](R) reader structure"]
impl crate::Readable for P7REN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p7ren::W](W) writer structure"]
impl crate::Writable for P7REN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P7REN to value 0"]
impl crate::Resettable for P7REN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
