#[doc = "Reader of register DMA2CTL"]
pub type R = crate::R<u16, super::DMA2CTL>;
#[doc = "Writer for register DMA2CTL"]
pub type W = crate::W<u16, super::DMA2CTL>;
#[doc = "Register DMA2CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA2CTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMAREQ`"]
pub type DMAREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAREQ`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `DMAABORT`"]
pub type DMAABORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAABORT`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DMAIE`"]
pub type DMAIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAIE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DMAIFG`"]
pub type DMAIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAIFG`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAEN`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DMALEVEL`"]
pub type DMALEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMALEVEL`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DMASRCBYTE`"]
pub type DMASRCBYTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASRCBYTE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DMADSTBYTE`"]
pub type DMADSTBYTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMADSTBYTE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
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
#[doc = "Reader of field `DMASRCINCR`"]
pub type DMASRCINCR_R = crate::R<u8, DMASRCINCR_A>;
impl DMASRCINCR_R {
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
        *self == DMASRCINCR_A::DMASRCINCR_0
    }
    #[doc = "Checks if the value of the field is `DMASRCINCR_1`"]
    #[inline(always)]
    pub fn is_dmasrcincr_1(&self) -> bool {
        *self == DMASRCINCR_A::DMASRCINCR_1
    }
    #[doc = "Checks if the value of the field is `DMASRCINCR_2`"]
    #[inline(always)]
    pub fn is_dmasrcincr_2(&self) -> bool {
        *self == DMASRCINCR_A::DMASRCINCR_2
    }
    #[doc = "Checks if the value of the field is `DMASRCINCR_3`"]
    #[inline(always)]
    pub fn is_dmasrcincr_3(&self) -> bool {
        *self == DMASRCINCR_A::DMASRCINCR_3
    }
}
#[doc = "Write proxy for field `DMASRCINCR`"]
pub struct DMASRCINCR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASRCINCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMASRCINCR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
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
#[doc = "Reader of field `DMADSTINCR`"]
pub type DMADSTINCR_R = crate::R<u8, DMADSTINCR_A>;
impl DMADSTINCR_R {
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
        *self == DMADSTINCR_A::DMADSTINCR_0
    }
    #[doc = "Checks if the value of the field is `DMADSTINCR_1`"]
    #[inline(always)]
    pub fn is_dmadstincr_1(&self) -> bool {
        *self == DMADSTINCR_A::DMADSTINCR_1
    }
    #[doc = "Checks if the value of the field is `DMADSTINCR_2`"]
    #[inline(always)]
    pub fn is_dmadstincr_2(&self) -> bool {
        *self == DMADSTINCR_A::DMADSTINCR_2
    }
    #[doc = "Checks if the value of the field is `DMADSTINCR_3`"]
    #[inline(always)]
    pub fn is_dmadstincr_3(&self) -> bool {
        *self == DMADSTINCR_A::DMADSTINCR_3
    }
}
#[doc = "Write proxy for field `DMADSTINCR`"]
pub struct DMADSTINCR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADSTINCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMADSTINCR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u16) & 0x03) << 10);
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
#[doc = "Reader of field `DMADT`"]
pub type DMADT_R = crate::R<u8, DMADT_A>;
impl DMADT_R {
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
        *self == DMADT_A::DMADT_0
    }
    #[doc = "Checks if the value of the field is `DMADT_1`"]
    #[inline(always)]
    pub fn is_dmadt_1(&self) -> bool {
        *self == DMADT_A::DMADT_1
    }
    #[doc = "Checks if the value of the field is `DMADT_2`"]
    #[inline(always)]
    pub fn is_dmadt_2(&self) -> bool {
        *self == DMADT_A::DMADT_2
    }
    #[doc = "Checks if the value of the field is `DMADT_3`"]
    #[inline(always)]
    pub fn is_dmadt_3(&self) -> bool {
        *self == DMADT_A::DMADT_3
    }
    #[doc = "Checks if the value of the field is `DMADT_4`"]
    #[inline(always)]
    pub fn is_dmadt_4(&self) -> bool {
        *self == DMADT_A::DMADT_4
    }
    #[doc = "Checks if the value of the field is `DMADT_5`"]
    #[inline(always)]
    pub fn is_dmadt_5(&self) -> bool {
        *self == DMADT_A::DMADT_5
    }
    #[doc = "Checks if the value of the field is `DMADT_6`"]
    #[inline(always)]
    pub fn is_dmadt_6(&self) -> bool {
        *self == DMADT_A::DMADT_6
    }
    #[doc = "Checks if the value of the field is `DMADT_7`"]
    #[inline(always)]
    pub fn is_dmadt_7(&self) -> bool {
        *self == DMADT_A::DMADT_7
    }
}
#[doc = "Write proxy for field `DMADT`"]
pub struct DMADT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMADT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u16) & 0x07) << 12);
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
}
