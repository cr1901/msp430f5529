#[doc = "Reader of register USBMAINT"]
pub type R = crate::R<u16, super::USBMAINT>;
#[doc = "Writer for register USBMAINT"]
pub type W = crate::W<u16, super::USBMAINT>;
#[doc = "Register USBMAINT `reset()`'s with value 0"]
impl crate::ResetValue for super::USBMAINT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UTIFG`"]
pub type UTIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UTIFG`"]
pub struct UTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UTIFG_W<'a> {
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
#[doc = "Reader of field `UTIE`"]
pub type UTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UTIE`"]
pub struct UTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UTIE_W<'a> {
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
#[doc = "Reader of field `TSGEN`"]
pub type TSGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSGEN`"]
pub struct TSGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "USB - Time Stamp Event Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSESEL_A {
    #[doc = "0: USB - Time Stamp Event Select: 0"]
    TSESEL_0 = 0,
    #[doc = "1: USB - Time Stamp Event Select: 1"]
    TSESEL_1 = 1,
    #[doc = "2: USB - Time Stamp Event Select: 2"]
    TSESEL_2 = 2,
    #[doc = "3: USB - Time Stamp Event Select: 3"]
    TSESEL_3 = 3,
}
impl From<TSESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TSESEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TSESEL`"]
pub type TSESEL_R = crate::R<u8, TSESEL_A>;
impl TSESEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSESEL_A {
        match self.bits {
            0 => TSESEL_A::TSESEL_0,
            1 => TSESEL_A::TSESEL_1,
            2 => TSESEL_A::TSESEL_2,
            3 => TSESEL_A::TSESEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TSESEL_0`"]
    #[inline(always)]
    pub fn is_tsesel_0(&self) -> bool {
        *self == TSESEL_A::TSESEL_0
    }
    #[doc = "Checks if the value of the field is `TSESEL_1`"]
    #[inline(always)]
    pub fn is_tsesel_1(&self) -> bool {
        *self == TSESEL_A::TSESEL_1
    }
    #[doc = "Checks if the value of the field is `TSESEL_2`"]
    #[inline(always)]
    pub fn is_tsesel_2(&self) -> bool {
        *self == TSESEL_A::TSESEL_2
    }
    #[doc = "Checks if the value of the field is `TSESEL_3`"]
    #[inline(always)]
    pub fn is_tsesel_3(&self) -> bool {
        *self == TSESEL_A::TSESEL_3
    }
}
#[doc = "Write proxy for field `TSESEL`"]
pub struct TSESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSESEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "USB - Time Stamp Event Select: 0"]
    #[inline(always)]
    pub fn tsesel_0(self) -> &'a mut W {
        self.variant(TSESEL_A::TSESEL_0)
    }
    #[doc = "USB - Time Stamp Event Select: 1"]
    #[inline(always)]
    pub fn tsesel_1(self) -> &'a mut W {
        self.variant(TSESEL_A::TSESEL_1)
    }
    #[doc = "USB - Time Stamp Event Select: 2"]
    #[inline(always)]
    pub fn tsesel_2(self) -> &'a mut W {
        self.variant(TSESEL_A::TSESEL_2)
    }
    #[doc = "USB - Time Stamp Event Select: 3"]
    #[inline(always)]
    pub fn tsesel_3(self) -> &'a mut W {
        self.variant(TSESEL_A::TSESEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u16) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `TSE3`"]
