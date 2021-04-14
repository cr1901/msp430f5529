#[doc = "Register `UCB0I2COA` reader"]
pub struct R(crate::R<UCB0I2COA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0I2COA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0I2COA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0I2COA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0I2COA` writer"]
pub struct W(crate::W<UCB0I2COA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0I2COA_SPEC>;
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
impl From<crate::W<UCB0I2COA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0I2COA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCOA0` reader - I2C Own Address 0"]
pub struct UCOA0_R(crate::FieldReader<bool, bool>);
impl UCOA0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCOA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCOA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCOA0` writer - I2C Own Address 0"]
pub struct UCOA0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `UCOA1` reader - I2C Own Address 1"]
pub struct UCOA1_R(crate::FieldReader<bool, bool>);
impl UCOA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCOA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCOA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCOA1` writer - I2C Own Address 1"]
pub struct UCOA1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `UCOA2` reader - I2C Own Address 2"]
pub struct UCOA2_R(crate::FieldReader<bool, bool>);
impl UCOA2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCOA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCOA2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCOA2` writer - I2C Own Address 2"]
pub struct UCOA2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `UCOA3` reader - I2C Own Address 3"]
pub struct UCOA3_R(crate::FieldReader<bool, bool>);
impl UCOA3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCOA3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCOA3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCOA3` writer - I2C Own Address 3"]
pub struct UCOA3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `UCOA4` reader - I2C Own Address 4"]
pub struct UCOA4_R(crate::FieldReader<bool, bool>);
impl UCOA4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCOA4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCOA4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCOA4` writer - I2C Own Address 4"]
pub struct UCOA4_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `UCOA5` reader - I2C Own Address 5"]
pub struct UCOA5_R(crate::FieldReader<bool, bool>);
impl UCOA5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCOA5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCOA5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCOA5` writer - I2C Own Address 5"]
pub struct UCOA5_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `UCOA6` reader - I2C Own Address 6"]
pub struct UCOA6_R(crate::FieldReader<bool, bool>);
impl UCOA6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCOA6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCOA6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCOA6` writer - I2C Own Address 6"]
pub struct UCOA6_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `UCOA7` reader - I2C Own Address 7"]
pub struct UCOA7_R(crate::FieldReader<bool, bool>);
impl UCOA7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCOA7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCOA7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCOA7` writer - I2C Own Address 7"]
pub struct UCOA7_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `UCOA8` reader - I2C Own Address 8"]
pub struct UCOA8_R(crate::FieldReader<bool, bool>);
impl UCOA8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCOA8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCOA8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCOA8` writer - I2C Own Address 8"]
pub struct UCOA8_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `UCOA9` reader - I2C Own Address 9"]
pub struct UCOA9_R(crate::FieldReader<bool, bool>);
impl UCOA9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCOA9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCOA9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCOA9` writer - I2C Own Address 9"]
pub struct UCOA9_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `UCGCEN` reader - I2C General Call enable"]
pub struct UCGCEN_R(crate::FieldReader<bool, bool>);
impl UCGCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCGCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCGCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCGCEN` writer - I2C General Call enable"]
pub struct UCGCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCGCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u16 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - I2C Own Address 0"]
    #[inline(always)]
    pub fn ucoa0(&self) -> UCOA0_R {
        UCOA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C Own Address 1"]
    #[inline(always)]
    pub fn ucoa1(&self) -> UCOA1_R {
        UCOA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C Own Address 2"]
    #[inline(always)]
    pub fn ucoa2(&self) -> UCOA2_R {
        UCOA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C Own Address 3"]
    #[inline(always)]
    pub fn ucoa3(&self) -> UCOA3_R {
        UCOA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C Own Address 4"]
    #[inline(always)]
    pub fn ucoa4(&self) -> UCOA4_R {
        UCOA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C Own Address 5"]
    #[inline(always)]
    pub fn ucoa5(&self) -> UCOA5_R {
        UCOA5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C Own Address 6"]
    #[inline(always)]
    pub fn ucoa6(&self) -> UCOA6_R {
        UCOA6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C Own Address 7"]
    #[inline(always)]
    pub fn ucoa7(&self) -> UCOA7_R {
        UCOA7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C Own Address 8"]
    #[inline(always)]
    pub fn ucoa8(&self) -> UCOA8_R {
        UCOA8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - I2C Own Address 9"]
    #[inline(always)]
    pub fn ucoa9(&self) -> UCOA9_R {
        UCOA9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 15 - I2C General Call enable"]
    #[inline(always)]
    pub fn ucgcen(&self) -> UCGCEN_R {
        UCGCEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Own Address 0"]
    #[inline(always)]
    pub fn ucoa0(&mut self) -> UCOA0_W {
        UCOA0_W { w: self }
    }
    #[doc = "Bit 1 - I2C Own Address 1"]
    #[inline(always)]
    pub fn ucoa1(&mut self) -> UCOA1_W {
        UCOA1_W { w: self }
    }
    #[doc = "Bit 2 - I2C Own Address 2"]
    #[inline(always)]
    pub fn ucoa2(&mut self) -> UCOA2_W {
        UCOA2_W { w: self }
    }
    #[doc = "Bit 3 - I2C Own Address 3"]
    #[inline(always)]
    pub fn ucoa3(&mut self) -> UCOA3_W {
        UCOA3_W { w: self }
    }
    #[doc = "Bit 4 - I2C Own Address 4"]
    #[inline(always)]
    pub fn ucoa4(&mut self) -> UCOA4_W {
        UCOA4_W { w: self }
    }
    #[doc = "Bit 5 - I2C Own Address 5"]
    #[inline(always)]
    pub fn ucoa5(&mut self) -> UCOA5_W {
        UCOA5_W { w: self }
    }
    #[doc = "Bit 6 - I2C Own Address 6"]
    #[inline(always)]
    pub fn ucoa6(&mut self) -> UCOA6_W {
        UCOA6_W { w: self }
    }
    #[doc = "Bit 7 - I2C Own Address 7"]
    #[inline(always)]
    pub fn ucoa7(&mut self) -> UCOA7_W {
        UCOA7_W { w: self }
    }
    #[doc = "Bit 8 - I2C Own Address 8"]
    #[inline(always)]
    pub fn ucoa8(&mut self) -> UCOA8_W {
        UCOA8_W { w: self }
    }
    #[doc = "Bit 9 - I2C Own Address 9"]
    #[inline(always)]
    pub fn ucoa9(&mut self) -> UCOA9_W {
        UCOA9_W { w: self }
    }
    #[doc = "Bit 15 - I2C General Call enable"]
    #[inline(always)]
    pub fn ucgcen(&mut self) -> UCGCEN_W {
        UCGCEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 I2C Own Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0i2coa](index.html) module"]
pub struct UCB0I2COA_SPEC;
impl crate::RegisterSpec for UCB0I2COA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb0i2coa::R](R) reader structure"]
impl crate::Readable for UCB0I2COA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0i2coa::W](W) writer structure"]
impl crate::Writable for UCB0I2COA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0I2COA to value 0"]
impl crate::Resettable for UCB0I2COA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
