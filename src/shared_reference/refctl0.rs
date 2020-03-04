#[doc = "Reader of register REFCTL0"]
pub type R = crate::R<u16, super::REFCTL0>;
#[doc = "Writer for register REFCTL0"]
pub type W = crate::W<u16, super::REFCTL0>;
#[doc = "Register REFCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::REFCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REFON`"]
pub type REFON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFON`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `REFOUT`"]
pub type REFOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFOUT`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `REFTCOFF`"]
pub type REFTCOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFTCOFF`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
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
#[doc = "Reader of field `REFVSEL`"]
pub type REFVSEL_R = crate::R<u8, REFVSEL_A>;
impl REFVSEL_R {
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
        *self == REFVSEL_A::REFVSEL_0
    }
    #[doc = "Checks if the value of the field is `REFVSEL_1`"]
    #[inline(always)]
    pub fn is_refvsel_1(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_1
    }
    #[doc = "Checks if the value of the field is `REFVSEL_2`"]
    #[inline(always)]
    pub fn is_refvsel_2(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_2
    }
    #[doc = "Checks if the value of the field is `REFVSEL_3`"]
    #[inline(always)]
    pub fn is_refvsel_3(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_3
    }
}
#[doc = "Write proxy for field `REFVSEL`"]
pub struct REFVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFVSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFVSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `REFMSTR`"]
pub type REFMSTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFMSTR`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `REFGENACT`"]
pub type REFGENACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFGENACT`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `REFBGACT`"]
pub type REFBGACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFBGACT`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `REFGENBUSY`"]
pub type REFGENBUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFGENBUSY`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `BGMODE`"]
pub type BGMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BGMODE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
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
}
