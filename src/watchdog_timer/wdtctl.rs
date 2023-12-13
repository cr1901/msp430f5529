#[doc = "Register `WDTCTL` reader"]
pub type R = crate::R<WDTCTL_SPEC>;
#[doc = "Register `WDTCTL` writer"]
pub type W = crate::W<WDTCTL_SPEC>;
#[doc = "Field `WDTIS` reader - WDT - Timer Interval Select 0"]
pub type WDTIS_R = crate::FieldReader;
#[doc = "Field `WDTIS` writer - WDT - Timer Interval Select 0"]
pub type WDTIS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDTCNTCL` reader - WDT - Timer Clear"]
pub type WDTCNTCL_R = crate::BitReader;
#[doc = "Field `WDTCNTCL` writer - WDT - Timer Clear"]
pub type WDTCNTCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTTMSEL` reader - WDT - Timer Mode Select"]
pub type WDTTMSEL_R = crate::BitReader;
#[doc = "Field `WDTTMSEL` writer - WDT - Timer Mode Select"]
pub type WDTTMSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTSSEL` reader - WDT - Timer Clock Source Select 0"]
pub type WDTSSEL_R = crate::FieldReader<WDTSSEL_A>;
#[doc = "WDT - Timer Clock Source Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDTSSEL_A {
    #[doc = "0: WDT - Timer Clock Source Select: SMCLK"]
    WDTSSEL_0 = 0,
    #[doc = "1: WDT - Timer Clock Source Select: ACLK"]
    WDTSSEL_1 = 1,
    #[doc = "2: WDT - Timer Clock Source Select: VLO_CLK"]
    WDTSSEL_2 = 2,
    #[doc = "3: WDT - Timer Clock Source Select: reserved"]
    WDTSSEL_3 = 3,
}
impl From<WDTSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDTSSEL_A {
    type Ux = u8;
}
impl WDTSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDTSSEL_A {
        match self.bits {
            0 => WDTSSEL_A::WDTSSEL_0,
            1 => WDTSSEL_A::WDTSSEL_1,
            2 => WDTSSEL_A::WDTSSEL_2,
            3 => WDTSSEL_A::WDTSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "WDT - Timer Clock Source Select: SMCLK"]
    #[inline(always)]
    pub fn is_wdtssel_0(&self) -> bool {
        *self == WDTSSEL_A::WDTSSEL_0
    }
    #[doc = "WDT - Timer Clock Source Select: ACLK"]
    #[inline(always)]
    pub fn is_wdtssel_1(&self) -> bool {
        *self == WDTSSEL_A::WDTSSEL_1
    }
    #[doc = "WDT - Timer Clock Source Select: VLO_CLK"]
    #[inline(always)]
    pub fn is_wdtssel_2(&self) -> bool {
        *self == WDTSSEL_A::WDTSSEL_2
    }
    #[doc = "WDT - Timer Clock Source Select: reserved"]
    #[inline(always)]
    pub fn is_wdtssel_3(&self) -> bool {
        *self == WDTSSEL_A::WDTSSEL_3
    }
}
#[doc = "Field `WDTSSEL` writer - WDT - Timer Clock Source Select 0"]
pub type WDTSSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, WDTSSEL_A>;
impl<'a, REG> WDTSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "WDT - Timer Clock Source Select: SMCLK"]
    #[inline(always)]
    pub fn wdtssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(WDTSSEL_A::WDTSSEL_0)
    }
    #[doc = "WDT - Timer Clock Source Select: ACLK"]
    #[inline(always)]
    pub fn wdtssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTSSEL_A::WDTSSEL_1)
    }
    #[doc = "WDT - Timer Clock Source Select: VLO_CLK"]
    #[inline(always)]
    pub fn wdtssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(WDTSSEL_A::WDTSSEL_2)
    }
    #[doc = "WDT - Timer Clock Source Select: reserved"]
    #[inline(always)]
    pub fn wdtssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(WDTSSEL_A::WDTSSEL_3)
    }
}
#[doc = "Field `WDTHOLD` reader - WDT - Timer hold"]
pub type WDTHOLD_R = crate::BitReader;
#[doc = "Field `WDTHOLD` writer - WDT - Timer hold"]
pub type WDTHOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTPW` reader - Watchdog Timer Password"]
pub type WDTPW_R = crate::FieldReader<WDTPWR_A>;
#[doc = "Watchdog Timer Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDTPWR_A {
    #[doc = "105: Value always read from the Watchdog Password register"]
    PASSWORD = 105,
}
impl From<WDTPWR_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTPWR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDTPWR_A {
    type Ux = u8;
}
impl WDTPW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WDTPWR_A> {
        match self.bits {
            105 => Some(WDTPWR_A::PASSWORD),
            _ => None,
        }
    }
    #[doc = "Value always read from the Watchdog Password register"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == WDTPWR_A::PASSWORD
    }
}
#[doc = "Watchdog Timer Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDTPWW_AW {
    #[doc = "90: Value which must be written to the Watchdog Password register"]
    PASSWORD = 90,
}
impl From<WDTPWW_AW> for u8 {
    #[inline(always)]
    fn from(variant: WDTPWW_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDTPWW_AW {
    type Ux = u8;
}
#[doc = "Field `WDTPW` writer - Watchdog Timer Password"]
pub type WDTPW_W<'a, REG> = crate::FieldWriter<'a, REG, 8, WDTPWW_AW>;
impl<'a, REG> WDTPW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value which must be written to the Watchdog Password register"]
    #[inline(always)]
    pub fn password(self) -> &'a mut crate::W<REG> {
        self.variant(WDTPWW_AW::PASSWORD)
    }
}
impl R {
    #[doc = "Bits 0:2 - WDT - Timer Interval Select 0"]
    #[inline(always)]
    pub fn wdtis(&self) -> WDTIS_R {
        WDTIS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - WDT - Timer Clear"]
    #[inline(always)]
    pub fn wdtcntcl(&self) -> WDTCNTCL_R {
        WDTCNTCL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WDT - Timer Mode Select"]
    #[inline(always)]
    pub fn wdttmsel(&self) -> WDTTMSEL_R {
        WDTTMSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - WDT - Timer Clock Source Select 0"]
    #[inline(always)]
    pub fn wdtssel(&self) -> WDTSSEL_R {
        WDTSSEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - WDT - Timer hold"]
    #[inline(always)]
    pub fn wdthold(&self) -> WDTHOLD_R {
        WDTHOLD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Watchdog Timer Password"]
    #[inline(always)]
    pub fn wdtpw(&self) -> WDTPW_R {
        WDTPW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - WDT - Timer Interval Select 0"]
    #[inline(always)]
    #[must_use]
    pub fn wdtis(&mut self) -> WDTIS_W<WDTCTL_SPEC> {
        WDTIS_W::new(self, 0)
    }
    #[doc = "Bit 3 - WDT - Timer Clear"]
    #[inline(always)]
    #[must_use]
    pub fn wdtcntcl(&mut self) -> WDTCNTCL_W<WDTCTL_SPEC> {
        WDTCNTCL_W::new(self, 3)
    }
    #[doc = "Bit 4 - WDT - Timer Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn wdttmsel(&mut self) -> WDTTMSEL_W<WDTCTL_SPEC> {
        WDTTMSEL_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - WDT - Timer Clock Source Select 0"]
    #[inline(always)]
    #[must_use]
    pub fn wdtssel(&mut self) -> WDTSSEL_W<WDTCTL_SPEC> {
        WDTSSEL_W::new(self, 5)
    }
    #[doc = "Bit 7 - WDT - Timer hold"]
    #[inline(always)]
    #[must_use]
    pub fn wdthold(&mut self) -> WDTHOLD_W<WDTCTL_SPEC> {
        WDTHOLD_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - Watchdog Timer Password"]
    #[inline(always)]
    #[must_use]
    pub fn wdtpw(&mut self) -> WDTPW_W<WDTCTL_SPEC> {
        WDTPW_W::new(self, 8)
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
#[doc = "Watchdog Timer Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTCTL_SPEC;
impl crate::RegisterSpec for WDTCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wdtctl::R`](R) reader structure"]
impl crate::Readable for WDTCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtctl::W`](W) writer structure"]
impl crate::Writable for WDTCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCTL to value 0"]
impl crate::Resettable for WDTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
