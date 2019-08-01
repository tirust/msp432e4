#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPICTLSTAT {
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
pub struct EMAC_LPICTLSTAT_TLPIENR {
    bits: bool,
}
impl EMAC_LPICTLSTAT_TLPIENR {
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
pub struct _EMAC_LPICTLSTAT_TLPIENW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_LPICTLSTAT_TLPIENW<'a> {
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
pub struct EMAC_LPICTLSTAT_TLPIEXR {
    bits: bool,
}
impl EMAC_LPICTLSTAT_TLPIEXR {
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
pub struct _EMAC_LPICTLSTAT_TLPIEXW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_LPICTLSTAT_TLPIEXW<'a> {
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
pub struct EMAC_LPICTLSTAT_RLPIENR {
    bits: bool,
}
impl EMAC_LPICTLSTAT_RLPIENR {
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
pub struct _EMAC_LPICTLSTAT_RLPIENW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_LPICTLSTAT_RLPIENW<'a> {
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
pub struct EMAC_LPICTLSTAT_RLPIEXR {
    bits: bool,
}
impl EMAC_LPICTLSTAT_RLPIEXR {
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
pub struct _EMAC_LPICTLSTAT_RLPIEXW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_LPICTLSTAT_RLPIEXW<'a> {
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
pub struct EMAC_LPICTLSTAT_TLPISTR {
    bits: bool,
}
impl EMAC_LPICTLSTAT_TLPISTR {
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
pub struct _EMAC_LPICTLSTAT_TLPISTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_LPICTLSTAT_TLPISTW<'a> {
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
        self.w.bits &= !(1 << 8);
        self.w.bits |= ((value as u32) & 1) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_LPICTLSTAT_RLPISTR {
    bits: bool,
}
impl EMAC_LPICTLSTAT_RLPISTR {
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
pub struct _EMAC_LPICTLSTAT_RLPISTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_LPICTLSTAT_RLPISTW<'a> {
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
pub struct EMAC_LPICTLSTAT_LPIENR {
    bits: bool,
}
impl EMAC_LPICTLSTAT_LPIENR {
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
pub struct _EMAC_LPICTLSTAT_LPIENW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_LPICTLSTAT_LPIENW<'a> {
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
        self.w.bits &= !(1 << 16);
        self.w.bits |= ((value as u32) & 1) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_LPICTLSTAT_PLSR {
    bits: bool,
}
impl EMAC_LPICTLSTAT_PLSR {
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
pub struct _EMAC_LPICTLSTAT_PLSW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_LPICTLSTAT_PLSW<'a> {
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
        self.w.bits &= !(1 << 17);
        self.w.bits |= ((value as u32) & 1) << 17;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_LPICTLSTAT_PLSENR {
    bits: bool,
}
impl EMAC_LPICTLSTAT_PLSENR {
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
pub struct _EMAC_LPICTLSTAT_PLSENW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_LPICTLSTAT_PLSENW<'a> {
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
#[doc = r"Value of the field"]
pub struct EMAC_LPICTLSTAT_LPITXAR {
    bits: bool,
}
impl EMAC_LPICTLSTAT_LPITXAR {
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
pub struct _EMAC_LPICTLSTAT_LPITXAW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_LPICTLSTAT_LPITXAW<'a> {
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
        self.w.bits &= !(1 << 19);
        self.w.bits |= ((value as u32) & 1) << 19;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit LPI Entry"]
    #[inline(always)]
    pub fn emac_lpictlstat_tlpien(&self) -> EMAC_LPICTLSTAT_TLPIENR {
        let bits = ((self.bits >> 0) & 1) != 0;
        EMAC_LPICTLSTAT_TLPIENR { bits }
    }
    #[doc = "Bit 1 - Transmit LPI Exit"]
    #[inline(always)]
    pub fn emac_lpictlstat_tlpiex(&self) -> EMAC_LPICTLSTAT_TLPIEXR {
        let bits = ((self.bits >> 1) & 1) != 0;
        EMAC_LPICTLSTAT_TLPIEXR { bits }
    }
    #[doc = "Bit 2 - Receive LPI Entry"]
    #[inline(always)]
    pub fn emac_lpictlstat_rlpien(&self) -> EMAC_LPICTLSTAT_RLPIENR {
        let bits = ((self.bits >> 2) & 1) != 0;
        EMAC_LPICTLSTAT_RLPIENR { bits }
    }
    #[doc = "Bit 3 - Receive LPI Exit"]
    #[inline(always)]
    pub fn emac_lpictlstat_rlpiex(&self) -> EMAC_LPICTLSTAT_RLPIEXR {
        let bits = ((self.bits >> 3) & 1) != 0;
        EMAC_LPICTLSTAT_RLPIEXR { bits }
    }
    #[doc = "Bit 8 - Transmit LPI State"]
    #[inline(always)]
    pub fn emac_lpictlstat_tlpist(&self) -> EMAC_LPICTLSTAT_TLPISTR {
        let bits = ((self.bits >> 8) & 1) != 0;
        EMAC_LPICTLSTAT_TLPISTR { bits }
    }
    #[doc = "Bit 9 - Receive LPI State"]
    #[inline(always)]
    pub fn emac_lpictlstat_rlpist(&self) -> EMAC_LPICTLSTAT_RLPISTR {
        let bits = ((self.bits >> 9) & 1) != 0;
        EMAC_LPICTLSTAT_RLPISTR { bits }
    }
    #[doc = "Bit 16 - LPI Enable"]
    #[inline(always)]
    pub fn emac_lpictlstat_lpien(&self) -> EMAC_LPICTLSTAT_LPIENR {
        let bits = ((self.bits >> 16) & 1) != 0;
        EMAC_LPICTLSTAT_LPIENR { bits }
    }
    #[doc = "Bit 17 - PHY Link Status"]
    #[inline(always)]
    pub fn emac_lpictlstat_pls(&self) -> EMAC_LPICTLSTAT_PLSR {
        let bits = ((self.bits >> 17) & 1) != 0;
        EMAC_LPICTLSTAT_PLSR { bits }
    }
    #[doc = "Bit 18 - PHY Link Status Enable"]
    #[inline(always)]
    pub fn emac_lpictlstat_plsen(&self) -> EMAC_LPICTLSTAT_PLSENR {
        let bits = ((self.bits >> 18) & 1) != 0;
        EMAC_LPICTLSTAT_PLSENR { bits }
    }
    #[doc = "Bit 19 - LPI TX Automate"]
    #[inline(always)]
    pub fn emac_lpictlstat_lpitxa(&self) -> EMAC_LPICTLSTAT_LPITXAR {
        let bits = ((self.bits >> 19) & 1) != 0;
        EMAC_LPICTLSTAT_LPITXAR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Transmit LPI Entry"]
    #[inline(always)]
    pub fn emac_lpictlstat_tlpien(&mut self) -> _EMAC_LPICTLSTAT_TLPIENW {
        _EMAC_LPICTLSTAT_TLPIENW { w: self }
    }
    #[doc = "Bit 1 - Transmit LPI Exit"]
    #[inline(always)]
    pub fn emac_lpictlstat_tlpiex(&mut self) -> _EMAC_LPICTLSTAT_TLPIEXW {
        _EMAC_LPICTLSTAT_TLPIEXW { w: self }
    }
    #[doc = "Bit 2 - Receive LPI Entry"]
    #[inline(always)]
    pub fn emac_lpictlstat_rlpien(&mut self) -> _EMAC_LPICTLSTAT_RLPIENW {
        _EMAC_LPICTLSTAT_RLPIENW { w: self }
    }
    #[doc = "Bit 3 - Receive LPI Exit"]
    #[inline(always)]
    pub fn emac_lpictlstat_rlpiex(&mut self) -> _EMAC_LPICTLSTAT_RLPIEXW {
        _EMAC_LPICTLSTAT_RLPIEXW { w: self }
    }
    #[doc = "Bit 8 - Transmit LPI State"]
    #[inline(always)]
    pub fn emac_lpictlstat_tlpist(&mut self) -> _EMAC_LPICTLSTAT_TLPISTW {
        _EMAC_LPICTLSTAT_TLPISTW { w: self }
    }
    #[doc = "Bit 9 - Receive LPI State"]
    #[inline(always)]
    pub fn emac_lpictlstat_rlpist(&mut self) -> _EMAC_LPICTLSTAT_RLPISTW {
        _EMAC_LPICTLSTAT_RLPISTW { w: self }
    }
    #[doc = "Bit 16 - LPI Enable"]
    #[inline(always)]
    pub fn emac_lpictlstat_lpien(&mut self) -> _EMAC_LPICTLSTAT_LPIENW {
        _EMAC_LPICTLSTAT_LPIENW { w: self }
    }
    #[doc = "Bit 17 - PHY Link Status"]
    #[inline(always)]
    pub fn emac_lpictlstat_pls(&mut self) -> _EMAC_LPICTLSTAT_PLSW {
        _EMAC_LPICTLSTAT_PLSW { w: self }
    }
    #[doc = "Bit 18 - PHY Link Status Enable"]
    #[inline(always)]
    pub fn emac_lpictlstat_plsen(&mut self) -> _EMAC_LPICTLSTAT_PLSENW {
        _EMAC_LPICTLSTAT_PLSENW { w: self }
    }
    #[doc = "Bit 19 - LPI TX Automate"]
    #[inline(always)]
    pub fn emac_lpictlstat_lpitxa(&mut self) -> _EMAC_LPICTLSTAT_LPITXAW {
        _EMAC_LPICTLSTAT_LPITXAW { w: self }
    }
}
