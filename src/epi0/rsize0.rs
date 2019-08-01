#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RSIZE0 {
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
#[doc = "Possible values of the field `EPI_RSIZE0_SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_RSIZE0_SIZER {
    #[doc = "Byte (8 bits)"]
    EPI_RSIZE0_SIZE_8BIT,
    #[doc = "Half-word (16 bits)"]
    EPI_RSIZE0_SIZE_16BIT,
    #[doc = "Word (32 bits)"]
    EPI_RSIZE0_SIZE_32BIT,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl EPI_RSIZE0_SIZER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_RSIZE0_SIZER::EPI_RSIZE0_SIZE_8BIT => 1,
            EPI_RSIZE0_SIZER::EPI_RSIZE0_SIZE_16BIT => 2,
            EPI_RSIZE0_SIZER::EPI_RSIZE0_SIZE_32BIT => 3,
            EPI_RSIZE0_SIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_RSIZE0_SIZER {
        match value {
            1 => EPI_RSIZE0_SIZER::EPI_RSIZE0_SIZE_8BIT,
            2 => EPI_RSIZE0_SIZER::EPI_RSIZE0_SIZE_16BIT,
            3 => EPI_RSIZE0_SIZER::EPI_RSIZE0_SIZE_32BIT,
            i => EPI_RSIZE0_SIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_RSIZE0_SIZE_8BIT`"]
    #[inline(always)]
    pub fn is_epi_rsize0_size_8bit(&self) -> bool {
        *self == EPI_RSIZE0_SIZER::EPI_RSIZE0_SIZE_8BIT
    }
    #[doc = "Checks if the value of the field is `EPI_RSIZE0_SIZE_16BIT`"]
    #[inline(always)]
    pub fn is_epi_rsize0_size_16bit(&self) -> bool {
        *self == EPI_RSIZE0_SIZER::EPI_RSIZE0_SIZE_16BIT
    }
    #[doc = "Checks if the value of the field is `EPI_RSIZE0_SIZE_32BIT`"]
    #[inline(always)]
    pub fn is_epi_rsize0_size_32bit(&self) -> bool {
        *self == EPI_RSIZE0_SIZER::EPI_RSIZE0_SIZE_32BIT
    }
}
#[doc = "Values that can be written to the field `EPI_RSIZE0_SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_RSIZE0_SIZEW {
    #[doc = "Byte (8 bits)"]
    EPI_RSIZE0_SIZE_8BIT,
    #[doc = "Half-word (16 bits)"]
    EPI_RSIZE0_SIZE_16BIT,
    #[doc = "Word (32 bits)"]
    EPI_RSIZE0_SIZE_32BIT,
}
impl EPI_RSIZE0_SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_RSIZE0_SIZEW::EPI_RSIZE0_SIZE_8BIT => 1,
            EPI_RSIZE0_SIZEW::EPI_RSIZE0_SIZE_16BIT => 2,
            EPI_RSIZE0_SIZEW::EPI_RSIZE0_SIZE_32BIT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_RSIZE0_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_RSIZE0_SIZEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_RSIZE0_SIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Byte (8 bits)"]
    #[inline(always)]
    pub fn epi_rsize0_size_8bit(self) -> &'a mut W {
        self.variant(EPI_RSIZE0_SIZEW::EPI_RSIZE0_SIZE_8BIT)
    }
    #[doc = "Half-word (16 bits)"]
    #[inline(always)]
    pub fn epi_rsize0_size_16bit(self) -> &'a mut W {
        self.variant(EPI_RSIZE0_SIZEW::EPI_RSIZE0_SIZE_16BIT)
    }
    #[doc = "Word (32 bits)"]
    #[inline(always)]
    pub fn epi_rsize0_size_32bit(self) -> &'a mut W {
        self.variant(EPI_RSIZE0_SIZEW::EPI_RSIZE0_SIZE_32BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Current Size"]
    #[inline(always)]
    pub fn epi_rsize0_size(&self) -> EPI_RSIZE0_SIZER {
        EPI_RSIZE0_SIZER::_from(((self.bits >> 0) & 3) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Current Size"]
    #[inline(always)]
    pub fn epi_rsize0_size(&mut self) -> _EPI_RSIZE0_SIZEW {
        _EPI_RSIZE0_SIZEW { w: self }
    }
}
