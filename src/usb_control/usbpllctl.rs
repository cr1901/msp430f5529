#[doc = "Register `USBPLLCTL` reader"]
pub type R = crate::R<USBPLLCTL_SPEC>;
#[doc = "Register `USBPLLCTL` writer"]
pub type W = crate::W<USBPLLCTL_SPEC>;
#[doc = "Field `UCLKSEL` reader - USB - Module Clock Select Bit 0"]
pub type UCLKSEL_R = crate::FieldReader<UCLKSEL_A>;
#[doc = "USB - Module Clock Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCLKSEL_A {
    #[doc = "0: USB - Module Clock Select: 0"]
    UCLKSEL_0 = 0,
    #[doc = "1: USB - Module Clock Select: 1"]
    UCLKSEL_1 = 1,
    #[doc = "2: USB - Module Clock Select: 2"]
    UCLKSEL_2 = 2,
    #[doc = "3: USB - Module Clock Select: 3 (Reserved)"]
    UCLKSEL_3 = 3,
}
impl From<UCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UCLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UCLKSEL_A {
    type Ux = u8;
}
impl UCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UCLKSEL_A {
        match self.bits {
            0 => UCLKSEL_A::UCLKSEL_0,
            1 => UCLKSEL_A::UCLKSEL_1,
            2 => UCLKSEL_A::UCLKSEL_2,
            3 => UCLKSEL_A::UCLKSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "USB - Module Clock Select: 0"]
    #[inline(always)]
    pub fn is_uclksel_0(&self) -> bool {
        *self == UCLKSEL_A::UCLKSEL_0
    }
    #[doc = "USB - Module Clock Select: 1"]
    #[inline(always)]
    pub fn is_uclksel_1(&self) -> bool {
        *self == UCLKSEL_A::UCLKSEL_1
    }
    #[doc = "USB - Module Clock Select: 2"]
    #[inline(always)]
    pub fn is_uclksel_2(&self) -> bool {
        *self == UCLKSEL_A::UCLKSEL_2
    }
    #[doc = "USB - Module Clock Select: 3 (Reserved)"]
    #[inline(always)]
    pub fn is_uclksel_3(&self) -> bool {
        *self == UCLKSEL_A::UCLKSEL_3
    }
}
#[doc = "Field `UCLKSEL` writer - USB - Module Clock Select Bit 0"]
pub type UCLKSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, UCLKSEL_A>;
impl<'a, REG> UCLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USB - Module Clock Select: 0"]
    #[inline(always)]
    pub fn uclksel_0(self) -> &'a mut crate::W<REG> {
        self.variant(UCLKSEL_A::UCLKSEL_0)
    }
    #[doc = "USB - Module Clock Select: 1"]
    #[inline(always)]
    pub fn uclksel_1(self) -> &'a mut crate::W<REG> {
        self.variant(UCLKSEL_A::UCLKSEL_1)
    }
    #[doc = "USB - Module Clock Select: 2"]
    #[inline(always)]
    pub fn uclksel_2(self) -> &'a mut crate::W<REG> {
        self.variant(UCLKSEL_A::UCLKSEL_2)
    }
    #[doc = "USB - Module Clock Select: 3 (Reserved)"]
    #[inline(always)]
    pub fn uclksel_3(self) -> &'a mut crate::W<REG> {
        self.variant(UCLKSEL_A::UCLKSEL_3)
    }
}
#[doc = "Field `UPLLEN` reader - USB - PLL enable"]
pub type UPLLEN_R = crate::BitReader;
#[doc = "Field `UPLLEN` writer - USB - PLL enable"]
pub type UPLLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPFDEN` reader - USB - Phase Freq. Discriminator enable"]
pub type UPFDEN_R = crate::BitReader;
#[doc = "Field `UPFDEN` writer - USB - Phase Freq. Discriminator enable"]
pub type UPFDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 6:7 - USB - Module Clock Select Bit 0"]
    #[inline(always)]
    pub fn uclksel(&self) -> UCLKSEL_R {
        UCLKSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - USB - PLL enable"]
    #[inline(always)]
    pub fn upllen(&self) -> UPLLEN_R {
        UPLLEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USB - Phase Freq. Discriminator enable"]
    #[inline(always)]
    pub fn upfden(&self) -> UPFDEN_R {
        UPFDEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 6:7 - USB - Module Clock Select Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn uclksel(&mut self) -> UCLKSEL_W<USBPLLCTL_SPEC> {
        UCLKSEL_W::new(self, 6)
    }
    #[doc = "Bit 8 - USB - PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn upllen(&mut self) -> UPLLEN_W<USBPLLCTL_SPEC> {
        UPLLEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - USB - Phase Freq. Discriminator enable"]
    #[inline(always)]
    #[must_use]
    pub fn upfden(&mut self) -> UPFDEN_W<USBPLLCTL_SPEC> {
        UPFDEN_W::new(self, 9)
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
#[doc = "USB PLL control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbpllctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbpllctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBPLLCTL_SPEC;
impl crate::RegisterSpec for USBPLLCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usbpllctl::R`](R) reader structure"]
impl crate::Readable for USBPLLCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbpllctl::W`](W) writer structure"]
impl crate::Writable for USBPLLCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBPLLCTL to value 0"]
impl crate::Resettable for USBPLLCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
