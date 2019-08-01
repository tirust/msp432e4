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
pub struct DES_CTRL_OUTPUT_READYR {
    bits: bool,
}
impl DES_CTRL_OUTPUT_READYR {
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
pub struct _DES_CTRL_OUTPUT_READYW<'a> {
    w: &'a mut W,
}
impl<'a> _DES_CTRL_OUTPUT_READYW<'a> {
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
pub struct DES_CTRL_INPUT_READYR {
    bits: bool,
}
impl DES_CTRL_INPUT_READYR {
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
pub struct _DES_CTRL_INPUT_READYW<'a> {
    w: &'a mut W,
}
impl<'a> _DES_CTRL_INPUT_READYW<'a> {
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
pub struct DES_CTRL_DIRECTIONR {
    bits: bool,
}
impl DES_CTRL_DIRECTIONR {
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
pub struct _DES_CTRL_DIRECTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _DES_CTRL_DIRECTIONW<'a> {
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
#[doc = r"Value of the field"]
pub struct DES_CTRL_TDESR {
    bits: bool,
}
impl DES_CTRL_TDESR {
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
pub struct _DES_CTRL_TDESW<'a> {
    w: &'a mut W,
}
impl<'a> _DES_CTRL_TDESW<'a> {
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
        self.w.bits &= !(1 << 3);
        self.w.bits |= ((value as u32) & 1) << 3;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct DES_CTRL_MODER {
    bits: u8,
}
impl DES_CTRL_MODER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _DES_CTRL_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DES_CTRL_MODEW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct DES_CTRL_CONTEXTR {
    bits: bool,
}
impl DES_CTRL_CONTEXTR {
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
pub struct _DES_CTRL_CONTEXTW<'a> {
    w: &'a mut W,
}
impl<'a> _DES_CTRL_CONTEXTW<'a> {
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
    #[doc = "Bit 0 - When 1, Data decrypted/encrypted ready"]
    #[inline(always)]
    pub fn des_ctrl_output_ready(&self) -> DES_CTRL_OUTPUT_READYR {
        let bits = ((self.bits >> 0) & 1) != 0;
        DES_CTRL_OUTPUT_READYR { bits }
    }
    #[doc = "Bit 1 - When 1, ready to encrypt/decrypt data"]
    #[inline(always)]
    pub fn des_ctrl_input_ready(&self) -> DES_CTRL_INPUT_READYR {
        let bits = ((self.bits >> 1) & 1) != 0;
        DES_CTRL_INPUT_READYR { bits }
    }
    #[doc = "Bit 2 - Select encryption/decryption 0x0: decryption is selected0x1: Encryption is selected"]
    #[inline(always)]
    pub fn des_ctrl_direction(&self) -> DES_CTRL_DIRECTIONR {
        let bits = ((self.bits >> 2) & 1) != 0;
        DES_CTRL_DIRECTIONR { bits }
    }
    #[doc = "Bit 3 - Select DES or triple DES encryption/decryption"]
    #[inline(always)]
    pub fn des_ctrl_tdes(&self) -> DES_CTRL_TDESR {
        let bits = ((self.bits >> 3) & 1) != 0;
        DES_CTRL_TDESR { bits }
    }
    #[doc = "Bits 4:5 - Select CBC, ECB or CFB mode0x0: ECB mode0x1: CBC mode0x2: CFB mode0x3: reserved"]
    #[inline(always)]
    pub fn des_ctrl_mode(&self) -> DES_CTRL_MODER {
        let bits = ((self.bits >> 4) & 3) as u8;
        DES_CTRL_MODER { bits }
    }
    #[doc = "Bit 31 - If 1, this read-only status bit indicates that the context data registers can be overwritten and the host is permitted to write the next context"]
    #[inline(always)]
    pub fn des_ctrl_context(&self) -> DES_CTRL_CONTEXTR {
        let bits = ((self.bits >> 31) & 1) != 0;
        DES_CTRL_CONTEXTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - When 1, Data decrypted/encrypted ready"]
    #[inline(always)]
    pub fn des_ctrl_output_ready(&mut self) -> _DES_CTRL_OUTPUT_READYW {
        _DES_CTRL_OUTPUT_READYW { w: self }
    }
    #[doc = "Bit 1 - When 1, ready to encrypt/decrypt data"]
    #[inline(always)]
    pub fn des_ctrl_input_ready(&mut self) -> _DES_CTRL_INPUT_READYW {
        _DES_CTRL_INPUT_READYW { w: self }
    }
    #[doc = "Bit 2 - Select encryption/decryption 0x0: decryption is selected0x1: Encryption is selected"]
    #[inline(always)]
    pub fn des_ctrl_direction(&mut self) -> _DES_CTRL_DIRECTIONW {
        _DES_CTRL_DIRECTIONW { w: self }
    }
    #[doc = "Bit 3 - Select DES or triple DES encryption/decryption"]
    #[inline(always)]
    pub fn des_ctrl_tdes(&mut self) -> _DES_CTRL_TDESW {
        _DES_CTRL_TDESW { w: self }
    }
    #[doc = "Bits 4:5 - Select CBC, ECB or CFB mode0x0: ECB mode0x1: CBC mode0x2: CFB mode0x3: reserved"]
    #[inline(always)]
    pub fn des_ctrl_mode(&mut self) -> _DES_CTRL_MODEW {
        _DES_CTRL_MODEW { w: self }
    }
    #[doc = "Bit 31 - If 1, this read-only status bit indicates that the context data registers can be overwritten and the host is permitted to write the next context"]
    #[inline(always)]
    pub fn des_ctrl_context(&mut self) -> _DES_CTRL_CONTEXTW {
        _DES_CTRL_CONTEXTW { w: self }
    }
}
