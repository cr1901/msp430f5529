#[doc = "Register `P8DS` reader"]
pub struct R(crate::R<P8DS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P8DS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P8DS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P8DS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P8DS` writer"]
pub struct W(crate::W<P8DS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P8DS_SPEC>;
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
impl From<crate::W<P8DS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P8DS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P8DS0` reader - P8DS0"]
pub struct P8DS0_R(crate::FieldReader<bool, bool>);
impl P8DS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8DS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8DS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8DS0` writer - P8DS0"]
pub struct P8DS0_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DS0_W<'a> {
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
#[doc = "Field `P8DS1` reader - P8DS1"]
pub struct P8DS1_R(crate::FieldReader<bool, bool>);
impl P8DS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8DS1` writer - P8DS1"]
pub struct P8DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DS1_W<'a> {
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
#[doc = "Field `P8DS2` reader - P8DS2"]
pub struct P8DS2_R(crate::FieldReader<bool, bool>);
impl P8DS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8DS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8DS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8DS2` writer - P8DS2"]
pub struct P8DS2_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DS2_W<'a> {
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
#[doc = "Field `P8DS3` reader - P8DS3"]
pub struct P8DS3_R(crate::FieldReader<bool, bool>);
impl P8DS3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8DS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8DS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8DS3` writer - P8DS3"]
pub struct P8DS3_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DS3_W<'a> {
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
#[doc = "Field `P8DS4` reader - P8DS4"]
pub struct P8DS4_R(crate::FieldReader<bool, bool>);
impl P8DS4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8DS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8DS4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8DS4` writer - P8DS4"]
pub struct P8DS4_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DS4_W<'a> {
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
#[doc = "Field `P8DS5` reader - P8DS5"]
pub struct P8DS5_R(crate::FieldReader<bool, bool>);
impl P8DS5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8DS5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8DS5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8DS5` writer - P8DS5"]
pub struct P8DS5_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DS5_W<'a> {
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
#[doc = "Field `P8DS6` reader - P8DS6"]
pub struct P8DS6_R(crate::FieldReader<bool, bool>);
impl P8DS6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8DS6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8DS6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8DS6` writer - P8DS6"]
pub struct P8DS6_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DS6_W<'a> {
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
#[doc = "Field `P8DS7` reader - P8DS7"]
pub struct P8DS7_R(crate::FieldReader<bool, bool>);
impl P8DS7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8DS7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8DS7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8DS7` writer - P8DS7"]
pub struct P8DS7_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DS7_W<'a> {
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
    #[doc = "Bit 0 - P8DS0"]
    #[inline(always)]
    pub fn p8ds0(&self) -> P8DS0_R {
        P8DS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P8DS1"]
    #[inline(always)]
    pub fn p8ds1(&self) -> P8DS1_R {
        P8DS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P8DS2"]
    #[inline(always)]
    pub fn p8ds2(&self) -> P8DS2_R {
        P8DS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P8DS3"]
    #[inline(always)]
    pub fn p8ds3(&self) -> P8DS3_R {
        P8DS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P8DS4"]
    #[inline(always)]
    pub fn p8ds4(&self) -> P8DS4_R {
        P8DS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P8DS5"]
    #[inline(always)]
    pub fn p8ds5(&self) -> P8DS5_R {
        P8DS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P8DS6"]
    #[inline(always)]
    pub fn p8ds6(&self) -> P8DS6_R {
        P8DS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P8DS7"]
    #[inline(always)]
    pub fn p8ds7(&self) -> P8DS7_R {
        P8DS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P8DS0"]
    #[inline(always)]
    pub fn p8ds0(&mut self) -> P8DS0_W {
        P8DS0_W { w: self }
    }
    #[doc = "Bit 1 - P8DS1"]
    #[inline(always)]
    pub fn p8ds1(&mut self) -> P8DS1_W {
        P8DS1_W { w: self }
    }
    #[doc = "Bit 2 - P8DS2"]
    #[inline(always)]
    pub fn p8ds2(&mut self) -> P8DS2_W {
        P8DS2_W { w: self }
    }
    #[doc = "Bit 3 - P8DS3"]
    #[inline(always)]
    pub fn p8ds3(&mut self) -> P8DS3_W {
        P8DS3_W { w: self }
    }
    #[doc = "Bit 4 - P8DS4"]
    #[inline(always)]
    pub fn p8ds4(&mut self) -> P8DS4_W {
        P8DS4_W { w: self }
    }
    #[doc = "Bit 5 - P8DS5"]
    #[inline(always)]
    pub fn p8ds5(&mut self) -> P8DS5_W {
        P8DS5_W { w: self }
    }
    #[doc = "Bit 6 - P8DS6"]
    #[inline(always)]
    pub fn p8ds6(&mut self) -> P8DS6_W {
        P8DS6_W { w: self }
    }
    #[doc = "Bit 7 - P8DS7"]
    #[inline(always)]
    pub fn p8ds7(&mut self) -> P8DS7_W {
        P8DS7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 8 Drive Strenght\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p8ds](index.html) module"]
pub struct P8DS_SPEC;
impl crate::RegisterSpec for P8DS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p8ds::R](R) reader structure"]
impl crate::Readable for P8DS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p8ds::W](W) writer structure"]
impl crate::Writable for P8DS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P8DS to value 0"]
impl crate::Resettable for P8DS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
