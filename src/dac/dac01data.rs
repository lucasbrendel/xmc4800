#[doc = "Reader of register DAC01DATA"]
pub type R = crate::R<u32, super::DAC01DATA>;
#[doc = "Writer for register DAC01DATA"]
pub type W = crate::W<u32, super::DAC01DATA>;
#[doc = "Register DAC01DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC01DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA0`"]
pub type DATA0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DATA0`"]
pub struct DATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `DATA1`"]
pub type DATA1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DATA1`"]
pub struct DATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - DAC0 Data Bits"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC1 Data Bits"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC0 Data Bits"]
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W {
        DATA0_W { w: self }
    }
    #[doc = "Bits 16:27 - DAC1 Data Bits"]
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W {
        DATA1_W { w: self }
    }
}
