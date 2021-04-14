#[doc = "Register `RTCPS0CTL` reader"]
pub struct R(crate::R<RTCPS0CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCPS0CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCPS0CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCPS0CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCPS0CTL` writer"]
pub struct W(crate::W<RTCPS0CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCPS0CTL_SPEC>;
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
impl From<crate::W<RTCPS0CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCPS0CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RT0PSIFG` reader - RTC Prescale Timer 0 Interrupt Flag"]
pub struct RT0PSIFG_R(crate::FieldReader<bool, bool>);
impl RT0PSIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT0PSIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT0PSIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT0PSIFG` writer - RTC Prescale Timer 0 Interrupt Flag"]
pub struct RT0PSIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0PSIFG_W<'a> {
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
#[doc = "Field `RT0PSIE` reader - RTC Prescale Timer 0 Interrupt Enable Flag"]
pub struct RT0PSIE_R(crate::FieldReader<bool, bool>);
impl RT0PSIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT0PSIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT0PSIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT0PSIE` writer - RTC Prescale Timer 0 Interrupt Enable Flag"]
pub struct RT0PSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0PSIE_W<'a> {
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
#[doc = "RTC Prescale Timer 0 Interrupt Interval Bit: 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RT0IP_A {
    #[doc = "0: RTC Prescale Timer 0 Interrupt Interval /2"]
    RT0IP_0 = 0,
    #[doc = "1: RTC Prescale Timer 0 Interrupt Interval /4"]
    RT0IP_1 = 1,
    #[doc = "2: RTC Prescale Timer 0 Interrupt Interval /8"]
    RT0IP_2 = 2,
    #[doc = "3: RTC Prescale Timer 0 Interrupt Interval /16"]
    RT0IP_3 = 3,
    #[doc = "4: RTC Prescale Timer 0 Interrupt Interval /32"]
    RT0IP_4 = 4,
    #[doc = "5: RTC Prescale Timer 0 Interrupt Interval /64"]
    RT0IP_5 = 5,
    #[doc = "6: RTC Prescale Timer 0 Interrupt Interval /128"]
    RT0IP_6 = 6,
    #[doc = "7: RTC Prescale Timer 0 Interrupt Interval /256"]
    RT0IP_7 = 7,
}
impl From<RT0IP_A> for u8 {
    #[inline(always)]
    fn from(variant: RT0IP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RT0IP` reader - RTC Prescale Timer 0 Interrupt Interval Bit: 2"]
pub struct RT0IP_R(crate::FieldReader<u8, RT0IP_A>);
impl RT0IP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RT0IP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT0IP_A {
        match self.bits {
            0 => RT0IP_A::RT0IP_0,
            1 => RT0IP_A::RT0IP_1,
            2 => RT0IP_A::RT0IP_2,
            3 => RT0IP_A::RT0IP_3,
            4 => RT0IP_A::RT0IP_4,
            5 => RT0IP_A::RT0IP_5,
            6 => RT0IP_A::RT0IP_6,
            7 => RT0IP_A::RT0IP_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RT0IP_0`"]
    #[inline(always)]
    pub fn is_rt0ip_0(&self) -> bool {
        **self == RT0IP_A::RT0IP_0
    }
    #[doc = "Checks if the value of the field is `RT0IP_1`"]
    #[inline(always)]
    pub fn is_rt0ip_1(&self) -> bool {
        **self == RT0IP_A::RT0IP_1
    }
    #[doc = "Checks if the value of the field is `RT0IP_2`"]
    #[inline(always)]
    pub fn is_rt0ip_2(&self) -> bool {
        **self == RT0IP_A::RT0IP_2
    }
    #[doc = "Checks if the value of the field is `RT0IP_3`"]
    #[inline(always)]
    pub fn is_rt0ip_3(&self) -> bool {
        **self == RT0IP_A::RT0IP_3
    }
    #[doc = "Checks if the value of the field is `RT0IP_4`"]
    #[inline(always)]
    pub fn is_rt0ip_4(&self) -> bool {
        **self == RT0IP_A::RT0IP_4
    }
    #[doc = "Checks if the value of the field is `RT0IP_5`"]
    #[inline(always)]
    pub fn is_rt0ip_5(&self) -> bool {
        **self == RT0IP_A::RT0IP_5
    }
    #[doc = "Checks if the value of the field is `RT0IP_6`"]
    #[inline(always)]
    pub fn is_rt0ip_6(&self) -> bool {
        **self == RT0IP_A::RT0IP_6
    }
    #[doc = "Checks if the value of the field is `RT0IP_7`"]
    #[inline(always)]
    pub fn is_rt0ip_7(&self) -> bool {
        **self == RT0IP_A::RT0IP_7
    }
}
impl core::ops::Deref for RT0IP_R {
    type Target = crate::FieldReader<u8, RT0IP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT0IP` writer - RTC Prescale Timer 0 Interrupt Interval Bit: 2"]
pub struct RT0IP_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0IP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT0IP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "RTC Prescale Timer 0 Interrupt Interval /2"]
    #[inline(always)]
    pub fn rt0ip_0(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_0)
    }
    #[doc = "RTC Prescale Timer 0 Interrupt Interval /4"]
    #[inline(always)]
    pub fn rt0ip_1(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_1)
    }
    #[doc = "RTC Prescale Timer 0 Interrupt Interval /8"]
    #[inline(always)]
    pub fn rt0ip_2(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_2)
    }
    #[doc = "RTC Prescale Timer 0 Interrupt Interval /16"]
    #[inline(always)]
    pub fn rt0ip_3(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_3)
    }
    #[doc = "RTC Prescale Timer 0 Interrupt Interval /32"]
    #[inline(always)]
    pub fn rt0ip_4(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_4)
    }
    #[doc = "RTC Prescale Timer 0 Interrupt Interval /64"]
    #[inline(always)]
    pub fn rt0ip_5(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_5)
    }
    #[doc = "RTC Prescale Timer 0 Interrupt Interval /128"]
    #[inline(always)]
    pub fn rt0ip_6(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_6)
    }
    #[doc = "RTC Prescale Timer 0 Interrupt Interval /256"]
    #[inline(always)]
    pub fn rt0ip_7(self) -> &'a mut W {
        self.variant(RT0IP_A::RT0IP_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u16 & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `RT0PSHOLD` reader - RTC Prescale Timer 0 Hold"]
pub struct RT0PSHOLD_R(crate::FieldReader<bool, bool>);
impl RT0PSHOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT0PSHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT0PSHOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT0PSHOLD` writer - RTC Prescale Timer 0 Hold"]
pub struct RT0PSHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0PSHOLD_W<'a> {
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
#[doc = "RTC Prescale Timer 0 Clock Divide Bit: 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RT0PSDIV_A {
    #[doc = "0: RTC Prescale Timer 0 Clock Divide /2"]
    RT0PSDIV_0 = 0,
    #[doc = "1: RTC Prescale Timer 0 Clock Divide /4"]
    RT0PSDIV_1 = 1,
    #[doc = "2: RTC Prescale Timer 0 Clock Divide /8"]
    RT0PSDIV_2 = 2,
    #[doc = "3: RTC Prescale Timer 0 Clock Divide /16"]
    RT0PSDIV_3 = 3,
    #[doc = "4: RTC Prescale Timer 0 Clock Divide /32"]
    RT0PSDIV_4 = 4,
    #[doc = "5: RTC Prescale Timer 0 Clock Divide /64"]
    RT0PSDIV_5 = 5,
    #[doc = "6: RTC Prescale Timer 0 Clock Divide /128"]
    RT0PSDIV_6 = 6,
    #[doc = "7: RTC Prescale Timer 0 Clock Divide /256"]
    RT0PSDIV_7 = 7,
}
impl From<RT0PSDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: RT0PSDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RT0PSDIV` reader - RTC Prescale Timer 0 Clock Divide Bit: 2"]
pub struct RT0PSDIV_R(crate::FieldReader<u8, RT0PSDIV_A>);
impl RT0PSDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RT0PSDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT0PSDIV_A {
        match self.bits {
            0 => RT0PSDIV_A::RT0PSDIV_0,
            1 => RT0PSDIV_A::RT0PSDIV_1,
            2 => RT0PSDIV_A::RT0PSDIV_2,
            3 => RT0PSDIV_A::RT0PSDIV_3,
            4 => RT0PSDIV_A::RT0PSDIV_4,
            5 => RT0PSDIV_A::RT0PSDIV_5,
            6 => RT0PSDIV_A::RT0PSDIV_6,
            7 => RT0PSDIV_A::RT0PSDIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RT0PSDIV_0`"]
    #[inline(always)]
    pub fn is_rt0psdiv_0(&self) -> bool {
        **self == RT0PSDIV_A::RT0PSDIV_0
    }
    #[doc = "Checks if the value of the field is `RT0PSDIV_1`"]
    #[inline(always)]
    pub fn is_rt0psdiv_1(&self) -> bool {
        **self == RT0PSDIV_A::RT0PSDIV_1
    }
    #[doc = "Checks if the value of the field is `RT0PSDIV_2`"]
    #[inline(always)]
    pub fn is_rt0psdiv_2(&self) -> bool {
        **self == RT0PSDIV_A::RT0PSDIV_2
    }
    #[doc = "Checks if the value of the field is `RT0PSDIV_3`"]
    #[inline(always)]
    pub fn is_rt0psdiv_3(&self) -> bool {
        **self == RT0PSDIV_A::RT0PSDIV_3
    }
    #[doc = "Checks if the value of the field is `RT0PSDIV_4`"]
    #[inline(always)]
    pub fn is_rt0psdiv_4(&self) -> bool {
        **self == RT0PSDIV_A::RT0PSDIV_4
    }
    #[doc = "Checks if the value of the field is `RT0PSDIV_5`"]
    #[inline(always)]
    pub fn is_rt0psdiv_5(&self) -> bool {
        **self == RT0PSDIV_A::RT0PSDIV_5
    }
    #[doc = "Checks if the value of the field is `RT0PSDIV_6`"]
    #[inline(always)]
    pub fn is_rt0psdiv_6(&self) -> bool {
        **self == RT0PSDIV_A::RT0PSDIV_6
    }
    #[doc = "Checks if the value of the field is `RT0PSDIV_7`"]
    #[inline(always)]
    pub fn is_rt0psdiv_7(&self) -> bool {
        **self == RT0PSDIV_A::RT0PSDIV_7
    }
}
impl core::ops::Deref for RT0PSDIV_R {
    type Target = crate::FieldReader<u8, RT0PSDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT0PSDIV` writer - RTC Prescale Timer 0 Clock Divide Bit: 2"]
pub struct RT0PSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0PSDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT0PSDIV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "RTC Prescale Timer 0 Clock Divide /2"]
    #[inline(always)]
    pub fn rt0psdiv_0(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::RT0PSDIV_0)
    }
    #[doc = "RTC Prescale Timer 0 Clock Divide /4"]
    #[inline(always)]
    pub fn rt0psdiv_1(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::RT0PSDIV_1)
    }
    #[doc = "RTC Prescale Timer 0 Clock Divide /8"]
    #[inline(always)]
    pub fn rt0psdiv_2(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::RT0PSDIV_2)
    }
    #[doc = "RTC Prescale Timer 0 Clock Divide /16"]
    #[inline(always)]
    pub fn rt0psdiv_3(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::RT0PSDIV_3)
    }
    #[doc = "RTC Prescale Timer 0 Clock Divide /32"]
    #[inline(always)]
    pub fn rt0psdiv_4(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::RT0PSDIV_4)
    }
    #[doc = "RTC Prescale Timer 0 Clock Divide /64"]
    #[inline(always)]
    pub fn rt0psdiv_5(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::RT0PSDIV_5)
    }
    #[doc = "RTC Prescale Timer 0 Clock Divide /128"]
    #[inline(always)]
    pub fn rt0psdiv_6(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::RT0PSDIV_6)
    }
    #[doc = "RTC Prescale Timer 0 Clock Divide /256"]
    #[inline(always)]
    pub fn rt0psdiv_7(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::RT0PSDIV_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u16 & 0x07) << 11);
        self.w
    }
}
#[doc = "Field `RT0SSEL` reader - RTC Prescale Timer 0 Source Select 0:ACLK / 1:SMCLK"]
pub struct RT0SSEL_R(crate::FieldReader<bool, bool>);
impl RT0SSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT0SSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT0SSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT0SSEL` writer - RTC Prescale Timer 0 Source Select 0:ACLK / 1:SMCLK"]
pub struct RT0SSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0SSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u16 & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RTC Prescale Timer 0 Interrupt Flag"]
    #[inline(always)]
    pub fn rt0psifg(&self) -> RT0PSIFG_R {
        RT0PSIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC Prescale Timer 0 Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rt0psie(&self) -> RT0PSIE_R {
        RT0PSIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - RTC Prescale Timer 0 Interrupt Interval Bit: 2"]
    #[inline(always)]
    pub fn rt0ip(&self) -> RT0IP_R {
        RT0IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 8 - RTC Prescale Timer 0 Hold"]
    #[inline(always)]
    pub fn rt0pshold(&self) -> RT0PSHOLD_R {
        RT0PSHOLD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - RTC Prescale Timer 0 Clock Divide Bit: 2"]
    #[inline(always)]
    pub fn rt0psdiv(&self) -> RT0PSDIV_R {
        RT0PSDIV_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 14 - RTC Prescale Timer 0 Source Select 0:ACLK / 1:SMCLK"]
    #[inline(always)]
    pub fn rt0ssel(&self) -> RT0SSEL_R {
        RT0SSEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Prescale Timer 0 Interrupt Flag"]
    #[inline(always)]
    pub fn rt0psifg(&mut self) -> RT0PSIFG_W {
        RT0PSIFG_W { w: self }
    }
    #[doc = "Bit 1 - RTC Prescale Timer 0 Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rt0psie(&mut self) -> RT0PSIE_W {
        RT0PSIE_W { w: self }
    }
    #[doc = "Bits 2:4 - RTC Prescale Timer 0 Interrupt Interval Bit: 2"]
    #[inline(always)]
    pub fn rt0ip(&mut self) -> RT0IP_W {
        RT0IP_W { w: self }
    }
    #[doc = "Bit 8 - RTC Prescale Timer 0 Hold"]
    #[inline(always)]
    pub fn rt0pshold(&mut self) -> RT0PSHOLD_W {
        RT0PSHOLD_W { w: self }
    }
    #[doc = "Bits 11:13 - RTC Prescale Timer 0 Clock Divide Bit: 2"]
    #[inline(always)]
    pub fn rt0psdiv(&mut self) -> RT0PSDIV_W {
        RT0PSDIV_W { w: self }
    }
    #[doc = "Bit 14 - RTC Prescale Timer 0 Source Select 0:ACLK / 1:SMCLK"]
    #[inline(always)]
    pub fn rt0ssel(&mut self) -> RT0SSEL_W {
        RT0SSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Timer Prescale Timer 0 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcps0ctl](index.html) module"]
pub struct RTCPS0CTL_SPEC;
impl crate::RegisterSpec for RTCPS0CTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcps0ctl::R](R) reader structure"]
impl crate::Readable for RTCPS0CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcps0ctl::W](W) writer structure"]
impl crate::Writable for RTCPS0CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCPS0CTL to value 0"]
impl crate::Resettable for RTCPS0CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
