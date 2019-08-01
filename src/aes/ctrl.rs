#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
pub struct AES_CTRL_OUTPUT_READYR {
    bits: bool,
}
impl AES_CTRL_OUTPUT_READYR {
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
pub struct _AES_CTRL_OUTPUT_READYW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_CTRL_OUTPUT_READYW<'a> {
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
pub struct AES_CTRL_INPUT_READYR {
    bits: bool,
}
impl AES_CTRL_INPUT_READYR {
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
pub struct _AES_CTRL_INPUT_READYW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_CTRL_INPUT_READYW<'a> {
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
        self.w.bits &= !(1 << 1);
        self.w.bits |= ((value as u32) & 1) << 1;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct AES_CTRL_DIRECTIONR {
    bits: bool,
}
impl AES_CTRL_DIRECTIONR {
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
pub struct _AES_CTRL_DIRECTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_CTRL_DIRECTIONW<'a> {
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
        self.w.bits &= !(1 << 2);
        self.w.bits |= ((value as u32) & 1) << 2;
        self.w
    }
}
#[doc = "Possible values of the field `AES_CTRL_KEY_SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AES_CTRL_KEY_SIZER {
    #[doc = "Key is 128 bits"]
    AES_CTRL_KEY_SIZE_128,
    #[doc = "Key is 192 bits"]
    AES_CTRL_KEY_SIZE_192,
    #[doc = "Key is 256 bits"]
    AES_CTRL_KEY_SIZE_256,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl AES_CTRL_KEY_SIZER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            AES_CTRL_KEY_SIZER::AES_CTRL_KEY_SIZE_128 => 1,
            AES_CTRL_KEY_SIZER::AES_CTRL_KEY_SIZE_192 => 2,
            AES_CTRL_KEY_SIZER::AES_CTRL_KEY_SIZE_256 => 3,
            AES_CTRL_KEY_SIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> AES_CTRL_KEY_SIZER {
        match value {
            1 => AES_CTRL_KEY_SIZER::AES_CTRL_KEY_SIZE_128,
            2 => AES_CTRL_KEY_SIZER::AES_CTRL_KEY_SIZE_192,
            3 => AES_CTRL_KEY_SIZER::AES_CTRL_KEY_SIZE_256,
            i => AES_CTRL_KEY_SIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_KEY_SIZE_128`"]
    #[inline(always)]
    pub fn is_aes_ctrl_key_size_128(&self) -> bool {
        *self == AES_CTRL_KEY_SIZER::AES_CTRL_KEY_SIZE_128
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_KEY_SIZE_192`"]
    #[inline(always)]
    pub fn is_aes_ctrl_key_size_192(&self) -> bool {
        *self == AES_CTRL_KEY_SIZER::AES_CTRL_KEY_SIZE_192
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_KEY_SIZE_256`"]
    #[inline(always)]
    pub fn is_aes_ctrl_key_size_256(&self) -> bool {
        *self == AES_CTRL_KEY_SIZER::AES_CTRL_KEY_SIZE_256
    }
}
#[doc = "Values that can be written to the field `AES_CTRL_KEY_SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AES_CTRL_KEY_SIZEW {
    #[doc = "Key is 128 bits"]
    AES_CTRL_KEY_SIZE_128,
    #[doc = "Key is 192 bits"]
    AES_CTRL_KEY_SIZE_192,
    #[doc = "Key is 256 bits"]
    AES_CTRL_KEY_SIZE_256,
}
impl AES_CTRL_KEY_SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            AES_CTRL_KEY_SIZEW::AES_CTRL_KEY_SIZE_128 => 1,
            AES_CTRL_KEY_SIZEW::AES_CTRL_KEY_SIZE_192 => 2,
            AES_CTRL_KEY_SIZEW::AES_CTRL_KEY_SIZE_256 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _AES_CTRL_KEY_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_CTRL_KEY_SIZEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AES_CTRL_KEY_SIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Key is 128 bits"]
    #[inline(always)]
    pub fn aes_ctrl_key_size_128(self) -> &'a mut W {
        self.variant(AES_CTRL_KEY_SIZEW::AES_CTRL_KEY_SIZE_128)
    }
    #[doc = "Key is 192 bits"]
    #[inline(always)]
    pub fn aes_ctrl_key_size_192(self) -> &'a mut W {
        self.variant(AES_CTRL_KEY_SIZEW::AES_CTRL_KEY_SIZE_192)
    }
    #[doc = "Key is 256 bits"]
    #[inline(always)]
    pub fn aes_ctrl_key_size_256(self) -> &'a mut W {
        self.variant(AES_CTRL_KEY_SIZEW::AES_CTRL_KEY_SIZE_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 3);
        self.w.bits |= ((value as u32) & 3) << 3;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct AES_CTRL_MODER {
    bits: bool,
}
impl AES_CTRL_MODER {
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
pub struct _AES_CTRL_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_CTRL_MODEW<'a> {
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
#[doc = r"Value of the field"]
pub struct AES_CTRL_CTRR {
    bits: bool,
}
impl AES_CTRL_CTRR {
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
pub struct _AES_CTRL_CTRW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_CTRL_CTRW<'a> {
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
        self.w.bits &= !(1 << 6);
        self.w.bits |= ((value as u32) & 1) << 6;
        self.w
    }
}
#[doc = "Possible values of the field `AES_CTRL_CTR_WIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AES_CTRL_CTR_WIDTHR {
    #[doc = "Counter is 32 bits"]
    AES_CTRL_CTR_WIDTH_32,
    #[doc = "Counter is 64 bits"]
    AES_CTRL_CTR_WIDTH_64,
    #[doc = "Counter is 96 bits"]
    AES_CTRL_CTR_WIDTH_96,
    #[doc = "Counter is 128 bits"]
    AES_CTRL_CTR_WIDTH_128,
}
impl AES_CTRL_CTR_WIDTHR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            AES_CTRL_CTR_WIDTHR::AES_CTRL_CTR_WIDTH_32 => 0,
            AES_CTRL_CTR_WIDTHR::AES_CTRL_CTR_WIDTH_64 => 1,
            AES_CTRL_CTR_WIDTHR::AES_CTRL_CTR_WIDTH_96 => 2,
            AES_CTRL_CTR_WIDTHR::AES_CTRL_CTR_WIDTH_128 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> AES_CTRL_CTR_WIDTHR {
        match value {
            0 => AES_CTRL_CTR_WIDTHR::AES_CTRL_CTR_WIDTH_32,
            1 => AES_CTRL_CTR_WIDTHR::AES_CTRL_CTR_WIDTH_64,
            2 => AES_CTRL_CTR_WIDTHR::AES_CTRL_CTR_WIDTH_96,
            3 => AES_CTRL_CTR_WIDTHR::AES_CTRL_CTR_WIDTH_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_CTR_WIDTH_32`"]
    #[inline(always)]
    pub fn is_aes_ctrl_ctr_width_32(&self) -> bool {
        *self == AES_CTRL_CTR_WIDTHR::AES_CTRL_CTR_WIDTH_32
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_CTR_WIDTH_64`"]
    #[inline(always)]
    pub fn is_aes_ctrl_ctr_width_64(&self) -> bool {
        *self == AES_CTRL_CTR_WIDTHR::AES_CTRL_CTR_WIDTH_64
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_CTR_WIDTH_96`"]
    #[inline(always)]
    pub fn is_aes_ctrl_ctr_width_96(&self) -> bool {
        *self == AES_CTRL_CTR_WIDTHR::AES_CTRL_CTR_WIDTH_96
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_CTR_WIDTH_128`"]
    #[inline(always)]
    pub fn is_aes_ctrl_ctr_width_128(&self) -> bool {
        *self == AES_CTRL_CTR_WIDTHR::AES_CTRL_CTR_WIDTH_128
    }
}
#[doc = "Values that can be written to the field `AES_CTRL_CTR_WIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AES_CTRL_CTR_WIDTHW {
    #[doc = "Counter is 32 bits"]
    AES_CTRL_CTR_WIDTH_32,
    #[doc = "Counter is 64 bits"]
    AES_CTRL_CTR_WIDTH_64,
    #[doc = "Counter is 96 bits"]
    AES_CTRL_CTR_WIDTH_96,
    #[doc = "Counter is 128 bits"]
    AES_CTRL_CTR_WIDTH_128,
}
impl AES_CTRL_CTR_WIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            AES_CTRL_CTR_WIDTHW::AES_CTRL_CTR_WIDTH_32 => 0,
            AES_CTRL_CTR_WIDTHW::AES_CTRL_CTR_WIDTH_64 => 1,
            AES_CTRL_CTR_WIDTHW::AES_CTRL_CTR_WIDTH_96 => 2,
            AES_CTRL_CTR_WIDTHW::AES_CTRL_CTR_WIDTH_128 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _AES_CTRL_CTR_WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_CTRL_CTR_WIDTHW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AES_CTRL_CTR_WIDTHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Counter is 32 bits"]
    #[inline(always)]
    pub fn aes_ctrl_ctr_width_32(self) -> &'a mut W {
        self.variant(AES_CTRL_CTR_WIDTHW::AES_CTRL_CTR_WIDTH_32)
    }
    #[doc = "Counter is 64 bits"]
    #[inline(always)]
    pub fn aes_ctrl_ctr_width_64(self) -> &'a mut W {
        self.variant(AES_CTRL_CTR_WIDTHW::AES_CTRL_CTR_WIDTH_64)
    }
    #[doc = "Counter is 96 bits"]
    #[inline(always)]
    pub fn aes_ctrl_ctr_width_96(self) -> &'a mut W {
        self.variant(AES_CTRL_CTR_WIDTHW::AES_CTRL_CTR_WIDTH_96)
    }
    #[doc = "Counter is 128 bits"]
    #[inline(always)]
    pub fn aes_ctrl_ctr_width_128(self) -> &'a mut W {
        self.variant(AES_CTRL_CTR_WIDTHW::AES_CTRL_CTR_WIDTH_128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 7);
        self.w.bits |= ((value as u32) & 3) << 7;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct AES_CTRL_ICMR {
    bits: bool,
}
impl AES_CTRL_ICMR {
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
pub struct _AES_CTRL_ICMW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_CTRL_ICMW<'a> {
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
        self.w.bits &= !(1 << 9);
        self.w.bits |= ((value as u32) & 1) << 9;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct AES_CTRL_CFBR {
    bits: bool,
}
impl AES_CTRL_CFBR {
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
pub struct _AES_CTRL_CFBW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_CTRL_CFBW<'a> {
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
        self.w.bits &= !(1 << 10);
        self.w.bits |= ((value as u32) & 1) << 10;
        self.w
    }
}
#[doc = "Possible values of the field `AES_CTRL_XTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AES_CTRL_XTSR {
    #[doc = "No operation"]
    AES_CTRL_XTS_NOP,
    #[doc = "Previous/intermediate tweak value and j loaded (value is loaded via IV, j is loaded via the AAD length register)"]
    AES_CTRL_XTS_TWEAKJL,
    #[doc = "Key2, n and j are loaded (n is loaded via IV, j is loaded via the AAD length register)"]
    AES_CTRL_XTS_K2IJL,
    #[doc = "Key2 and n are loaded; j=0 (n is loaded via IV)"]
    AES_CTRL_XTS_K2ILJ0,
}
impl AES_CTRL_XTSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            AES_CTRL_XTSR::AES_CTRL_XTS_NOP => 0,
            AES_CTRL_XTSR::AES_CTRL_XTS_TWEAKJL => 1,
            AES_CTRL_XTSR::AES_CTRL_XTS_K2IJL => 2,
            AES_CTRL_XTSR::AES_CTRL_XTS_K2ILJ0 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> AES_CTRL_XTSR {
        match value {
            0 => AES_CTRL_XTSR::AES_CTRL_XTS_NOP,
            1 => AES_CTRL_XTSR::AES_CTRL_XTS_TWEAKJL,
            2 => AES_CTRL_XTSR::AES_CTRL_XTS_K2IJL,
            3 => AES_CTRL_XTSR::AES_CTRL_XTS_K2ILJ0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_XTS_NOP`"]
    #[inline(always)]
    pub fn is_aes_ctrl_xts_nop(&self) -> bool {
        *self == AES_CTRL_XTSR::AES_CTRL_XTS_NOP
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_XTS_TWEAKJL`"]
    #[inline(always)]
    pub fn is_aes_ctrl_xts_tweakjl(&self) -> bool {
        *self == AES_CTRL_XTSR::AES_CTRL_XTS_TWEAKJL
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_XTS_K2IJL`"]
    #[inline(always)]
    pub fn is_aes_ctrl_xts_k2ijl(&self) -> bool {
        *self == AES_CTRL_XTSR::AES_CTRL_XTS_K2IJL
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_XTS_K2ILJ0`"]
    #[inline(always)]
    pub fn is_aes_ctrl_xts_k2ilj0(&self) -> bool {
        *self == AES_CTRL_XTSR::AES_CTRL_XTS_K2ILJ0
    }
}
#[doc = "Values that can be written to the field `AES_CTRL_XTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AES_CTRL_XTSW {
    #[doc = "No operation"]
    AES_CTRL_XTS_NOP,
    #[doc = "Previous/intermediate tweak value and j loaded (value is loaded via IV, j is loaded via the AAD length register)"]
    AES_CTRL_XTS_TWEAKJL,
    #[doc = "Key2, n and j are loaded (n is loaded via IV, j is loaded via the AAD length register)"]
    AES_CTRL_XTS_K2IJL,
    #[doc = "Key2 and n are loaded; j=0 (n is loaded via IV)"]
    AES_CTRL_XTS_K2ILJ0,
}
impl AES_CTRL_XTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            AES_CTRL_XTSW::AES_CTRL_XTS_NOP => 0,
            AES_CTRL_XTSW::AES_CTRL_XTS_TWEAKJL => 1,
            AES_CTRL_XTSW::AES_CTRL_XTS_K2IJL => 2,
            AES_CTRL_XTSW::AES_CTRL_XTS_K2ILJ0 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _AES_CTRL_XTSW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_CTRL_XTSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AES_CTRL_XTSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn aes_ctrl_xts_nop(self) -> &'a mut W {
        self.variant(AES_CTRL_XTSW::AES_CTRL_XTS_NOP)
    }
    #[doc = "Previous/intermediate tweak value and j loaded (value is loaded via IV, j is loaded via the AAD length register)"]
    #[inline(always)]
    pub fn aes_ctrl_xts_tweakjl(self) -> &'a mut W {
        self.variant(AES_CTRL_XTSW::AES_CTRL_XTS_TWEAKJL)
    }
    #[doc = "Key2, n and j are loaded (n is loaded via IV, j is loaded via the AAD length register)"]
    #[inline(always)]
    pub fn aes_ctrl_xts_k2ijl(self) -> &'a mut W {
        self.variant(AES_CTRL_XTSW::AES_CTRL_XTS_K2IJL)
    }
    #[doc = "Key2 and n are loaded; j=0 (n is loaded via IV)"]
    #[inline(always)]
    pub fn aes_ctrl_xts_k2ilj0(self) -> &'a mut W {
        self.variant(AES_CTRL_XTSW::AES_CTRL_XTS_K2ILJ0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 11);
        self.w.bits |= ((value as u32) & 3) << 11;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct AES_CTRL_F8R {
    bits: bool,
}
impl AES_CTRL_F8R {
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
pub struct _AES_CTRL_F8W<'a> {
    w: &'a mut W,
}
impl<'a> _AES_CTRL_F8W<'a> {
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
        self.w.bits &= !(1 << 13);
        self.w.bits |= ((value as u32) & 1) << 13;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct AES_CTRL_F9R {
    bits: bool,
}
impl AES_CTRL_F9R {
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
pub struct _AES_CTRL_F9W<'a> {
    w: &'a mut W,
}
impl<'a> _AES_CTRL_F9W<'a> {
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
        self.w.bits &= !(1 << 14);
        self.w.bits |= ((value as u32) & 1) << 14;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct AES_CTRL_CBCMACR {
    bits: bool,
}
impl AES_CTRL_CBCMACR {
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
pub struct _AES_CTRL_CBCMACW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_CTRL_CBCMACW<'a> {
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
        self.w.bits &= !(1 << 15);
        self.w.bits |= ((value as u32) & 1) << 15;
        self.w
    }
}
#[doc = "Possible values of the field `AES_CTRL_GCM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AES_CTRL_GCMR {
    #[doc = "No operation"]
    AES_CTRL_GCM_NOP,
    #[doc = "GHASH with H loaded and Y0-encrypted forced to zero"]
    AES_CTRL_GCM_HLY0ZERO,
    #[doc = "GHASH with H loaded and Y0-encrypted calculated internally"]
    AES_CTRL_GCM_HLY0CALC,
    #[doc = "Autonomous GHASH (both H and Y0-encrypted calculated internally)"]
    AES_CTRL_GCM_HY0CALC,
}
impl AES_CTRL_GCMR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            AES_CTRL_GCMR::AES_CTRL_GCM_NOP => 0,
            AES_CTRL_GCMR::AES_CTRL_GCM_HLY0ZERO => 1,
            AES_CTRL_GCMR::AES_CTRL_GCM_HLY0CALC => 2,
            AES_CTRL_GCMR::AES_CTRL_GCM_HY0CALC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> AES_CTRL_GCMR {
        match value {
            0 => AES_CTRL_GCMR::AES_CTRL_GCM_NOP,
            1 => AES_CTRL_GCMR::AES_CTRL_GCM_HLY0ZERO,
            2 => AES_CTRL_GCMR::AES_CTRL_GCM_HLY0CALC,
            3 => AES_CTRL_GCMR::AES_CTRL_GCM_HY0CALC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_GCM_NOP`"]
    #[inline(always)]
    pub fn is_aes_ctrl_gcm_nop(&self) -> bool {
        *self == AES_CTRL_GCMR::AES_CTRL_GCM_NOP
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_GCM_HLY0ZERO`"]
    #[inline(always)]
    pub fn is_aes_ctrl_gcm_hly0zero(&self) -> bool {
        *self == AES_CTRL_GCMR::AES_CTRL_GCM_HLY0ZERO
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_GCM_HLY0CALC`"]
    #[inline(always)]
    pub fn is_aes_ctrl_gcm_hly0calc(&self) -> bool {
        *self == AES_CTRL_GCMR::AES_CTRL_GCM_HLY0CALC
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_GCM_HY0CALC`"]
    #[inline(always)]
    pub fn is_aes_ctrl_gcm_hy0calc(&self) -> bool {
        *self == AES_CTRL_GCMR::AES_CTRL_GCM_HY0CALC
    }
}
#[doc = "Values that can be written to the field `AES_CTRL_GCM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AES_CTRL_GCMW {
    #[doc = "No operation"]
    AES_CTRL_GCM_NOP,
    #[doc = "GHASH with H loaded and Y0-encrypted forced to zero"]
    AES_CTRL_GCM_HLY0ZERO,
    #[doc = "GHASH with H loaded and Y0-encrypted calculated internally"]
    AES_CTRL_GCM_HLY0CALC,
    #[doc = "Autonomous GHASH (both H and Y0-encrypted calculated internally)"]
    AES_CTRL_GCM_HY0CALC,
}
impl AES_CTRL_GCMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            AES_CTRL_GCMW::AES_CTRL_GCM_NOP => 0,
            AES_CTRL_GCMW::AES_CTRL_GCM_HLY0ZERO => 1,
            AES_CTRL_GCMW::AES_CTRL_GCM_HLY0CALC => 2,
            AES_CTRL_GCMW::AES_CTRL_GCM_HY0CALC => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _AES_CTRL_GCMW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_CTRL_GCMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AES_CTRL_GCMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn aes_ctrl_gcm_nop(self) -> &'a mut W {
        self.variant(AES_CTRL_GCMW::AES_CTRL_GCM_NOP)
    }
    #[doc = "GHASH with H loaded and Y0-encrypted forced to zero"]
    #[inline(always)]
    pub fn aes_ctrl_gcm_hly0zero(self) -> &'a mut W {
        self.variant(AES_CTRL_GCMW::AES_CTRL_GCM_HLY0ZERO)
    }
    #[doc = "GHASH with H loaded and Y0-encrypted calculated internally"]
    #[inline(always)]
    pub fn aes_ctrl_gcm_hly0calc(self) -> &'a mut W {
        self.variant(AES_CTRL_GCMW::AES_CTRL_GCM_HLY0CALC)
    }
    #[doc = "Autonomous GHASH (both H and Y0-encrypted calculated internally)"]
    #[inline(always)]
    pub fn aes_ctrl_gcm_hy0calc(self) -> &'a mut W {
        self.variant(AES_CTRL_GCMW::AES_CTRL_GCM_HY0CALC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 16);
        self.w.bits |= ((value as u32) & 3) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct AES_CTRL_CCMR {
    bits: bool,
}
impl AES_CTRL_CCMR {
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
pub struct _AES_CTRL_CCMW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_CTRL_CCMW<'a> {
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
        self.w.bits &= !(1 << 18);
        self.w.bits |= ((value as u32) & 1) << 18;
        self.w
    }
}
#[doc = "Possible values of the field `AES_CTRL_CCM_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AES_CTRL_CCM_LR {
    #[doc = "width = 2"]
    AES_CTRL_CCM_L_2,
    #[doc = "width = 4"]
    AES_CTRL_CCM_L_4,
    #[doc = "width = 8"]
    AES_CTRL_CCM_L_8,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl AES_CTRL_CCM_LR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            AES_CTRL_CCM_LR::AES_CTRL_CCM_L_2 => 1,
            AES_CTRL_CCM_LR::AES_CTRL_CCM_L_4 => 3,
            AES_CTRL_CCM_LR::AES_CTRL_CCM_L_8 => 7,
            AES_CTRL_CCM_LR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> AES_CTRL_CCM_LR {
        match value {
            1 => AES_CTRL_CCM_LR::AES_CTRL_CCM_L_2,
            3 => AES_CTRL_CCM_LR::AES_CTRL_CCM_L_4,
            7 => AES_CTRL_CCM_LR::AES_CTRL_CCM_L_8,
            i => AES_CTRL_CCM_LR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_CCM_L_2`"]
    #[inline(always)]
    pub fn is_aes_ctrl_ccm_l_2(&self) -> bool {
        *self == AES_CTRL_CCM_LR::AES_CTRL_CCM_L_2
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_CCM_L_4`"]
    #[inline(always)]
    pub fn is_aes_ctrl_ccm_l_4(&self) -> bool {
        *self == AES_CTRL_CCM_LR::AES_CTRL_CCM_L_4
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_CCM_L_8`"]
    #[inline(always)]
    pub fn is_aes_ctrl_ccm_l_8(&self) -> bool {
        *self == AES_CTRL_CCM_LR::AES_CTRL_CCM_L_8
    }
}
#[doc = "Values that can be written to the field `AES_CTRL_CCM_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AES_CTRL_CCM_LW {
    #[doc = "width = 2"]
    AES_CTRL_CCM_L_2,
    #[doc = "width = 4"]
    AES_CTRL_CCM_L_4,
    #[doc = "width = 8"]
    AES_CTRL_CCM_L_8,
}
impl AES_CTRL_CCM_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            AES_CTRL_CCM_LW::AES_CTRL_CCM_L_2 => 1,
            AES_CTRL_CCM_LW::AES_CTRL_CCM_L_4 => 3,
            AES_CTRL_CCM_LW::AES_CTRL_CCM_L_8 => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _AES_CTRL_CCM_LW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_CTRL_CCM_LW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AES_CTRL_CCM_LW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "width = 2"]
    #[inline(always)]
    pub fn aes_ctrl_ccm_l_2(self) -> &'a mut W {
        self.variant(AES_CTRL_CCM_LW::AES_CTRL_CCM_L_2)
    }
    #[doc = "width = 4"]
    #[inline(always)]
    pub fn aes_ctrl_ccm_l_4(self) -> &'a mut W {
        self.variant(AES_CTRL_CCM_LW::AES_CTRL_CCM_L_4)
    }
    #[doc = "width = 8"]
    #[inline(always)]
    pub fn aes_ctrl_ccm_l_8(self) -> &'a mut W {
        self.variant(AES_CTRL_CCM_LW::AES_CTRL_CCM_L_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 19);
        self.w.bits |= ((value as u32) & 7) << 19;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct AES_CTRL_CCM_MR {
    bits: u8,
}
impl AES_CTRL_CCM_MR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _AES_CTRL_CCM_MW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_CTRL_CCM_MW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 22);
        self.w.bits |= ((value as u32) & 7) << 22;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct AES_CTRL_SAVE_CONTEXTR {
    bits: bool,
}
impl AES_CTRL_SAVE_CONTEXTR {
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
pub struct _AES_CTRL_SAVE_CONTEXTW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_CTRL_SAVE_CONTEXTW<'a> {
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
        self.w.bits &= !(1 << 29);
        self.w.bits |= ((value as u32) & 1) << 29;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct AES_CTRL_SVCTXTRDYR {
    bits: bool,
}
impl AES_CTRL_SVCTXTRDYR {
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
pub struct _AES_CTRL_SVCTXTRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_CTRL_SVCTXTRDYW<'a> {
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
        self.w.bits &= !(1 << 30);
        self.w.bits |= ((value as u32) & 1) << 30;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct AES_CTRL_CTXTRDYR {
    bits: bool,
}
impl AES_CTRL_CTXTRDYR {
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
pub struct _AES_CTRL_CTXTRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_CTRL_CTXTRDYW<'a> {
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
        self.w.bits &= !(1 << 31);
        self.w.bits |= ((value as u32) & 1) << 31;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Output Ready Status"]
    #[inline(always)]
    pub fn aes_ctrl_output_ready(&self) -> AES_CTRL_OUTPUT_READYR {
        let bits = ((self.bits >> 0) & 1) != 0;
        AES_CTRL_OUTPUT_READYR { bits }
    }
    #[doc = "Bit 1 - Input Ready Status"]
    #[inline(always)]
    pub fn aes_ctrl_input_ready(&self) -> AES_CTRL_INPUT_READYR {
        let bits = ((self.bits >> 1) & 1) != 0;
        AES_CTRL_INPUT_READYR { bits }
    }
    #[doc = "Bit 2 - Encryption/Decryption Selection"]
    #[inline(always)]
    pub fn aes_ctrl_direction(&self) -> AES_CTRL_DIRECTIONR {
        let bits = ((self.bits >> 2) & 1) != 0;
        AES_CTRL_DIRECTIONR { bits }
    }
    #[doc = "Bits 3:4 - Key Size"]
    #[inline(always)]
    pub fn aes_ctrl_key_size(&self) -> AES_CTRL_KEY_SIZER {
        AES_CTRL_KEY_SIZER::_from(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - ECB/CBC Mode"]
    #[inline(always)]
    pub fn aes_ctrl_mode(&self) -> AES_CTRL_MODER {
        let bits = ((self.bits >> 5) & 1) != 0;
        AES_CTRL_MODER { bits }
    }
    #[doc = "Bit 6 - Counter Mode"]
    #[inline(always)]
    pub fn aes_ctrl_ctr(&self) -> AES_CTRL_CTRR {
        let bits = ((self.bits >> 6) & 1) != 0;
        AES_CTRL_CTRR { bits }
    }
    #[doc = "Bits 7:8 - AES-CTR Mode Counter Width"]
    #[inline(always)]
    pub fn aes_ctrl_ctr_width(&self) -> AES_CTRL_CTR_WIDTHR {
        AES_CTRL_CTR_WIDTHR::_from(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - AES Integer Counter Mode (ICM) Enable"]
    #[inline(always)]
    pub fn aes_ctrl_icm(&self) -> AES_CTRL_ICMR {
        let bits = ((self.bits >> 9) & 1) != 0;
        AES_CTRL_ICMR { bits }
    }
    #[doc = "Bit 10 - Full block AES cipher feedback mode (CFB128) Enable"]
    #[inline(always)]
    pub fn aes_ctrl_cfb(&self) -> AES_CTRL_CFBR {
        let bits = ((self.bits >> 10) & 1) != 0;
        AES_CTRL_CFBR { bits }
    }
    #[doc = "Bits 11:12 - AES-XTS Operation Enabled"]
    #[inline(always)]
    pub fn aes_ctrl_xts(&self) -> AES_CTRL_XTSR {
        AES_CTRL_XTSR::_from(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - AES f8 Mode Enable"]
    #[inline(always)]
    pub fn aes_ctrl_f8(&self) -> AES_CTRL_F8R {
        let bits = ((self.bits >> 13) & 1) != 0;
        AES_CTRL_F8R { bits }
    }
    #[doc = "Bit 14 - AES f9 Mode Enable"]
    #[inline(always)]
    pub fn aes_ctrl_f9(&self) -> AES_CTRL_F9R {
        let bits = ((self.bits >> 14) & 1) != 0;
        AES_CTRL_F9R { bits }
    }
    #[doc = "Bit 15 - AES-CBC MAC Enable"]
    #[inline(always)]
    pub fn aes_ctrl_cbcmac(&self) -> AES_CTRL_CBCMACR {
        let bits = ((self.bits >> 15) & 1) != 0;
        AES_CTRL_CBCMACR { bits }
    }
    #[doc = "Bits 16:17 - AES-GCM Mode Enable"]
    #[inline(always)]
    pub fn aes_ctrl_gcm(&self) -> AES_CTRL_GCMR {
        AES_CTRL_GCMR::_from(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - AES-CCM Mode Enable"]
    #[inline(always)]
    pub fn aes_ctrl_ccm(&self) -> AES_CTRL_CCMR {
        let bits = ((self.bits >> 18) & 1) != 0;
        AES_CTRL_CCMR { bits }
    }
    #[doc = "Bits 19:21 - L Value"]
    #[inline(always)]
    pub fn aes_ctrl_ccm_l(&self) -> AES_CTRL_CCM_LR {
        AES_CTRL_CCM_LR::_from(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - Counter with CBC-MAC (CCM)"]
    #[inline(always)]
    pub fn aes_ctrl_ccm_m(&self) -> AES_CTRL_CCM_MR {
        let bits = ((self.bits >> 22) & 7) as u8;
        AES_CTRL_CCM_MR { bits }
    }
    #[doc = "Bit 29 - TAG or Result IV Save"]
    #[inline(always)]
    pub fn aes_ctrl_save_context(&self) -> AES_CTRL_SAVE_CONTEXTR {
        let bits = ((self.bits >> 29) & 1) != 0;
        AES_CTRL_SAVE_CONTEXTR { bits }
    }
    #[doc = "Bit 30 - AES TAG/IV Block(s) Ready"]
    #[inline(always)]
    pub fn aes_ctrl_svctxtrdy(&self) -> AES_CTRL_SVCTXTRDYR {
        let bits = ((self.bits >> 30) & 1) != 0;
        AES_CTRL_SVCTXTRDYR { bits }
    }
    #[doc = "Bit 31 - Context Data Registers Ready"]
    #[inline(always)]
    pub fn aes_ctrl_ctxtrdy(&self) -> AES_CTRL_CTXTRDYR {
        let bits = ((self.bits >> 31) & 1) != 0;
        AES_CTRL_CTXTRDYR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Output Ready Status"]
    #[inline(always)]
    pub fn aes_ctrl_output_ready(&mut self) -> _AES_CTRL_OUTPUT_READYW {
        _AES_CTRL_OUTPUT_READYW { w: self }
    }
    #[doc = "Bit 1 - Input Ready Status"]
    #[inline(always)]
    pub fn aes_ctrl_input_ready(&mut self) -> _AES_CTRL_INPUT_READYW {
        _AES_CTRL_INPUT_READYW { w: self }
    }
    #[doc = "Bit 2 - Encryption/Decryption Selection"]
    #[inline(always)]
    pub fn aes_ctrl_direction(&mut self) -> _AES_CTRL_DIRECTIONW {
        _AES_CTRL_DIRECTIONW { w: self }
    }
    #[doc = "Bits 3:4 - Key Size"]
    #[inline(always)]
    pub fn aes_ctrl_key_size(&mut self) -> _AES_CTRL_KEY_SIZEW {
        _AES_CTRL_KEY_SIZEW { w: self }
    }
    #[doc = "Bit 5 - ECB/CBC Mode"]
    #[inline(always)]
    pub fn aes_ctrl_mode(&mut self) -> _AES_CTRL_MODEW {
        _AES_CTRL_MODEW { w: self }
    }
    #[doc = "Bit 6 - Counter Mode"]
    #[inline(always)]
    pub fn aes_ctrl_ctr(&mut self) -> _AES_CTRL_CTRW {
        _AES_CTRL_CTRW { w: self }
    }
    #[doc = "Bits 7:8 - AES-CTR Mode Counter Width"]
    #[inline(always)]
    pub fn aes_ctrl_ctr_width(&mut self) -> _AES_CTRL_CTR_WIDTHW {
        _AES_CTRL_CTR_WIDTHW { w: self }
    }
    #[doc = "Bit 9 - AES Integer Counter Mode (ICM) Enable"]
    #[inline(always)]
    pub fn aes_ctrl_icm(&mut self) -> _AES_CTRL_ICMW {
        _AES_CTRL_ICMW { w: self }
    }
    #[doc = "Bit 10 - Full block AES cipher feedback mode (CFB128) Enable"]
    #[inline(always)]
    pub fn aes_ctrl_cfb(&mut self) -> _AES_CTRL_CFBW {
        _AES_CTRL_CFBW { w: self }
    }
    #[doc = "Bits 11:12 - AES-XTS Operation Enabled"]
    #[inline(always)]
    pub fn aes_ctrl_xts(&mut self) -> _AES_CTRL_XTSW {
        _AES_CTRL_XTSW { w: self }
    }
    #[doc = "Bit 13 - AES f8 Mode Enable"]
    #[inline(always)]
    pub fn aes_ctrl_f8(&mut self) -> _AES_CTRL_F8W {
        _AES_CTRL_F8W { w: self }
    }
    #[doc = "Bit 14 - AES f9 Mode Enable"]
    #[inline(always)]
    pub fn aes_ctrl_f9(&mut self) -> _AES_CTRL_F9W {
        _AES_CTRL_F9W { w: self }
    }
    #[doc = "Bit 15 - AES-CBC MAC Enable"]
    #[inline(always)]
    pub fn aes_ctrl_cbcmac(&mut self) -> _AES_CTRL_CBCMACW {
        _AES_CTRL_CBCMACW { w: self }
    }
    #[doc = "Bits 16:17 - AES-GCM Mode Enable"]
    #[inline(always)]
    pub fn aes_ctrl_gcm(&mut self) -> _AES_CTRL_GCMW {
        _AES_CTRL_GCMW { w: self }
    }
    #[doc = "Bit 18 - AES-CCM Mode Enable"]
    #[inline(always)]
    pub fn aes_ctrl_ccm(&mut self) -> _AES_CTRL_CCMW {
        _AES_CTRL_CCMW { w: self }
    }
    #[doc = "Bits 19:21 - L Value"]
    #[inline(always)]
    pub fn aes_ctrl_ccm_l(&mut self) -> _AES_CTRL_CCM_LW {
        _AES_CTRL_CCM_LW { w: self }
    }
    #[doc = "Bits 22:24 - Counter with CBC-MAC (CCM)"]
    #[inline(always)]
    pub fn aes_ctrl_ccm_m(&mut self) -> _AES_CTRL_CCM_MW {
        _AES_CTRL_CCM_MW { w: self }
    }
    #[doc = "Bit 29 - TAG or Result IV Save"]
    #[inline(always)]
    pub fn aes_ctrl_save_context(&mut self) -> _AES_CTRL_SAVE_CONTEXTW {
        _AES_CTRL_SAVE_CONTEXTW { w: self }
    }
    #[doc = "Bit 30 - AES TAG/IV Block(s) Ready"]
    #[inline(always)]
    pub fn aes_ctrl_svctxtrdy(&mut self) -> _AES_CTRL_SVCTXTRDYW {
        _AES_CTRL_SVCTXTRDYW { w: self }
    }
    #[doc = "Bit 31 - Context Data Registers Ready"]
    #[inline(always)]
    pub fn aes_ctrl_ctxtrdy(&mut self) -> _AES_CTRL_CTXTRDYW {
        _AES_CTRL_CTXTRDYW { w: self }
    }
}
