#[doc = "Register `REFCTL0` reader"]
pub struct R(crate::R<REFCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REFCTL0` writer"]
pub struct W(crate::W<REFCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFCTL0_SPEC>;
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
impl From<crate::W<REFCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFON` reader - REF Reference On"]
pub struct REFON_R(crate::FieldReader<bool, bool>);
impl REFON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REFON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFON` writer - REF Reference On"]
pub struct REFON_W<'a> {
    w: &'a mut W,
}
impl<'a> REFON_W<'a> {
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
#[doc = "Field `REFOUT` reader - REF Reference output Buffer On"]
pub struct REFOUT_R(crate::FieldReader<bool, bool>);
impl REFOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REFOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFOUT` writer - REF Reference output Buffer On"]
pub struct REFOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> REFOUT_W<'a> {
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
#[doc = "Field `REFTCOFF` reader - REF Temp.Sensor off"]
pub struct REFTCOFF_R(crate::FieldReader<bool, bool>);
impl REFTCOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REFTCOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFTCOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFTCOFF` writer - REF Temp.Sensor off"]
pub struct REFTCOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> REFTCOFF_W<'a> {
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
#[doc = "REF Reference Voltage Level Select Bit:0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFVSEL_A {
    #[doc = "0: REF Reference Voltage Level Select 1.5V"]
    REFVSEL_0 = 0,
    #[doc = "1: REF Reference Voltage Level Select 2.0V"]
    REFVSEL_1 = 1,
    #[doc = "2: REF Reference Voltage Level Select 2.5V"]
    REFVSEL_2 = 2,
    #[doc = "3: REF Reference Voltage Level Select 2.5V"]
    REFVSEL_3 = 3,
}
impl From<REFVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFVSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REFVSEL` reader - REF Reference Voltage Level Select Bit:0"]
pub struct REFVSEL_R(crate::FieldReader<u8, REFVSEL_A>);
impl REFVSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REFVSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFVSEL_A {
        match self.bits {
            0 => REFVSEL_A::REFVSEL_0,
            1 => REFVSEL_A::REFVSEL_1,
            2 => REFVSEL_A::REFVSEL_2,
            3 => REFVSEL_A::REFVSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REFVSEL_0`"]
    #[inline(always)]
    pub fn is_refvsel_0(&self) -> bool {
        **self == REFVSEL_A::REFVSEL_0
    }
    #[doc = "Checks if the value of the field is `REFVSEL_1`"]
    #[inline(always)]
    pub fn is_refvsel_1(&self) -> bool {
        **self == REFVSEL_A::REFVSEL_1
    }
    #[doc = "Checks if the value of the field is `REFVSEL_2`"]
    #[inline(always)]
    pub fn is_refvsel_2(&self) -> bool {
        **self == REFVSEL_A::REFVSEL_2
    }
    #[doc = "Checks if the value of the field is `REFVSEL_3`"]
    #[inline(always)]
    pub fn is_refvsel_3(&self) -> bool {
        **self == REFVSEL_A::REFVSEL_3
    }
}
impl core::ops::Deref for REFVSEL_R {
    type Target = crate::FieldReader<u8, REFVSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFVSEL` writer - REF Reference Voltage Level Select Bit:0"]
pub struct REFVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFVSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFVSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "REF Reference Voltage Level Select 1.5V"]
    #[inline(always)]
    pub fn refvsel_0(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_0)
    }
    #[doc = "REF Reference Voltage Level Select 2.0V"]
    #[inline(always)]
    pub fn refvsel_1(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_1)
    }
    #[doc = "REF Reference Voltage Level Select 2.5V"]
    #[inline(always)]
    pub fn refvsel_2(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_2)
    }
    #[doc = "REF Reference Voltage Level Select 2.5V"]
    #[inline(always)]
    pub fn refvsel_3(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u16 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `REFMSTR` reader - REF Master Control"]
pub struct REFMSTR_R(crate::FieldReader<bool, bool>);
impl REFMSTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REFMSTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFMSTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFMSTR` writer - REF Master Control"]
pub struct REFMSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> REFMSTR_W<'a> {
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
#[doc = "Field `REFGENACT` reader - REF Reference generator active"]
pub struct REFGENACT_R(crate::FieldReader<bool, bool>);
impl REFGENACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REFGENACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFGENACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFGENACT` writer - REF Reference generator active"]
pub struct REFGENACT_W<'a> {
    w: &'a mut W,
}
impl<'a> REFGENACT_W<'a> {
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
#[doc = "Field `REFBGACT` reader - REF Reference bandgap active"]
pub struct REFBGACT_R(crate::FieldReader<bool, bool>);
impl REFBGACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REFBGACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFBGACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFBGACT` writer - REF Reference bandgap active"]
pub struct REFBGACT_W<'a> {
    w: &'a mut W,
}
impl<'a> REFBGACT_W<'a> {
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
#[doc = "Field `REFGENBUSY` reader - REF Reference generator busy"]
pub struct REFGENBUSY_R(crate::FieldReader<bool, bool>);
impl REFGENBUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REFGENBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFGENBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFGENBUSY` writer - REF Reference generator busy"]
pub struct REFGENBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> REFGENBUSY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `BGMODE` reader - REF Bandgap mode"]
pub struct BGMODE_R(crate::FieldReader<bool, bool>);
impl BGMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BGMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BGMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGMODE` writer - REF Bandgap mode"]
pub struct BGMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BGMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u16 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - REF Reference On"]
    #[inline(always)]
    pub fn refon(&self) -> REFON_R {
        REFON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - REF Reference output Buffer On"]
    #[inline(always)]
    pub fn refout(&self) -> REFOUT_R {
        REFOUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - REF Temp.Sensor off"]
    #[inline(always)]
    pub fn reftcoff(&self) -> REFTCOFF_R {
        REFTCOFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - REF Reference Voltage Level Select Bit:0"]
    #[inline(always)]
    pub fn refvsel(&self) -> REFVSEL_R {
        REFVSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - REF Master Control"]
    #[inline(always)]
    pub fn refmstr(&self) -> REFMSTR_R {
        REFMSTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - REF Reference generator active"]
    #[inline(always)]
    pub fn refgenact(&self) -> REFGENACT_R {
        REFGENACT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - REF Reference bandgap active"]
    #[inline(always)]
    pub fn refbgact(&self) -> REFBGACT_R {
        REFBGACT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - REF Reference generator busy"]
    #[inline(always)]
    pub fn refgenbusy(&self) -> REFGENBUSY_R {
        REFGENBUSY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - REF Bandgap mode"]
    #[inline(always)]
    pub fn bgmode(&self) -> BGMODE_R {
        BGMODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REF Reference On"]
    #[inline(always)]
    pub fn refon(&mut self) -> REFON_W {
        REFON_W { w: self }
    }
    #[doc = "Bit 1 - REF Reference output Buffer On"]
    #[inline(always)]
    pub fn refout(&mut self) -> REFOUT_W {
        REFOUT_W { w: self }
    }
    #[doc = "Bit 3 - REF Temp.Sensor off"]
    #[inline(always)]
    pub fn reftcoff(&mut self) -> REFTCOFF_W {
        REFTCOFF_W { w: self }
    }
    #[doc = "Bits 4:5 - REF Reference Voltage Level Select Bit:0"]
    #[inline(always)]
    pub fn refvsel(&mut self) -> REFVSEL_W {
        REFVSEL_W { w: self }
    }
    #[doc = "Bit 7 - REF Master Control"]
    #[inline(always)]
    pub fn refmstr(&mut self) -> REFMSTR_W {
        REFMSTR_W { w: self }
    }
    #[doc = "Bit 8 - REF Reference generator active"]
    #[inline(always)]
    pub fn refgenact(&mut self) -> REFGENACT_W {
        REFGENACT_W { w: self }
    }
    #[doc = "Bit 9 - REF Reference bandgap active"]
    #[inline(always)]
    pub fn refbgact(&mut self) -> REFBGACT_W {
        REFBGACT_W { w: self }
    }
    #[doc = "Bit 10 - REF Reference generator busy"]
    #[inline(always)]
    pub fn refgenbusy(&mut self) -> REFGENBUSY_W {
        REFGENBUSY_W { w: self }
    }
    #[doc = "Bit 11 - REF Bandgap mode"]
    #[inline(always)]
    pub fn bgmode(&mut self) -> BGMODE_W {
        BGMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "REF Shared Reference control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refctl0](index.html) module"]
pub struct REFCTL0_SPEC;
impl crate::RegisterSpec for REFCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [refctl0::R](R) reader structure"]
impl crate::Readable for REFCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [refctl0::W](W) writer structure"]
impl crate::Writable for REFCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REFCTL0 to value 0"]
impl crate::Resettable for REFCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
