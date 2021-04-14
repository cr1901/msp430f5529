#[doc = "Register `PMMCTL0` reader"]
pub struct R(crate::R<PMMCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMMCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMMCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMMCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMMCTL0` writer"]
pub struct W(crate::W<PMMCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMMCTL0_SPEC>;
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
impl From<crate::W<PMMCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMMCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PMM Core Voltage Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PMMCOREV_A {
    #[doc = "0: PMM Core Voltage 0 (1.35V)"]
    PMMCOREV_0 = 0,
    #[doc = "1: PMM Core Voltage 1 (1.55V)"]
    PMMCOREV_1 = 1,
    #[doc = "2: PMM Core Voltage 2 (1.75V)"]
    PMMCOREV_2 = 2,
    #[doc = "3: PMM Core Voltage 3 (1.85V)"]
    PMMCOREV_3 = 3,
}
impl From<PMMCOREV_A> for u8 {
    #[inline(always)]
    fn from(variant: PMMCOREV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PMMCOREV` reader - PMM Core Voltage Bit: 0"]
pub struct PMMCOREV_R(crate::FieldReader<u8, PMMCOREV_A>);
impl PMMCOREV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PMMCOREV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMMCOREV_A {
        match self.bits {
            0 => PMMCOREV_A::PMMCOREV_0,
            1 => PMMCOREV_A::PMMCOREV_1,
            2 => PMMCOREV_A::PMMCOREV_2,
            3 => PMMCOREV_A::PMMCOREV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PMMCOREV_0`"]
    #[inline(always)]
    pub fn is_pmmcorev_0(&self) -> bool {
        **self == PMMCOREV_A::PMMCOREV_0
    }
    #[doc = "Checks if the value of the field is `PMMCOREV_1`"]
    #[inline(always)]
    pub fn is_pmmcorev_1(&self) -> bool {
        **self == PMMCOREV_A::PMMCOREV_1
    }
    #[doc = "Checks if the value of the field is `PMMCOREV_2`"]
    #[inline(always)]
    pub fn is_pmmcorev_2(&self) -> bool {
        **self == PMMCOREV_A::PMMCOREV_2
    }
    #[doc = "Checks if the value of the field is `PMMCOREV_3`"]
    #[inline(always)]
    pub fn is_pmmcorev_3(&self) -> bool {
        **self == PMMCOREV_A::PMMCOREV_3
    }
}
impl core::ops::Deref for PMMCOREV_R {
    type Target = crate::FieldReader<u8, PMMCOREV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMMCOREV` writer - PMM Core Voltage Bit: 0"]
pub struct PMMCOREV_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMCOREV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMMCOREV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PMM Core Voltage 0 (1.35V)"]
    #[inline(always)]
    pub fn pmmcorev_0(self) -> &'a mut W {
        self.variant(PMMCOREV_A::PMMCOREV_0)
    }
    #[doc = "PMM Core Voltage 1 (1.55V)"]
    #[inline(always)]
    pub fn pmmcorev_1(self) -> &'a mut W {
        self.variant(PMMCOREV_A::PMMCOREV_1)
    }
    #[doc = "PMM Core Voltage 2 (1.75V)"]
    #[inline(always)]
    pub fn pmmcorev_2(self) -> &'a mut W {
        self.variant(PMMCOREV_A::PMMCOREV_2)
    }
    #[doc = "PMM Core Voltage 3 (1.85V)"]
    #[inline(always)]
    pub fn pmmcorev_3(self) -> &'a mut W {
        self.variant(PMMCOREV_A::PMMCOREV_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u16 & 0x03);
        self.w
    }
}
#[doc = "Field `PMMSWBOR` reader - PMM Software BOR"]
pub struct PMMSWBOR_R(crate::FieldReader<bool, bool>);
impl PMMSWBOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMMSWBOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMMSWBOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMMSWBOR` writer - PMM Software BOR"]
pub struct PMMSWBOR_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMSWBOR_W<'a> {
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
#[doc = "Field `PMMSWPOR` reader - PMM Software POR"]
pub struct PMMSWPOR_R(crate::FieldReader<bool, bool>);
impl PMMSWPOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMMSWPOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMMSWPOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMMSWPOR` writer - PMM Software POR"]
pub struct PMMSWPOR_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMSWPOR_W<'a> {
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
#[doc = "Field `PMMREGOFF` reader - PMM Turn Regulator off"]
pub struct PMMREGOFF_R(crate::FieldReader<bool, bool>);
impl PMMREGOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMMREGOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMMREGOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMMREGOFF` writer - PMM Turn Regulator off"]
pub struct PMMREGOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMREGOFF_W<'a> {
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
#[doc = "Field `PMMHPMRE` reader - PMM Global High Power Module Request Enable"]
pub struct PMMHPMRE_R(crate::FieldReader<bool, bool>);
impl PMMHPMRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMMHPMRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMMHPMRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMMHPMRE` writer - PMM Global High Power Module Request Enable"]
pub struct PMMHPMRE_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMHPMRE_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - PMM Core Voltage Bit: 0"]
    #[inline(always)]
    pub fn pmmcorev(&self) -> PMMCOREV_R {
        PMMCOREV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - PMM Software BOR"]
    #[inline(always)]
    pub fn pmmswbor(&self) -> PMMSWBOR_R {
        PMMSWBOR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PMM Software POR"]
    #[inline(always)]
    pub fn pmmswpor(&self) -> PMMSWPOR_R {
        PMMSWPOR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PMM Turn Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&self) -> PMMREGOFF_R {
        PMMREGOFF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PMM Global High Power Module Request Enable"]
    #[inline(always)]
    pub fn pmmhpmre(&self) -> PMMHPMRE_R {
        PMMHPMRE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PMM Core Voltage Bit: 0"]
    #[inline(always)]
    pub fn pmmcorev(&mut self) -> PMMCOREV_W {
        PMMCOREV_W { w: self }
    }
    #[doc = "Bit 2 - PMM Software BOR"]
    #[inline(always)]
    pub fn pmmswbor(&mut self) -> PMMSWBOR_W {
        PMMSWBOR_W { w: self }
    }
    #[doc = "Bit 3 - PMM Software POR"]
    #[inline(always)]
    pub fn pmmswpor(&mut self) -> PMMSWPOR_W {
        PMMSWPOR_W { w: self }
    }
    #[doc = "Bit 4 - PMM Turn Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&mut self) -> PMMREGOFF_W {
        PMMREGOFF_W { w: self }
    }
    #[doc = "Bit 7 - PMM Global High Power Module Request Enable"]
    #[inline(always)]
    pub fn pmmhpmre(&mut self) -> PMMHPMRE_W {
        PMMHPMRE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMM Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmctl0](index.html) module"]
pub struct PMMCTL0_SPEC;
impl crate::RegisterSpec for PMMCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pmmctl0::R](R) reader structure"]
impl crate::Readable for PMMCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmmctl0::W](W) writer structure"]
impl crate::Writable for PMMCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMMCTL0 to value 0"]
impl crate::Resettable for PMMCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
