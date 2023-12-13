#[doc = "Register `DMA1CTL` reader"]
pub type R = crate::R<DMA1CTL_SPEC>;
#[doc = "Register `DMA1CTL` writer"]
pub type W = crate::W<DMA1CTL_SPEC>;
#[doc = "Field `DMAREQ` reader - Initiate DMA transfer with DMATSEL"]
pub type DMAREQ_R = crate::BitReader;
#[doc = "Field `DMAREQ` writer - Initiate DMA transfer with DMATSEL"]
pub type DMAREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAABORT` reader - DMA transfer aborted by NMI"]
pub type DMAABORT_R = crate::BitReader;
#[doc = "Field `DMAABORT` writer - DMA transfer aborted by NMI"]
pub type DMAABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIE` reader - DMA interrupt enable"]
pub type DMAIE_R = crate::BitReader;
#[doc = "Field `DMAIE` writer - DMA interrupt enable"]
pub type DMAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIFG` reader - DMA interrupt flag"]
pub type DMAIFG_R = crate::BitReader;
#[doc = "Field `DMAIFG` writer - DMA interrupt flag"]
pub type DMAIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMALEVEL` reader - DMA level sensitive trigger select"]
pub type DMALEVEL_R = crate::BitReader;
#[doc = "Field `DMALEVEL` writer - DMA level sensitive trigger select"]
pub type DMALEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASRCBYTE` reader - DMA source byte"]
pub type DMASRCBYTE_R = crate::BitReader;
#[doc = "Field `DMASRCBYTE` writer - DMA source byte"]
pub type DMASRCBYTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMADSTBYTE` reader - DMA destination byte"]
pub type DMADSTBYTE_R = crate::BitReader;
#[doc = "Field `DMADSTBYTE` writer - DMA destination byte"]
pub type DMADSTBYTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASRCINCR` reader - DMA source increment bit 0"]
pub type DMASRCINCR_R = crate::FieldReader<DMASRCINCR_A>;
#[doc = "DMA source increment bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMASRCINCR_A {
    #[doc = "0: DMA source increment 0: source address unchanged"]
    DMASRCINCR_0 = 0,
    #[doc = "1: DMA source increment 1: source address unchanged"]
    DMASRCINCR_1 = 1,
    #[doc = "2: DMA source increment 2: source address decremented"]
    DMASRCINCR_2 = 2,
    #[doc = "3: DMA source increment 3: source address incremented"]
    DMASRCINCR_3 = 3,
}
impl From<DMASRCINCR_A> for u8 {
    #[inline(always)]
    fn from(variant: DMASRCINCR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMASRCINCR_A {
    type Ux = u8;
}
impl DMASRCINCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMASRCINCR_A {
        match self.bits {
            0 => DMASRCINCR_A::DMASRCINCR_0,
            1 => DMASRCINCR_A::DMASRCINCR_1,
            2 => DMASRCINCR_A::DMASRCINCR_2,
            3 => DMASRCINCR_A::DMASRCINCR_3,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA source increment 0: source address unchanged"]
    #[inline(always)]
    pub fn is_dmasrcincr_0(&self) -> bool {
        *self == DMASRCINCR_A::DMASRCINCR_0
    }
    #[doc = "DMA source increment 1: source address unchanged"]
    #[inline(always)]
    pub fn is_dmasrcincr_1(&self) -> bool {
        *self == DMASRCINCR_A::DMASRCINCR_1
    }
    #[doc = "DMA source increment 2: source address decremented"]
    #[inline(always)]
    pub fn is_dmasrcincr_2(&self) -> bool {
        *self == DMASRCINCR_A::DMASRCINCR_2
    }
    #[doc = "DMA source increment 3: source address incremented"]
    #[inline(always)]
    pub fn is_dmasrcincr_3(&self) -> bool {
        *self == DMASRCINCR_A::DMASRCINCR_3
    }
}
#[doc = "Field `DMASRCINCR` writer - DMA source increment bit 0"]
pub type DMASRCINCR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DMASRCINCR_A>;
impl<'a, REG> DMASRCINCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA source increment 0: source address unchanged"]
    #[inline(always)]
    pub fn dmasrcincr_0(self) -> &'a mut crate::W<REG> {
        self.variant(DMASRCINCR_A::DMASRCINCR_0)
    }
    #[doc = "DMA source increment 1: source address unchanged"]
    #[inline(always)]
    pub fn dmasrcincr_1(self) -> &'a mut crate::W<REG> {
        self.variant(DMASRCINCR_A::DMASRCINCR_1)
    }
    #[doc = "DMA source increment 2: source address decremented"]
    #[inline(always)]
    pub fn dmasrcincr_2(self) -> &'a mut crate::W<REG> {
        self.variant(DMASRCINCR_A::DMASRCINCR_2)
    }
    #[doc = "DMA source increment 3: source address incremented"]
    #[inline(always)]
    pub fn dmasrcincr_3(self) -> &'a mut crate::W<REG> {
        self.variant(DMASRCINCR_A::DMASRCINCR_3)
    }
}
#[doc = "Field `DMADSTINCR` reader - DMA destination increment bit 0"]
pub type DMADSTINCR_R = crate::FieldReader<DMADSTINCR_A>;
#[doc = "DMA destination increment bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMADSTINCR_A {
    #[doc = "0: DMA destination increment 0: destination address unchanged"]
    DMADSTINCR_0 = 0,
    #[doc = "1: DMA destination increment 1: destination address unchanged"]
    DMADSTINCR_1 = 1,
    #[doc = "2: DMA destination increment 2: destination address decremented"]
    DMADSTINCR_2 = 2,
    #[doc = "3: DMA destination increment 3: destination address incremented"]
    DMADSTINCR_3 = 3,
}
impl From<DMADSTINCR_A> for u8 {
    #[inline(always)]
    fn from(variant: DMADSTINCR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMADSTINCR_A {
    type Ux = u8;
}
impl DMADSTINCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMADSTINCR_A {
        match self.bits {
            0 => DMADSTINCR_A::DMADSTINCR_0,
            1 => DMADSTINCR_A::DMADSTINCR_1,
            2 => DMADSTINCR_A::DMADSTINCR_2,
            3 => DMADSTINCR_A::DMADSTINCR_3,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA destination increment 0: destination address unchanged"]
    #[inline(always)]
    pub fn is_dmadstincr_0(&self) -> bool {
        *self == DMADSTINCR_A::DMADSTINCR_0
    }
    #[doc = "DMA destination increment 1: destination address unchanged"]
    #[inline(always)]
    pub fn is_dmadstincr_1(&self) -> bool {
        *self == DMADSTINCR_A::DMADSTINCR_1
    }
    #[doc = "DMA destination increment 2: destination address decremented"]
    #[inline(always)]
    pub fn is_dmadstincr_2(&self) -> bool {
        *self == DMADSTINCR_A::DMADSTINCR_2
    }
    #[doc = "DMA destination increment 3: destination address incremented"]
    #[inline(always)]
    pub fn is_dmadstincr_3(&self) -> bool {
        *self == DMADSTINCR_A::DMADSTINCR_3
    }
}
#[doc = "Field `DMADSTINCR` writer - DMA destination increment bit 0"]
pub type DMADSTINCR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DMADSTINCR_A>;
impl<'a, REG> DMADSTINCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA destination increment 0: destination address unchanged"]
    #[inline(always)]
    pub fn dmadstincr_0(self) -> &'a mut crate::W<REG> {
        self.variant(DMADSTINCR_A::DMADSTINCR_0)
    }
    #[doc = "DMA destination increment 1: destination address unchanged"]
    #[inline(always)]
    pub fn dmadstincr_1(self) -> &'a mut crate::W<REG> {
        self.variant(DMADSTINCR_A::DMADSTINCR_1)
    }
    #[doc = "DMA destination increment 2: destination address decremented"]
    #[inline(always)]
    pub fn dmadstincr_2(self) -> &'a mut crate::W<REG> {
        self.variant(DMADSTINCR_A::DMADSTINCR_2)
    }
    #[doc = "DMA destination increment 3: destination address incremented"]
    #[inline(always)]
    pub fn dmadstincr_3(self) -> &'a mut crate::W<REG> {
        self.variant(DMADSTINCR_A::DMADSTINCR_3)
    }
}
#[doc = "Field `DMADT` reader - DMA transfer mode bit 0"]
pub type DMADT_R = crate::FieldReader<DMADT_A>;
#[doc = "DMA transfer mode bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMADT_A {
    #[doc = "0: DMA transfer mode 0: Single transfer"]
    DMADT_0 = 0,
    #[doc = "1: DMA transfer mode 1: Block transfer"]
    DMADT_1 = 1,
    #[doc = "2: DMA transfer mode 2: Burst-Block transfer"]
    DMADT_2 = 2,
    #[doc = "3: DMA transfer mode 3: Burst-Block transfer"]
    DMADT_3 = 3,
    #[doc = "4: DMA transfer mode 4: Repeated Single transfer"]
    DMADT_4 = 4,
    #[doc = "5: DMA transfer mode 5: Repeated Block transfer"]
    DMADT_5 = 5,
    #[doc = "6: DMA transfer mode 6: Repeated Burst-Block transfer"]
    DMADT_6 = 6,
    #[doc = "7: DMA transfer mode 7: Repeated Burst-Block transfer"]
    DMADT_7 = 7,
}
impl From<DMADT_A> for u8 {
    #[inline(always)]
    fn from(variant: DMADT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMADT_A {
    type Ux = u8;
}
impl DMADT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMADT_A {
        match self.bits {
            0 => DMADT_A::DMADT_0,
            1 => DMADT_A::DMADT_1,
            2 => DMADT_A::DMADT_2,
            3 => DMADT_A::DMADT_3,
            4 => DMADT_A::DMADT_4,
            5 => DMADT_A::DMADT_5,
            6 => DMADT_A::DMADT_6,
            7 => DMADT_A::DMADT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA transfer mode 0: Single transfer"]
    #[inline(always)]
    pub fn is_dmadt_0(&self) -> bool {
        *self == DMADT_A::DMADT_0
    }
    #[doc = "DMA transfer mode 1: Block transfer"]
    #[inline(always)]
    pub fn is_dmadt_1(&self) -> bool {
        *self == DMADT_A::DMADT_1
    }
    #[doc = "DMA transfer mode 2: Burst-Block transfer"]
    #[inline(always)]
    pub fn is_dmadt_2(&self) -> bool {
        *self == DMADT_A::DMADT_2
    }
    #[doc = "DMA transfer mode 3: Burst-Block transfer"]
    #[inline(always)]
    pub fn is_dmadt_3(&self) -> bool {
        *self == DMADT_A::DMADT_3
    }
    #[doc = "DMA transfer mode 4: Repeated Single transfer"]
    #[inline(always)]
    pub fn is_dmadt_4(&self) -> bool {
        *self == DMADT_A::DMADT_4
    }
    #[doc = "DMA transfer mode 5: Repeated Block transfer"]
    #[inline(always)]
    pub fn is_dmadt_5(&self) -> bool {
        *self == DMADT_A::DMADT_5
    }
    #[doc = "DMA transfer mode 6: Repeated Burst-Block transfer"]
    #[inline(always)]
    pub fn is_dmadt_6(&self) -> bool {
        *self == DMADT_A::DMADT_6
    }
    #[doc = "DMA transfer mode 7: Repeated Burst-Block transfer"]
    #[inline(always)]
    pub fn is_dmadt_7(&self) -> bool {
        *self == DMADT_A::DMADT_7
    }
}
#[doc = "Field `DMADT` writer - DMA transfer mode bit 0"]
pub type DMADT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, DMADT_A>;
impl<'a, REG> DMADT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA transfer mode 0: Single transfer"]
    #[inline(always)]
    pub fn dmadt_0(self) -> &'a mut crate::W<REG> {
        self.variant(DMADT_A::DMADT_0)
    }
    #[doc = "DMA transfer mode 1: Block transfer"]
    #[inline(always)]
    pub fn dmadt_1(self) -> &'a mut crate::W<REG> {
        self.variant(DMADT_A::DMADT_1)
    }
    #[doc = "DMA transfer mode 2: Burst-Block transfer"]
    #[inline(always)]
    pub fn dmadt_2(self) -> &'a mut crate::W<REG> {
        self.variant(DMADT_A::DMADT_2)
    }
    #[doc = "DMA transfer mode 3: Burst-Block transfer"]
    #[inline(always)]
    pub fn dmadt_3(self) -> &'a mut crate::W<REG> {
        self.variant(DMADT_A::DMADT_3)
    }
    #[doc = "DMA transfer mode 4: Repeated Single transfer"]
    #[inline(always)]
    pub fn dmadt_4(self) -> &'a mut crate::W<REG> {
        self.variant(DMADT_A::DMADT_4)
    }
    #[doc = "DMA transfer mode 5: Repeated Block transfer"]
    #[inline(always)]
    pub fn dmadt_5(self) -> &'a mut crate::W<REG> {
        self.variant(DMADT_A::DMADT_5)
    }
    #[doc = "DMA transfer mode 6: Repeated Burst-Block transfer"]
    #[inline(always)]
    pub fn dmadt_6(self) -> &'a mut crate::W<REG> {
        self.variant(DMADT_A::DMADT_6)
    }
    #[doc = "DMA transfer mode 7: Repeated Burst-Block transfer"]
    #[inline(always)]
    pub fn dmadt_7(self) -> &'a mut crate::W<REG> {
        self.variant(DMADT_A::DMADT_7)
    }
}
impl R {
    #[doc = "Bit 0 - Initiate DMA transfer with DMATSEL"]
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA transfer aborted by NMI"]
    #[inline(always)]
    pub fn dmaabort(&self) -> DMAABORT_R {
        DMAABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA interrupt enable"]
    #[inline(always)]
    pub fn dmaie(&self) -> DMAIE_R {
        DMAIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA interrupt flag"]
    #[inline(always)]
    pub fn dmaifg(&self) -> DMAIFG_R {
        DMAIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA level sensitive trigger select"]
    #[inline(always)]
    pub fn dmalevel(&self) -> DMALEVEL_R {
        DMALEVEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA source byte"]
    #[inline(always)]
    pub fn dmasrcbyte(&self) -> DMASRCBYTE_R {
        DMASRCBYTE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA destination byte"]
    #[inline(always)]
    pub fn dmadstbyte(&self) -> DMADSTBYTE_R {
        DMADSTBYTE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - DMA source increment bit 0"]
    #[inline(always)]
    pub fn dmasrcincr(&self) -> DMASRCINCR_R {
        DMASRCINCR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - DMA destination increment bit 0"]
    #[inline(always)]
    pub fn dmadstincr(&self) -> DMADSTINCR_R {
        DMADSTINCR_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - DMA transfer mode bit 0"]
    #[inline(always)]
    pub fn dmadt(&self) -> DMADT_R {
        DMADT_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Initiate DMA transfer with DMATSEL"]
    #[inline(always)]
    #[must_use]
    pub fn dmareq(&mut self) -> DMAREQ_W<DMA1CTL_SPEC> {
        DMAREQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA transfer aborted by NMI"]
    #[inline(always)]
    #[must_use]
    pub fn dmaabort(&mut self) -> DMAABORT_W<DMA1CTL_SPEC> {
        DMAABORT_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaie(&mut self) -> DMAIE_W<DMA1CTL_SPEC> {
        DMAIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmaifg(&mut self) -> DMAIFG_W<DMA1CTL_SPEC> {
        DMAIFG_W::new(self, 3)
    }
    #[doc = "Bit 4 - DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<DMA1CTL_SPEC> {
        DMAEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - DMA level sensitive trigger select"]
    #[inline(always)]
    #[must_use]
    pub fn dmalevel(&mut self) -> DMALEVEL_W<DMA1CTL_SPEC> {
        DMALEVEL_W::new(self, 5)
    }
    #[doc = "Bit 6 - DMA source byte"]
    #[inline(always)]
    #[must_use]
    pub fn dmasrcbyte(&mut self) -> DMASRCBYTE_W<DMA1CTL_SPEC> {
        DMASRCBYTE_W::new(self, 6)
    }
    #[doc = "Bit 7 - DMA destination byte"]
    #[inline(always)]
    #[must_use]
    pub fn dmadstbyte(&mut self) -> DMADSTBYTE_W<DMA1CTL_SPEC> {
        DMADSTBYTE_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - DMA source increment bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dmasrcincr(&mut self) -> DMASRCINCR_W<DMA1CTL_SPEC> {
        DMASRCINCR_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - DMA destination increment bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dmadstincr(&mut self) -> DMADSTINCR_W<DMA1CTL_SPEC> {
        DMADSTINCR_W::new(self, 10)
    }
    #[doc = "Bits 12:14 - DMA transfer mode bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dmadt(&mut self) -> DMADT_W<DMA1CTL_SPEC> {
        DMADT_W::new(self, 12)
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
#[doc = "DMA Channel 1 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma1ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma1ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA1CTL_SPEC;
impl crate::RegisterSpec for DMA1CTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dma1ctl::R`](R) reader structure"]
impl crate::Readable for DMA1CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma1ctl::W`](W) writer structure"]
impl crate::Writable for DMA1CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA1CTL to value 0"]
impl crate::Resettable for DMA1CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
