#[doc = "Register `DMA1CTL` reader"]
pub struct R(crate::R<DMA1CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA1CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA1CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA1CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA1CTL` writer"]
pub struct W(crate::W<DMA1CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA1CTL_SPEC>;
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
impl From<crate::W<DMA1CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA1CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAREQ` reader - Initiate DMA transfer with DMATSEL"]
pub struct DMAREQ_R(crate::FieldReader<bool, bool>);
impl DMAREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAREQ` writer - Initiate DMA transfer with DMATSEL"]
pub struct DMAREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAREQ_W<'a> {
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
#[doc = "Field `DMAABORT` reader - DMA transfer aborted by NMI"]
pub struct DMAABORT_R(crate::FieldReader<bool, bool>);
impl DMAABORT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAABORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAABORT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAABORT` writer - DMA transfer aborted by NMI"]
pub struct DMAABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAABORT_W<'a> {
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
#[doc = "Field `DMAIE` reader - DMA interrupt enable"]
pub struct DMAIE_R(crate::FieldReader<bool, bool>);
impl DMAIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAIE` writer - DMA interrupt enable"]
pub struct DMAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAIE_W<'a> {
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
#[doc = "Field `DMAIFG` reader - DMA interrupt flag"]
pub struct DMAIFG_R(crate::FieldReader<bool, bool>);
impl DMAIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAIFG` writer - DMA interrupt flag"]
pub struct DMAIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAIFG_W<'a> {
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
#[doc = "Field `DMAEN` reader - DMA enable"]
pub struct DMAEN_R(crate::FieldReader<bool, bool>);
impl DMAEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEN` writer - DMA enable"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
#[doc = "Field `DMALEVEL` reader - DMA level sensitive trigger select"]
pub struct DMALEVEL_R(crate::FieldReader<bool, bool>);
impl DMALEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMALEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMALEVEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMALEVEL` writer - DMA level sensitive trigger select"]
pub struct DMALEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMALEVEL_W<'a> {
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
#[doc = "Field `DMASRCBYTE` reader - DMA source byte"]
pub struct DMASRCBYTE_R(crate::FieldReader<bool, bool>);
impl DMASRCBYTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMASRCBYTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMASRCBYTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMASRCBYTE` writer - DMA source byte"]
pub struct DMASRCBYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASRCBYTE_W<'a> {
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
#[doc = "Field `DMADSTBYTE` reader - DMA destination byte"]
pub struct DMADSTBYTE_R(crate::FieldReader<bool, bool>);
impl DMADSTBYTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMADSTBYTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMADSTBYTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMADSTBYTE` writer - DMA destination byte"]
pub struct DMADSTBYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADSTBYTE_W<'a> {
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
#[doc = "DMA source increment bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMASRCINCR_A {
    #[doc = "0: DMA source increment 0: source address unchanged"]
    DMASRCINCR_0 = 0,
    #[doc = "1: DMA source increment 1: source address unchanged"]
    DMASRCINCR_1 = 1,
    #[doc = "2: DMA source increment 2: source address decremented"]
    DMASRCINCR_2 = 2,
    #[doc = "3: DMA source increment 3: source address incremented"]
    DMASRCINCR_3 = 3,
}
impl From<DMASRCINCR_A> for u8 {
    #[inline(always)]
    fn from(variant: DMASRCINCR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DMASRCINCR` reader - DMA source increment bit 0"]
pub struct DMASRCINCR_R(crate::FieldReader<u8, DMASRCINCR_A>);
impl DMASRCINCR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMASRCINCR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMASRCINCR_A {
        match self.bits {
            0 => DMASRCINCR_A::DMASRCINCR_0,
            1 => DMASRCINCR_A::DMASRCINCR_1,
            2 => DMASRCINCR_A::DMASRCINCR_2,
            3 => DMASRCINCR_A::DMASRCINCR_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMASRCINCR_0`"]
    #[inline(always)]
    pub fn is_dmasrcincr_0(&self) -> bool {
        **self == DMASRCINCR_A::DMASRCINCR_0
    }
    #[doc = "Checks if the value of the field is `DMASRCINCR_1`"]
    #[inline(always)]
    pub fn is_dmasrcincr_1(&self) -> bool {
        **self == DMASRCINCR_A::DMASRCINCR_1
    }
    #[doc = "Checks if the value of the field is `DMASRCINCR_2`"]
    #[inline(always)]
    pub fn is_dmasrcincr_2(&self) -> bool {
        **self == DMASRCINCR_A::DMASRCINCR_2
    }
    #[doc = "Checks if the value of the field is `DMASRCINCR_3`"]
    #[inline(always)]
    pub fn is_dmasrcincr_3(&self) -> bool {
        **self == DMASRCINCR_A::DMASRCINCR_3
    }
}
impl core::ops::Deref for DMASRCINCR_R {
    type Target = crate::FieldReader<u8, DMASRCINCR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMASRCINCR` writer - DMA source increment bit 0"]
pub struct DMASRCINCR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASRCINCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMASRCINCR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA source increment 0: source address unchanged"]
    #[inline(always)]
    pub fn dmasrcincr_0(self) -> &'a mut W {
        self.variant(DMASRCINCR_A::DMASRCINCR_0)
    }
    #[doc = "DMA source increment 1: source address unchanged"]
    #[inline(always)]
    pub fn dmasrcincr_1(self) -> &'a mut W {
        self.variant(DMASRCINCR_A::DMASRCINCR_1)
    }
    #[doc = "DMA source increment 2: source address decremented"]
    #[inline(always)]
    pub fn dmasrcincr_2(self) -> &'a mut W {
        self.variant(DMASRCINCR_A::DMASRCINCR_2)
    }
    #[doc = "DMA source increment 3: source address incremented"]
    #[inline(always)]
    pub fn dmasrcincr_3(self) -> &'a mut W {
        self.variant(DMASRCINCR_A::DMASRCINCR_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u16 & 0x03) << 8);
        self.w
    }
}
#[doc = "DMA destination increment bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMADSTINCR_A {
    #[doc = "0: DMA destination increment 0: destination address unchanged"]
    DMADSTINCR_0 = 0,
    #[doc = "1: DMA destination increment 1: destination address unchanged"]
    DMADSTINCR_1 = 1,
    #[doc = "2: DMA destination increment 2: destination address decremented"]
    DMADSTINCR_2 = 2,
    #[doc = "3: DMA destination increment 3: destination address incremented"]
    DMADSTINCR_3 = 3,
}
impl From<DMADSTINCR_A> for u8 {
    #[inline(always)]
    fn from(variant: DMADSTINCR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DMADSTINCR` reader - DMA destination increment bit 0"]
pub struct DMADSTINCR_R(crate::FieldReader<u8, DMADSTINCR_A>);
impl DMADSTINCR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMADSTINCR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMADSTINCR_A {
        match self.bits {
            0 => DMADSTINCR_A::DMADSTINCR_0,
            1 => DMADSTINCR_A::DMADSTINCR_1,
            2 => DMADSTINCR_A::DMADSTINCR_2,
            3 => DMADSTINCR_A::DMADSTINCR_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMADSTINCR_0`"]
    #[inline(always)]
    pub fn is_dmadstincr_0(&self) -> bool {
        **self == DMADSTINCR_A::DMADSTINCR_0
    }
    #[doc = "Checks if the value of the field is `DMADSTINCR_1`"]
    #[inline(always)]
    pub fn is_dmadstincr_1(&self) -> bool {
        **self == DMADSTINCR_A::DMADSTINCR_1
    }
    #[doc = "Checks if the value of the field is `DMADSTINCR_2`"]
    #[inline(always)]
    pub fn is_dmadstincr_2(&self) -> bool {
        **self == DMADSTINCR_A::DMADSTINCR_2
    }
    #[doc = "Checks if the value of the field is `DMADSTINCR_3`"]
    #[inline(always)]
    pub fn is_dmadstincr_3(&self) -> bool {
        **self == DMADSTINCR_A::DMADSTINCR_3
    }
}
impl core::ops::Deref for DMADSTINCR_R {
    type Target = crate::FieldReader<u8, DMADSTINCR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMADSTINCR` writer - DMA destination increment bit 0"]
pub struct DMADSTINCR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADSTINCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMADSTINCR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA destination increment 0: destination address unchanged"]
    #[inline(always)]
    pub fn dmadstincr_0(self) -> &'a mut W {
        self.variant(DMADSTINCR_A::DMADSTINCR_0)
    }
    #[doc = "DMA destination increment 1: destination address unchanged"]
    #[inline(always)]
    pub fn dmadstincr_1(self) -> &'a mut W {
        self.variant(DMADSTINCR_A::DMADSTINCR_1)
    }
    #[doc = "DMA destination increment 2: destination address decremented"]
    #[inline(always)]
    pub fn dmadstincr_2(self) -> &'a mut W {
        self.variant(DMADSTINCR_A::DMADSTINCR_2)
    }
    #[doc = "DMA destination increment 3: destination address incremented"]
    #[inline(always)]
    pub fn dmadstincr_3(self) -> &'a mut W {
        self.variant(DMADSTINCR_A::DMADSTINCR_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u16 & 0x03) << 10);
        self.w
    }
}
#[doc = "DMA transfer mode bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMADT_A {
    #[doc = "0: DMA transfer mode 0: Single transfer"]
    DMADT_0 = 0,
    #[doc = "1: DMA transfer mode 1: Block transfer"]
    DMADT_1 = 1,
    #[doc = "2: DMA transfer mode 2: Burst-Block transfer"]
    DMADT_2 = 2,
    #[doc = "3: DMA transfer mode 3: Burst-Block transfer"]
    DMADT_3 = 3,
    #[doc = "4: DMA transfer mode 4: Repeated Single transfer"]
    DMADT_4 = 4,
    #[doc = "5: DMA transfer mode 5: Repeated Block transfer"]
    DMADT_5 = 5,
    #[doc = "6: DMA transfer mode 6: Repeated Burst-Block transfer"]
    DMADT_6 = 6,
    #[doc = "7: DMA transfer mode 7: Repeated Burst-Block transfer"]
    DMADT_7 = 7,
}
impl From<DMADT_A> for u8 {
    #[inline(always)]
    fn from(variant: DMADT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DMADT` reader - DMA transfer mode bit 0"]
pub struct DMADT_R(crate::FieldReader<u8, DMADT_A>);
impl DMADT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMADT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMADT_A {
        match self.bits {
            0 => DMADT_A::DMADT_0,
            1 => DMADT_A::DMADT_1,
            2 => DMADT_A::DMADT_2,
            3 => DMADT_A::DMADT_3,
            4 => DMADT_A::DMADT_4,
            5 => DMADT_A::DMADT_5,
            6 => DMADT_A::DMADT_6,
            7 => DMADT_A::DMADT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMADT_0`"]
    #[inline(always)]
    pub fn is_dmadt_0(&self) -> bool {
        **self == DMADT_A::DMADT_0
    }
    #[doc = "Checks if the value of the field is `DMADT_1`"]
    #[inline(always)]
    pub fn is_dmadt_1(&self) -> bool {
        **self == DMADT_A::DMADT_1
    }
    #[doc = "Checks if the value of the field is `DMADT_2`"]
    #[inline(always)]
    pub fn is_dmadt_2(&self) -> bool {
        **self == DMADT_A::DMADT_2
    }
    #[doc = "Checks if the value of the field is `DMADT_3`"]
    #[inline(always)]
    pub fn is_dmadt_3(&self) -> bool {
        **self == DMADT_A::DMADT_3
    }
    #[doc = "Checks if the value of the field is `DMADT_4`"]
    #[inline(always)]
    pub fn is_dmadt_4(&self) -> bool {
        **self == DMADT_A::DMADT_4
    }
    #[doc = "Checks if the value of the field is `DMADT_5`"]
    #[inline(always)]
    pub fn is_dmadt_5(&self) -> bool {
        **self == DMADT_A::DMADT_5
    }
    #[doc = "Checks if the value of the field is `DMADT_6`"]
    #[inline(always)]
    pub fn is_dmadt_6(&self) -> bool {
        **self == DMADT_A::DMADT_6
    }
    #[doc = "Checks if the value of the field is `DMADT_7`"]
    #[inline(always)]
    pub fn is_dmadt_7(&self) -> bool {
        **self == DMADT_A::DMADT_7
    }
}
impl core::ops::Deref for DMADT_R {
    type Target = crate::FieldReader<u8, DMADT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMADT` writer - DMA transfer mode bit 0"]
pub struct DMADT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMADT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA transfer mode 0: Single transfer"]
    #[inline(always)]
    pub fn dmadt_0(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_0)
    }
    #[doc = "DMA transfer mode 1: Block transfer"]
    #[inline(always)]
    pub fn dmadt_1(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_1)
    }
    #[doc = "DMA transfer mode 2: Burst-Block transfer"]
    #[inline(always)]
    pub fn dmadt_2(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_2)
    }
    #[doc = "DMA transfer mode 3: Burst-Block transfer"]
    #[inline(always)]
    pub fn dmadt_3(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_3)
    }
    #[doc = "DMA transfer mode 4: Repeated Single transfer"]
    #[inline(always)]
    pub fn dmadt_4(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_4)
    }
    #[doc = "DMA transfer mode 5: Repeated Block transfer"]
    #[inline(always)]
    pub fn dmadt_5(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_5)
    }
    #[doc = "DMA transfer mode 6: Repeated Burst-Block transfer"]
    #[inline(always)]
    pub fn dmadt_6(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_6)
    }
    #[doc = "DMA transfer mode 7: Repeated Burst-Block transfer"]
    #[inline(always)]
    pub fn dmadt_7(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u16 & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Initiate DMA transfer with DMATSEL"]
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA transfer aborted by NMI"]
    #[inline(always)]
    pub fn dmaabort(&self) -> DMAABORT_R {
        DMAABORT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA interrupt enable"]
    #[inline(always)]
    pub fn dmaie(&self) -> DMAIE_R {
        DMAIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA interrupt flag"]
    #[inline(always)]
    pub fn dmaifg(&self) -> DMAIFG_R {
        DMAIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DMA level sensitive trigger select"]
    #[inline(always)]
    pub fn dmalevel(&self) -> DMALEVEL_R {
        DMALEVEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DMA source byte"]
    #[inline(always)]
    pub fn dmasrcbyte(&self) -> DMASRCBYTE_R {
        DMASRCBYTE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DMA destination byte"]
    #[inline(always)]
    pub fn dmadstbyte(&self) -> DMADSTBYTE_R {
        DMADSTBYTE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - DMA source increment bit 0"]
    #[inline(always)]
    pub fn dmasrcincr(&self) -> DMASRCINCR_R {
        DMASRCINCR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - DMA destination increment bit 0"]
    #[inline(always)]
    pub fn dmadstincr(&self) -> DMADSTINCR_R {
        DMADSTINCR_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:14 - DMA transfer mode bit 0"]
    #[inline(always)]
    pub fn dmadt(&self) -> DMADT_R {
        DMADT_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Initiate DMA transfer with DMATSEL"]
    #[inline(always)]
    pub fn dmareq(&mut self) -> DMAREQ_W {
        DMAREQ_W { w: self }
    }
    #[doc = "Bit 1 - DMA transfer aborted by NMI"]
    #[inline(always)]
    pub fn dmaabort(&mut self) -> DMAABORT_W {
        DMAABORT_W { w: self }
    }
    #[doc = "Bit 2 - DMA interrupt enable"]
    #[inline(always)]
    pub fn dmaie(&mut self) -> DMAIE_W {
        DMAIE_W { w: self }
    }
    #[doc = "Bit 3 - DMA interrupt flag"]
    #[inline(always)]
    pub fn dmaifg(&mut self) -> DMAIFG_W {
        DMAIFG_W { w: self }
    }
    #[doc = "Bit 4 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 5 - DMA level sensitive trigger select"]
    #[inline(always)]
    pub fn dmalevel(&mut self) -> DMALEVEL_W {
        DMALEVEL_W { w: self }
    }
    #[doc = "Bit 6 - DMA source byte"]
    #[inline(always)]
    pub fn dmasrcbyte(&mut self) -> DMASRCBYTE_W {
        DMASRCBYTE_W { w: self }
    }
    #[doc = "Bit 7 - DMA destination byte"]
    #[inline(always)]
    pub fn dmadstbyte(&mut self) -> DMADSTBYTE_W {
        DMADSTBYTE_W { w: self }
    }
    #[doc = "Bits 8:9 - DMA source increment bit 0"]
    #[inline(always)]
    pub fn dmasrcincr(&mut self) -> DMASRCINCR_W {
        DMASRCINCR_W { w: self }
    }
    #[doc = "Bits 10:11 - DMA destination increment bit 0"]
    #[inline(always)]
    pub fn dmadstincr(&mut self) -> DMADSTINCR_W {
        DMADSTINCR_W { w: self }
    }
    #[doc = "Bits 12:14 - DMA transfer mode bit 0"]
    #[inline(always)]
    pub fn dmadt(&mut self) -> DMADT_W {
        DMADT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel 1 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1ctl](index.html) module"]
pub struct DMA1CTL_SPEC;
impl crate::RegisterSpec for DMA1CTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dma1ctl::R](R) reader structure"]
impl crate::Readable for DMA1CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma1ctl::W](W) writer structure"]
impl crate::Writable for DMA1CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA1CTL to value 0"]
impl crate::Resettable for DMA1CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
