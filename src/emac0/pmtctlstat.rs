#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMTCTLSTAT {
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
pub struct EMAC_PMTCTLSTAT_PWRDWNR {
    bits: bool,
}
impl EMAC_PMTCTLSTAT_PWRDWNR {
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
pub struct _EMAC_PMTCTLSTAT_PWRDWNW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PMTCTLSTAT_PWRDWNW<'a> {
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
pub struct EMAC_PMTCTLSTAT_MGKPKTENR {
    bits: bool,
}
impl EMAC_PMTCTLSTAT_MGKPKTENR {
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
pub struct _EMAC_PMTCTLSTAT_MGKPKTENW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PMTCTLSTAT_MGKPKTENW<'a> {
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
pub struct EMAC_PMTCTLSTAT_WUPFRENR {
    bits: bool,
}
impl EMAC_PMTCTLSTAT_WUPFRENR {
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
pub struct _EMAC_PMTCTLSTAT_WUPFRENW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PMTCTLSTAT_WUPFRENW<'a> {
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
pub struct EMAC_PMTCTLSTAT_MGKPRXR {
    bits: bool,
}
impl EMAC_PMTCTLSTAT_MGKPRXR {
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
pub struct _EMAC_PMTCTLSTAT_MGKPRXW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PMTCTLSTAT_MGKPRXW<'a> {
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
pub struct EMAC_PMTCTLSTAT_WUPRXR {
    bits: bool,
}
impl EMAC_PMTCTLSTAT_WUPRXR {
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
pub struct _EMAC_PMTCTLSTAT_WUPRXW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PMTCTLSTAT_WUPRXW<'a> {
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
#[doc = r"Value of the field"]
pub struct EMAC_PMTCTLSTAT_GLBLUCASTR {
    bits: bool,
}
impl EMAC_PMTCTLSTAT_GLBLUCASTR {
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
pub struct _EMAC_PMTCTLSTAT_GLBLUCASTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PMTCTLSTAT_GLBLUCASTW<'a> {
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
pub struct EMAC_PMTCTLSTAT_RWKPTRR {
    bits: u8,
}
impl EMAC_PMTCTLSTAT_RWKPTRR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_PMTCTLSTAT_RWKPTRW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PMTCTLSTAT_RWKPTRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 24);
        self.w.bits |= ((value as u32) & 7) << 24;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_PMTCTLSTAT_WUPFRRSTR {
    bits: bool,
}
impl EMAC_PMTCTLSTAT_WUPFRRSTR {
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
pub struct _EMAC_PMTCTLSTAT_WUPFRRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PMTCTLSTAT_WUPFRRSTW<'a> {
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
    #[doc = "Bit 0 - Power Down"]
    #[inline(always)]
    pub fn emac_pmtctlstat_pwrdwn(&self) -> EMAC_PMTCTLSTAT_PWRDWNR {
        let bits = ((self.bits >> 0) & 1) != 0;
        EMAC_PMTCTLSTAT_PWRDWNR { bits }
    }
    #[doc = "Bit 1 - Magic Packet Enable"]
    #[inline(always)]
    pub fn emac_pmtctlstat_mgkpkten(&self) -> EMAC_PMTCTLSTAT_MGKPKTENR {
        let bits = ((self.bits >> 1) & 1) != 0;
        EMAC_PMTCTLSTAT_MGKPKTENR { bits }
    }
    #[doc = "Bit 2 - Wake-Up Frame Enable"]
    #[inline(always)]
    pub fn emac_pmtctlstat_wupfren(&self) -> EMAC_PMTCTLSTAT_WUPFRENR {
        let bits = ((self.bits >> 2) & 1) != 0;
        EMAC_PMTCTLSTAT_WUPFRENR { bits }
    }
    #[doc = "Bit 5 - Magic Packet Received"]
    #[inline(always)]
    pub fn emac_pmtctlstat_mgkprx(&self) -> EMAC_PMTCTLSTAT_MGKPRXR {
        let bits = ((self.bits >> 5) & 1) != 0;
        EMAC_PMTCTLSTAT_MGKPRXR { bits }
    }
    #[doc = "Bit 6 - Wake-Up Frame Received"]
    #[inline(always)]
    pub fn emac_pmtctlstat_wuprx(&self) -> EMAC_PMTCTLSTAT_WUPRXR {
        let bits = ((self.bits >> 6) & 1) != 0;
        EMAC_PMTCTLSTAT_WUPRXR { bits }
    }
    #[doc = "Bit 9 - Global Unicast"]
    #[inline(always)]
    pub fn emac_pmtctlstat_glblucast(&self) -> EMAC_PMTCTLSTAT_GLBLUCASTR {
        let bits = ((self.bits >> 9) & 1) != 0;
        EMAC_PMTCTLSTAT_GLBLUCASTR { bits }
    }
    #[doc = "Bits 24:26 - Remote Wake-Up FIFO Pointer"]
    #[inline(always)]
    pub fn emac_pmtctlstat_rwkptr(&self) -> EMAC_PMTCTLSTAT_RWKPTRR {
        let bits = ((self.bits >> 24) & 7) as u8;
        EMAC_PMTCTLSTAT_RWKPTRR { bits }
    }
    #[doc = "Bit 31 - Wake-Up Frame Filter Register Pointer Reset"]
    #[inline(always)]
    pub fn emac_pmtctlstat_wupfrrst(&self) -> EMAC_PMTCTLSTAT_WUPFRRSTR {
        let bits = ((self.bits >> 31) & 1) != 0;
        EMAC_PMTCTLSTAT_WUPFRRSTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Power Down"]
    #[inline(always)]
    pub fn emac_pmtctlstat_pwrdwn(&mut self) -> _EMAC_PMTCTLSTAT_PWRDWNW {
        _EMAC_PMTCTLSTAT_PWRDWNW { w: self }
    }
    #[doc = "Bit 1 - Magic Packet Enable"]
    #[inline(always)]
    pub fn emac_pmtctlstat_mgkpkten(&mut self) -> _EMAC_PMTCTLSTAT_MGKPKTENW {
        _EMAC_PMTCTLSTAT_MGKPKTENW { w: self }
    }
    #[doc = "Bit 2 - Wake-Up Frame Enable"]
    #[inline(always)]
    pub fn emac_pmtctlstat_wupfren(&mut self) -> _EMAC_PMTCTLSTAT_WUPFRENW {
        _EMAC_PMTCTLSTAT_WUPFRENW { w: self }
    }
    #[doc = "Bit 5 - Magic Packet Received"]
    #[inline(always)]
    pub fn emac_pmtctlstat_mgkprx(&mut self) -> _EMAC_PMTCTLSTAT_MGKPRXW {
        _EMAC_PMTCTLSTAT_MGKPRXW { w: self }
    }
    #[doc = "Bit 6 - Wake-Up Frame Received"]
    #[inline(always)]
    pub fn emac_pmtctlstat_wuprx(&mut self) -> _EMAC_PMTCTLSTAT_WUPRXW {
        _EMAC_PMTCTLSTAT_WUPRXW { w: self }
    }
    #[doc = "Bit 9 - Global Unicast"]
    #[inline(always)]
    pub fn emac_pmtctlstat_glblucast(&mut self) -> _EMAC_PMTCTLSTAT_GLBLUCASTW {
        _EMAC_PMTCTLSTAT_GLBLUCASTW { w: self }
    }
    #[doc = "Bits 24:26 - Remote Wake-Up FIFO Pointer"]
    #[inline(always)]
    pub fn emac_pmtctlstat_rwkptr(&mut self) -> _EMAC_PMTCTLSTAT_RWKPTRW {
        _EMAC_PMTCTLSTAT_RWKPTRW { w: self }
    }
    #[doc = "Bit 31 - Wake-Up Frame Filter Register Pointer Reset"]
    #[inline(always)]
    pub fn emac_pmtctlstat_wupfrrst(&mut self) -> _EMAC_PMTCTLSTAT_WUPFRRSTW {
        _EMAC_PMTCTLSTAT_WUPFRRSTW { w: self }
    }
}
