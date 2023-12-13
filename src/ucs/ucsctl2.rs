#[doc = "Register `UCSCTL2` reader"]
pub type R = crate::R<UCSCTL2_SPEC>;
#[doc = "Register `UCSCTL2` writer"]
pub type W = crate::W<UCSCTL2_SPEC>;
#[doc = "Field `FLLN0` reader - FLL Multipier Bit : 0"]
pub type FLLN0_R = crate::BitReader;
#[doc = "Field `FLLN0` writer - FLL Multipier Bit : 0"]
pub type FLLN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLN1` reader - FLL Multipier Bit : 1"]
pub type FLLN1_R = crate::BitReader;
#[doc = "Field `FLLN1` writer - FLL Multipier Bit : 1"]
pub type FLLN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLN2` reader - FLL Multipier Bit : 2"]
pub type FLLN2_R = crate::BitReader;
#[doc = "Field `FLLN2` writer - FLL Multipier Bit : 2"]
pub type FLLN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLN3` reader - FLL Multipier Bit : 3"]
pub type FLLN3_R = crate::BitReader;
#[doc = "Field `FLLN3` writer - FLL Multipier Bit : 3"]
pub type FLLN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLN4` reader - FLL Multipier Bit : 4"]
pub type FLLN4_R = crate::BitReader;
#[doc = "Field `FLLN4` writer - FLL Multipier Bit : 4"]
pub type FLLN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLN5` reader - FLL Multipier Bit : 5"]
pub type FLLN5_R = crate::BitReader;
#[doc = "Field `FLLN5` writer - FLL Multipier Bit : 5"]
pub type FLLN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLN6` reader - FLL Multipier Bit : 6"]
pub type FLLN6_R = crate::BitReader;
#[doc = "Field `FLLN6` writer - FLL Multipier Bit : 6"]
pub type FLLN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLN7` reader - FLL Multipier Bit : 7"]
pub type FLLN7_R = crate::BitReader;
#[doc = "Field `FLLN7` writer - FLL Multipier Bit : 7"]
pub type FLLN7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLN8` reader - FLL Multipier Bit : 8"]
pub type FLLN8_R = crate::BitReader;
#[doc = "Field `FLLN8` writer - FLL Multipier Bit : 8"]
pub type FLLN8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLN9` reader - FLL Multipier Bit : 9"]
pub type FLLN9_R = crate::BitReader;
#[doc = "Field `FLLN9` writer - FLL Multipier Bit : 9"]
pub type FLLN9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLD` reader - Loop Divider Bit : 0"]
pub type FLLD_R = crate::FieldReader<FLLD_A>;
#[doc = "Loop Divider Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLLD_A {
    #[doc = "0: Multiply Selected Loop Freq. 1"]
    FLLD_0 = 0,
    #[doc = "1: Multiply Selected Loop Freq. 2"]
    FLLD_1 = 1,
    #[doc = "2: Multiply Selected Loop Freq. 4"]
    FLLD_2 = 2,
    #[doc = "3: Multiply Selected Loop Freq. 8"]
    FLLD_3 = 3,
    #[doc = "4: Multiply Selected Loop Freq. 16"]
    FLLD_4 = 4,
    #[doc = "5: Multiply Selected Loop Freq. 32"]
    FLLD_5 = 5,
    #[doc = "6: Multiply Selected Loop Freq. 32"]
    FLLD_6 = 6,
    #[doc = "7: Multiply Selected Loop Freq. 32"]
    FLLD_7 = 7,
}
impl From<FLLD_A> for u8 {
    #[inline(always)]
    fn from(variant: FLLD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FLLD_A {
    type Ux = u8;
}
impl FLLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLLD_A {
        match self.bits {
            0 => FLLD_A::FLLD_0,
            1 => FLLD_A::FLLD_1,
            2 => FLLD_A::FLLD_2,
            3 => FLLD_A::FLLD_3,
            4 => FLLD_A::FLLD_4,
            5 => FLLD_A::FLLD_5,
            6 => FLLD_A::FLLD_6,
            7 => FLLD_A::FLLD_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Multiply Selected Loop Freq. 1"]
    #[inline(always)]
    pub fn is_flld_0(&self) -> bool {
        *self == FLLD_A::FLLD_0
    }
    #[doc = "Multiply Selected Loop Freq. 2"]
    #[inline(always)]
    pub fn is_flld_1(&self) -> bool {
        *self == FLLD_A::FLLD_1
    }
    #[doc = "Multiply Selected Loop Freq. 4"]
    #[inline(always)]
    pub fn is_flld_2(&self) -> bool {
        *self == FLLD_A::FLLD_2
    }
    #[doc = "Multiply Selected Loop Freq. 8"]
    #[inline(always)]
    pub fn is_flld_3(&self) -> bool {
        *self == FLLD_A::FLLD_3
    }
    #[doc = "Multiply Selected Loop Freq. 16"]
    #[inline(always)]
    pub fn is_flld_4(&self) -> bool {
        *self == FLLD_A::FLLD_4
    }
    #[doc = "Multiply Selected Loop Freq. 32"]
    #[inline(always)]
    pub fn is_flld_5(&self) -> bool {
        *self == FLLD_A::FLLD_5
    }
    #[doc = "Multiply Selected Loop Freq. 32"]
    #[inline(always)]
    pub fn is_flld_6(&self) -> bool {
        *self == FLLD_A::FLLD_6
    }
    #[doc = "Multiply Selected Loop Freq. 32"]
    #[inline(always)]
    pub fn is_flld_7(&self) -> bool {
        *self == FLLD_A::FLLD_7
    }
}
#[doc = "Field `FLLD` writer - Loop Divider Bit : 0"]
pub type FLLD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FLLD_A>;
impl<'a, REG> FLLD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Multiply Selected Loop Freq. 1"]
    #[inline(always)]
    pub fn flld_0(self) -> &'a mut crate::W<REG> {
        self.variant(FLLD_A::FLLD_0)
    }
    #[doc = "Multiply Selected Loop Freq. 2"]
    #[inline(always)]
    pub fn flld_1(self) -> &'a mut crate::W<REG> {
        self.variant(FLLD_A::FLLD_1)
    }
    #[doc = "Multiply Selected Loop Freq. 4"]
    #[inline(always)]
    pub fn flld_2(self) -> &'a mut crate::W<REG> {
        self.variant(FLLD_A::FLLD_2)
    }
    #[doc = "Multiply Selected Loop Freq. 8"]
    #[inline(always)]
    pub fn flld_3(self) -> &'a mut crate::W<REG> {
        self.variant(FLLD_A::FLLD_3)
    }
    #[doc = "Multiply Selected Loop Freq. 16"]
    #[inline(always)]
    pub fn flld_4(self) -> &'a mut crate::W<REG> {
        self.variant(FLLD_A::FLLD_4)
    }
    #[doc = "Multiply Selected Loop Freq. 32"]
    #[inline(always)]
    pub fn flld_5(self) -> &'a mut crate::W<REG> {
        self.variant(FLLD_A::FLLD_5)
    }
    #[doc = "Multiply Selected Loop Freq. 32"]
    #[inline(always)]
    pub fn flld_6(self) -> &'a mut crate::W<REG> {
        self.variant(FLLD_A::FLLD_6)
    }
    #[doc = "Multiply Selected Loop Freq. 32"]
    #[inline(always)]
    pub fn flld_7(self) -> &'a mut crate::W<REG> {
        self.variant(FLLD_A::FLLD_7)
    }
}
impl R {
    #[doc = "Bit 0 - FLL Multipier Bit : 0"]
    #[inline(always)]
    pub fn flln0(&self) -> FLLN0_R {
        FLLN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLL Multipier Bit : 1"]
    #[inline(always)]
    pub fn flln1(&self) -> FLLN1_R {
        FLLN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FLL Multipier Bit : 2"]
    #[inline(always)]
    pub fn flln2(&self) -> FLLN2_R {
        FLLN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FLL Multipier Bit : 3"]
    #[inline(always)]
    pub fn flln3(&self) -> FLLN3_R {
        FLLN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FLL Multipier Bit : 4"]
    #[inline(always)]
    pub fn flln4(&self) -> FLLN4_R {
        FLLN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FLL Multipier Bit : 5"]
    #[inline(always)]
    pub fn flln5(&self) -> FLLN5_R {
        FLLN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FLL Multipier Bit : 6"]
    #[inline(always)]
    pub fn flln6(&self) -> FLLN6_R {
        FLLN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FLL Multipier Bit : 7"]
    #[inline(always)]
    pub fn flln7(&self) -> FLLN7_R {
        FLLN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FLL Multipier Bit : 8"]
    #[inline(always)]
    pub fn flln8(&self) -> FLLN8_R {
        FLLN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FLL Multipier Bit : 9"]
    #[inline(always)]
    pub fn flln9(&self) -> FLLN9_R {
        FLLN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Loop Divider Bit : 0"]
    #[inline(always)]
    pub fn flld(&self) -> FLLD_R {
        FLLD_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FLL Multipier Bit : 0"]
    #[inline(always)]
    #[must_use]
    pub fn flln0(&mut self) -> FLLN0_W<UCSCTL2_SPEC> {
        FLLN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - FLL Multipier Bit : 1"]
    #[inline(always)]
    #[must_use]
    pub fn flln1(&mut self) -> FLLN1_W<UCSCTL2_SPEC> {
        FLLN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - FLL Multipier Bit : 2"]
    #[inline(always)]
    #[must_use]
    pub fn flln2(&mut self) -> FLLN2_W<UCSCTL2_SPEC> {
        FLLN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - FLL Multipier Bit : 3"]
    #[inline(always)]
    #[must_use]
    pub fn flln3(&mut self) -> FLLN3_W<UCSCTL2_SPEC> {
        FLLN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - FLL Multipier Bit : 4"]
    #[inline(always)]
    #[must_use]
    pub fn flln4(&mut self) -> FLLN4_W<UCSCTL2_SPEC> {
        FLLN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - FLL Multipier Bit : 5"]
    #[inline(always)]
    #[must_use]
    pub fn flln5(&mut self) -> FLLN5_W<UCSCTL2_SPEC> {
        FLLN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - FLL Multipier Bit : 6"]
    #[inline(always)]
    #[must_use]
    pub fn flln6(&mut self) -> FLLN6_W<UCSCTL2_SPEC> {
        FLLN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - FLL Multipier Bit : 7"]
    #[inline(always)]
    #[must_use]
    pub fn flln7(&mut self) -> FLLN7_W<UCSCTL2_SPEC> {
        FLLN7_W::new(self, 7)
    }
    #[doc = "Bit 8 - FLL Multipier Bit : 8"]
    #[inline(always)]
    #[must_use]
    pub fn flln8(&mut self) -> FLLN8_W<UCSCTL2_SPEC> {
        FLLN8_W::new(self, 8)
    }
    #[doc = "Bit 9 - FLL Multipier Bit : 9"]
    #[inline(always)]
    #[must_use]
    pub fn flln9(&mut self) -> FLLN9_W<UCSCTL2_SPEC> {
        FLLN9_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Loop Divider Bit : 0"]
    #[inline(always)]
    #[must_use]
    pub fn flld(&mut self) -> FLLD_W<UCSCTL2_SPEC> {
        FLLD_W::new(self, 12)
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
#[doc = "UCS Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSCTL2_SPEC;
impl crate::RegisterSpec for UCSCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucsctl2::R`](R) reader structure"]
impl crate::Readable for UCSCTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsctl2::W`](W) writer structure"]
impl crate::Writable for UCSCTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSCTL2 to value 0"]
impl crate::Resettable for UCSCTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
