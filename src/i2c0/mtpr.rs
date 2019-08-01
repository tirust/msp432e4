#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MTPR {
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
pub struct I2C_MTPR_TPRR {
    bits: u8,
}
impl I2C_MTPR_TPRR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _I2C_MTPR_TPRW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MTPR_TPRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(127 << 0);
        self.w.bits |= ((value as u32) & 127) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct I2C_MTPR_HSR {
    bits: bool,
}
impl I2C_MTPR_HSR {
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
pub struct _I2C_MTPR_HSW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MTPR_HSW<'a> {
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
        self.w.bits &= !(1 << 7);
        self.w.bits |= ((value as u32) & 1) << 7;
        self.w
    }
}
#[doc = "Possible values of the field `I2C_MTPR_PULSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_MTPR_PULSELR {
    #[doc = "Bypass"]
    I2C_MTPR_PULSEL_BYPASS,
    #[doc = "1 clock"]
    I2C_MTPR_PULSEL_1,
    #[doc = "2 clocks"]
    I2C_MTPR_PULSEL_2,
    #[doc = "3 clocks"]
    I2C_MTPR_PULSEL_3,
    #[doc = "4 clocks"]
    I2C_MTPR_PULSEL_4,
    #[doc = "8 clocks"]
    I2C_MTPR_PULSEL_8,
    #[doc = "16 clocks"]
    I2C_MTPR_PULSEL_16,
    #[doc = "31 clocks"]
    I2C_MTPR_PULSEL_31,
}
impl I2C_MTPR_PULSELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_BYPASS => 0,
            I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_1 => 1,
            I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_2 => 2,
            I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_3 => 3,
            I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_4 => 4,
            I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_8 => 5,
            I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_16 => 6,
            I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_31 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> I2C_MTPR_PULSELR {
        match value {
            0 => I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_BYPASS,
            1 => I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_1,
            2 => I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_2,
            3 => I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_3,
            4 => I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_4,
            5 => I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_8,
            6 => I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_16,
            7 => I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `I2C_MTPR_PULSEL_BYPASS`"]
    #[inline(always)]
    pub fn is_i2c_mtpr_pulsel_bypass(&self) -> bool {
        *self == I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_BYPASS
    }
    #[doc = "Checks if the value of the field is `I2C_MTPR_PULSEL_1`"]
    #[inline(always)]
    pub fn is_i2c_mtpr_pulsel_1(&self) -> bool {
        *self == I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_1
    }
    #[doc = "Checks if the value of the field is `I2C_MTPR_PULSEL_2`"]
    #[inline(always)]
    pub fn is_i2c_mtpr_pulsel_2(&self) -> bool {
        *self == I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_2
    }
    #[doc = "Checks if the value of the field is `I2C_MTPR_PULSEL_3`"]
    #[inline(always)]
    pub fn is_i2c_mtpr_pulsel_3(&self) -> bool {
        *self == I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_3
    }
    #[doc = "Checks if the value of the field is `I2C_MTPR_PULSEL_4`"]
    #[inline(always)]
    pub fn is_i2c_mtpr_pulsel_4(&self) -> bool {
        *self == I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_4
    }
    #[doc = "Checks if the value of the field is `I2C_MTPR_PULSEL_8`"]
    #[inline(always)]
    pub fn is_i2c_mtpr_pulsel_8(&self) -> bool {
        *self == I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_8
    }
    #[doc = "Checks if the value of the field is `I2C_MTPR_PULSEL_16`"]
    #[inline(always)]
    pub fn is_i2c_mtpr_pulsel_16(&self) -> bool {
        *self == I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_16
    }
    #[doc = "Checks if the value of the field is `I2C_MTPR_PULSEL_31`"]
    #[inline(always)]
    pub fn is_i2c_mtpr_pulsel_31(&self) -> bool {
        *self == I2C_MTPR_PULSELR::I2C_MTPR_PULSEL_31
    }
}
#[doc = "Values that can be written to the field `I2C_MTPR_PULSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_MTPR_PULSELW {
    #[doc = "Bypass"]
    I2C_MTPR_PULSEL_BYPASS,
    #[doc = "1 clock"]
    I2C_MTPR_PULSEL_1,
    #[doc = "2 clocks"]
    I2C_MTPR_PULSEL_2,
    #[doc = "3 clocks"]
    I2C_MTPR_PULSEL_3,
    #[doc = "4 clocks"]
    I2C_MTPR_PULSEL_4,
    #[doc = "8 clocks"]
    I2C_MTPR_PULSEL_8,
    #[doc = "16 clocks"]
    I2C_MTPR_PULSEL_16,
    #[doc = "31 clocks"]
    I2C_MTPR_PULSEL_31,
}
impl I2C_MTPR_PULSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            I2C_MTPR_PULSELW::I2C_MTPR_PULSEL_BYPASS => 0,
            I2C_MTPR_PULSELW::I2C_MTPR_PULSEL_1 => 1,
            I2C_MTPR_PULSELW::I2C_MTPR_PULSEL_2 => 2,
            I2C_MTPR_PULSELW::I2C_MTPR_PULSEL_3 => 3,
            I2C_MTPR_PULSELW::I2C_MTPR_PULSEL_4 => 4,
            I2C_MTPR_PULSELW::I2C_MTPR_PULSEL_8 => 5,
            I2C_MTPR_PULSELW::I2C_MTPR_PULSEL_16 => 6,
            I2C_MTPR_PULSELW::I2C_MTPR_PULSEL_31 => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _I2C_MTPR_PULSELW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MTPR_PULSELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_MTPR_PULSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn i2c_mtpr_pulsel_bypass(self) -> &'a mut W {
        self.variant(I2C_MTPR_PULSELW::I2C_MTPR_PULSEL_BYPASS)
    }
    #[doc = "1 clock"]
    #[inline(always)]
    pub fn i2c_mtpr_pulsel_1(self) -> &'a mut W {
        self.variant(I2C_MTPR_PULSELW::I2C_MTPR_PULSEL_1)
    }
    #[doc = "2 clocks"]
    #[inline(always)]
    pub fn i2c_mtpr_pulsel_2(self) -> &'a mut W {
        self.variant(I2C_MTPR_PULSELW::I2C_MTPR_PULSEL_2)
    }
    #[doc = "3 clocks"]
    #[inline(always)]
    pub fn i2c_mtpr_pulsel_3(self) -> &'a mut W {
        self.variant(I2C_MTPR_PULSELW::I2C_MTPR_PULSEL_3)
    }
    #[doc = "4 clocks"]
    #[inline(always)]
    pub fn i2c_mtpr_pulsel_4(self) -> &'a mut W {
        self.variant(I2C_MTPR_PULSELW::I2C_MTPR_PULSEL_4)
    }
    #[doc = "8 clocks"]
    #[inline(always)]
    pub fn i2c_mtpr_pulsel_8(self) -> &'a mut W {
        self.variant(I2C_MTPR_PULSELW::I2C_MTPR_PULSEL_8)
    }
    #[doc = "16 clocks"]
    #[inline(always)]
    pub fn i2c_mtpr_pulsel_16(self) -> &'a mut W {
        self.variant(I2C_MTPR_PULSELW::I2C_MTPR_PULSEL_16)
    }
    #[doc = "31 clocks"]
    #[inline(always)]
    pub fn i2c_mtpr_pulsel_31(self) -> &'a mut W {
        self.variant(I2C_MTPR_PULSELW::I2C_MTPR_PULSEL_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 16);
        self.w.bits |= ((value as u32) & 7) << 16;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - Timer Period"]
    #[inline(always)]
    pub fn i2c_mtpr_tpr(&self) -> I2C_MTPR_TPRR {
        let bits = ((self.bits >> 0) & 127) as u8;
        I2C_MTPR_TPRR { bits }
    }
    #[doc = "Bit 7 - High-Speed Enable"]
    #[inline(always)]
    pub fn i2c_mtpr_hs(&self) -> I2C_MTPR_HSR {
        let bits = ((self.bits >> 7) & 1) != 0;
        I2C_MTPR_HSR { bits }
    }
    #[doc = "Bits 16:18 - Glitch Suppression Pulse Width"]
    #[inline(always)]
    pub fn i2c_mtpr_pulsel(&self) -> I2C_MTPR_PULSELR {
        I2C_MTPR_PULSELR::_from(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - Timer Period"]
    #[inline(always)]
    pub fn i2c_mtpr_tpr(&mut self) -> _I2C_MTPR_TPRW {
        _I2C_MTPR_TPRW { w: self }
    }
    #[doc = "Bit 7 - High-Speed Enable"]
    #[inline(always)]
    pub fn i2c_mtpr_hs(&mut self) -> _I2C_MTPR_HSW {
        _I2C_MTPR_HSW { w: self }
    }
    #[doc = "Bits 16:18 - Glitch Suppression Pulse Width"]
    #[inline(always)]
    pub fn i2c_mtpr_pulsel(&mut self) -> _I2C_MTPR_PULSELW {
        _I2C_MTPR_PULSELW { w: self }
    }
}
