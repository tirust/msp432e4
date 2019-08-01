#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TPCTL {
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
pub struct HIB_TPCTL_TPENR {
    bits: bool,
}
impl HIB_TPCTL_TPENR {
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
pub struct _HIB_TPCTL_TPENW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPCTL_TPENW<'a> {
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
        self.w.bits &= !(1 << 0);
        self.w.bits |= ((value as u32) & 1) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_TPCTL_TPCLRR {
    bits: bool,
}
impl HIB_TPCTL_TPCLRR {
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
pub struct _HIB_TPCTL_TPCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPCTL_TPCLRW<'a> {
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
#[doc = "Possible values of the field `HIB_TPCTL_MEMCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIB_TPCTL_MEMCLRR {
    #[doc = "Do not Clear HIB memory on tamper event"]
    HIB_TPCTL_MEMCLR_NONE,
    #[doc = "Clear Lower 32 Bytes of HIB memory on tamper event"]
    HIB_TPCTL_MEMCLR_LOW32,
    #[doc = "Clear upper 32 Bytes of HIB memory on tamper event"]
    HIB_TPCTL_MEMCLR_HIGH32,
    #[doc = "Clear all HIB memory on tamper event"]
    HIB_TPCTL_MEMCLR_ALL,
}
impl HIB_TPCTL_MEMCLRR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            HIB_TPCTL_MEMCLRR::HIB_TPCTL_MEMCLR_NONE => 0,
            HIB_TPCTL_MEMCLRR::HIB_TPCTL_MEMCLR_LOW32 => 1,
            HIB_TPCTL_MEMCLRR::HIB_TPCTL_MEMCLR_HIGH32 => 2,
            HIB_TPCTL_MEMCLRR::HIB_TPCTL_MEMCLR_ALL => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> HIB_TPCTL_MEMCLRR {
        match value {
            0 => HIB_TPCTL_MEMCLRR::HIB_TPCTL_MEMCLR_NONE,
            1 => HIB_TPCTL_MEMCLRR::HIB_TPCTL_MEMCLR_LOW32,
            2 => HIB_TPCTL_MEMCLRR::HIB_TPCTL_MEMCLR_HIGH32,
            3 => HIB_TPCTL_MEMCLRR::HIB_TPCTL_MEMCLR_ALL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HIB_TPCTL_MEMCLR_NONE`"]
    #[inline(always)]
    pub fn is_hib_tpctl_memclr_none(&self) -> bool {
        *self == HIB_TPCTL_MEMCLRR::HIB_TPCTL_MEMCLR_NONE
    }
    #[doc = "Checks if the value of the field is `HIB_TPCTL_MEMCLR_LOW32`"]
    #[inline(always)]
    pub fn is_hib_tpctl_memclr_low32(&self) -> bool {
        *self == HIB_TPCTL_MEMCLRR::HIB_TPCTL_MEMCLR_LOW32
    }
    #[doc = "Checks if the value of the field is `HIB_TPCTL_MEMCLR_HIGH32`"]
    #[inline(always)]
    pub fn is_hib_tpctl_memclr_high32(&self) -> bool {
        *self == HIB_TPCTL_MEMCLRR::HIB_TPCTL_MEMCLR_HIGH32
    }
    #[doc = "Checks if the value of the field is `HIB_TPCTL_MEMCLR_ALL`"]
    #[inline(always)]
    pub fn is_hib_tpctl_memclr_all(&self) -> bool {
        *self == HIB_TPCTL_MEMCLRR::HIB_TPCTL_MEMCLR_ALL
    }
}
#[doc = "Values that can be written to the field `HIB_TPCTL_MEMCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIB_TPCTL_MEMCLRW {
    #[doc = "Do not Clear HIB memory on tamper event"]
    HIB_TPCTL_MEMCLR_NONE,
    #[doc = "Clear Lower 32 Bytes of HIB memory on tamper event"]
    HIB_TPCTL_MEMCLR_LOW32,
    #[doc = "Clear upper 32 Bytes of HIB memory on tamper event"]
    HIB_TPCTL_MEMCLR_HIGH32,
    #[doc = "Clear all HIB memory on tamper event"]
    HIB_TPCTL_MEMCLR_ALL,
}
impl HIB_TPCTL_MEMCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            HIB_TPCTL_MEMCLRW::HIB_TPCTL_MEMCLR_NONE => 0,
            HIB_TPCTL_MEMCLRW::HIB_TPCTL_MEMCLR_LOW32 => 1,
            HIB_TPCTL_MEMCLRW::HIB_TPCTL_MEMCLR_HIGH32 => 2,
            HIB_TPCTL_MEMCLRW::HIB_TPCTL_MEMCLR_ALL => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _HIB_TPCTL_MEMCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPCTL_MEMCLRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIB_TPCTL_MEMCLRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do not Clear HIB memory on tamper event"]
    #[inline(always)]
    pub fn hib_tpctl_memclr_none(self) -> &'a mut W {
        self.variant(HIB_TPCTL_MEMCLRW::HIB_TPCTL_MEMCLR_NONE)
    }
    #[doc = "Clear Lower 32 Bytes of HIB memory on tamper event"]
    #[inline(always)]
    pub fn hib_tpctl_memclr_low32(self) -> &'a mut W {
        self.variant(HIB_TPCTL_MEMCLRW::HIB_TPCTL_MEMCLR_LOW32)
    }
    #[doc = "Clear upper 32 Bytes of HIB memory on tamper event"]
    #[inline(always)]
    pub fn hib_tpctl_memclr_high32(self) -> &'a mut W {
        self.variant(HIB_TPCTL_MEMCLRW::HIB_TPCTL_MEMCLR_HIGH32)
    }
    #[doc = "Clear all HIB memory on tamper event"]
    #[inline(always)]
    pub fn hib_tpctl_memclr_all(self) -> &'a mut W {
        self.variant(HIB_TPCTL_MEMCLRW::HIB_TPCTL_MEMCLR_ALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 8);
        self.w.bits |= ((value as u32) & 3) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_TPCTL_WAKER {
    bits: bool,
}
impl HIB_TPCTL_WAKER {
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
pub struct _HIB_TPCTL_WAKEW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPCTL_WAKEW<'a> {
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
        self.w.bits &= !(1 << 11);
        self.w.bits |= ((value as u32) & 1) << 11;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Tamper Module Enable"]
    #[inline(always)]
    pub fn hib_tpctl_tpen(&self) -> HIB_TPCTL_TPENR {
        let bits = ((self.bits >> 0) & 1) != 0;
        HIB_TPCTL_TPENR { bits }
    }
    #[doc = "Bit 4 - Tamper Event Clear"]
    #[inline(always)]
    pub fn hib_tpctl_tpclr(&self) -> HIB_TPCTL_TPCLRR {
        let bits = ((self.bits >> 4) & 1) != 0;
        HIB_TPCTL_TPCLRR { bits }
    }
    #[doc = "Bits 8:9 - HIB Memory Clear on Tamper Event"]
    #[inline(always)]
    pub fn hib_tpctl_memclr(&self) -> HIB_TPCTL_MEMCLRR {
        HIB_TPCTL_MEMCLRR::_from(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Wake from Hibernate on a Tamper Event"]
    #[inline(always)]
    pub fn hib_tpctl_wake(&self) -> HIB_TPCTL_WAKER {
        let bits = ((self.bits >> 11) & 1) != 0;
        HIB_TPCTL_WAKER { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Tamper Module Enable"]
    #[inline(always)]
    pub fn hib_tpctl_tpen(&mut self) -> _HIB_TPCTL_TPENW {
        _HIB_TPCTL_TPENW { w: self }
    }
    #[doc = "Bit 4 - Tamper Event Clear"]
    #[inline(always)]
    pub fn hib_tpctl_tpclr(&mut self) -> _HIB_TPCTL_TPCLRW {
        _HIB_TPCTL_TPCLRW { w: self }
    }
    #[doc = "Bits 8:9 - HIB Memory Clear on Tamper Event"]
    #[inline(always)]
    pub fn hib_tpctl_memclr(&mut self) -> _HIB_TPCTL_MEMCLRW {
        _HIB_TPCTL_MEMCLRW { w: self }
    }
    #[doc = "Bit 11 - Wake from Hibernate on a Tamper Event"]
    #[inline(always)]
    pub fn hib_tpctl_wake(&mut self) -> _HIB_TPCTL_WAKEW {
        _HIB_TPCTL_WAKEW { w: self }
    }
}
