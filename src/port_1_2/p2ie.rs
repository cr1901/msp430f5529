#[doc = "Register `P2IE` reader"]
pub struct R(crate::R<P2IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P2IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P2IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P2IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P2IE` writer"]
pub struct W(crate::W<P2IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P2IE_SPEC>;
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
impl From<crate::W<P2IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P2IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P2IE0` reader - P2IE0"]
pub struct P2IE0_R(crate::FieldReader<bool, bool>);
impl P2IE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2IE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2IE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2IE0` writer - P2IE0"]
pub struct P2IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IE0_W<'a> {
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
#[doc = "Field `P2IE1` reader - P2IE1"]
pub struct P2IE1_R(crate::FieldReader<bool, bool>);
impl P2IE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2IE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2IE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2IE1` writer - P2IE1"]
pub struct P2IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IE1_W<'a> {
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
#[doc = "Field `P2IE2` reader - P2IE2"]
pub struct P2IE2_R(crate::FieldReader<bool, bool>);
impl P2IE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2IE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2IE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2IE2` writer - P2IE2"]
pub struct P2IE2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IE2_W<'a> {
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
#[doc = "Field `P2IE3` reader - P2IE3"]
pub struct P2IE3_R(crate::FieldReader<bool, bool>);
impl P2IE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2IE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2IE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2IE3` writer - P2IE3"]
pub struct P2IE3_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IE3_W<'a> {
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
#[doc = "Field `P2IE4` reader - P2IE4"]
pub struct P2IE4_R(crate::FieldReader<bool, bool>);
impl P2IE4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2IE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2IE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2IE4` writer - P2IE4"]
pub struct P2IE4_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IE4_W<'a> {
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
#[doc = "Field `P2IE5` reader - P2IE5"]
pub struct P2IE5_R(crate::FieldReader<bool, bool>);
impl P2IE5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2IE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2IE5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2IE5` writer - P2IE5"]
pub struct P2IE5_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IE5_W<'a> {
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
#[doc = "Field `P2IE6` reader - P2IE6"]
pub struct P2IE6_R(crate::FieldReader<bool, bool>);
impl P2IE6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2IE6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2IE6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2IE6` writer - P2IE6"]
pub struct P2IE6_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IE6_W<'a> {
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
#[doc = "Field `P2IE7` reader - P2IE7"]
pub struct P2IE7_R(crate::FieldReader<bool, bool>);
impl P2IE7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2IE7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2IE7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2IE7` writer - P2IE7"]
pub struct P2IE7_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IE7_W<'a> {
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
    #[doc = "Bit 0 - P2IE0"]
    #[inline(always)]
    pub fn p2ie0(&self) -> P2IE0_R {
        P2IE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P2IE1"]
    #[inline(always)]
    pub fn p2ie1(&self) -> P2IE1_R {
        P2IE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P2IE2"]
    #[inline(always)]
    pub fn p2ie2(&self) -> P2IE2_R {
        P2IE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P2IE3"]
    #[inline(always)]
    pub fn p2ie3(&self) -> P2IE3_R {
        P2IE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P2IE4"]
    #[inline(always)]
    pub fn p2ie4(&self) -> P2IE4_R {
        P2IE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P2IE5"]
    #[inline(always)]
    pub fn p2ie5(&self) -> P2IE5_R {
        P2IE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P2IE6"]
    #[inline(always)]
    pub fn p2ie6(&self) -> P2IE6_R {
        P2IE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P2IE7"]
    #[inline(always)]
    pub fn p2ie7(&self) -> P2IE7_R {
        P2IE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2IE0"]
    #[inline(always)]
    pub fn p2ie0(&mut self) -> P2IE0_W {
        P2IE0_W { w: self }
    }
    #[doc = "Bit 1 - P2IE1"]
    #[inline(always)]
    pub fn p2ie1(&mut self) -> P2IE1_W {
        P2IE1_W { w: self }
    }
    #[doc = "Bit 2 - P2IE2"]
    #[inline(always)]
    pub fn p2ie2(&mut self) -> P2IE2_W {
        P2IE2_W { w: self }
    }
    #[doc = "Bit 3 - P2IE3"]
    #[inline(always)]
    pub fn p2ie3(&mut self) -> P2IE3_W {
        P2IE3_W { w: self }
    }
    #[doc = "Bit 4 - P2IE4"]
    #[inline(always)]
    pub fn p2ie4(&mut self) -> P2IE4_W {
        P2IE4_W { w: self }
    }
    #[doc = "Bit 5 - P2IE5"]
    #[inline(always)]
    pub fn p2ie5(&mut self) -> P2IE5_W {
        P2IE5_W { w: self }
    }
    #[doc = "Bit 6 - P2IE6"]
    #[inline(always)]
    pub fn p2ie6(&mut self) -> P2IE6_W {
        P2IE6_W { w: self }
    }
    #[doc = "Bit 7 - P2IE7"]
    #[inline(always)]
    pub fn p2ie7(&mut self) -> P2IE7_W {
        P2IE7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 2 Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2ie](index.html) module"]
pub struct P2IE_SPEC;
impl crate::RegisterSpec for P2IE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p2ie::R](R) reader structure"]
impl crate::Readable for P2IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p2ie::W](W) writer structure"]
impl crate::Writable for P2IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P2IE to value 0"]
impl crate::Resettable for P2IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
