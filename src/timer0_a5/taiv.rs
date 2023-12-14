#[doc = "Register `TAIV` reader"]
pub type R = crate::R<TAIV_SPEC>;
#[doc = "Register `TAIV` writer"]
pub type W = crate::W<TAIV_SPEC>;
#[doc = "Field `TAIV` reader - Timer A Interrupt Vector value"]
pub type TAIV_R = crate::FieldReader<TAIV_A>;
#[doc = "Timer A Interrupt Vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Capture/Compare 1"]
    TACCR1 = 2,
    #[doc = "4: Capture/Compare 2"]
    TACCR2 = 4,
    #[doc = "10: Timer overflow"]
    TAIFG = 10,
}
impl From<TAIV_A> for u8 {
    #[inline(always)]
    fn from(variant: TAIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TAIV_A {
    type Ux = u8;
}
impl TAIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TAIV_A> {
        match self.bits {
            0 => Some(TAIV_A::NONE),
            2 => Some(TAIV_A::TACCR1),
            4 => Some(TAIV_A::TACCR2),
            10 => Some(TAIV_A::TAIFG),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TAIV_A::NONE
    }
    #[doc = "Capture/Compare 1"]
    #[inline(always)]
    pub fn is_taccr1(&self) -> bool {
        *self == TAIV_A::TACCR1
    }
    #[doc = "Capture/Compare 2"]
    #[inline(always)]
    pub fn is_taccr2(&self) -> bool {
        *self == TAIV_A::TACCR2
    }
    #[doc = "Timer overflow"]
    #[inline(always)]
    pub fn is_taifg(&self) -> bool {
        *self == TAIV_A::TAIFG
    }
}
#[doc = "Field `TAIV` writer - Timer A Interrupt Vector value"]
pub type TAIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TAIV_A>;
impl<'a, REG> TAIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(TAIV_A::NONE)
    }
    #[doc = "Capture/Compare 1"]
    #[inline(always)]
    pub fn taccr1(self) -> &'a mut crate::W<REG> {
        self.variant(TAIV_A::TACCR1)
    }
    #[doc = "Capture/Compare 2"]
    #[inline(always)]
    pub fn taccr2(self) -> &'a mut crate::W<REG> {
        self.variant(TAIV_A::TACCR2)
    }
    #[doc = "Timer overflow"]
    #[inline(always)]
    pub fn taifg(self) -> &'a mut crate::W<REG> {
        self.variant(TAIV_A::TAIFG)
    }
}
impl R {
    #[doc = "Bits 0:3 - Timer A Interrupt Vector value"]
    #[inline(always)]
    pub fn taiv(&self) -> TAIV_R {
        TAIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Timer A Interrupt Vector value"]
    #[inline(always)]
    #[must_use]
    pub fn taiv(&mut self) -> TAIV_W<TAIV_SPEC> {
        TAIV_W::new(self, 0)
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
#[doc = "Timer0_A5 Interrupt Vector Word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`taiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`taiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAIV_SPEC;
impl crate::RegisterSpec for TAIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`taiv::R`](R) reader structure"]
impl crate::Readable for TAIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`taiv::W`](W) writer structure"]
impl crate::Writable for TAIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAIV to value 0"]
impl crate::Resettable for TAIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
