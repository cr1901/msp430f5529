#[doc = "Register `PMMCTL0` reader"]
pub type R = crate::R<PMMCTL0_SPEC>;
#[doc = "Register `PMMCTL0` writer"]
pub type W = crate::W<PMMCTL0_SPEC>;
#[doc = "Field `PMMCOREV` reader - PMM Core Voltage Bit: 0"]
pub type PMMCOREV_R = crate::FieldReader<PMMCOREV_A>;
#[doc = "PMM Core Voltage Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PMMCOREV_A {
    #[doc = "0: PMM Core Voltage 0 (1.35V)"]
    PMMCOREV_0 = 0,
    #[doc = "1: PMM Core Voltage 1 (1.55V)"]
    PMMCOREV_1 = 1,
    #[doc = "2: PMM Core Voltage 2 (1.75V)"]
    PMMCOREV_2 = 2,
    #[doc = "3: PMM Core Voltage 3 (1.85V)"]
    PMMCOREV_3 = 3,
}
impl From<PMMCOREV_A> for u8 {
    #[inline(always)]
    fn from(variant: PMMCOREV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PMMCOREV_A {
    type Ux = u8;
}
impl PMMCOREV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PMMCOREV_A {
        match self.bits {
            0 => PMMCOREV_A::PMMCOREV_0,
            1 => PMMCOREV_A::PMMCOREV_1,
            2 => PMMCOREV_A::PMMCOREV_2,
            3 => PMMCOREV_A::PMMCOREV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "PMM Core Voltage 0 (1.35V)"]
    #[inline(always)]
    pub fn is_pmmcorev_0(&self) -> bool {
        *self == PMMCOREV_A::PMMCOREV_0
    }
    #[doc = "PMM Core Voltage 1 (1.55V)"]
    #[inline(always)]
    pub fn is_pmmcorev_1(&self) -> bool {
        *self == PMMCOREV_A::PMMCOREV_1
    }
    #[doc = "PMM Core Voltage 2 (1.75V)"]
    #[inline(always)]
    pub fn is_pmmcorev_2(&self) -> bool {
        *self == PMMCOREV_A::PMMCOREV_2
    }
    #[doc = "PMM Core Voltage 3 (1.85V)"]
    #[inline(always)]
    pub fn is_pmmcorev_3(&self) -> bool {
        *self == PMMCOREV_A::PMMCOREV_3
    }
}
#[doc = "Field `PMMCOREV` writer - PMM Core Voltage Bit: 0"]
pub type PMMCOREV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PMMCOREV_A>;
impl<'a, REG> PMMCOREV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PMM Core Voltage 0 (1.35V)"]
    #[inline(always)]
    pub fn pmmcorev_0(self) -> &'a mut crate::W<REG> {
        self.variant(PMMCOREV_A::PMMCOREV_0)
    }
    #[doc = "PMM Core Voltage 1 (1.55V)"]
    #[inline(always)]
    pub fn pmmcorev_1(self) -> &'a mut crate::W<REG> {
        self.variant(PMMCOREV_A::PMMCOREV_1)
    }
    #[doc = "PMM Core Voltage 2 (1.75V)"]
    #[inline(always)]
    pub fn pmmcorev_2(self) -> &'a mut crate::W<REG> {
        self.variant(PMMCOREV_A::PMMCOREV_2)
    }
    #[doc = "PMM Core Voltage 3 (1.85V)"]
    #[inline(always)]
    pub fn pmmcorev_3(self) -> &'a mut crate::W<REG> {
        self.variant(PMMCOREV_A::PMMCOREV_3)
    }
}
#[doc = "Field `PMMSWBOR` reader - PMM Software BOR"]
pub type PMMSWBOR_R = crate::BitReader;
#[doc = "Field `PMMSWBOR` writer - PMM Software BOR"]
pub type PMMSWBOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMMSWPOR` reader - PMM Software POR"]
pub type PMMSWPOR_R = crate::BitReader;
#[doc = "Field `PMMSWPOR` writer - PMM Software POR"]
pub type PMMSWPOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMMREGOFF` reader - PMM Turn Regulator off"]
pub type PMMREGOFF_R = crate::BitReader;
#[doc = "Field `PMMREGOFF` writer - PMM Turn Regulator off"]
pub type PMMREGOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMMHPMRE` reader - PMM Global High Power Module Request Enable"]
pub type PMMHPMRE_R = crate::BitReader;
#[doc = "Field `PMMHPMRE` writer - PMM Global High Power Module Request Enable"]
pub type PMMHPMRE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - PMM Core Voltage Bit: 0"]
    #[inline(always)]
    pub fn pmmcorev(&self) -> PMMCOREV_R {
        PMMCOREV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - PMM Software BOR"]
    #[inline(always)]
    pub fn pmmswbor(&self) -> PMMSWBOR_R {
        PMMSWBOR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PMM Software POR"]
    #[inline(always)]
    pub fn pmmswpor(&self) -> PMMSWPOR_R {
        PMMSWPOR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMM Turn Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&self) -> PMMREGOFF_R {
        PMMREGOFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - PMM Global High Power Module Request Enable"]
    #[inline(always)]
    pub fn pmmhpmre(&self) -> PMMHPMRE_R {
        PMMHPMRE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PMM Core Voltage Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn pmmcorev(&mut self) -> PMMCOREV_W<PMMCTL0_SPEC> {
        PMMCOREV_W::new(self, 0)
    }
    #[doc = "Bit 2 - PMM Software BOR"]
    #[inline(always)]
    #[must_use]
    pub fn pmmswbor(&mut self) -> PMMSWBOR_W<PMMCTL0_SPEC> {
        PMMSWBOR_W::new(self, 2)
    }
    #[doc = "Bit 3 - PMM Software POR"]
    #[inline(always)]
    #[must_use]
    pub fn pmmswpor(&mut self) -> PMMSWPOR_W<PMMCTL0_SPEC> {
        PMMSWPOR_W::new(self, 3)
    }
    #[doc = "Bit 4 - PMM Turn Regulator off"]
    #[inline(always)]
    #[must_use]
    pub fn pmmregoff(&mut self) -> PMMREGOFF_W<PMMCTL0_SPEC> {
        PMMREGOFF_W::new(self, 4)
    }
    #[doc = "Bit 7 - PMM Global High Power Module Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pmmhpmre(&mut self) -> PMMHPMRE_W<PMMCTL0_SPEC> {
        PMMHPMRE_W::new(self, 7)
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
#[doc = "PMM Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmmctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmmctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMMCTL0_SPEC;
impl crate::RegisterSpec for PMMCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pmmctl0::R`](R) reader structure"]
impl crate::Readable for PMMCTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmmctl0::W`](W) writer structure"]
impl crate::Writable for PMMCTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMMCTL0 to value 0"]
impl crate::Resettable for PMMCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