pub type TSE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSE3`"]
pub struct TSE3_W<'a> {
    w: &'a mut W,
}
impl<'a> TSE3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "USB - Timer Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UTSEL_A {
    #[doc = "0: USB - Timer Select: 0"]
    UTSEL_0 = 0,
    #[doc = "1: USB - Timer Select: 1"]
    UTSEL_1 = 1,
    #[doc = "2: USB - Timer Select: 2"]
    UTSEL_2 = 2,
    #[doc = "3: USB - Timer Select: 3"]
    UTSEL_3 = 3,
    #[doc = "4: USB - Timer Select: 4"]
    UTSEL_4 = 4,
    #[doc = "5: USB - Timer Select: 5"]
    UTSEL_5 = 5,
    #[doc = "6: USB - Timer Select: 6"]
    UTSEL_6 = 6,
    #[doc = "7: USB - Timer Select: 7"]
    UTSEL_7 = 7,
}
impl From<UTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UTSEL`"]
pub type UTSEL_R = crate::R<u8, UTSEL_A>;
impl UTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTSEL_A {
        match self.bits {
            0 => UTSEL_A::UTSEL_0,
            1 => UTSEL_A::UTSEL_1,
            2 => UTSEL_A::UTSEL_2,
            3 => UTSEL_A::UTSEL_3,
            4 => UTSEL_A::UTSEL_4,
            5 => UTSEL_A::UTSEL_5,
            6 => UTSEL_A::UTSEL_6,
            7 => UTSEL_A::UTSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UTSEL_0`"]
    #[inline(always)]
    pub fn is_utsel_0(&self) -> bool {
        *self == UTSEL_A::UTSEL_0
    }
    #[doc = "Checks if the value of the field is `UTSEL_1`"]
    #[inline(always)]
    pub fn is_utsel_1(&self) -> bool {
        *self == UTSEL_A::UTSEL_1
    }
    #[doc = "Checks if the value of the field is `UTSEL_2`"]
    #[inline(always)]
    pub fn is_utsel_2(&self) -> bool {
        *self == UTSEL_A::UTSEL_2
    }
    #[doc = "Checks if the value of the field is `UTSEL_3`"]
    #[inline(always)]
    pub fn is_utsel_3(&self) -> bool {
        *self == UTSEL_A::UTSEL_3
    }
    #[doc = "Checks if the value of the field is `UTSEL_4`"]
    #[inline(always)]
    pub fn is_utsel_4(&self) -> bool {
        *self == UTSEL_A::UTSEL_4
    }
    #[doc = "Checks if the value of the field is `UTSEL_5`"]
    #[inline(always)]
    pub fn is_utsel_5(&self) -> bool {
        *self == UTSEL_A::UTSEL_5
    }
    #[doc = "Checks if the value of the field is `UTSEL_6`"]
    #[inline(always)]
    pub fn is_utsel_6(&self) -> bool {
        *self == UTSEL_A::UTSEL_6
    }
    #[doc = "Checks if the value of the field is `UTSEL_7`"]
    #[inline(always)]
    pub fn is_utsel_7(&self) -> bool {
        *self == UTSEL_A::UTSEL_7
    }
}
#[doc = "Write proxy for field `UTSEL`"]
pub struct UTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UTSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "USB - Timer Select: 0"]
    #[inline(always)]
    pub fn utsel_0(self) -> &'a mut W {
        self.variant(UTSEL_A::UTSEL_0)
    }
    #[doc = "USB - Timer Select: 1"]
    #[inline(always)]
    pub fn utsel_1(self) -> &'a mut W {
        self.variant(UTSEL_A::UTSEL_1)
    }
    #[doc = "USB - Timer Select: 2"]
    #[inline(always)]
    pub fn utsel_2(self) -> &'a mut W {
        self.variant(UTSEL_A::UTSEL_2)
    }
    #[doc = "USB - Timer Select: 3"]
    #[inline(always)]
    pub fn utsel_3(self) -> &'a mut W {
        self.variant(UTSEL_A::UTSEL_3)
    }
    #[doc = "USB - Timer Select: 4"]
    #[inline(always)]
    pub fn utsel_4(self) -> &'a mut W {
        self.variant(UTSEL_A::UTSEL_4)
    }
    #[doc = "USB - Timer Select: 5"]
    #[inline(always)]
    pub fn utsel_5(self) -> &'a mut W {
        self.variant(UTSEL_A::UTSEL_5)
    }
    #[doc = "USB - Timer Select: 6"]
    #[inline(always)]
    pub fn utsel_6(self) -> &'a mut W {
        self.variant(UTSEL_A::UTSEL_6)
    }
    #[doc = "USB - Timer Select: 7"]
    #[inline(always)]
    pub fn utsel_7(self) -> &'a mut W {
        self.variant(UTSEL_A::UTSEL_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u16) & 0x07) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB - Timer Interrupt Flag"]
    #[inline(always)]
    pub fn utifg(&self) -> UTIFG_R {
        UTIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn utie(&self) -> UTIE_R {
        UTIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USB - Time Stamp Generator Enable"]
    #[inline(always)]
    pub fn tsgen(&self) -> TSGEN_R {
        TSGEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - USB - Time Stamp Event Select Bit 0"]
    #[inline(always)]
    pub fn tsesel(&self) -> TSESEL_R {
        TSESEL_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - USB - Time Stamp Event #3 Bit"]
    #[inline(always)]
    pub fn tse3(&self) -> TSE3_R {
        TSE3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - USB - Timer Select Bit 0"]
    #[inline(always)]
    pub fn utsel(&self) -> UTSEL_R {
        UTSEL_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USB - Timer Interrupt Flag"]
    #[inline(always)]
    pub fn utifg(&mut self) -> UTIFG_W {
        UTIFG_W { w: self }
    }
    #[doc = "Bit 1 - USB - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn utie(&mut self) -> UTIE_W {
        UTIE_W { w: self }
    }
    #[doc = "Bit 8 - USB - Time Stamp Generator Enable"]
    #[inline(always)]
    pub fn tsgen(&mut self) -> TSGEN_W {
        TSGEN_W { w: self }
    }
    #[doc = "Bits 9:10 - USB - Time Stamp Event Select Bit 0"]
    #[inline(always)]
    pub fn tsesel(&mut self) -> TSESEL_W {
        TSESEL_W { w: self }
    }
    #[doc = "Bit 11 - USB - Time Stamp Event #3 Bit"]
    #[inline(always)]
    pub fn tse3(&mut self) -> TSE3_W {
        TSE3_W { w: self }
    }
    #[doc = "Bits 13:15 - USB - Timer Select Bit 0"]
    #[inline(always)]
    pub fn utsel(&mut self) -> UTSEL_W {
        UTSEL_W { w: self }
    }
}
