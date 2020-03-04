#[doc = "Reader of register ADC12CTL2"]
pub type R = crate::R<u16, super::ADC12CTL2>;
#[doc = "Writer for register ADC12CTL2"]
pub type W = crate::W<u16, super::ADC12CTL2>;
#[doc = "Register ADC12CTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12CTL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC12REFBURST`"]
pub type ADC12REFBURST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12REFBURST`"]
pub struct ADC12REFBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12REFBURST_W<'a> {
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
#[doc = "Reader of field `ADC12REFOUT`"]
pub type ADC12REFOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12REFOUT`"]
pub struct ADC12REFOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12REFOUT_W<'a> {
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
#[doc = "Reader of field `ADC12SR`"]
pub type ADC12SR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12SR`"]
pub struct ADC12SR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SR_W<'a> {
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
#[doc = "Reader of field `ADC12DF`"]
pub type ADC12DF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12DF`"]
pub struct ADC12DF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12DF_W<'a> {
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
#[doc = "ADC12+ Resolution Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12RES_A {
    #[doc = "0: ADC12+ Resolution : 8 Bit"]
    ADC12RES_0 = 0,
    #[doc = "1: ADC12+ Resolution : 10 Bit"]
    ADC12RES_1 = 1,
    #[doc = "2: ADC12+ Resolution : 12 Bit"]
    ADC12RES_2 = 2,
    #[doc = "3: ADC12+ Resolution : reserved"]
    ADC12RES_3 = 3,
}
impl From<ADC12RES_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12RES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12RES`"]
pub type ADC12RES_R = crate::R<u8, ADC12RES_A>;
impl ADC12RES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12RES_A {
        match self.bits {
            0 => ADC12RES_A::ADC12RES_0,
            1 => ADC12RES_A::ADC12RES_1,
            2 => ADC12RES_A::ADC12RES_2,
            3 => ADC12RES_A::ADC12RES_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12RES_0`"]
    #[inline(always)]
    pub fn is_adc12res_0(&self) -> bool {
        *self == ADC12RES_A::ADC12RES_0
    }
    #[doc = "Checks if the value of the field is `ADC12RES_1`"]
    #[inline(always)]
    pub fn is_adc12res_1(&self) -> bool {
        *self == ADC12RES_A::ADC12RES_1
    }
    #[doc = "Checks if the value of the field is `ADC12RES_2`"]
    #[inline(always)]
    pub fn is_adc12res_2(&self) -> bool {
        *self == ADC12RES_A::ADC12RES_2
    }
    #[doc = "Checks if the value of the field is `ADC12RES_3`"]
    #[inline(always)]
    pub fn is_adc12res_3(&self) -> bool {
        *self == ADC12RES_A::ADC12RES_3
    }
}
#[doc = "Write proxy for field `ADC12RES`"]
pub struct ADC12RES_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC12+ Resolution : 8 Bit"]
    #[inline(always)]
    pub fn adc12res_0(self) -> &'a mut W {
        self.variant(ADC12RES_A::ADC12RES_0)
    }
    #[doc = "ADC12+ Resolution : 10 Bit"]
    #[inline(always)]
    pub fn adc12res_1(self) -> &'a mut W {
        self.variant(ADC12RES_A::ADC12RES_1)
    }
    #[doc = "ADC12+ Resolution : 12 Bit"]
    #[inline(always)]
    pub fn adc12res_2(self) -> &'a mut W {
        self.variant(ADC12RES_A::ADC12RES_2)
    }
    #[doc = "ADC12+ Resolution : reserved"]
    #[inline(always)]
    pub fn adc12res_3(self) -> &'a mut W {
        self.variant(ADC12RES_A::ADC12RES_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `ADC12TCOFF`"]
pub type ADC12TCOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12TCOFF`"]
pub struct ADC12TCOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12TCOFF_W<'a> {
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
#[doc = "Reader of field `ADC12PDIV`"]
pub type ADC12PDIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12PDIV`"]
pub struct ADC12PDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12PDIV_W<'a> {
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
impl R {
    #[doc = "Bit 0 - ADC12+ Reference Burst"]
    #[inline(always)]
    pub fn adc12refburst(&self) -> ADC12REFBURST_R {
        ADC12REFBURST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC12+ Reference Out"]
    #[inline(always)]
    pub fn adc12refout(&self) -> ADC12REFOUT_R {
        ADC12REFOUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC12+ Sampling Rate"]
    #[inline(always)]
    pub fn adc12sr(&self) -> ADC12SR_R {
        ADC12SR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC12+ Data Format"]
    #[inline(always)]
    pub fn adc12df(&self) -> ADC12DF_R {
        ADC12DF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - ADC12+ Resolution Bit: 0"]
    #[inline(always)]
    pub fn adc12res(&self) -> ADC12RES_R {
        ADC12RES_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - ADC12+ Temperature Sensor Off"]
    #[inline(always)]
    pub fn adc12tcoff(&self) -> ADC12TCOFF_R {
        ADC12TCOFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC12+ predivider 0:/1 1:/4"]
    #[inline(always)]
    pub fn adc12pdiv(&self) -> ADC12PDIV_R {
        ADC12PDIV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12+ Reference Burst"]
    #[inline(always)]
    pub fn adc12refburst(&mut self) -> ADC12REFBURST_W {
        ADC12REFBURST_W { w: self }
    }
    #[doc = "Bit 1 - ADC12+ Reference Out"]
    #[inline(always)]
    pub fn adc12refout(&mut self) -> ADC12REFOUT_W {
        ADC12REFOUT_W { w: self }
    }
    #[doc = "Bit 2 - ADC12+ Sampling Rate"]
    #[inline(always)]
    pub fn adc12sr(&mut self) -> ADC12SR_W {
        ADC12SR_W { w: self }
    }
    #[doc = "Bit 3 - ADC12+ Data Format"]
    #[inline(always)]
    pub fn adc12df(&mut self) -> ADC12DF_W {
        ADC12DF_W { w: self }
    }
    #[doc = "Bits 4:5 - ADC12+ Resolution Bit: 0"]
    #[inline(always)]
    pub fn adc12res(&mut self) -> ADC12RES_W {
        ADC12RES_W { w: self }
    }
    #[doc = "Bit 7 - ADC12+ Temperature Sensor Off"]
    #[inline(always)]
    pub fn adc12tcoff(&mut self) -> ADC12TCOFF_W {
        ADC12TCOFF_W { w: self }
    }
    #[doc = "Bit 8 - ADC12+ predivider 0:/1 1:/4"]
    #[inline(always)]
    pub fn adc12pdiv(&mut self) -> ADC12PDIV_W {
        ADC12PDIV_W { w: self }
    }
}
