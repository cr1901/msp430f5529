#[doc = "Register `UCSCTL1` reader"]
pub type R = crate::R<UCSCTL1_SPEC>;
#[doc = "Register `UCSCTL1` writer"]
pub type W = crate::W<UCSCTL1_SPEC>;
#[doc = "Field `DISMOD` reader - Disable Modulation"]
pub type DISMOD_R = crate::BitReader;
#[doc = "Field `DISMOD` writer - Disable Modulation"]
pub type DISMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCORSEL` reader - DCO Freq. Range Select Bit : 0"]
pub type DCORSEL_R = crate::FieldReader<DCORSEL_A>;
#[doc = "DCO Freq. Range Select Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCORSEL_A {
    #[doc = "0: DCO RSEL 0"]
    DCORSEL_0 = 0,
    #[doc = "1: DCO RSEL 1"]
    DCORSEL_1 = 1,
    #[doc = "2: DCO RSEL 2"]
    DCORSEL_2 = 2,
    #[doc = "3: DCO RSEL 3"]
    DCORSEL_3 = 3,
    #[doc = "4: DCO RSEL 4"]
    DCORSEL_4 = 4,
    #[doc = "5: DCO RSEL 5"]
    DCORSEL_5 = 5,
    #[doc = "6: DCO RSEL 6"]
    DCORSEL_6 = 6,
    #[doc = "7: DCO RSEL 7"]
    DCORSEL_7 = 7,
}
impl From<DCORSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DCORSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DCORSEL_A {
    type Ux = u8;
}
impl DCORSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCORSEL_A {
        match self.bits {
            0 => DCORSEL_A::DCORSEL_0,
            1 => DCORSEL_A::DCORSEL_1,
            2 => DCORSEL_A::DCORSEL_2,
            3 => DCORSEL_A::DCORSEL_3,
            4 => DCORSEL_A::DCORSEL_4,
            5 => DCORSEL_A::DCORSEL_5,
            6 => DCORSEL_A::DCORSEL_6,
            7 => DCORSEL_A::DCORSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "DCO RSEL 0"]
    #[inline(always)]
    pub fn is_dcorsel_0(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_0
    }
    #[doc = "DCO RSEL 1"]
    #[inline(always)]
    pub fn is_dcorsel_1(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_1
    }
    #[doc = "DCO RSEL 2"]
    #[inline(always)]
    pub fn is_dcorsel_2(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_2
    }
    #[doc = "DCO RSEL 3"]
    #[inline(always)]
    pub fn is_dcorsel_3(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_3
    }
    #[doc = "DCO RSEL 4"]
    #[inline(always)]
    pub fn is_dcorsel_4(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_4
    }
    #[doc = "DCO RSEL 5"]
    #[inline(always)]
    pub fn is_dcorsel_5(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_5
    }
    #[doc = "DCO RSEL 6"]
    #[inline(always)]
    pub fn is_dcorsel_6(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_6
    }
    #[doc = "DCO RSEL 7"]
    #[inline(always)]
    pub fn is_dcorsel_7(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_7
    }
}
#[doc = "Field `DCORSEL` writer - DCO Freq. Range Select Bit : 0"]
pub type DCORSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, DCORSEL_A>;
impl<'a, REG> DCORSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DCO RSEL 0"]
    #[inline(always)]
    pub fn dcorsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(DCORSEL_A::DCORSEL_0)
    }
    #[doc = "DCO RSEL 1"]
    #[inline(always)]
    pub fn dcorsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(DCORSEL_A::DCORSEL_1)
    }
    #[doc = "DCO RSEL 2"]
    #[inline(always)]
    pub fn dcorsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(DCORSEL_A::DCORSEL_2)
    }
    #[doc = "DCO RSEL 3"]
    #[inline(always)]
    pub fn dcorsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(DCORSEL_A::DCORSEL_3)
    }
    #[doc = "DCO RSEL 4"]
    #[inline(always)]
    pub fn dcorsel_4(self) -> &'a mut crate::W<REG> {
        self.variant(DCORSEL_A::DCORSEL_4)
    }
    #[doc = "DCO RSEL 5"]
    #[inline(always)]
    pub fn dcorsel_5(self) -> &'a mut crate::W<REG> {
        self.variant(DCORSEL_A::DCORSEL_5)
    }
    #[doc = "DCO RSEL 6"]
    #[inline(always)]
    pub fn dcorsel_6(self) -> &'a mut crate::W<REG> {
        self.variant(DCORSEL_A::DCORSEL_6)
    }
    #[doc = "DCO RSEL 7"]
    #[inline(always)]
    pub fn dcorsel_7(self) -> &'a mut crate::W<REG> {
        self.variant(DCORSEL_A::DCORSEL_7)
    }
}
impl R {
    #[doc = "Bit 0 - Disable Modulation"]
    #[inline(always)]
    pub fn dismod(&self) -> DISMOD_R {
        DISMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - DCO Freq. Range Select Bit : 0"]
    #[inline(always)]
    pub fn dcorsel(&self) -> DCORSEL_R {
        DCORSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Disable Modulation"]
    #[inline(always)]
    #[must_use]
    pub fn dismod(&mut self) -> DISMOD_W<UCSCTL1_SPEC> {
        DISMOD_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - DCO Freq. Range Select Bit : 0"]
    #[inline(always)]
    #[must_use]
    pub fn dcorsel(&mut self) -> DCORSEL_W<UCSCTL1_SPEC> {
        DCORSEL_W::new(self, 4)
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
#[doc = "UCS Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSCTL1_SPEC;
impl crate::RegisterSpec for UCSCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucsctl1::R`](R) reader structure"]
impl crate::Readable for UCSCTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsctl1::W`](W) writer structure"]
impl crate::Writable for UCSCTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSCTL1 to value 0"]
impl crate::Resettable for UCSCTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
