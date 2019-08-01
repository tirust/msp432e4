#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MEMTIM0 {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_MEMTIM0_FWSR {
    bits: u8,
}
impl SYSCTL_MEMTIM0_FWSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_MEMTIM0_FWSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_MEMTIM0_FWSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct RESERVED0R {
    bits: bool,
}
impl RESERVED0R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _RESERVED0W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED0W<'a> {
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
        self.w.bits &= !(1 << 4);
        self.w.bits |= ((value as u32) & 1) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_MEMTIM0_FBCER {
    bits: bool,
}
impl SYSCTL_MEMTIM0_FBCER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_MEMTIM0_FBCEW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_MEMTIM0_FBCEW<'a> {
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
        self.w.bits &= !(1 << 5);
        self.w.bits |= ((value as u32) & 1) << 5;
        self.w
    }
}
#[doc = "Possible values of the field `SYSCTL_MEMTIM0_FBCHT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_MEMTIM0_FBCHTR {
    #[doc = "1/2 system clock period"]
    SYSCTL_MEMTIM0_FBCHT_0_5,
    #[doc = "1 system clock period"]
    SYSCTL_MEMTIM0_FBCHT_1,
    #[doc = "1.5 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_1_5,
    #[doc = "2 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_2,
    #[doc = "2.5 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_2_5,
    #[doc = "3 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_3,
    #[doc = "3.5 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_3_5,
    #[doc = "4 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_4,
    #[doc = "4.5 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_4_5,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_MEMTIM0_FBCHTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_0_5 => 0,
            SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_1 => 1,
            SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_1_5 => 2,
            SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_2 => 3,
            SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_2_5 => 4,
            SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_3 => 5,
            SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_3_5 => 6,
            SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_4 => 7,
            SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_4_5 => 8,
            SYSCTL_MEMTIM0_FBCHTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_MEMTIM0_FBCHTR {
        match value {
            0 => SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_0_5,
            1 => SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_1,
            2 => SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_1_5,
            3 => SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_2,
            4 => SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_2_5,
            5 => SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_3,
            6 => SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_3_5,
            7 => SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_4,
            8 => SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_4_5,
            i => SYSCTL_MEMTIM0_FBCHTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_FBCHT_0_5`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_fbcht_0_5(&self) -> bool {
        *self == SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_0_5
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_FBCHT_1`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_fbcht_1(&self) -> bool {
        *self == SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_1
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_FBCHT_1_5`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_fbcht_1_5(&self) -> bool {
        *self == SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_1_5
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_FBCHT_2`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_fbcht_2(&self) -> bool {
        *self == SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_2
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_FBCHT_2_5`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_fbcht_2_5(&self) -> bool {
        *self == SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_2_5
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_FBCHT_3`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_fbcht_3(&self) -> bool {
        *self == SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_3
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_FBCHT_3_5`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_fbcht_3_5(&self) -> bool {
        *self == SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_3_5
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_FBCHT_4`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_fbcht_4(&self) -> bool {
        *self == SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_4
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_FBCHT_4_5`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_fbcht_4_5(&self) -> bool {
        *self == SYSCTL_MEMTIM0_FBCHTR::SYSCTL_MEMTIM0_FBCHT_4_5
    }
}
#[doc = "Values that can be written to the field `SYSCTL_MEMTIM0_FBCHT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_MEMTIM0_FBCHTW {
    #[doc = "1/2 system clock period"]
    SYSCTL_MEMTIM0_FBCHT_0_5,
    #[doc = "1 system clock period"]
    SYSCTL_MEMTIM0_FBCHT_1,
    #[doc = "1.5 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_1_5,
    #[doc = "2 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_2,
    #[doc = "2.5 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_2_5,
    #[doc = "3 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_3,
    #[doc = "3.5 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_3_5,
    #[doc = "4 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_4,
    #[doc = "4.5 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_4_5,
}
impl SYSCTL_MEMTIM0_FBCHTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_MEMTIM0_FBCHTW::SYSCTL_MEMTIM0_FBCHT_0_5 => 0,
            SYSCTL_MEMTIM0_FBCHTW::SYSCTL_MEMTIM0_FBCHT_1 => 1,
            SYSCTL_MEMTIM0_FBCHTW::SYSCTL_MEMTIM0_FBCHT_1_5 => 2,
            SYSCTL_MEMTIM0_FBCHTW::SYSCTL_MEMTIM0_FBCHT_2 => 3,
            SYSCTL_MEMTIM0_FBCHTW::SYSCTL_MEMTIM0_FBCHT_2_5 => 4,
            SYSCTL_MEMTIM0_FBCHTW::SYSCTL_MEMTIM0_FBCHT_3 => 5,
            SYSCTL_MEMTIM0_FBCHTW::SYSCTL_MEMTIM0_FBCHT_3_5 => 6,
            SYSCTL_MEMTIM0_FBCHTW::SYSCTL_MEMTIM0_FBCHT_4 => 7,
            SYSCTL_MEMTIM0_FBCHTW::SYSCTL_MEMTIM0_FBCHT_4_5 => 8,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_MEMTIM0_FBCHTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_MEMTIM0_FBCHTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_MEMTIM0_FBCHTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1/2 system clock period"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht_0_5(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_FBCHTW::SYSCTL_MEMTIM0_FBCHT_0_5)
    }
    #[doc = "1 system clock period"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht_1(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_FBCHTW::SYSCTL_MEMTIM0_FBCHT_1)
    }
    #[doc = "1.5 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht_1_5(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_FBCHTW::SYSCTL_MEMTIM0_FBCHT_1_5)
    }
    #[doc = "2 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht_2(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_FBCHTW::SYSCTL_MEMTIM0_FBCHT_2)
    }
    #[doc = "2.5 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht_2_5(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_FBCHTW::SYSCTL_MEMTIM0_FBCHT_2_5)
    }
    #[doc = "3 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht_3(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_FBCHTW::SYSCTL_MEMTIM0_FBCHT_3)
    }
    #[doc = "3.5 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht_3_5(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_FBCHTW::SYSCTL_MEMTIM0_FBCHT_3_5)
    }
    #[doc = "4 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht_4(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_FBCHTW::SYSCTL_MEMTIM0_FBCHT_4)
    }
    #[doc = "4.5 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht_4_5(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_FBCHTW::SYSCTL_MEMTIM0_FBCHT_4_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 6);
        self.w.bits |= ((value as u32) & 15) << 6;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_MEMTIM0_EWSR {
    bits: u8,
}
impl SYSCTL_MEMTIM0_EWSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_MEMTIM0_EWSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_MEMTIM0_EWSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 16);
        self.w.bits |= ((value as u32) & 15) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct RESERVED1R {
    bits: bool,
}
impl RESERVED1R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _RESERVED1W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED1W<'a> {
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
        self.w.bits &= !(1 << 20);
        self.w.bits |= ((value as u32) & 1) << 20;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_MEMTIM0_EBCER {
    bits: bool,
}
impl SYSCTL_MEMTIM0_EBCER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_MEMTIM0_EBCEW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_MEMTIM0_EBCEW<'a> {
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
        self.w.bits &= !(1 << 21);
        self.w.bits |= ((value as u32) & 1) << 21;
        self.w
    }
}
#[doc = "Possible values of the field `SYSCTL_MEMTIM0_EBCHT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_MEMTIM0_EBCHTR {
    #[doc = "1/2 system clock period"]
    SYSCTL_MEMTIM0_EBCHT_0_5,
    #[doc = "1 system clock period"]
    SYSCTL_MEMTIM0_EBCHT_1,
    #[doc = "1.5 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_1_5,
    #[doc = "2 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_2,
    #[doc = "2.5 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_2_5,
    #[doc = "3 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_3,
    #[doc = "3.5 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_3_5,
    #[doc = "4 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_4,
    #[doc = "4.5 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_4_5,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_MEMTIM0_EBCHTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_0_5 => 0,
            SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_1 => 1,
            SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_1_5 => 2,
            SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_2 => 3,
            SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_2_5 => 4,
            SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_3 => 5,
            SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_3_5 => 6,
            SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_4 => 7,
            SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_4_5 => 8,
            SYSCTL_MEMTIM0_EBCHTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_MEMTIM0_EBCHTR {
        match value {
            0 => SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_0_5,
            1 => SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_1,
            2 => SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_1_5,
            3 => SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_2,
            4 => SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_2_5,
            5 => SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_3,
            6 => SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_3_5,
            7 => SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_4,
            8 => SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_4_5,
            i => SYSCTL_MEMTIM0_EBCHTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_EBCHT_0_5`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_ebcht_0_5(&self) -> bool {
        *self == SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_0_5
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_EBCHT_1`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_ebcht_1(&self) -> bool {
        *self == SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_1
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_EBCHT_1_5`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_ebcht_1_5(&self) -> bool {
        *self == SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_1_5
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_EBCHT_2`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_ebcht_2(&self) -> bool {
        *self == SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_2
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_EBCHT_2_5`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_ebcht_2_5(&self) -> bool {
        *self == SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_2_5
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_EBCHT_3`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_ebcht_3(&self) -> bool {
        *self == SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_3
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_EBCHT_3_5`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_ebcht_3_5(&self) -> bool {
        *self == SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_3_5
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_EBCHT_4`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_ebcht_4(&self) -> bool {
        *self == SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_4
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_EBCHT_4_5`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_ebcht_4_5(&self) -> bool {
        *self == SYSCTL_MEMTIM0_EBCHTR::SYSCTL_MEMTIM0_EBCHT_4_5
    }
}
#[doc = "Values that can be written to the field `SYSCTL_MEMTIM0_EBCHT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_MEMTIM0_EBCHTW {
    #[doc = "1/2 system clock period"]
    SYSCTL_MEMTIM0_EBCHT_0_5,
    #[doc = "1 system clock period"]
    SYSCTL_MEMTIM0_EBCHT_1,
    #[doc = "1.5 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_1_5,
    #[doc = "2 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_2,
    #[doc = "2.5 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_2_5,
    #[doc = "3 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_3,
    #[doc = "3.5 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_3_5,
    #[doc = "4 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_4,
    #[doc = "4.5 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_4_5,
}
impl SYSCTL_MEMTIM0_EBCHTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_MEMTIM0_EBCHTW::SYSCTL_MEMTIM0_EBCHT_0_5 => 0,
            SYSCTL_MEMTIM0_EBCHTW::SYSCTL_MEMTIM0_EBCHT_1 => 1,
            SYSCTL_MEMTIM0_EBCHTW::SYSCTL_MEMTIM0_EBCHT_1_5 => 2,
            SYSCTL_MEMTIM0_EBCHTW::SYSCTL_MEMTIM0_EBCHT_2 => 3,
            SYSCTL_MEMTIM0_EBCHTW::SYSCTL_MEMTIM0_EBCHT_2_5 => 4,
            SYSCTL_MEMTIM0_EBCHTW::SYSCTL_MEMTIM0_EBCHT_3 => 5,
            SYSCTL_MEMTIM0_EBCHTW::SYSCTL_MEMTIM0_EBCHT_3_5 => 6,
            SYSCTL_MEMTIM0_EBCHTW::SYSCTL_MEMTIM0_EBCHT_4 => 7,
            SYSCTL_MEMTIM0_EBCHTW::SYSCTL_MEMTIM0_EBCHT_4_5 => 8,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_MEMTIM0_EBCHTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_MEMTIM0_EBCHTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_MEMTIM0_EBCHTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1/2 system clock period"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht_0_5(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_EBCHTW::SYSCTL_MEMTIM0_EBCHT_0_5)
    }
    #[doc = "1 system clock period"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht_1(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_EBCHTW::SYSCTL_MEMTIM0_EBCHT_1)
    }
    #[doc = "1.5 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht_1_5(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_EBCHTW::SYSCTL_MEMTIM0_EBCHT_1_5)
    }
    #[doc = "2 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht_2(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_EBCHTW::SYSCTL_MEMTIM0_EBCHT_2)
    }
    #[doc = "2.5 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht_2_5(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_EBCHTW::SYSCTL_MEMTIM0_EBCHT_2_5)
    }
    #[doc = "3 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht_3(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_EBCHTW::SYSCTL_MEMTIM0_EBCHT_3)
    }
    #[doc = "3.5 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht_3_5(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_EBCHTW::SYSCTL_MEMTIM0_EBCHT_3_5)
    }
    #[doc = "4 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht_4(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_EBCHTW::SYSCTL_MEMTIM0_EBCHT_4)
    }
    #[doc = "4.5 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht_4_5(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_EBCHTW::SYSCTL_MEMTIM0_EBCHT_4_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 22);
        self.w.bits |= ((value as u32) & 15) << 22;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Flash Wait State"]
    #[inline(always)]
    pub fn sysctl_memtim0_fws(&self) -> SYSCTL_MEMTIM0_FWSR {
        let bits = ((self.bits >> 0) & 15) as u8;
        SYSCTL_MEMTIM0_FWSR { bits }
    }
    #[doc = "Bit 4 - Value of this reserved bit must be read as 1"]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0R {
        let bits = ((self.bits >> 4) & 1) != 0;
        RESERVED0R { bits }
    }
    #[doc = "Bit 5 - Flash Bank Clock Edge"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbce(&self) -> SYSCTL_MEMTIM0_FBCER {
        let bits = ((self.bits >> 5) & 1) != 0;
        SYSCTL_MEMTIM0_FBCER { bits }
    }
    #[doc = "Bits 6:9 - Flash Bank Clock High Time"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht(&self) -> SYSCTL_MEMTIM0_FBCHTR {
        SYSCTL_MEMTIM0_FBCHTR::_from(((self.bits >> 6) & 15) as u8)
    }
    #[doc = "Bits 16:19 - EEPROM Wait States"]
    #[inline(always)]
    pub fn sysctl_memtim0_ews(&self) -> SYSCTL_MEMTIM0_EWSR {
        let bits = ((self.bits >> 16) & 15) as u8;
        SYSCTL_MEMTIM0_EWSR { bits }
    }
    #[doc = "Bit 20 - Value of this reserved bit must be read as 1"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1R {
        let bits = ((self.bits >> 20) & 1) != 0;
        RESERVED1R { bits }
    }
    #[doc = "Bit 21 - EEPROM Bank Clock Edge"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebce(&self) -> SYSCTL_MEMTIM0_EBCER {
        let bits = ((self.bits >> 21) & 1) != 0;
        SYSCTL_MEMTIM0_EBCER { bits }
    }
    #[doc = "Bits 22:25 - EEPROM Clock High Time"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht(&self) -> SYSCTL_MEMTIM0_EBCHTR {
        SYSCTL_MEMTIM0_EBCHTR::_from(((self.bits >> 22) & 15) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Flash Wait State"]
    #[inline(always)]
    pub fn sysctl_memtim0_fws(&mut self) -> _SYSCTL_MEMTIM0_FWSW {
        _SYSCTL_MEMTIM0_FWSW { w: self }
    }
    #[doc = "Bit 4 - Value of this reserved bit must be read as 1"]
    #[inline(always)]
    pub fn reserved0(&mut self) -> _RESERVED0W {
        _RESERVED0W { w: self }
    }
    #[doc = "Bit 5 - Flash Bank Clock Edge"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbce(&mut self) -> _SYSCTL_MEMTIM0_FBCEW {
        _SYSCTL_MEMTIM0_FBCEW { w: self }
    }
    #[doc = "Bits 6:9 - Flash Bank Clock High Time"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht(&mut self) -> _SYSCTL_MEMTIM0_FBCHTW {
        _SYSCTL_MEMTIM0_FBCHTW { w: self }
    }
    #[doc = "Bits 16:19 - EEPROM Wait States"]
    #[inline(always)]
    pub fn sysctl_memtim0_ews(&mut self) -> _SYSCTL_MEMTIM0_EWSW {
        _SYSCTL_MEMTIM0_EWSW { w: self }
    }
    #[doc = "Bit 20 - Value of this reserved bit must be read as 1"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> _RESERVED1W {
        _RESERVED1W { w: self }
    }
    #[doc = "Bit 21 - EEPROM Bank Clock Edge"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebce(&mut self) -> _SYSCTL_MEMTIM0_EBCEW {
        _SYSCTL_MEMTIM0_EBCEW { w: self }
    }
    #[doc = "Bits 22:25 - EEPROM Clock High Time"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht(&mut self) -> _SYSCTL_MEMTIM0_EBCHTW {
        _SYSCTL_MEMTIM0_EBCHTW { w: self }
    }
}
