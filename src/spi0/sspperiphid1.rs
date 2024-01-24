#[doc = "Register `SSPPERIPHID1` reader"]
pub type R = crate::R<SSPPERIPHID1_SPEC>;
#[doc = "Field `PARTNUMBER1` reader - These bits read back as 0x0"]
pub type PARTNUMBER1_R = crate::FieldReader;
#[doc = "Field `DESIGNER0` reader - These bits read back as 0x1"]
pub type DESIGNER0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - These bits read back as 0x0"]
    #[inline(always)]
    pub fn partnumber1(&self) -> PARTNUMBER1_R {
        PARTNUMBER1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - These bits read back as 0x1"]
    #[inline(always)]
    pub fn designer0(&self) -> DESIGNER0_R {
        DESIGNER0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13  

You can [`read`](crate::generic::Reg::read) this register and get [`sspperiphid1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPPERIPHID1_SPEC;
impl crate::RegisterSpec for SSPPERIPHID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspperiphid1::R`](R) reader structure"]
impl crate::Readable for SSPPERIPHID1_SPEC {}
#[doc = "`reset()` method sets SSPPERIPHID1 to value 0x10"]
impl crate::Resettable for SSPPERIPHID1_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
