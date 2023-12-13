#[doc = "Register `ADC12MCTL15` reader"]
pub type R = crate::R<ADC12MCTL15_SPEC>;
#[doc = "Register `ADC12MCTL15` writer"]
pub type W = crate::W<ADC12MCTL15_SPEC>;
#[doc = "Field `ADC12INCH` reader - ADC12 Input Channel Select Bit 0"]
pub type ADC12INCH_R = crate::FieldReader<ADC12INCH_A>;
#[doc = "ADC12 Input Channel Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC12INCH_A {
    #[doc = "0: ADC12 Input Channel 0"]
    ADC12INCH_0 = 0,
    #[doc = "1: ADC12 Input Channel 1"]
    ADC12INCH_1 = 1,
    #[doc = "2: ADC12 Input Channel 2"]
    ADC12INCH_2 = 2,
    #[doc = "3: ADC12 Input Channel 3"]
    ADC12INCH_3 = 3,
    #[doc = "4: ADC12 Input Channel 4"]
    ADC12INCH_4 = 4,
    #[doc = "5: ADC12 Input Channel 5"]
    ADC12INCH_5 = 5,
    #[doc = "6: ADC12 Input Channel 6"]
    ADC12INCH_6 = 6,
    #[doc = "7: ADC12 Input Channel 7"]
    ADC12INCH_7 = 7,
    #[doc = "8: ADC12 Input Channel 8"]
    ADC12INCH_8 = 8,
    #[doc = "9: ADC12 Input Channel 9"]
    ADC12INCH_9 = 9,
    #[doc = "10: ADC12 Input Channel 10"]
    ADC12INCH_10 = 10,
    #[doc = "11: ADC12 Input Channel 11"]
    ADC12INCH_11 = 11,
    #[doc = "12: ADC12 Input Channel 12"]
    ADC12INCH_12 = 12,
    #[doc = "13: ADC12 Input Channel 13"]
    ADC12INCH_13 = 13,
    #[doc = "14: ADC12 Input Channel 14"]
    ADC12INCH_14 = 14,
    #[doc = "15: ADC12 Input Channel 15"]
    ADC12INCH_15 = 15,
}
impl From<ADC12INCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12INCH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC12INCH_A {
    type Ux = u8;
}
impl ADC12INCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC12INCH_A {
        match self.bits {
            0 => ADC12INCH_A::ADC12INCH_0,
            1 => ADC12INCH_A::ADC12INCH_1,
            2 => ADC12INCH_A::ADC12INCH_2,
            3 => ADC12INCH_A::ADC12INCH_3,
            4 => ADC12INCH_A::ADC12INCH_4,
            5 => ADC12INCH_A::ADC12INCH_5,
            6 => ADC12INCH_A::ADC12INCH_6,
            7 => ADC12INCH_A::ADC12INCH_7,
            8 => ADC12INCH_A::ADC12INCH_8,
            9 => ADC12INCH_A::ADC12INCH_9,
            10 => ADC12INCH_A::ADC12INCH_10,
            11 => ADC12INCH_A::ADC12INCH_11,
            12 => ADC12INCH_A::ADC12INCH_12,
            13 => ADC12INCH_A::ADC12INCH_13,
            14 => ADC12INCH_A::ADC12INCH_14,
            15 => ADC12INCH_A::ADC12INCH_15,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC12 Input Channel 0"]
    #[inline(always)]
    pub fn is_adc12inch_0(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_0
    }
    #[doc = "ADC12 Input Channel 1"]
    #[inline(always)]
    pub fn is_adc12inch_1(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_1
    }
    #[doc = "ADC12 Input Channel 2"]
    #[inline(always)]
    pub fn is_adc12inch_2(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_2
    }
    #[doc = "ADC12 Input Channel 3"]
    #[inline(always)]
    pub fn is_adc12inch_3(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_3
    }
    #[doc = "ADC12 Input Channel 4"]
    #[inline(always)]
    pub fn is_adc12inch_4(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_4
    }
    #[doc = "ADC12 Input Channel 5"]
    #[inline(always)]
    pub fn is_adc12inch_5(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_5
    }
    #[doc = "ADC12 Input Channel 6"]
    #[inline(always)]
    pub fn is_adc12inch_6(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_6
    }
    #[doc = "ADC12 Input Channel 7"]
    #[inline(always)]
    pub fn is_adc12inch_7(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_7
    }
    #[doc = "ADC12 Input Channel 8"]
    #[inline(always)]
    pub fn is_adc12inch_8(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_8
    }
    #[doc = "ADC12 Input Channel 9"]
    #[inline(always)]
    pub fn is_adc12inch_9(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_9
    }
    #[doc = "ADC12 Input Channel 10"]
    #[inline(always)]
    pub fn is_adc12inch_10(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_10
    }
    #[doc = "ADC12 Input Channel 11"]
    #[inline(always)]
    pub fn is_adc12inch_11(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_11
    }
    #[doc = "ADC12 Input Channel 12"]
    #[inline(always)]
    pub fn is_adc12inch_12(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_12
    }
    #[doc = "ADC12 Input Channel 13"]
    #[inline(always)]
    pub fn is_adc12inch_13(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_13
    }
    #[doc = "ADC12 Input Channel 14"]
    #[inline(always)]
    pub fn is_adc12inch_14(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_14
    }
    #[doc = "ADC12 Input Channel 15"]
    #[inline(always)]
    pub fn is_adc12inch_15(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_15
    }
}
#[doc = "Field `ADC12INCH` writer - ADC12 Input Channel Select Bit 0"]
pub type ADC12INCH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, ADC12INCH_A>;
impl<'a, REG> ADC12INCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC12 Input Channel 0"]
    #[inline(always)]
    pub fn adc12inch_0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12INCH_A::ADC12INCH_0)
    }
    #[doc = "ADC12 Input Channel 1"]
    #[inline(always)]
    pub fn adc12inch_1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12INCH_A::ADC12INCH_1)
    }
    #[doc = "ADC12 Input Channel 2"]
    #[inline(always)]
    pub fn adc12inch_2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12INCH_A::ADC12INCH_2)
    }
    #[doc = "ADC12 Input Channel 3"]
    #[inline(always)]
    pub fn adc12inch_3(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12INCH_A::ADC12INCH_3)
    }
    #[doc = "ADC12 Input Channel 4"]
    #[inline(always)]
    pub fn adc12inch_4(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12INCH_A::ADC12INCH_4)
    }
    #[doc = "ADC12 Input Channel 5"]
    #[inline(always)]
    pub fn adc12inch_5(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12INCH_A::ADC12INCH_5)
    }
    #[doc = "ADC12 Input Channel 6"]
    #[inline(always)]
    pub fn adc12inch_6(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12INCH_A::ADC12INCH_6)
    }
    #[doc = "ADC12 Input Channel 7"]
    #[inline(always)]
    pub fn adc12inch_7(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12INCH_A::ADC12INCH_7)
    }
    #[doc = "ADC12 Input Channel 8"]
    #[inline(always)]
    pub fn adc12inch_8(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12INCH_A::ADC12INCH_8)
    }
    #[doc = "ADC12 Input Channel 9"]
    #[inline(always)]
    pub fn adc12inch_9(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12INCH_A::ADC12INCH_9)
    }
    #[doc = "ADC12 Input Channel 10"]
    #[inline(always)]
    pub fn adc12inch_10(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12INCH_A::ADC12INCH_10)
    }
    #[doc = "ADC12 Input Channel 11"]
    #[inline(always)]
    pub fn adc12inch_11(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12INCH_A::ADC12INCH_11)
    }
    #[doc = "ADC12 Input Channel 12"]
    #[inline(always)]
    pub fn adc12inch_12(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12INCH_A::ADC12INCH_12)
    }
    #[doc = "ADC12 Input Channel 13"]
    #[inline(always)]
    pub fn adc12inch_13(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12INCH_A::ADC12INCH_13)
    }
    #[doc = "ADC12 Input Channel 14"]
    #[inline(always)]
    pub fn adc12inch_14(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12INCH_A::ADC12INCH_14)
    }
    #[doc = "ADC12 Input Channel 15"]
    #[inline(always)]
    pub fn adc12inch_15(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12INCH_A::ADC12INCH_15)
    }
}
#[doc = "Field `ADC12SREF` reader - ADC12 Select Reference Bit 0"]
pub type ADC12SREF_R = crate::FieldReader<ADC12SREF_A>;
#[doc = "ADC12 Select Reference Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC12SREF_A {
    #[doc = "0: ADC12 Select Reference 0"]
    ADC12SREF_0 = 0,
    #[doc = "1: ADC12 Select Reference 1"]
    ADC12SREF_1 = 1,
    #[doc = "2: ADC12 Select Reference 2"]
    ADC12SREF_2 = 2,
    #[doc = "3: ADC12 Select Reference 3"]
    ADC12SREF_3 = 3,
    #[doc = "4: ADC12 Select Reference 4"]
    ADC12SREF_4 = 4,
    #[doc = "5: ADC12 Select Reference 5"]
    ADC12SREF_5 = 5,
    #[doc = "6: ADC12 Select Reference 6"]
    ADC12SREF_6 = 6,
    #[doc = "7: ADC12 Select Reference 7"]
    ADC12SREF_7 = 7,
}
impl From<ADC12SREF_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12SREF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC12SREF_A {
    type Ux = u8;
}
impl ADC12SREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC12SREF_A {
        match self.bits {
            0 => ADC12SREF_A::ADC12SREF_0,
            1 => ADC12SREF_A::ADC12SREF_1,
            2 => ADC12SREF_A::ADC12SREF_2,
            3 => ADC12SREF_A::ADC12SREF_3,
            4 => ADC12SREF_A::ADC12SREF_4,
            5 => ADC12SREF_A::ADC12SREF_5,
            6 => ADC12SREF_A::ADC12SREF_6,
            7 => ADC12SREF_A::ADC12SREF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC12 Select Reference 0"]
    #[inline(always)]
    pub fn is_adc12sref_0(&self) -> bool {
        *self == ADC12SREF_A::ADC12SREF_0
    }
    #[doc = "ADC12 Select Reference 1"]
    #[inline(always)]
    pub fn is_adc12sref_1(&self) -> bool {
        *self == ADC12SREF_A::ADC12SREF_1
    }
    #[doc = "ADC12 Select Reference 2"]
    #[inline(always)]
    pub fn is_adc12sref_2(&self) -> bool {
        *self == ADC12SREF_A::ADC12SREF_2
    }
    #[doc = "ADC12 Select Reference 3"]
    #[inline(always)]
    pub fn is_adc12sref_3(&self) -> bool {
        *self == ADC12SREF_A::ADC12SREF_3
    }
    #[doc = "ADC12 Select Reference 4"]
    #[inline(always)]
    pub fn is_adc12sref_4(&self) -> bool {
        *self == ADC12SREF_A::ADC12SREF_4
    }
    #[doc = "ADC12 Select Reference 5"]
    #[inline(always)]
    pub fn is_adc12sref_5(&self) -> bool {
        *self == ADC12SREF_A::ADC12SREF_5
    }
    #[doc = "ADC12 Select Reference 6"]
    #[inline(always)]
    pub fn is_adc12sref_6(&self) -> bool {
        *self == ADC12SREF_A::ADC12SREF_6
    }
    #[doc = "ADC12 Select Reference 7"]
    #[inline(always)]
    pub fn is_adc12sref_7(&self) -> bool {
        *self == ADC12SREF_A::ADC12SREF_7
    }
}
#[doc = "Field `ADC12SREF` writer - ADC12 Select Reference Bit 0"]
pub type ADC12SREF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, ADC12SREF_A>;
impl<'a, REG> ADC12SREF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC12 Select Reference 0"]
    #[inline(always)]
    pub fn adc12sref_0(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SREF_A::ADC12SREF_0)
    }
    #[doc = "ADC12 Select Reference 1"]
    #[inline(always)]
    pub fn adc12sref_1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SREF_A::ADC12SREF_1)
    }
    #[doc = "ADC12 Select Reference 2"]
    #[inline(always)]
    pub fn adc12sref_2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SREF_A::ADC12SREF_2)
    }
    #[doc = "ADC12 Select Reference 3"]
    #[inline(always)]
    pub fn adc12sref_3(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SREF_A::ADC12SREF_3)
    }
    #[doc = "ADC12 Select Reference 4"]
    #[inline(always)]
    pub fn adc12sref_4(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SREF_A::ADC12SREF_4)
    }
    #[doc = "ADC12 Select Reference 5"]
    #[inline(always)]
    pub fn adc12sref_5(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SREF_A::ADC12SREF_5)
    }
    #[doc = "ADC12 Select Reference 6"]
    #[inline(always)]
    pub fn adc12sref_6(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SREF_A::ADC12SREF_6)
    }
    #[doc = "ADC12 Select Reference 7"]
    #[inline(always)]
    pub fn adc12sref_7(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12SREF_A::ADC12SREF_7)
    }
}
#[doc = "Field `ADC12EOS` reader - ADC12 End of Sequence"]
pub type ADC12EOS_R = crate::BitReader;
#[doc = "Field `ADC12EOS` writer - ADC12 End of Sequence"]
pub type ADC12EOS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - ADC12 Input Channel Select Bit 0"]
    #[inline(always)]
    pub fn adc12inch(&self) -> ADC12INCH_R {
        ADC12INCH_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - ADC12 Select Reference Bit 0"]
    #[inline(always)]
    pub fn adc12sref(&self) -> ADC12SREF_R {
        ADC12SREF_R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - ADC12 End of Sequence"]
    #[inline(always)]
    pub fn adc12eos(&self) -> ADC12EOS_R {
        ADC12EOS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC12 Input Channel Select Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc12inch(&mut self) -> ADC12INCH_W<ADC12MCTL15_SPEC> {
        ADC12INCH_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - ADC12 Select Reference Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc12sref(&mut self) -> ADC12SREF_W<ADC12MCTL15_SPEC> {
        ADC12SREF_W::new(self, 4)
    }
    #[doc = "Bit 7 - ADC12 End of Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn adc12eos(&mut self) -> ADC12EOS_W<ADC12MCTL15_SPEC> {
        ADC12EOS_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC12 Memory Control 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12mctl15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12mctl15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC12MCTL15_SPEC;
impl crate::RegisterSpec for ADC12MCTL15_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adc12mctl15::R`](R) reader structure"]
impl crate::Readable for ADC12MCTL15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc12mctl15::W`](W) writer structure"]
impl crate::Writable for ADC12MCTL15_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC12MCTL15 to value 0"]
impl crate::Resettable for ADC12MCTL15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
