#[doc = "Register `UCB0I2CSA` reader"]
pub struct R(crate::R<UCB0I2CSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0I2CSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0I2CSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0I2CSA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0I2CSA` writer"]
pub struct W(crate::W<UCB0I2CSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0I2CSA_SPEC>;
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
impl From<crate::W<UCB0I2CSA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0I2CSA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCSA0` reader - I2C Slave Address 0"]
pub struct UCSA0_R(crate::FieldReader<bool, bool>);
impl UCSA0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCSA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSA0` writer - I2C Slave Address 0"]
pub struct UCSA0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA0_W<'a> {
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
#[doc = "Field `UCSA1` reader - I2C Slave Address 1"]
pub struct UCSA1_R(crate::FieldReader<bool, bool>);
impl UCSA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCSA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSA1` writer - I2C Slave Address 1"]
pub struct UCSA1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA1_W<'a> {
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
#[doc = "Field `UCSA2` reader - I2C Slave Address 2"]
pub struct UCSA2_R(crate::FieldReader<bool, bool>);
impl UCSA2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCSA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSA2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSA2` writer - I2C Slave Address 2"]
pub struct UCSA2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA2_W<'a> {
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
#[doc = "Field `UCSA3` reader - I2C Slave Address 3"]
pub struct UCSA3_R(crate::FieldReader<bool, bool>);
impl UCSA3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCSA3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSA3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSA3` writer - I2C Slave Address 3"]
pub struct UCSA3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA3_W<'a> {
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
#[doc = "Field `UCSA4` reader - I2C Slave Address 4"]
pub struct UCSA4_R(crate::FieldReader<bool, bool>);
impl UCSA4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCSA4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSA4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSA4` writer - I2C Slave Address 4"]
pub struct UCSA4_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA4_W<'a> {
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
#[doc = "Field `UCSA5` reader - I2C Slave Address 5"]
pub struct UCSA5_R(crate::FieldReader<bool, bool>);
impl UCSA5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCSA5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSA5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSA5` writer - I2C Slave Address 5"]
pub struct UCSA5_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA5_W<'a> {
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
#[doc = "Field `UCSA6` reader - I2C Slave Address 6"]
pub struct UCSA6_R(crate::FieldReader<bool, bool>);
impl UCSA6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCSA6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSA6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSA6` writer - I2C Slave Address 6"]
pub struct UCSA6_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA6_W<'a> {
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
#[doc = "Field `UCSA7` reader - I2C Slave Address 7"]
pub struct UCSA7_R(crate::FieldReader<bool, bool>);
impl UCSA7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCSA7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSA7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSA7` writer - I2C Slave Address 7"]
pub struct UCSA7_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA7_W<'a> {
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
#[doc = "Field `UCSA8` reader - I2C Slave Address 8"]
pub struct UCSA8_R(crate::FieldReader<bool, bool>);
impl UCSA8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCSA8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSA8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSA8` writer - I2C Slave Address 8"]
pub struct UCSA8_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA8_W<'a> {
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
#[doc = "Field `UCSA9` reader - I2C Slave Address 9"]
pub struct UCSA9_R(crate::FieldReader<bool, bool>);
impl UCSA9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCSA9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSA9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSA9` writer - I2C Slave Address 9"]
pub struct UCSA9_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA9_W<'a> {
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
impl R {
    #[doc = "Bit 0 - I2C Slave Address 0"]
    #[inline(always)]
    pub fn ucsa0(&self) -> UCSA0_R {
        UCSA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C Slave Address 1"]
    #[inline(always)]
    pub fn ucsa1(&self) -> UCSA1_R {
        UCSA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C Slave Address 2"]
    #[inline(always)]
    pub fn ucsa2(&self) -> UCSA2_R {
        UCSA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C Slave Address 3"]
    #[inline(always)]
    pub fn ucsa3(&self) -> UCSA3_R {
        UCSA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C Slave Address 4"]
    #[inline(always)]
    pub fn ucsa4(&self) -> UCSA4_R {
        UCSA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C Slave Address 5"]
    #[inline(always)]
    pub fn ucsa5(&self) -> UCSA5_R {
        UCSA5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C Slave Address 6"]
    #[inline(always)]
    pub fn ucsa6(&self) -> UCSA6_R {
        UCSA6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C Slave Address 7"]
    #[inline(always)]
    pub fn ucsa7(&self) -> UCSA7_R {
        UCSA7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C Slave Address 8"]
    #[inline(always)]
    pub fn ucsa8(&self) -> UCSA8_R {
        UCSA8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - I2C Slave Address 9"]
    #[inline(always)]
    pub fn ucsa9(&self) -> UCSA9_R {
        UCSA9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Slave Address 0"]
    #[inline(always)]
    pub fn ucsa0(&mut self) -> UCSA0_W {
        UCSA0_W { w: self }
    }
    #[doc = "Bit 1 - I2C Slave Address 1"]
    #[inline(always)]
    pub fn ucsa1(&mut self) -> UCSA1_W {
        UCSA1_W { w: self }
    }
    #[doc = "Bit 2 - I2C Slave Address 2"]
    #[inline(always)]
    pub fn ucsa2(&mut self) -> UCSA2_W {
        UCSA2_W { w: self }
    }
    #[doc = "Bit 3 - I2C Slave Address 3"]
    #[inline(always)]
    pub fn ucsa3(&mut self) -> UCSA3_W {
        UCSA3_W { w: self }
    }
    #[doc = "Bit 4 - I2C Slave Address 4"]
    #[inline(always)]
    pub fn ucsa4(&mut self) -> UCSA4_W {
        UCSA4_W { w: self }
    }
    #[doc = "Bit 5 - I2C Slave Address 5"]
    #[inline(always)]
    pub fn ucsa5(&mut self) -> UCSA5_W {
        UCSA5_W { w: self }
    }
    #[doc = "Bit 6 - I2C Slave Address 6"]
    #[inline(always)]
    pub fn ucsa6(&mut self) -> UCSA6_W {
        UCSA6_W { w: self }
    }
    #[doc = "Bit 7 - I2C Slave Address 7"]
    #[inline(always)]
    pub fn ucsa7(&mut self) -> UCSA7_W {
        UCSA7_W { w: self }
    }
    #[doc = "Bit 8 - I2C Slave Address 8"]
    #[inline(always)]
    pub fn ucsa8(&mut self) -> UCSA8_W {
        UCSA8_W { w: self }
    }
    #[doc = "Bit 9 - I2C Slave Address 9"]
    #[inline(always)]
    pub fn ucsa9(&mut self) -> UCSA9_W {
        UCSA9_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 I2C Slave Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0i2csa](index.html) module"]
pub struct UCB0I2CSA_SPEC;
impl crate::RegisterSpec for UCB0I2CSA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb0i2csa::R](R) reader structure"]
impl crate::Readable for UCB0I2CSA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0i2csa::W](W) writer structure"]
impl crate::Writable for UCB0I2CSA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0I2CSA to value 0"]
impl crate::Resettable for UCB0I2CSA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
