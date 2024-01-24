#[doc = "Register `FC0_MIN_KHZ` reader"]
pub type R = crate::R<FC0_MIN_KHZ_SPEC>;
#[doc = "Register `FC0_MIN_KHZ` writer"]
pub type W = crate::W<FC0_MIN_KHZ_SPEC>;
#[doc = "Field `FC0_MIN_KHZ` reader - "]
pub type FC0_MIN_KHZ_R = crate::FieldReader<u32>;
#[doc = "Field `FC0_MIN_KHZ` writer - "]
pub type FC0_MIN_KHZ_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn fc0_min_khz(&self) -> FC0_MIN_KHZ_R {
        FC0_MIN_KHZ_R::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    #[must_use]
    pub fn fc0_min_khz(&mut self) -> FC0_MIN_KHZ_W<FC0_MIN_KHZ_SPEC> {
        FC0_MIN_KHZ_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Minimum pass frequency in kHz. This is optional. Set to 0 if you are not using the pass/fail flags  

You can [`read`](crate::generic::Reg::read) this register and get [`fc0_min_khz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc0_min_khz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FC0_MIN_KHZ_SPEC;
impl crate::RegisterSpec for FC0_MIN_KHZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc0_min_khz::R`](R) reader structure"]
impl crate::Readable for FC0_MIN_KHZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fc0_min_khz::W`](W) writer structure"]
impl crate::Writable for FC0_MIN_KHZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FC0_MIN_KHZ to value 0"]
impl crate::Resettable for FC0_MIN_KHZ_SPEC {
    const RESET_VALUE: u32 = 0;
}
