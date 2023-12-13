#[doc = "Register `REFCTL0` reader"]
pub type R = crate::R<REFCTL0_SPEC>;
#[doc = "Register `REFCTL0` writer"]
pub type W = crate::W<REFCTL0_SPEC>;
#[doc = "Field `REFON` reader - REF Reference On"]
pub type REFON_R = crate::BitReader;
#[doc = "Field `REFON` writer - REF Reference On"]
pub type REFON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFOUT` reader - REF Reference output Buffer On"]
pub type REFOUT_R = crate::BitReader;
#[doc = "Field `REFOUT` writer - REF Reference output Buffer On"]
pub type REFOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFTCOFF` reader - REF Temp.Sensor off"]
pub type REFTCOFF_R = crate::BitReader;
#[doc = "Field `REFTCOFF` writer - REF Temp.Sensor off"]
pub type REFTCOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFVSEL` reader - REF Reference Voltage Level Select Bit:0"]
pub type REFVSEL_R = crate::FieldReader<REFVSEL_A>;
#[doc = "REF Reference Voltage Level Select Bit:0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for REFVSEL_A {
    type Ux = u8;
}
impl REFVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REFVSEL_A {
        match self.bits {
            0 => REFVSEL_A::REFVSEL_0,
            1 => REFVSEL_A::REFVSEL_1,
            2 => REFVSEL_A::REFVSEL_2,
            3 => REFVSEL_A::REFVSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "REF Reference Voltage Level Select 1.5V"]
    #[inline(always)]
    pub fn is_refvsel_0(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_0
    }
    #[doc = "REF Reference Voltage Level Select 2.0V"]
    #[inline(always)]
    pub fn is_refvsel_1(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_1
    }
    #[doc = "REF Reference Voltage Level Select 2.5V"]
    #[inline(always)]
    pub fn is_refvsel_2(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_2
    }
    #[doc = "REF Reference Voltage Level Select 2.5V"]
    #[inline(always)]
    pub fn is_refvsel_3(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_3
    }
}
#[doc = "Field `REFVSEL` writer - REF Reference Voltage Level Select Bit:0"]
pub type REFVSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, REFVSEL_A>;
impl<'a, REG> REFVSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "REF Reference Voltage Level Select 1.5V"]
    #[inline(always)]
    pub fn refvsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(REFVSEL_A::REFVSEL_0)
    }
    #[doc = "REF Reference Voltage Level Select 2.0V"]
    #[inline(always)]
    pub fn refvsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(REFVSEL_A::REFVSEL_1)
    }
    #[doc = "REF Reference Voltage Level Select 2.5V"]
    #[inline(always)]
    pub fn refvsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(REFVSEL_A::REFVSEL_2)
    }
    #[doc = "REF Reference Voltage Level Select 2.5V"]
    #[inline(always)]
    pub fn refvsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(REFVSEL_A::REFVSEL_3)
    }
}
#[doc = "Field `REFMSTR` reader - REF Master Control"]
pub type REFMSTR_R = crate::BitReader;
#[doc = "Field `REFMSTR` writer - REF Master Control"]
pub type REFMSTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFGENACT` reader - REF Reference generator active"]
pub type REFGENACT_R = crate::BitReader;
#[doc = "Field `REFGENACT` writer - REF Reference generator active"]
pub type REFGENACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFBGACT` reader - REF Reference bandgap active"]
pub type REFBGACT_R = crate::BitReader;
#[doc = "Field `REFBGACT` writer - REF Reference bandgap active"]
pub type REFBGACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFGENBUSY` reader - REF Reference generator busy"]
pub type REFGENBUSY_R = crate::BitReader;
#[doc = "Field `REFGENBUSY` writer - REF Reference generator busy"]
pub type REFGENBUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BGMODE` reader - REF Bandgap mode"]
pub type BGMODE_R = crate::BitReader;
#[doc = "Field `BGMODE` writer - REF Bandgap mode"]
pub type BGMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REF Reference On"]
    #[inline(always)]
    pub fn refon(&self) -> REFON_R {
        REFON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REF Reference output Buffer On"]
    #[inline(always)]
    pub fn refout(&self) -> REFOUT_R {
        REFOUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - REF Temp.Sensor off"]
    #[inline(always)]
    pub fn reftcoff(&self) -> REFTCOFF_R {
        REFTCOFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - REF Reference Voltage Level Select Bit:0"]
    #[inline(always)]
    pub fn refvsel(&self) -> REFVSEL_R {
        REFVSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - REF Master Control"]
    #[inline(always)]
    pub fn refmstr(&self) -> REFMSTR_R {
        REFMSTR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - REF Reference generator active"]
    #[inline(always)]
    pub fn refgenact(&self) -> REFGENACT_R {
        REFGENACT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - REF Reference bandgap active"]
    #[inline(always)]
    pub fn refbgact(&self) -> REFBGACT_R {
        REFBGACT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REF Reference generator busy"]
    #[inline(always)]
    pub fn refgenbusy(&self) -> REFGENBUSY_R {
        REFGENBUSY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - REF Bandgap mode"]
    #[inline(always)]
    pub fn bgmode(&self) -> BGMODE_R {
        BGMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REF Reference On"]
    #[inline(always)]
    #[must_use]
    pub fn refon(&mut self) -> REFON_W<REFCTL0_SPEC> {
        REFON_W::new(self, 0)
    }
    #[doc = "Bit 1 - REF Reference output Buffer On"]
    #[inline(always)]
    #[must_use]
    pub fn refout(&mut self) -> REFOUT_W<REFCTL0_SPEC> {
        REFOUT_W::new(self, 1)
    }
    #[doc = "Bit 3 - REF Temp.Sensor off"]
    #[inline(always)]
    #[must_use]
    pub fn reftcoff(&mut self) -> REFTCOFF_W<REFCTL0_SPEC> {
        REFTCOFF_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - REF Reference Voltage Level Select Bit:0"]
    #[inline(always)]
    #[must_use]
    pub fn refvsel(&mut self) -> REFVSEL_W<REFCTL0_SPEC> {
        REFVSEL_W::new(self, 4)
    }
    #[doc = "Bit 7 - REF Master Control"]
    #[inline(always)]
    #[must_use]
    pub fn refmstr(&mut self) -> REFMSTR_W<REFCTL0_SPEC> {
        REFMSTR_W::new(self, 7)
    }
    #[doc = "Bit 8 - REF Reference generator active"]
    #[inline(always)]
    #[must_use]
    pub fn refgenact(&mut self) -> REFGENACT_W<REFCTL0_SPEC> {
        REFGENACT_W::new(self, 8)
    }
    #[doc = "Bit 9 - REF Reference bandgap active"]
    #[inline(always)]
    #[must_use]
    pub fn refbgact(&mut self) -> REFBGACT_W<REFCTL0_SPEC> {
        REFBGACT_W::new(self, 9)
    }
    #[doc = "Bit 10 - REF Reference generator busy"]
    #[inline(always)]
    #[must_use]
    pub fn refgenbusy(&mut self) -> REFGENBUSY_W<REFCTL0_SPEC> {
        REFGENBUSY_W::new(self, 10)
    }
    #[doc = "Bit 11 - REF Bandgap mode"]
    #[inline(always)]
    #[must_use]
    pub fn bgmode(&mut self) -> BGMODE_W<REFCTL0_SPEC> {
        BGMODE_W::new(self, 11)
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
#[doc = "REF Shared Reference control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`refctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`refctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REFCTL0_SPEC;
impl crate::RegisterSpec for REFCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`refctl0::R`](R) reader structure"]
impl crate::Readable for REFCTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`refctl0::W`](W) writer structure"]
impl crate::Writable for REFCTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REFCTL0 to value 0"]
impl crate::Resettable for REFCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
