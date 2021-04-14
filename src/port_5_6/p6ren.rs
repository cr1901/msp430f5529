#[doc = "Register `P6REN` reader"]
pub struct R(crate::R<P6REN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P6REN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P6REN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P6REN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P6REN` writer"]
pub struct W(crate::W<P6REN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P6REN_SPEC>;
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
impl From<crate::W<P6REN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P6REN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P6REN0` reader - P6REN0"]
pub struct P6REN0_R(crate::FieldReader<bool, bool>);
impl P6REN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6REN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6REN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6REN0` writer - P6REN0"]
pub struct P6REN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P6REN0_W<'a> {
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
#[doc = "Field `P6REN1` reader - P6REN1"]
pub struct P6REN1_R(crate::FieldReader<bool, bool>);
impl P6REN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6REN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6REN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6REN1` writer - P6REN1"]
pub struct P6REN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P6REN1_W<'a> {
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
#[doc = "Field `P6REN2` reader - P6REN2"]
pub struct P6REN2_R(crate::FieldReader<bool, bool>);
impl P6REN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6REN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6REN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6REN2` writer - P6REN2"]
pub struct P6REN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P6REN2_W<'a> {
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
#[doc = "Field `P6REN3` reader - P6REN3"]
pub struct P6REN3_R(crate::FieldReader<bool, bool>);
impl P6REN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6REN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6REN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6REN3` writer - P6REN3"]
pub struct P6REN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P6REN3_W<'a> {
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
#[doc = "Field `P6REN4` reader - P6REN4"]
pub struct P6REN4_R(crate::FieldReader<bool, bool>);
impl P6REN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6REN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6REN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6REN4` writer - P6REN4"]
pub struct P6REN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P6REN4_W<'a> {
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
#[doc = "Field `P6REN5` reader - P6REN5"]
pub struct P6REN5_R(crate::FieldReader<bool, bool>);
impl P6REN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6REN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6REN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6REN5` writer - P6REN5"]
pub struct P6REN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P6REN5_W<'a> {
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
#[doc = "Field `P6REN6` reader - P6REN6"]
pub struct P6REN6_R(crate::FieldReader<bool, bool>);
impl P6REN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6REN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6REN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6REN6` writer - P6REN6"]
pub struct P6REN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P6REN6_W<'a> {
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
#[doc = "Field `P6REN7` reader - P6REN7"]
pub struct P6REN7_R(crate::FieldReader<bool, bool>);
impl P6REN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6REN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6REN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6REN7` writer - P6REN7"]
pub struct P6REN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P6REN7_W<'a> {
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
    #[doc = "Bit 0 - P6REN0"]
    #[inline(always)]
    pub fn p6ren0(&self) -> P6REN0_R {
        P6REN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P6REN1"]
    #[inline(always)]
    pub fn p6ren1(&self) -> P6REN1_R {
        P6REN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P6REN2"]
    #[inline(always)]
    pub fn p6ren2(&self) -> P6REN2_R {
        P6REN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P6REN3"]
    #[inline(always)]
    pub fn p6ren3(&self) -> P6REN3_R {
        P6REN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P6REN4"]
    #[inline(always)]
    pub fn p6ren4(&self) -> P6REN4_R {
        P6REN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P6REN5"]
    #[inline(always)]
    pub fn p6ren5(&self) -> P6REN5_R {
        P6REN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P6REN6"]
    #[inline(always)]
    pub fn p6ren6(&self) -> P6REN6_R {
        P6REN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P6REN7"]
    #[inline(always)]
    pub fn p6ren7(&self) -> P6REN7_R {
        P6REN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P6REN0"]
    #[inline(always)]
    pub fn p6ren0(&mut self) -> P6REN0_W {
        P6REN0_W { w: self }
    }
    #[doc = "Bit 1 - P6REN1"]
    #[inline(always)]
    pub fn p6ren1(&mut self) -> P6REN1_W {
        P6REN1_W { w: self }
    }
    #[doc = "Bit 2 - P6REN2"]
    #[inline(always)]
    pub fn p6ren2(&mut self) -> P6REN2_W {
        P6REN2_W { w: self }
    }
    #[doc = "Bit 3 - P6REN3"]
    #[inline(always)]
    pub fn p6ren3(&mut self) -> P6REN3_W {
        P6REN3_W { w: self }
    }
    #[doc = "Bit 4 - P6REN4"]
    #[inline(always)]
    pub fn p6ren4(&mut self) -> P6REN4_W {
        P6REN4_W { w: self }
    }
    #[doc = "Bit 5 - P6REN5"]
    #[inline(always)]
    pub fn p6ren5(&mut self) -> P6REN5_W {
        P6REN5_W { w: self }
    }
    #[doc = "Bit 6 - P6REN6"]
    #[inline(always)]
    pub fn p6ren6(&mut self) -> P6REN6_W {
        P6REN6_W { w: self }
    }
    #[doc = "Bit 7 - P6REN7"]
    #[inline(always)]
    pub fn p6ren7(&mut self) -> P6REN7_W {
        P6REN7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 6 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6ren](index.html) module"]
pub struct P6REN_SPEC;
impl crate::RegisterSpec for P6REN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p6ren::R](R) reader structure"]
impl crate::Readable for P6REN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p6ren::W](W) writer structure"]
impl crate::Writable for P6REN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P6REN to value 0"]
impl crate::Resettable for P6REN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
