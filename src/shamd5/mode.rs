#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODE {
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
#[doc = "Possible values of the field `SHAMD5_MODE_ALGO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHAMD5_MODE_ALGOR {
    #[doc = "MD5"]
    SHAMD5_MODE_ALGO_MD5,
    #[doc = "SHA-1"]
    SHAMD5_MODE_ALGO_SHA1,
    #[doc = "SHA-224"]
    SHAMD5_MODE_ALGO_SHA224,
    #[doc = "SHA-256"]
    SHAMD5_MODE_ALGO_SHA256,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SHAMD5_MODE_ALGOR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SHAMD5_MODE_ALGOR::SHAMD5_MODE_ALGO_MD5 => 0,
            SHAMD5_MODE_ALGOR::SHAMD5_MODE_ALGO_SHA1 => 2,
            SHAMD5_MODE_ALGOR::SHAMD5_MODE_ALGO_SHA224 => 4,
            SHAMD5_MODE_ALGOR::SHAMD5_MODE_ALGO_SHA256 => 6,
            SHAMD5_MODE_ALGOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SHAMD5_MODE_ALGOR {
        match value {
            0 => SHAMD5_MODE_ALGOR::SHAMD5_MODE_ALGO_MD5,
            2 => SHAMD5_MODE_ALGOR::SHAMD5_MODE_ALGO_SHA1,
            4 => SHAMD5_MODE_ALGOR::SHAMD5_MODE_ALGO_SHA224,
            6 => SHAMD5_MODE_ALGOR::SHAMD5_MODE_ALGO_SHA256,
            i => SHAMD5_MODE_ALGOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SHAMD5_MODE_ALGO_MD5`"]
    #[inline(always)]
    pub fn is_shamd5_mode_algo_md5(&self) -> bool {
        *self == SHAMD5_MODE_ALGOR::SHAMD5_MODE_ALGO_MD5
    }
    #[doc = "Checks if the value of the field is `SHAMD5_MODE_ALGO_SHA1`"]
    #[inline(always)]
    pub fn is_shamd5_mode_algo_sha1(&self) -> bool {
        *self == SHAMD5_MODE_ALGOR::SHAMD5_MODE_ALGO_SHA1
    }
    #[doc = "Checks if the value of the field is `SHAMD5_MODE_ALGO_SHA224`"]
    #[inline(always)]
    pub fn is_shamd5_mode_algo_sha224(&self) -> bool {
        *self == SHAMD5_MODE_ALGOR::SHAMD5_MODE_ALGO_SHA224
    }
    #[doc = "Checks if the value of the field is `SHAMD5_MODE_ALGO_SHA256`"]
    #[inline(always)]
    pub fn is_shamd5_mode_algo_sha256(&self) -> bool {
        *self == SHAMD5_MODE_ALGOR::SHAMD5_MODE_ALGO_SHA256
    }
}
#[doc = "Values that can be written to the field `SHAMD5_MODE_ALGO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHAMD5_MODE_ALGOW {
    #[doc = "MD5"]
    SHAMD5_MODE_ALGO_MD5,
    #[doc = "SHA-1"]
    SHAMD5_MODE_ALGO_SHA1,
    #[doc = "SHA-224"]
    SHAMD5_MODE_ALGO_SHA224,
    #[doc = "SHA-256"]
    SHAMD5_MODE_ALGO_SHA256,
}
impl SHAMD5_MODE_ALGOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SHAMD5_MODE_ALGOW::SHAMD5_MODE_ALGO_MD5 => 0,
            SHAMD5_MODE_ALGOW::SHAMD5_MODE_ALGO_SHA1 => 2,
            SHAMD5_MODE_ALGOW::SHAMD5_MODE_ALGO_SHA224 => 4,
            SHAMD5_MODE_ALGOW::SHAMD5_MODE_ALGO_SHA256 => 6,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SHAMD5_MODE_ALGOW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAMD5_MODE_ALGOW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHAMD5_MODE_ALGOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "MD5"]
    #[inline(always)]
    pub fn shamd5_mode_algo_md5(self) -> &'a mut W {
        self.variant(SHAMD5_MODE_ALGOW::SHAMD5_MODE_ALGO_MD5)
    }
    #[doc = "SHA-1"]
    #[inline(always)]
    pub fn shamd5_mode_algo_sha1(self) -> &'a mut W {
        self.variant(SHAMD5_MODE_ALGOW::SHAMD5_MODE_ALGO_SHA1)
    }
    #[doc = "SHA-224"]
    #[inline(always)]
    pub fn shamd5_mode_algo_sha224(self) -> &'a mut W {
        self.variant(SHAMD5_MODE_ALGOW::SHAMD5_MODE_ALGO_SHA224)
    }
    #[doc = "SHA-256"]
    #[inline(always)]
    pub fn shamd5_mode_algo_sha256(self) -> &'a mut W {
        self.variant(SHAMD5_MODE_ALGOW::SHAMD5_MODE_ALGO_SHA256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 0);
        self.w.bits |= ((value as u32) & 7) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SHAMD5_MODE_ALGO_CONSTANTR {
    bits: bool,
}
impl SHAMD5_MODE_ALGO_CONSTANTR {
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
pub struct _SHAMD5_MODE_ALGO_CONSTANTW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAMD5_MODE_ALGO_CONSTANTW<'a> {
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
pub struct SHAMD5_MODE_CLOSE_HASHR {
    bits: bool,
}
impl SHAMD5_MODE_CLOSE_HASHR {
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
pub struct _SHAMD5_MODE_CLOSE_HASHW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAMD5_MODE_CLOSE_HASHW<'a> {
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
pub struct SHAMD5_MODE_HMAC_KEY_PROCR {
    bits: bool,
}
impl SHAMD5_MODE_HMAC_KEY_PROCR {
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
pub struct _SHAMD5_MODE_HMAC_KEY_PROCW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAMD5_MODE_HMAC_KEY_PROCW<'a> {
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
pub struct SHAMD5_MODE_HMAC_OUTER_HASHR {
    bits: bool,
}
impl SHAMD5_MODE_HMAC_OUTER_HASHR {
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
pub struct _SHAMD5_MODE_HMAC_OUTER_HASHW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAMD5_MODE_HMAC_OUTER_HASHW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Hash Algorithm"]
    #[inline(always)]
    pub fn shamd5_mode_algo(&self) -> SHAMD5_MODE_ALGOR {
        SHAMD5_MODE_ALGOR::_from(((self.bits >> 0) & 7) as u8)
    }
    #[doc = "Bit 3 - The initial digest register will be overwritten with the algorithm constants for the selected algorithm when hashing and the initial digest count register will be reset to 0"]
    #[inline(always)]
    pub fn shamd5_mode_algo_constant(&self) -> SHAMD5_MODE_ALGO_CONSTANTR {
        let bits = ((self.bits >> 3) & 1) != 0;
        SHAMD5_MODE_ALGO_CONSTANTR { bits }
    }
    #[doc = "Bit 4 - Performs the padding, the Hash/HMAC will be 'closed' at the end of the block, as per MD5/SHA-1/SHA-2 specification"]
    #[inline(always)]
    pub fn shamd5_mode_close_hash(&self) -> SHAMD5_MODE_CLOSE_HASHR {
        let bits = ((self.bits >> 4) & 1) != 0;
        SHAMD5_MODE_CLOSE_HASHR { bits }
    }
    #[doc = "Bit 5 - HMAC Key Processing Enable"]
    #[inline(always)]
    pub fn shamd5_mode_hmac_key_proc(&self) -> SHAMD5_MODE_HMAC_KEY_PROCR {
        let bits = ((self.bits >> 5) & 1) != 0;
        SHAMD5_MODE_HMAC_KEY_PROCR { bits }
    }
    #[doc = "Bit 7 - HMAC Outer Hash Processing Enable"]
    #[inline(always)]
    pub fn shamd5_mode_hmac_outer_hash(&self) -> SHAMD5_MODE_HMAC_OUTER_HASHR {
        let bits = ((self.bits >> 7) & 1) != 0;
        SHAMD5_MODE_HMAC_OUTER_HASHR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Hash Algorithm"]
    #[inline(always)]
    pub fn shamd5_mode_algo(&mut self) -> _SHAMD5_MODE_ALGOW {
        _SHAMD5_MODE_ALGOW { w: self }
    }
    #[doc = "Bit 3 - The initial digest register will be overwritten with the algorithm constants for the selected algorithm when hashing and the initial digest count register will be reset to 0"]
    #[inline(always)]
    pub fn shamd5_mode_algo_constant(&mut self) -> _SHAMD5_MODE_ALGO_CONSTANTW {
        _SHAMD5_MODE_ALGO_CONSTANTW { w: self }
    }
    #[doc = "Bit 4 - Performs the padding, the Hash/HMAC will be 'closed' at the end of the block, as per MD5/SHA-1/SHA-2 specification"]
    #[inline(always)]
    pub fn shamd5_mode_close_hash(&mut self) -> _SHAMD5_MODE_CLOSE_HASHW {
        _SHAMD5_MODE_CLOSE_HASHW { w: self }
    }
    #[doc = "Bit 5 - HMAC Key Processing Enable"]
    #[inline(always)]
    pub fn shamd5_mode_hmac_key_proc(&mut self) -> _SHAMD5_MODE_HMAC_KEY_PROCW {
        _SHAMD5_MODE_HMAC_KEY_PROCW { w: self }
    }
    #[doc = "Bit 7 - HMAC Outer Hash Processing Enable"]
    #[inline(always)]
    pub fn shamd5_mode_hmac_outer_hash(&mut self) -> _SHAMD5_MODE_HMAC_OUTER_HASHW {
        _SHAMD5_MODE_HMAC_OUTER_HASHW { w: self }
    }
}
