#[doc = "Register `UCSCTL2` reader"]
pub struct R(crate::R<UCSCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSCTL2` writer"]
pub struct W(crate::W<UCSCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSCTL2_SPEC>;
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
impl From<crate::W<UCSCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLLN0` reader - FLL Multipier Bit : 0"]
pub struct FLLN0_R(crate::FieldReader<bool, bool>);
impl FLLN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLLN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLLN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLLN0` writer - FLL Multipier Bit : 0"]
pub struct FLLN0_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLN0_W<'a> {
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
#[doc = "Field `FLLN1` reader - FLL Multipier Bit : 1"]
pub struct FLLN1_R(crate::FieldReader<bool, bool>);
impl FLLN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLLN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLLN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLLN1` writer - FLL Multipier Bit : 1"]
pub struct FLLN1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLN1_W<'a> {
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
#[doc = "Field `FLLN2` reader - FLL Multipier Bit : 2"]
pub struct FLLN2_R(crate::FieldReader<bool, bool>);
impl FLLN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLLN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLLN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLLN2` writer - FLL Multipier Bit : 2"]
pub struct FLLN2_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLN2_W<'a> {
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
#[doc = "Field `FLLN3` reader - FLL Multipier Bit : 3"]
pub struct FLLN3_R(crate::FieldReader<bool, bool>);
impl FLLN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLLN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLLN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLLN3` writer - FLL Multipier Bit : 3"]
pub struct FLLN3_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLN3_W<'a> {
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
#[doc = "Field `FLLN4` reader - FLL Multipier Bit : 4"]
pub struct FLLN4_R(crate::FieldReader<bool, bool>);
impl FLLN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLLN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLLN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLLN4` writer - FLL Multipier Bit : 4"]
pub struct FLLN4_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLN4_W<'a> {
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
#[doc = "Field `FLLN5` reader - FLL Multipier Bit : 5"]
pub struct FLLN5_R(crate::FieldReader<bool, bool>);
impl FLLN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLLN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLLN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLLN5` writer - FLL Multipier Bit : 5"]
pub struct FLLN5_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLN5_W<'a> {
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
#[doc = "Field `FLLN6` reader - FLL Multipier Bit : 6"]
pub struct FLLN6_R(crate::FieldReader<bool, bool>);
impl FLLN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLLN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLLN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLLN6` writer - FLL Multipier Bit : 6"]
pub struct FLLN6_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLN6_W<'a> {
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
#[doc = "Field `FLLN7` reader - FLL Multipier Bit : 7"]
pub struct FLLN7_R(crate::FieldReader<bool, bool>);
impl FLLN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLLN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLLN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLLN7` writer - FLL Multipier Bit : 7"]
pub struct FLLN7_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLN7_W<'a> {
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
#[doc = "Field `FLLN8` reader - FLL Multipier Bit : 8"]
pub struct FLLN8_R(crate::FieldReader<bool, bool>);
impl FLLN8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLLN8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLLN8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLLN8` writer - FLL Multipier Bit : 8"]
pub struct FLLN8_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLN8_W<'a> {
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
#[doc = "Field `FLLN9` reader - FLL Multipier Bit : 9"]
pub struct FLLN9_R(crate::FieldReader<bool, bool>);
impl FLLN9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLLN9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLLN9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLLN9` writer - FLL Multipier Bit : 9"]
pub struct FLLN9_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLN9_W<'a> {
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
#[doc = "Loop Divider Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLLD_A {
    #[doc = "0: Multiply Selected Loop Freq. 1"]
    FLLD_0 = 0,
    #[doc = "1: Multiply Selected Loop Freq. 2"]
    FLLD_1 = 1,
    #[doc = "2: Multiply Selected Loop Freq. 4"]
    FLLD_2 = 2,
    #[doc = "3: Multiply Selected Loop Freq. 8"]
    FLLD_3 = 3,
    #[doc = "4: Multiply Selected Loop Freq. 16"]
    FLLD_4 = 4,
    #[doc = "5: Multiply Selected Loop Freq. 32"]
    FLLD_5 = 5,
    #[doc = "6: Multiply Selected Loop Freq. 32"]
    FLLD_6 = 6,
    #[doc = "7: Multiply Selected Loop Freq. 32"]
    FLLD_7 = 7,
}
impl From<FLLD_A> for u8 {
    #[inline(always)]
    fn from(variant: FLLD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLLD` reader - Loop Divider Bit : 0"]
pub struct FLLD_R(crate::FieldReader<u8, FLLD_A>);
impl FLLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLLD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLLD_A {
        match self.bits {
            0 => FLLD_A::FLLD_0,
            1 => FLLD_A::FLLD_1,
            2 => FLLD_A::FLLD_2,
            3 => FLLD_A::FLLD_3,
            4 => FLLD_A::FLLD_4,
            5 => FLLD_A::FLLD_5,
            6 => FLLD_A::FLLD_6,
            7 => FLLD_A::FLLD_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLLD_0`"]
    #[inline(always)]
    pub fn is_flld_0(&self) -> bool {
        **self == FLLD_A::FLLD_0
    }
    #[doc = "Checks if the value of the field is `FLLD_1`"]
    #[inline(always)]
    pub fn is_flld_1(&self) -> bool {
        **self == FLLD_A::FLLD_1
    }
    #[doc = "Checks if the value of the field is `FLLD_2`"]
    #[inline(always)]
    pub fn is_flld_2(&self) -> bool {
        **self == FLLD_A::FLLD_2
    }
    #[doc = "Checks if the value of the field is `FLLD_3`"]
    #[inline(always)]
    pub fn is_flld_3(&self) -> bool {
        **self == FLLD_A::FLLD_3
    }
    #[doc = "Checks if the value of the field is `FLLD_4`"]
    #[inline(always)]
    pub fn is_flld_4(&self) -> bool {
        **self == FLLD_A::FLLD_4
    }
    #[doc = "Checks if the value of the field is `FLLD_5`"]
    #[inline(always)]
    pub fn is_flld_5(&self) -> bool {
        **self == FLLD_A::FLLD_5
    }
    #[doc = "Checks if the value of the field is `FLLD_6`"]
    #[inline(always)]
    pub fn is_flld_6(&self) -> bool {
        **self == FLLD_A::FLLD_6
    }
    #[doc = "Checks if the value of the field is `FLLD_7`"]
    #[inline(always)]
    pub fn is_flld_7(&self) -> bool {
        **self == FLLD_A::FLLD_7
    }
}
impl core::ops::Deref for FLLD_R {
    type Target = crate::FieldReader<u8, FLLD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLLD` writer - Loop Divider Bit : 0"]
pub struct FLLD_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLLD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Multiply Selected Loop Freq. 1"]
    #[inline(always)]
    pub fn flld_0(self) -> &'a mut W {
        self.variant(FLLD_A::FLLD_0)
    }
    #[doc = "Multiply Selected Loop Freq. 2"]
    #[inline(always)]
    pub fn flld_1(self) -> &'a mut W {
        self.variant(FLLD_A::FLLD_1)
    }
    #[doc = "Multiply Selected Loop Freq. 4"]
    #[inline(always)]
    pub fn flld_2(self) -> &'a mut W {
        self.variant(FLLD_A::FLLD_2)
    }
    #[doc = "Multiply Selected Loop Freq. 8"]
    #[inline(always)]
    pub fn flld_3(self) -> &'a mut W {
        self.variant(FLLD_A::FLLD_3)
    }
    #[doc = "Multiply Selected Loop Freq. 16"]
    #[inline(always)]
    pub fn flld_4(self) -> &'a mut W {
        self.variant(FLLD_A::FLLD_4)
    }
    #[doc = "Multiply Selected Loop Freq. 32"]
    #[inline(always)]
    pub fn flld_5(self) -> &'a mut W {
        self.variant(FLLD_A::FLLD_5)
    }
    #[doc = "Multiply Selected Loop Freq. 32"]
    #[inline(always)]
    pub fn flld_6(self) -> &'a mut W {
        self.variant(FLLD_A::FLLD_6)
    }
    #[doc = "Multiply Selected Loop Freq. 32"]
    #[inline(always)]
    pub fn flld_7(self) -> &'a mut W {
        self.variant(FLLD_A::FLLD_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u16 & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FLL Multipier Bit : 0"]
    #[inline(always)]
    pub fn flln0(&self) -> FLLN0_R {
        FLLN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FLL Multipier Bit : 1"]
    #[inline(always)]
    pub fn flln1(&self) -> FLLN1_R {
        FLLN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FLL Multipier Bit : 2"]
    #[inline(always)]
    pub fn flln2(&self) -> FLLN2_R {
        FLLN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FLL Multipier Bit : 3"]
    #[inline(always)]
    pub fn flln3(&self) -> FLLN3_R {
        FLLN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FLL Multipier Bit : 4"]
    #[inline(always)]
    pub fn flln4(&self) -> FLLN4_R {
        FLLN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FLL Multipier Bit : 5"]
    #[inline(always)]
    pub fn flln5(&self) -> FLLN5_R {
        FLLN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FLL Multipier Bit : 6"]
    #[inline(always)]
    pub fn flln6(&self) -> FLLN6_R {
        FLLN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - FLL Multipier Bit : 7"]
    #[inline(always)]
    pub fn flln7(&self) -> FLLN7_R {
        FLLN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FLL Multipier Bit : 8"]
    #[inline(always)]
    pub fn flln8(&self) -> FLLN8_R {
        FLLN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FLL Multipier Bit : 9"]
    #[inline(always)]
    pub fn flln9(&self) -> FLLN9_R {
        FLLN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Loop Divider Bit : 0"]
    #[inline(always)]
    pub fn flld(&self) -> FLLD_R {
        FLLD_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FLL Multipier Bit : 0"]
    #[inline(always)]
    pub fn flln0(&mut self) -> FLLN0_W {
        FLLN0_W { w: self }
    }
    #[doc = "Bit 1 - FLL Multipier Bit : 1"]
    #[inline(always)]
    pub fn flln1(&mut self) -> FLLN1_W {
        FLLN1_W { w: self }
    }
    #[doc = "Bit 2 - FLL Multipier Bit : 2"]
    #[inline(always)]
    pub fn flln2(&mut self) -> FLLN2_W {
        FLLN2_W { w: self }
    }
    #[doc = "Bit 3 - FLL Multipier Bit : 3"]
    #[inline(always)]
    pub fn flln3(&mut self) -> FLLN3_W {
        FLLN3_W { w: self }
    }
    #[doc = "Bit 4 - FLL Multipier Bit : 4"]
    #[inline(always)]
    pub fn flln4(&mut self) -> FLLN4_W {
        FLLN4_W { w: self }
    }
    #[doc = "Bit 5 - FLL Multipier Bit : 5"]
    #[inline(always)]
    pub fn flln5(&mut self) -> FLLN5_W {
        FLLN5_W { w: self }
    }
    #[doc = "Bit 6 - FLL Multipier Bit : 6"]
    #[inline(always)]
    pub fn flln6(&mut self) -> FLLN6_W {
        FLLN6_W { w: self }
    }
    #[doc = "Bit 7 - FLL Multipier Bit : 7"]
    #[inline(always)]
    pub fn flln7(&mut self) -> FLLN7_W {
        FLLN7_W { w: self }
    }
    #[doc = "Bit 8 - FLL Multipier Bit : 8"]
    #[inline(always)]
    pub fn flln8(&mut self) -> FLLN8_W {
        FLLN8_W { w: self }
    }
    #[doc = "Bit 9 - FLL Multipier Bit : 9"]
    #[inline(always)]
    pub fn flln9(&mut self) -> FLLN9_W {
        FLLN9_W { w: self }
    }
    #[doc = "Bits 12:14 - Loop Divider Bit : 0"]
    #[inline(always)]
    pub fn flld(&mut self) -> FLLD_W {
        FLLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCS Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsctl2](index.html) module"]
pub struct UCSCTL2_SPEC;
impl crate::RegisterSpec for UCSCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucsctl2::R](R) reader structure"]
impl crate::Readable for UCSCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsctl2::W](W) writer structure"]
impl crate::Writable for UCSCTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCSCTL2 to value 0"]
impl crate::Resettable for UCSCTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
