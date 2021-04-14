#[doc = "Register `P2DS` reader"]
pub struct R(crate::R<P2DS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P2DS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P2DS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P2DS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P2DS` writer"]
pub struct W(crate::W<P2DS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P2DS_SPEC>;
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
impl From<crate::W<P2DS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P2DS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P2DS0` reader - P2DS0"]
pub struct P2DS0_R(crate::FieldReader<bool, bool>);
impl P2DS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2DS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2DS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2DS0` writer - P2DS0"]
pub struct P2DS0_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DS0_W<'a> {
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
#[doc = "Field `P2DS1` reader - P2DS1"]
pub struct P2DS1_R(crate::FieldReader<bool, bool>);
impl P2DS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2DS1` writer - P2DS1"]
pub struct P2DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DS1_W<'a> {
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
#[doc = "Field `P2DS2` reader - P2DS2"]
pub struct P2DS2_R(crate::FieldReader<bool, bool>);
impl P2DS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2DS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2DS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2DS2` writer - P2DS2"]
pub struct P2DS2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DS2_W<'a> {
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
#[doc = "Field `P2DS3` reader - P2DS3"]
pub struct P2DS3_R(crate::FieldReader<bool, bool>);
impl P2DS3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2DS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2DS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2DS3` writer - P2DS3"]
pub struct P2DS3_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DS3_W<'a> {
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
#[doc = "Field `P2DS4` reader - P2DS4"]
pub struct P2DS4_R(crate::FieldReader<bool, bool>);
impl P2DS4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2DS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2DS4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2DS4` writer - P2DS4"]
pub struct P2DS4_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DS4_W<'a> {
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
#[doc = "Field `P2DS5` reader - P2DS5"]
pub struct P2DS5_R(crate::FieldReader<bool, bool>);
impl P2DS5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2DS5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2DS5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2DS5` writer - P2DS5"]
pub struct P2DS5_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DS5_W<'a> {
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
#[doc = "Field `P2DS6` reader - P2DS6"]
pub struct P2DS6_R(crate::FieldReader<bool, bool>);
impl P2DS6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2DS6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2DS6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2DS6` writer - P2DS6"]
pub struct P2DS6_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DS6_W<'a> {
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
#[doc = "Field `P2DS7` reader - P2DS7"]
pub struct P2DS7_R(crate::FieldReader<bool, bool>);
impl P2DS7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2DS7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2DS7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2DS7` writer - P2DS7"]
pub struct P2DS7_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DS7_W<'a> {
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
    #[doc = "Bit 0 - P2DS0"]
    #[inline(always)]
    pub fn p2ds0(&self) -> P2DS0_R {
        P2DS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P2DS1"]
    #[inline(always)]
    pub fn p2ds1(&self) -> P2DS1_R {
        P2DS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P2DS2"]
    #[inline(always)]
    pub fn p2ds2(&self) -> P2DS2_R {
        P2DS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P2DS3"]
    #[inline(always)]
    pub fn p2ds3(&self) -> P2DS3_R {
        P2DS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P2DS4"]
    #[inline(always)]
    pub fn p2ds4(&self) -> P2DS4_R {
        P2DS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P2DS5"]
    #[inline(always)]
    pub fn p2ds5(&self) -> P2DS5_R {
        P2DS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P2DS6"]
    #[inline(always)]
    pub fn p2ds6(&self) -> P2DS6_R {
        P2DS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P2DS7"]
    #[inline(always)]
    pub fn p2ds7(&self) -> P2DS7_R {
        P2DS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2DS0"]
    #[inline(always)]
    pub fn p2ds0(&mut self) -> P2DS0_W {
        P2DS0_W { w: self }
    }
    #[doc = "Bit 1 - P2DS1"]
    #[inline(always)]
    pub fn p2ds1(&mut self) -> P2DS1_W {
        P2DS1_W { w: self }
    }
    #[doc = "Bit 2 - P2DS2"]
    #[inline(always)]
    pub fn p2ds2(&mut self) -> P2DS2_W {
        P2DS2_W { w: self }
    }
    #[doc = "Bit 3 - P2DS3"]
    #[inline(always)]
    pub fn p2ds3(&mut self) -> P2DS3_W {
        P2DS3_W { w: self }
    }
    #[doc = "Bit 4 - P2DS4"]
    #[inline(always)]
    pub fn p2ds4(&mut self) -> P2DS4_W {
        P2DS4_W { w: self }
    }
    #[doc = "Bit 5 - P2DS5"]
    #[inline(always)]
    pub fn p2ds5(&mut self) -> P2DS5_W {
        P2DS5_W { w: self }
    }
    #[doc = "Bit 6 - P2DS6"]
    #[inline(always)]
    pub fn p2ds6(&mut self) -> P2DS6_W {
        P2DS6_W { w: self }
    }
    #[doc = "Bit 7 - P2DS7"]
    #[inline(always)]
    pub fn p2ds7(&mut self) -> P2DS7_W {
        P2DS7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 2 Drive Strenght\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2ds](index.html) module"]
pub struct P2DS_SPEC;
impl crate::RegisterSpec for P2DS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p2ds::R](R) reader structure"]
impl crate::Readable for P2DS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p2ds::W](W) writer structure"]
impl crate::Writable for P2DS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P2DS to value 0"]
impl crate::Resettable for P2DS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
