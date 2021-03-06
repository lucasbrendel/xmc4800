#[doc = "Reader of register VFR"]
pub type R = crate::R<u32, super::VFR>;
#[doc = "Writer for register VFR"]
pub type W = crate::W<u32, super::VFR>;
#[doc = "Register VFR `reset()`'s with value 0"]
impl crate::ResetValue for super::VFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF0_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF0_A> for bool {
    #[inline(always)]
    fn from(variant: VF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VF0`"]
pub type VF0_R = crate::R<bool, VF0_A>;
impl VF0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VF0_A {
        match self.bits {
            false => VF0_A::VALUE1,
            true => VF0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF0_A::VALUE2
    }
}
#[doc = "Write proxy for field `VF0`"]
pub struct VF0_W<'a> {
    w: &'a mut W,
}
impl<'a> VF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VF0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF0_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF0_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF1_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF1_A> for bool {
    #[inline(always)]
    fn from(variant: VF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VF1`"]
pub type VF1_R = crate::R<bool, VF1_A>;
impl VF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VF1_A {
        match self.bits {
            false => VF1_A::VALUE1,
            true => VF1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF1_A::VALUE2
    }
}
#[doc = "Write proxy for field `VF1`"]
pub struct VF1_W<'a> {
    w: &'a mut W,
}
impl<'a> VF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VF1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF1_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF1_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF2_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF2_A> for bool {
    #[inline(always)]
    fn from(variant: VF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VF2`"]
pub type VF2_R = crate::R<bool, VF2_A>;
impl VF2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VF2_A {
        match self.bits {
            false => VF2_A::VALUE1,
            true => VF2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF2_A::VALUE2
    }
}
#[doc = "Write proxy for field `VF2`"]
pub struct VF2_W<'a> {
    w: &'a mut W,
}
impl<'a> VF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VF2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF2_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF2_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF3_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF3_A> for bool {
    #[inline(always)]
    fn from(variant: VF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VF3`"]
pub type VF3_R = crate::R<bool, VF3_A>;
impl VF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VF3_A {
        match self.bits {
            false => VF3_A::VALUE1,
            true => VF3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF3_A::VALUE2
    }
}
#[doc = "Write proxy for field `VF3`"]
pub struct VF3_W<'a> {
    w: &'a mut W,
}
impl<'a> VF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VF3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF3_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF3_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF4_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF4_A> for bool {
    #[inline(always)]
    fn from(variant: VF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VF4`"]
pub type VF4_R = crate::R<bool, VF4_A>;
impl VF4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VF4_A {
        match self.bits {
            false => VF4_A::VALUE1,
            true => VF4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF4_A::VALUE2
    }
}
#[doc = "Write proxy for field `VF4`"]
pub struct VF4_W<'a> {
    w: &'a mut W,
}
impl<'a> VF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VF4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF4_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF4_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF5_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF5_A> for bool {
    #[inline(always)]
    fn from(variant: VF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VF5`"]
pub type VF5_R = crate::R<bool, VF5_A>;
impl VF5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VF5_A {
        match self.bits {
            false => VF5_A::VALUE1,
            true => VF5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF5_A::VALUE2
    }
}
#[doc = "Write proxy for field `VF5`"]
pub struct VF5_W<'a> {
    w: &'a mut W,
}
impl<'a> VF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VF5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF5_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF5_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF6_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF6_A> for bool {
    #[inline(always)]
    fn from(variant: VF6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VF6`"]
pub type VF6_R = crate::R<bool, VF6_A>;
impl VF6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VF6_A {
        match self.bits {
            false => VF6_A::VALUE1,
            true => VF6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF6_A::VALUE2
    }
}
#[doc = "Write proxy for field `VF6`"]
pub struct VF6_W<'a> {
    w: &'a mut W,
}
impl<'a> VF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VF6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF6_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF6_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF7_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF7_A> for bool {
    #[inline(always)]
    fn from(variant: VF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VF7`"]
pub type VF7_R = crate::R<bool, VF7_A>;
impl VF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VF7_A {
        match self.bits {
            false => VF7_A::VALUE1,
            true => VF7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF7_A::VALUE2
    }
}
#[doc = "Write proxy for field `VF7`"]
pub struct VF7_W<'a> {
    w: &'a mut W,
}
impl<'a> VF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VF7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF7_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF7_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF8_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF8_A> for bool {
    #[inline(always)]
    fn from(variant: VF8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VF8`"]
pub type VF8_R = crate::R<bool, VF8_A>;
impl VF8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VF8_A {
        match self.bits {
            false => VF8_A::VALUE1,
            true => VF8_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF8_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF8_A::VALUE2
    }
}
#[doc = "Write proxy for field `VF8`"]
pub struct VF8_W<'a> {
    w: &'a mut W,
}
impl<'a> VF8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VF8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF8_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF8_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF9_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF9_A> for bool {
    #[inline(always)]
    fn from(variant: VF9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VF9`"]
pub type VF9_R = crate::R<bool, VF9_A>;
impl VF9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VF9_A {
        match self.bits {
            false => VF9_A::VALUE1,
            true => VF9_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF9_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF9_A::VALUE2
    }
}
#[doc = "Write proxy for field `VF9`"]
pub struct VF9_W<'a> {
    w: &'a mut W,
}
impl<'a> VF9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VF9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF9_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF9_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF10_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF10_A> for bool {
    #[inline(always)]
    fn from(variant: VF10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VF10`"]
pub type VF10_R = crate::R<bool, VF10_A>;
impl VF10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VF10_A {
        match self.bits {
            false => VF10_A::VALUE1,
            true => VF10_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF10_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF10_A::VALUE2
    }
}
#[doc = "Write proxy for field `VF10`"]
pub struct VF10_W<'a> {
    w: &'a mut W,
}
impl<'a> VF10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VF10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF10_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF10_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF11_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF11_A> for bool {
    #[inline(always)]
    fn from(variant: VF11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VF11`"]
pub type VF11_R = crate::R<bool, VF11_A>;
impl VF11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VF11_A {
        match self.bits {
            false => VF11_A::VALUE1,
            true => VF11_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF11_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF11_A::VALUE2
    }
}
#[doc = "Write proxy for field `VF11`"]
pub struct VF11_W<'a> {
    w: &'a mut W,
}
impl<'a> VF11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VF11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF11_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF11_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF12_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF12_A> for bool {
    #[inline(always)]
    fn from(variant: VF12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VF12`"]
pub type VF12_R = crate::R<bool, VF12_A>;
impl VF12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VF12_A {
        match self.bits {
            false => VF12_A::VALUE1,
            true => VF12_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF12_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF12_A::VALUE2
    }
}
#[doc = "Write proxy for field `VF12`"]
pub struct VF12_W<'a> {
    w: &'a mut W,
}
impl<'a> VF12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VF12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF12_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF12_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF13_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF13_A> for bool {
    #[inline(always)]
    fn from(variant: VF13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VF13`"]
pub type VF13_R = crate::R<bool, VF13_A>;
impl VF13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VF13_A {
        match self.bits {
            false => VF13_A::VALUE1,
            true => VF13_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF13_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF13_A::VALUE2
    }
}
#[doc = "Write proxy for field `VF13`"]
pub struct VF13_W<'a> {
    w: &'a mut W,
}
impl<'a> VF13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VF13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF13_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF13_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF14_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF14_A> for bool {
    #[inline(always)]
    fn from(variant: VF14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VF14`"]
pub type VF14_R = crate::R<bool, VF14_A>;
impl VF14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VF14_A {
        match self.bits {
            false => VF14_A::VALUE1,
            true => VF14_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF14_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF14_A::VALUE2
    }
}
#[doc = "Write proxy for field `VF14`"]
pub struct VF14_W<'a> {
    w: &'a mut W,
}
impl<'a> VF14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VF14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF14_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF14_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Valid Flag of Result Register x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF15_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF15_A> for bool {
    #[inline(always)]
    fn from(variant: VF15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VF15`"]
pub type VF15_R = crate::R<bool, VF15_A>;
impl VF15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VF15_A {
        match self.bits {
            false => VF15_A::VALUE1,
            true => VF15_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF15_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF15_A::VALUE2
    }
}
#[doc = "Write proxy for field `VF15`"]
pub struct VF15_W<'a> {
    w: &'a mut W,
}
impl<'a> VF15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VF15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF15_A::VALUE1)
    }
    #[doc = "Read access: Result register x contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and bitfield DRC in register GxRESy (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF15_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf0(&self) -> VF0_R {
        VF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf1(&self) -> VF1_R {
        VF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf2(&self) -> VF2_R {
        VF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf3(&self) -> VF3_R {
        VF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf4(&self) -> VF4_R {
        VF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf5(&self) -> VF5_R {
        VF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf6(&self) -> VF6_R {
        VF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf7(&self) -> VF7_R {
        VF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf8(&self) -> VF8_R {
        VF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf9(&self) -> VF9_R {
        VF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf10(&self) -> VF10_R {
        VF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf11(&self) -> VF11_R {
        VF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf12(&self) -> VF12_R {
        VF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf13(&self) -> VF13_R {
        VF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf14(&self) -> VF14_R {
        VF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf15(&self) -> VF15_R {
        VF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf0(&mut self) -> VF0_W {
        VF0_W { w: self }
    }
    #[doc = "Bit 1 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf1(&mut self) -> VF1_W {
        VF1_W { w: self }
    }
    #[doc = "Bit 2 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf2(&mut self) -> VF2_W {
        VF2_W { w: self }
    }
    #[doc = "Bit 3 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf3(&mut self) -> VF3_W {
        VF3_W { w: self }
    }
    #[doc = "Bit 4 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf4(&mut self) -> VF4_W {
        VF4_W { w: self }
    }
    #[doc = "Bit 5 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf5(&mut self) -> VF5_W {
        VF5_W { w: self }
    }
    #[doc = "Bit 6 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf6(&mut self) -> VF6_W {
        VF6_W { w: self }
    }
    #[doc = "Bit 7 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf7(&mut self) -> VF7_W {
        VF7_W { w: self }
    }
    #[doc = "Bit 8 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf8(&mut self) -> VF8_W {
        VF8_W { w: self }
    }
    #[doc = "Bit 9 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf9(&mut self) -> VF9_W {
        VF9_W { w: self }
    }
    #[doc = "Bit 10 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf10(&mut self) -> VF10_W {
        VF10_W { w: self }
    }
    #[doc = "Bit 11 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf11(&mut self) -> VF11_W {
        VF11_W { w: self }
    }
    #[doc = "Bit 12 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf12(&mut self) -> VF12_W {
        VF12_W { w: self }
    }
    #[doc = "Bit 13 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf13(&mut self) -> VF13_W {
        VF13_W { w: self }
    }
    #[doc = "Bit 14 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf14(&mut self) -> VF14_W {
        VF14_W { w: self }
    }
    #[doc = "Bit 15 - Valid Flag of Result Register x"]
    #[inline(always)]
    pub fn vf15(&mut self) -> VF15_W {
        VF15_W { w: self }
    }
}
