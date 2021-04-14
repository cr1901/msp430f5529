#[doc = "Register `P1IE` reader"]
pub struct R(crate::R<P1IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P1IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P1IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P1IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P1IE` writer"]
pub struct W(crate::W<P1IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P1IE_SPEC>;
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
impl From<crate::W<P1IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P1IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1IE0` reader - P1IE0"]
pub struct P1IE0_R(crate::FieldReader<bool, bool>);
impl P1IE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IE0` writer - P1IE0"]
pub struct P1IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IE0_W<'a> {
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
#[doc = "Field `P1IE1` reader - P1IE1"]
pub struct P1IE1_R(crate::FieldReader<bool, bool>);
impl P1IE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IE1` writer - P1IE1"]
pub struct P1IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IE1_W<'a> {
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
#[doc = "Field `P1IE2` reader - P1IE2"]
pub struct P1IE2_R(crate::FieldReader<bool, bool>);
impl P1IE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IE2` writer - P1IE2"]
pub struct P1IE2_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IE2_W<'a> {
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
#[doc = "Field `P1IE3` reader - P1IE3"]
pub struct P1IE3_R(crate::FieldReader<bool, bool>);
impl P1IE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IE3` writer - P1IE3"]
pub struct P1IE3_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IE3_W<'a> {
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
#[doc = "Field `P1IE4` reader - P1IE4"]
pub struct P1IE4_R(crate::FieldReader<bool, bool>);
impl P1IE4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IE4` writer - P1IE4"]
pub struct P1IE4_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IE4_W<'a> {
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
#[doc = "Field `P1IE5` reader - P1IE5"]
pub struct P1IE5_R(crate::FieldReader<bool, bool>);
impl P1IE5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IE5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IE5` writer - P1IE5"]
pub struct P1IE5_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IE5_W<'a> {
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
#[doc = "Field `P1IE6` reader - P1IE6"]
pub struct P1IE6_R(crate::FieldReader<bool, bool>);
impl P1IE6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IE6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IE6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IE6` writer - P1IE6"]
pub struct P1IE6_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IE6_W<'a> {
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
#[doc = "Field `P1IE7` reader - P1IE7"]
pub struct P1IE7_R(crate::FieldReader<bool, bool>);
impl P1IE7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IE7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IE7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IE7` writer - P1IE7"]
pub struct P1IE7_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IE7_W<'a> {
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
    #[doc = "Bit 0 - P1IE0"]
    #[inline(always)]
    pub fn p1ie0(&self) -> P1IE0_R {
        P1IE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P1IE1"]
    #[inline(always)]
    pub fn p1ie1(&self) -> P1IE1_R {
        P1IE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P1IE2"]
    #[inline(always)]
    pub fn p1ie2(&self) -> P1IE2_R {
        P1IE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P1IE3"]
    #[inline(always)]
    pub fn p1ie3(&self) -> P1IE3_R {
        P1IE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P1IE4"]
    #[inline(always)]
    pub fn p1ie4(&self) -> P1IE4_R {
        P1IE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P1IE5"]
    #[inline(always)]
    pub fn p1ie5(&self) -> P1IE5_R {
        P1IE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P1IE6"]
    #[inline(always)]
    pub fn p1ie6(&self) -> P1IE6_R {
        P1IE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P1IE7"]
    #[inline(always)]
    pub fn p1ie7(&self) -> P1IE7_R {
        P1IE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1IE0"]
    #[inline(always)]
    pub fn p1ie0(&mut self) -> P1IE0_W {
        P1IE0_W { w: self }
    }
    #[doc = "Bit 1 - P1IE1"]
    #[inline(always)]
    pub fn p1ie1(&mut self) -> P1IE1_W {
        P1IE1_W { w: self }
    }
    #[doc = "Bit 2 - P1IE2"]
    #[inline(always)]
    pub fn p1ie2(&mut self) -> P1IE2_W {
        P1IE2_W { w: self }
    }
    #[doc = "Bit 3 - P1IE3"]
    #[inline(always)]
    pub fn p1ie3(&mut self) -> P1IE3_W {
        P1IE3_W { w: self }
    }
    #[doc = "Bit 4 - P1IE4"]
    #[inline(always)]
    pub fn p1ie4(&mut self) -> P1IE4_W {
        P1IE4_W { w: self }
    }
    #[doc = "Bit 5 - P1IE5"]
    #[inline(always)]
    pub fn p1ie5(&mut self) -> P1IE5_W {
        P1IE5_W { w: self }
    }
    #[doc = "Bit 6 - P1IE6"]
    #[inline(always)]
    pub fn p1ie6(&mut self) -> P1IE6_W {
        P1IE6_W { w: self }
    }
    #[doc = "Bit 7 - P1IE7"]
    #[inline(always)]
    pub fn p1ie7(&mut self) -> P1IE7_W {
        P1IE7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 1 Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1ie](index.html) module"]
pub struct P1IE_SPEC;
impl crate::RegisterSpec for P1IE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p1ie::R](R) reader structure"]
impl crate::Readable for P1IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p1ie::W](W) writer structure"]
impl crate::Writable for P1IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P1IE to value 0"]
impl crate::Resettable for P1IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
