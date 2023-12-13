#[doc = "Register `ADC12CTL2` reader"]
pub type R = crate::R<ADC12CTL2_SPEC>;
#[doc = "Register `ADC12CTL2` writer"]
pub type W = crate::W<ADC12CTL2_SPEC>;
#[doc = "Field `ADC12REFBURST` reader - ADC12+ Reference Burst"]
pub type ADC12REFBURST_R = crate::BitReader;
#[doc = "Field `ADC12REFBURST` writer - ADC12+ Reference Burst"]
pub type ADC12REFBURST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12REFOUT` reader - ADC12+ Reference Out"]
pub type ADC12REFOUT_R = crate::BitReader;
#[doc = "Field `ADC12REFOUT` writer - ADC12+ Reference Out"]
pub type ADC12REFOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12SR` reader - ADC12+ Sampling Rate"]
pub type ADC12SR_R = crate::BitReader;
#[doc = "Field `ADC12SR` writer - ADC12+ Sampling Rate"]
pub type ADC12SR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12DF` reader - ADC12+ Data Format"]
pub type ADC12DF_R = crate::BitReader;
#[doc = "Field `ADC12DF` writer - ADC12+ Data Format"]
pub type ADC12DF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12RES` reader - ADC12+ Resolution Bit: 0"]
pub type ADC12RES_R = crate::FieldReader<ADC12RES_A>;
#[doc = "ADC12+ Resolution Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for ADC12RES_A {
    type Ux = u8;
}
impl ADC12RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC12RES_A {
        match self.bits {
            0 => ADC12RES_A::ADC12RES_0,
            1 => ADC12RES_A::ADC12RES_1,
            2 => ADC12RES_A::ADC12RES_2,
            3 => ADC12RES_A::ADC12RES_3,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC12+ Resolution : 8 Bit"]
    #[inline(always)]
    pub fn is_adc12res_0(&self) -> bool {
        *self == ADC12RES_A::ADC12RES_0
    }
    #[doc = "ADC12+ Resolution : 10 Bit"]
    #[inline(always)]
    pub fn is_adc12res_1(&self) -> bool {
        *self == ADC12RES_A::ADC12RES_1
    }
    #[doc = "ADC12+ Resolution : 12 Bit"]
    #[inline(always)]
    pub fn is_adc12res_2(&self) -> bool {
        *self == ADC12RES_A::ADC12RES_2
    }
    #[doc = "ADC12+ Resolution : reserved"]
    #[inline(always)]
    pub fn is_adc12res_3(&self) -> bool {
        *self == ADC12RES_A::ADC12RES_3
    }
}
#[doc = "Field `ADC12RES` writer - ADC12+ Resolution Bit: 0"]
pub type ADC12RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ADC12RES_A>;
impl<'a, REG> ADC12RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC12+ Resolution : 8 Bit"]
    #[inline(always)]
    pub fn adc12res_0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12RES_A::ADC12RES_0)
    }
    #[doc = "ADC12+ Resolution : 10 Bit"]
    #[inline(always)]
    pub fn adc12res_1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12RES_A::ADC12RES_1)
    }
    #[doc = "ADC12+ Resolution : 12 Bit"]
    #[inline(always)]
    pub fn adc12res_2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12RES_A::ADC12RES_2)
    }
    #[doc = "ADC12+ Resolution : reserved"]
    #[inline(always)]
    pub fn adc12res_3(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12RES_A::ADC12RES_3)
    }
}
#[doc = "Field `ADC12TCOFF` reader - ADC12+ Temperature Sensor Off"]
pub type ADC12TCOFF_R = crate::BitReader;
#[doc = "Field `ADC12TCOFF` writer - ADC12+ Temperature Sensor Off"]
pub type ADC12TCOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12PDIV` reader - ADC12+ predivider 0:/1 1:/4"]
pub type ADC12PDIV_R = crate::BitReader;
#[doc = "Field `ADC12PDIV` writer - ADC12+ predivider 0:/1 1:/4"]
pub type ADC12PDIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC12+ Reference Burst"]
    #[inline(always)]
    pub fn adc12refburst(&self) -> ADC12REFBURST_R {
        ADC12REFBURST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC12+ Reference Out"]
    #[inline(always)]
    pub fn adc12refout(&self) -> ADC12REFOUT_R {
        ADC12REFOUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC12+ Sampling Rate"]
    #[inline(always)]
    pub fn adc12sr(&self) -> ADC12SR_R {
        ADC12SR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC12+ Data Format"]
    #[inline(always)]
    pub fn adc12df(&self) -> ADC12DF_R {
        ADC12DF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - ADC12+ Resolution Bit: 0"]
    #[inline(always)]
    pub fn adc12res(&self) -> ADC12RES_R {
        ADC12RES_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - ADC12+ Temperature Sensor Off"]
    #[inline(always)]
    pub fn adc12tcoff(&self) -> ADC12TCOFF_R {
        ADC12TCOFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC12+ predivider 0:/1 1:/4"]
    #[inline(always)]
    pub fn adc12pdiv(&self) -> ADC12PDIV_R {
        ADC12PDIV_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12+ Reference Burst"]
    #[inline(always)]
    #[must_use]
    pub fn adc12refburst(&mut self) -> ADC12REFBURST_W<ADC12CTL2_SPEC> {
        ADC12REFBURST_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC12+ Reference Out"]
    #[inline(always)]
    #[must_use]
    pub fn adc12refout(&mut self) -> ADC12REFOUT_W<ADC12CTL2_SPEC> {
        ADC12REFOUT_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC12+ Sampling Rate"]
    #[inline(always)]
    #[must_use]
    pub fn adc12sr(&mut self) -> ADC12SR_W<ADC12CTL2_SPEC> {
        ADC12SR_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC12+ Data Format"]
    #[inline(always)]
    #[must_use]
    pub fn adc12df(&mut self) -> ADC12DF_W<ADC12CTL2_SPEC> {
        ADC12DF_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - ADC12+ Resolution Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc12res(&mut self) -> ADC12RES_W<ADC12CTL2_SPEC> {
        ADC12RES_W::new(self, 4)
    }
    #[doc = "Bit 7 - ADC12+ Temperature Sensor Off"]
    #[inline(always)]
    #[must_use]
    pub fn adc12tcoff(&mut self) -> ADC12TCOFF_W<ADC12CTL2_SPEC> {
        ADC12TCOFF_W::new(self, 7)
    }
    #[doc = "Bit 8 - ADC12+ predivider 0:/1 1:/4"]
    #[inline(always)]
    #[must_use]
    pub fn adc12pdiv(&mut self) -> ADC12PDIV_W<ADC12CTL2_SPEC> {
        ADC12PDIV_W::new(self, 8)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC12+ Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12ctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12ctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC12CTL2_SPEC;
impl crate::RegisterSpec for ADC12CTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc12ctl2::R`](R) reader structure"]
impl crate::Readable for ADC12CTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc12ctl2::W`](W) writer structure"]
impl crate::Writable for ADC12CTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC12CTL2 to value 0"]
impl crate::Resettable for ADC12CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
