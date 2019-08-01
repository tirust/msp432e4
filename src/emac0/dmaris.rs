#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMARIS {
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
pub struct EMAC_DMARIS_TIR {
    bits: bool,
}
impl EMAC_DMARIS_TIR {
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
pub struct _EMAC_DMARIS_TIW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_TIW<'a> {
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
pub struct EMAC_DMARIS_TPSR {
    bits: bool,
}
impl EMAC_DMARIS_TPSR {
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
pub struct _EMAC_DMARIS_TPSW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_TPSW<'a> {
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
pub struct EMAC_DMARIS_TUR {
    bits: bool,
}
impl EMAC_DMARIS_TUR {
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
pub struct _EMAC_DMARIS_TUW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_TUW<'a> {
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
pub struct EMAC_DMARIS_TJTR {
    bits: bool,
}
impl EMAC_DMARIS_TJTR {
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
pub struct _EMAC_DMARIS_TJTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_TJTW<'a> {
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
pub struct EMAC_DMARIS_OVFR {
    bits: bool,
}
impl EMAC_DMARIS_OVFR {
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
pub struct _EMAC_DMARIS_OVFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_OVFW<'a> {
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
pub struct EMAC_DMARIS_UNFR {
    bits: bool,
}
impl EMAC_DMARIS_UNFR {
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
pub struct _EMAC_DMARIS_UNFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_UNFW<'a> {
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
pub struct EMAC_DMARIS_RIR {
    bits: bool,
}
impl EMAC_DMARIS_RIR {
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
pub struct _EMAC_DMARIS_RIW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_RIW<'a> {
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
pub struct EMAC_DMARIS_RUR {
    bits: bool,
}
impl EMAC_DMARIS_RUR {
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
pub struct _EMAC_DMARIS_RUW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_RUW<'a> {
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
#[doc = r"Value of the field"]
pub struct EMAC_DMARIS_RPSR {
    bits: bool,
}
impl EMAC_DMARIS_RPSR {
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
pub struct _EMAC_DMARIS_RPSW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_RPSW<'a> {
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
pub struct EMAC_DMARIS_RWTR {
    bits: bool,
}
impl EMAC_DMARIS_RWTR {
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
pub struct _EMAC_DMARIS_RWTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_RWTW<'a> {
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
pub struct EMAC_DMARIS_ETIR {
    bits: bool,
}
impl EMAC_DMARIS_ETIR {
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
pub struct _EMAC_DMARIS_ETIW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_ETIW<'a> {
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
#[doc = r"Value of the field"]
pub struct EMAC_DMARIS_FBIR {
    bits: bool,
}
impl EMAC_DMARIS_FBIR {
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
pub struct _EMAC_DMARIS_FBIW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_FBIW<'a> {
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
pub struct EMAC_DMARIS_ERIR {
    bits: bool,
}
impl EMAC_DMARIS_ERIR {
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
pub struct _EMAC_DMARIS_ERIW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_ERIW<'a> {
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
pub struct EMAC_DMARIS_AISR {
    bits: bool,
}
impl EMAC_DMARIS_AISR {
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
pub struct _EMAC_DMARIS_AISW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_AISW<'a> {
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
#[doc = r"Value of the field"]
pub struct EMAC_DMARIS_NISR {
    bits: bool,
}
impl EMAC_DMARIS_NISR {
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
pub struct _EMAC_DMARIS_NISW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_NISW<'a> {
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
#[doc = "Possible values of the field `EMAC_DMARIS_RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_DMARIS_RSR {
    #[doc = "Stopped: Reset or stop receive command issued"]
    EMAC_DMARIS_RS_STOP,
    #[doc = "Running: Fetching receive transfer descriptor"]
    EMAC_DMARIS_RS_RUNRXTD,
    #[doc = "Running: Waiting for receive packet"]
    EMAC_DMARIS_RS_RUNRXD,
    #[doc = "Suspended: Receive descriptor unavailable"]
    EMAC_DMARIS_RS_SUSPEND,
    #[doc = "Running: Closing receive descriptor"]
    EMAC_DMARIS_RS_RUNCRD,
    #[doc = "Writing Timestamp"]
    EMAC_DMARIS_RS_TSWS,
    #[doc = "Running: Transferring the receive packet data from receive buffer to host memory"]
    EMAC_DMARIS_RS_RUNTXD,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl EMAC_DMARIS_RSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_DMARIS_RSR::EMAC_DMARIS_RS_STOP => 0,
            EMAC_DMARIS_RSR::EMAC_DMARIS_RS_RUNRXTD => 1,
            EMAC_DMARIS_RSR::EMAC_DMARIS_RS_RUNRXD => 3,
            EMAC_DMARIS_RSR::EMAC_DMARIS_RS_SUSPEND => 4,
            EMAC_DMARIS_RSR::EMAC_DMARIS_RS_RUNCRD => 5,
            EMAC_DMARIS_RSR::EMAC_DMARIS_RS_TSWS => 6,
            EMAC_DMARIS_RSR::EMAC_DMARIS_RS_RUNTXD => 7,
            EMAC_DMARIS_RSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_DMARIS_RSR {
        match value {
            0 => EMAC_DMARIS_RSR::EMAC_DMARIS_RS_STOP,
            1 => EMAC_DMARIS_RSR::EMAC_DMARIS_RS_RUNRXTD,
            3 => EMAC_DMARIS_RSR::EMAC_DMARIS_RS_RUNRXD,
            4 => EMAC_DMARIS_RSR::EMAC_DMARIS_RS_SUSPEND,
            5 => EMAC_DMARIS_RSR::EMAC_DMARIS_RS_RUNCRD,
            6 => EMAC_DMARIS_RSR::EMAC_DMARIS_RS_TSWS,
            7 => EMAC_DMARIS_RSR::EMAC_DMARIS_RS_RUNTXD,
            i => EMAC_DMARIS_RSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_RS_STOP`"]
    #[inline(always)]
    pub fn is_emac_dmaris_rs_stop(&self) -> bool {
        *self == EMAC_DMARIS_RSR::EMAC_DMARIS_RS_STOP
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_RS_RUNRXTD`"]
    #[inline(always)]
    pub fn is_emac_dmaris_rs_runrxtd(&self) -> bool {
        *self == EMAC_DMARIS_RSR::EMAC_DMARIS_RS_RUNRXTD
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_RS_RUNRXD`"]
    #[inline(always)]
    pub fn is_emac_dmaris_rs_runrxd(&self) -> bool {
        *self == EMAC_DMARIS_RSR::EMAC_DMARIS_RS_RUNRXD
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_RS_SUSPEND`"]
    #[inline(always)]
    pub fn is_emac_dmaris_rs_suspend(&self) -> bool {
        *self == EMAC_DMARIS_RSR::EMAC_DMARIS_RS_SUSPEND
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_RS_RUNCRD`"]
    #[inline(always)]
    pub fn is_emac_dmaris_rs_runcrd(&self) -> bool {
        *self == EMAC_DMARIS_RSR::EMAC_DMARIS_RS_RUNCRD
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_RS_TSWS`"]
    #[inline(always)]
    pub fn is_emac_dmaris_rs_tsws(&self) -> bool {
        *self == EMAC_DMARIS_RSR::EMAC_DMARIS_RS_TSWS
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_RS_RUNTXD`"]
    #[inline(always)]
    pub fn is_emac_dmaris_rs_runtxd(&self) -> bool {
        *self == EMAC_DMARIS_RSR::EMAC_DMARIS_RS_RUNTXD
    }
}
#[doc = "Values that can be written to the field `EMAC_DMARIS_RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_DMARIS_RSW {
    #[doc = "Stopped: Reset or stop receive command issued"]
    EMAC_DMARIS_RS_STOP,
    #[doc = "Running: Fetching receive transfer descriptor"]
    EMAC_DMARIS_RS_RUNRXTD,
    #[doc = "Running: Waiting for receive packet"]
    EMAC_DMARIS_RS_RUNRXD,
    #[doc = "Suspended: Receive descriptor unavailable"]
    EMAC_DMARIS_RS_SUSPEND,
    #[doc = "Running: Closing receive descriptor"]
    EMAC_DMARIS_RS_RUNCRD,
    #[doc = "Writing Timestamp"]
    EMAC_DMARIS_RS_TSWS,
    #[doc = "Running: Transferring the receive packet data from receive buffer to host memory"]
    EMAC_DMARIS_RS_RUNTXD,
}
impl EMAC_DMARIS_RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_DMARIS_RSW::EMAC_DMARIS_RS_STOP => 0,
            EMAC_DMARIS_RSW::EMAC_DMARIS_RS_RUNRXTD => 1,
            EMAC_DMARIS_RSW::EMAC_DMARIS_RS_RUNRXD => 3,
            EMAC_DMARIS_RSW::EMAC_DMARIS_RS_SUSPEND => 4,
            EMAC_DMARIS_RSW::EMAC_DMARIS_RS_RUNCRD => 5,
            EMAC_DMARIS_RSW::EMAC_DMARIS_RS_TSWS => 6,
            EMAC_DMARIS_RSW::EMAC_DMARIS_RS_RUNTXD => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_DMARIS_RSW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_RSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_DMARIS_RSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Stopped: Reset or stop receive command issued"]
    #[inline(always)]
    pub fn emac_dmaris_rs_stop(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_RSW::EMAC_DMARIS_RS_STOP)
    }
    #[doc = "Running: Fetching receive transfer descriptor"]
    #[inline(always)]
    pub fn emac_dmaris_rs_runrxtd(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_RSW::EMAC_DMARIS_RS_RUNRXTD)
    }
    #[doc = "Running: Waiting for receive packet"]
    #[inline(always)]
    pub fn emac_dmaris_rs_runrxd(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_RSW::EMAC_DMARIS_RS_RUNRXD)
    }
    #[doc = "Suspended: Receive descriptor unavailable"]
    #[inline(always)]
    pub fn emac_dmaris_rs_suspend(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_RSW::EMAC_DMARIS_RS_SUSPEND)
    }
    #[doc = "Running: Closing receive descriptor"]
    #[inline(always)]
    pub fn emac_dmaris_rs_runcrd(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_RSW::EMAC_DMARIS_RS_RUNCRD)
    }
    #[doc = "Writing Timestamp"]
    #[inline(always)]
    pub fn emac_dmaris_rs_tsws(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_RSW::EMAC_DMARIS_RS_TSWS)
    }
    #[doc = "Running: Transferring the receive packet data from receive buffer to host memory"]
    #[inline(always)]
    pub fn emac_dmaris_rs_runtxd(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_RSW::EMAC_DMARIS_RS_RUNTXD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 17);
        self.w.bits |= ((value as u32) & 7) << 17;
        self.w
    }
}
#[doc = "Possible values of the field `EMAC_DMARIS_TS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_DMARIS_TSR {
    #[doc = "Stopped; Reset or Stop transmit command processed"]
    EMAC_DMARIS_TS_STOP,
    #[doc = "Running; Fetching transmit transfer descriptor"]
    EMAC_DMARIS_TS_RUNTXTD,
    #[doc = "Running; Waiting for status"]
    EMAC_DMARIS_TS_STATUS,
    #[doc = "Running; Reading data from host memory buffer and queuing it to transmit buffer (TX FIFO)"]
    EMAC_DMARIS_TS_RUNTX,
    #[doc = "Writing Timestamp"]
    EMAC_DMARIS_TS_TSTAMP,
    #[doc = "Suspended; Transmit descriptor unavailable or transmit buffer underflow"]
    EMAC_DMARIS_TS_SUSPEND,
    #[doc = "Running; Closing transmit descriptor"]
    EMAC_DMARIS_TS_RUNCTD,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl EMAC_DMARIS_TSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_DMARIS_TSR::EMAC_DMARIS_TS_STOP => 0,
            EMAC_DMARIS_TSR::EMAC_DMARIS_TS_RUNTXTD => 1,
            EMAC_DMARIS_TSR::EMAC_DMARIS_TS_STATUS => 2,
            EMAC_DMARIS_TSR::EMAC_DMARIS_TS_RUNTX => 3,
            EMAC_DMARIS_TSR::EMAC_DMARIS_TS_TSTAMP => 4,
            EMAC_DMARIS_TSR::EMAC_DMARIS_TS_SUSPEND => 6,
            EMAC_DMARIS_TSR::EMAC_DMARIS_TS_RUNCTD => 7,
            EMAC_DMARIS_TSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_DMARIS_TSR {
        match value {
            0 => EMAC_DMARIS_TSR::EMAC_DMARIS_TS_STOP,
            1 => EMAC_DMARIS_TSR::EMAC_DMARIS_TS_RUNTXTD,
            2 => EMAC_DMARIS_TSR::EMAC_DMARIS_TS_STATUS,
            3 => EMAC_DMARIS_TSR::EMAC_DMARIS_TS_RUNTX,
            4 => EMAC_DMARIS_TSR::EMAC_DMARIS_TS_TSTAMP,
            6 => EMAC_DMARIS_TSR::EMAC_DMARIS_TS_SUSPEND,
            7 => EMAC_DMARIS_TSR::EMAC_DMARIS_TS_RUNCTD,
            i => EMAC_DMARIS_TSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_TS_STOP`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ts_stop(&self) -> bool {
        *self == EMAC_DMARIS_TSR::EMAC_DMARIS_TS_STOP
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_TS_RUNTXTD`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ts_runtxtd(&self) -> bool {
        *self == EMAC_DMARIS_TSR::EMAC_DMARIS_TS_RUNTXTD
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_TS_STATUS`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ts_status(&self) -> bool {
        *self == EMAC_DMARIS_TSR::EMAC_DMARIS_TS_STATUS
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_TS_RUNTX`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ts_runtx(&self) -> bool {
        *self == EMAC_DMARIS_TSR::EMAC_DMARIS_TS_RUNTX
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_TS_TSTAMP`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ts_tstamp(&self) -> bool {
        *self == EMAC_DMARIS_TSR::EMAC_DMARIS_TS_TSTAMP
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_TS_SUSPEND`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ts_suspend(&self) -> bool {
        *self == EMAC_DMARIS_TSR::EMAC_DMARIS_TS_SUSPEND
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_TS_RUNCTD`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ts_runctd(&self) -> bool {
        *self == EMAC_DMARIS_TSR::EMAC_DMARIS_TS_RUNCTD
    }
}
#[doc = "Values that can be written to the field `EMAC_DMARIS_TS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_DMARIS_TSW {
    #[doc = "Stopped; Reset or Stop transmit command processed"]
    EMAC_DMARIS_TS_STOP,
    #[doc = "Running; Fetching transmit transfer descriptor"]
    EMAC_DMARIS_TS_RUNTXTD,
    #[doc = "Running; Waiting for status"]
    EMAC_DMARIS_TS_STATUS,
    #[doc = "Running; Reading data from host memory buffer and queuing it to transmit buffer (TX FIFO)"]
    EMAC_DMARIS_TS_RUNTX,
    #[doc = "Writing Timestamp"]
    EMAC_DMARIS_TS_TSTAMP,
    #[doc = "Suspended; Transmit descriptor unavailable or transmit buffer underflow"]
    EMAC_DMARIS_TS_SUSPEND,
    #[doc = "Running; Closing transmit descriptor"]
    EMAC_DMARIS_TS_RUNCTD,
}
impl EMAC_DMARIS_TSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_DMARIS_TSW::EMAC_DMARIS_TS_STOP => 0,
            EMAC_DMARIS_TSW::EMAC_DMARIS_TS_RUNTXTD => 1,
            EMAC_DMARIS_TSW::EMAC_DMARIS_TS_STATUS => 2,
            EMAC_DMARIS_TSW::EMAC_DMARIS_TS_RUNTX => 3,
            EMAC_DMARIS_TSW::EMAC_DMARIS_TS_TSTAMP => 4,
            EMAC_DMARIS_TSW::EMAC_DMARIS_TS_SUSPEND => 6,
            EMAC_DMARIS_TSW::EMAC_DMARIS_TS_RUNCTD => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_DMARIS_TSW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_TSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_DMARIS_TSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Stopped; Reset or Stop transmit command processed"]
    #[inline(always)]
    pub fn emac_dmaris_ts_stop(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_TSW::EMAC_DMARIS_TS_STOP)
    }
    #[doc = "Running; Fetching transmit transfer descriptor"]
    #[inline(always)]
    pub fn emac_dmaris_ts_runtxtd(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_TSW::EMAC_DMARIS_TS_RUNTXTD)
    }
    #[doc = "Running; Waiting for status"]
    #[inline(always)]
    pub fn emac_dmaris_ts_status(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_TSW::EMAC_DMARIS_TS_STATUS)
    }
    #[doc = "Running; Reading data from host memory buffer and queuing it to transmit buffer (TX FIFO)"]
    #[inline(always)]
    pub fn emac_dmaris_ts_runtx(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_TSW::EMAC_DMARIS_TS_RUNTX)
    }
    #[doc = "Writing Timestamp"]
    #[inline(always)]
    pub fn emac_dmaris_ts_tstamp(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_TSW::EMAC_DMARIS_TS_TSTAMP)
    }
    #[doc = "Suspended; Transmit descriptor unavailable or transmit buffer underflow"]
    #[inline(always)]
    pub fn emac_dmaris_ts_suspend(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_TSW::EMAC_DMARIS_TS_SUSPEND)
    }
    #[doc = "Running; Closing transmit descriptor"]
    #[inline(always)]
    pub fn emac_dmaris_ts_runctd(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_TSW::EMAC_DMARIS_TS_RUNCTD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 20);
        self.w.bits |= ((value as u32) & 7) << 20;
        self.w
    }
}
#[doc = "Possible values of the field `EMAC_DMARIS_AE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_DMARIS_AER {
    #[doc = "Error during RX DMA Write Data Transfer"]
    EMAC_DMARIS_AE_RXDMAWD,
    #[doc = "Error during TX DMA Read Data Transfer"]
    EMAC_DMARIS_AE_TXDMARD,
    #[doc = "Error during RX DMA Descriptor Write Access"]
    EMAC_DMARIS_AE_RXDMADW,
    #[doc = "Error during TX DMA Descriptor Write Access"]
    EMAC_DMARIS_AE_TXDMADW,
    #[doc = "Error during RX DMA Descriptor Read Access"]
    EMAC_DMARIS_AE_RXDMADR,
    #[doc = "Error during TX DMA Descriptor Read Access"]
    EMAC_DMARIS_AE_TXDMADR,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl EMAC_DMARIS_AER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_DMARIS_AER::EMAC_DMARIS_AE_RXDMAWD => 0,
            EMAC_DMARIS_AER::EMAC_DMARIS_AE_TXDMARD => 3,
            EMAC_DMARIS_AER::EMAC_DMARIS_AE_RXDMADW => 4,
            EMAC_DMARIS_AER::EMAC_DMARIS_AE_TXDMADW => 5,
            EMAC_DMARIS_AER::EMAC_DMARIS_AE_RXDMADR => 6,
            EMAC_DMARIS_AER::EMAC_DMARIS_AE_TXDMADR => 7,
            EMAC_DMARIS_AER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_DMARIS_AER {
        match value {
            0 => EMAC_DMARIS_AER::EMAC_DMARIS_AE_RXDMAWD,
            3 => EMAC_DMARIS_AER::EMAC_DMARIS_AE_TXDMARD,
            4 => EMAC_DMARIS_AER::EMAC_DMARIS_AE_RXDMADW,
            5 => EMAC_DMARIS_AER::EMAC_DMARIS_AE_TXDMADW,
            6 => EMAC_DMARIS_AER::EMAC_DMARIS_AE_RXDMADR,
            7 => EMAC_DMARIS_AER::EMAC_DMARIS_AE_TXDMADR,
            i => EMAC_DMARIS_AER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_AE_RXDMAWD`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ae_rxdmawd(&self) -> bool {
        *self == EMAC_DMARIS_AER::EMAC_DMARIS_AE_RXDMAWD
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_AE_TXDMARD`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ae_txdmard(&self) -> bool {
        *self == EMAC_DMARIS_AER::EMAC_DMARIS_AE_TXDMARD
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_AE_RXDMADW`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ae_rxdmadw(&self) -> bool {
        *self == EMAC_DMARIS_AER::EMAC_DMARIS_AE_RXDMADW
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_AE_TXDMADW`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ae_txdmadw(&self) -> bool {
        *self == EMAC_DMARIS_AER::EMAC_DMARIS_AE_TXDMADW
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_AE_RXDMADR`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ae_rxdmadr(&self) -> bool {
        *self == EMAC_DMARIS_AER::EMAC_DMARIS_AE_RXDMADR
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_AE_TXDMADR`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ae_txdmadr(&self) -> bool {
        *self == EMAC_DMARIS_AER::EMAC_DMARIS_AE_TXDMADR
    }
}
#[doc = "Values that can be written to the field `EMAC_DMARIS_AE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_DMARIS_AEW {
    #[doc = "Error during RX DMA Write Data Transfer"]
    EMAC_DMARIS_AE_RXDMAWD,
    #[doc = "Error during TX DMA Read Data Transfer"]
    EMAC_DMARIS_AE_TXDMARD,
    #[doc = "Error during RX DMA Descriptor Write Access"]
    EMAC_DMARIS_AE_RXDMADW,
    #[doc = "Error during TX DMA Descriptor Write Access"]
    EMAC_DMARIS_AE_TXDMADW,
    #[doc = "Error during RX DMA Descriptor Read Access"]
    EMAC_DMARIS_AE_RXDMADR,
    #[doc = "Error during TX DMA Descriptor Read Access"]
    EMAC_DMARIS_AE_TXDMADR,
}
impl EMAC_DMARIS_AEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_DMARIS_AEW::EMAC_DMARIS_AE_RXDMAWD => 0,
            EMAC_DMARIS_AEW::EMAC_DMARIS_AE_TXDMARD => 3,
            EMAC_DMARIS_AEW::EMAC_DMARIS_AE_RXDMADW => 4,
            EMAC_DMARIS_AEW::EMAC_DMARIS_AE_TXDMADW => 5,
            EMAC_DMARIS_AEW::EMAC_DMARIS_AE_RXDMADR => 6,
            EMAC_DMARIS_AEW::EMAC_DMARIS_AE_TXDMADR => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_DMARIS_AEW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_AEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_DMARIS_AEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Error during RX DMA Write Data Transfer"]
    #[inline(always)]
    pub fn emac_dmaris_ae_rxdmawd(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_AEW::EMAC_DMARIS_AE_RXDMAWD)
    }
    #[doc = "Error during TX DMA Read Data Transfer"]
    #[inline(always)]
    pub fn emac_dmaris_ae_txdmard(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_AEW::EMAC_DMARIS_AE_TXDMARD)
    }
    #[doc = "Error during RX DMA Descriptor Write Access"]
    #[inline(always)]
    pub fn emac_dmaris_ae_rxdmadw(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_AEW::EMAC_DMARIS_AE_RXDMADW)
    }
    #[doc = "Error during TX DMA Descriptor Write Access"]
    #[inline(always)]
    pub fn emac_dmaris_ae_txdmadw(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_AEW::EMAC_DMARIS_AE_TXDMADW)
    }
    #[doc = "Error during RX DMA Descriptor Read Access"]
    #[inline(always)]
    pub fn emac_dmaris_ae_rxdmadr(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_AEW::EMAC_DMARIS_AE_RXDMADR)
    }
    #[doc = "Error during TX DMA Descriptor Read Access"]
    #[inline(always)]
    pub fn emac_dmaris_ae_txdmadr(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_AEW::EMAC_DMARIS_AE_TXDMADR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 23);
        self.w.bits |= ((value as u32) & 7) << 23;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_DMARIS_MMCR {
    bits: bool,
}
impl EMAC_DMARIS_MMCR {
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
pub struct _EMAC_DMARIS_MMCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_MMCW<'a> {
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
        self.w.bits &= !(1 << 27);
        self.w.bits |= ((value as u32) & 1) << 27;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_DMARIS_PMTR {
    bits: bool,
}
impl EMAC_DMARIS_PMTR {
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
pub struct _EMAC_DMARIS_PMTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_PMTW<'a> {
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
        self.w.bits &= !(1 << 28);
        self.w.bits |= ((value as u32) & 1) << 28;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_DMARIS_TTR {
    bits: bool,
}
impl EMAC_DMARIS_TTR {
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
pub struct _EMAC_DMARIS_TTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMARIS_TTW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_ti(&self) -> EMAC_DMARIS_TIR {
        let bits = ((self.bits >> 0) & 1) != 0;
        EMAC_DMARIS_TIR { bits }
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    pub fn emac_dmaris_tps(&self) -> EMAC_DMARIS_TPSR {
        let bits = ((self.bits >> 1) & 1) != 0;
        EMAC_DMARIS_TPSR { bits }
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    pub fn emac_dmaris_tu(&self) -> EMAC_DMARIS_TUR {
        let bits = ((self.bits >> 2) & 1) != 0;
        EMAC_DMARIS_TUR { bits }
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout"]
    #[inline(always)]
    pub fn emac_dmaris_tjt(&self) -> EMAC_DMARIS_TJTR {
        let bits = ((self.bits >> 3) & 1) != 0;
        EMAC_DMARIS_TJTR { bits }
    }
    #[doc = "Bit 4 - Receive Overflow"]
    #[inline(always)]
    pub fn emac_dmaris_ovf(&self) -> EMAC_DMARIS_OVFR {
        let bits = ((self.bits >> 4) & 1) != 0;
        EMAC_DMARIS_OVFR { bits }
    }
    #[doc = "Bit 5 - Transmit Underflow"]
    #[inline(always)]
    pub fn emac_dmaris_unf(&self) -> EMAC_DMARIS_UNFR {
        let bits = ((self.bits >> 5) & 1) != 0;
        EMAC_DMARIS_UNFR { bits }
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_ri(&self) -> EMAC_DMARIS_RIR {
        let bits = ((self.bits >> 6) & 1) != 0;
        EMAC_DMARIS_RIR { bits }
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    pub fn emac_dmaris_ru(&self) -> EMAC_DMARIS_RUR {
        let bits = ((self.bits >> 7) & 1) != 0;
        EMAC_DMARIS_RUR { bits }
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    pub fn emac_dmaris_rps(&self) -> EMAC_DMARIS_RPSR {
        let bits = ((self.bits >> 8) & 1) != 0;
        EMAC_DMARIS_RPSR { bits }
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn emac_dmaris_rwt(&self) -> EMAC_DMARIS_RWTR {
        let bits = ((self.bits >> 9) & 1) != 0;
        EMAC_DMARIS_RWTR { bits }
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_eti(&self) -> EMAC_DMARIS_ETIR {
        let bits = ((self.bits >> 10) & 1) != 0;
        EMAC_DMARIS_ETIR { bits }
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_fbi(&self) -> EMAC_DMARIS_FBIR {
        let bits = ((self.bits >> 13) & 1) != 0;
        EMAC_DMARIS_FBIR { bits }
    }
    #[doc = "Bit 14 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_eri(&self) -> EMAC_DMARIS_ERIR {
        let bits = ((self.bits >> 14) & 1) != 0;
        EMAC_DMARIS_ERIR { bits }
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn emac_dmaris_ais(&self) -> EMAC_DMARIS_AISR {
        let bits = ((self.bits >> 15) & 1) != 0;
        EMAC_DMARIS_AISR { bits }
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn emac_dmaris_nis(&self) -> EMAC_DMARIS_NISR {
        let bits = ((self.bits >> 16) & 1) != 0;
        EMAC_DMARIS_NISR { bits }
    }
    #[doc = "Bits 17:19 - Received Process State"]
    #[inline(always)]
    pub fn emac_dmaris_rs(&self) -> EMAC_DMARIS_RSR {
        EMAC_DMARIS_RSR::_from(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Transmit Process State"]
    #[inline(always)]
    pub fn emac_dmaris_ts(&self) -> EMAC_DMARIS_TSR {
        EMAC_DMARIS_TSR::_from(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Access Error"]
    #[inline(always)]
    pub fn emac_dmaris_ae(&self) -> EMAC_DMARIS_AER {
        EMAC_DMARIS_AER::_from(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 27 - MAC MMC Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_mmc(&self) -> EMAC_DMARIS_MMCR {
        let bits = ((self.bits >> 27) & 1) != 0;
        EMAC_DMARIS_MMCR { bits }
    }
    #[doc = "Bit 28 - MAC PMT Interrupt Status"]
    #[inline(always)]
    pub fn emac_dmaris_pmt(&self) -> EMAC_DMARIS_PMTR {
        let bits = ((self.bits >> 28) & 1) != 0;
        EMAC_DMARIS_PMTR { bits }
    }
    #[doc = "Bit 29 - Timestamp Trigger Interrupt Status"]
    #[inline(always)]
    pub fn emac_dmaris_tt(&self) -> EMAC_DMARIS_TTR {
        let bits = ((self.bits >> 29) & 1) != 0;
        EMAC_DMARIS_TTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_ti(&mut self) -> _EMAC_DMARIS_TIW {
        _EMAC_DMARIS_TIW { w: self }
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    pub fn emac_dmaris_tps(&mut self) -> _EMAC_DMARIS_TPSW {
        _EMAC_DMARIS_TPSW { w: self }
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    pub fn emac_dmaris_tu(&mut self) -> _EMAC_DMARIS_TUW {
        _EMAC_DMARIS_TUW { w: self }
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout"]
    #[inline(always)]
    pub fn emac_dmaris_tjt(&mut self) -> _EMAC_DMARIS_TJTW {
        _EMAC_DMARIS_TJTW { w: self }
    }
    #[doc = "Bit 4 - Receive Overflow"]
    #[inline(always)]
    pub fn emac_dmaris_ovf(&mut self) -> _EMAC_DMARIS_OVFW {
        _EMAC_DMARIS_OVFW { w: self }
    }
    #[doc = "Bit 5 - Transmit Underflow"]
    #[inline(always)]
    pub fn emac_dmaris_unf(&mut self) -> _EMAC_DMARIS_UNFW {
        _EMAC_DMARIS_UNFW { w: self }
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_ri(&mut self) -> _EMAC_DMARIS_RIW {
        _EMAC_DMARIS_RIW { w: self }
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    pub fn emac_dmaris_ru(&mut self) -> _EMAC_DMARIS_RUW {
        _EMAC_DMARIS_RUW { w: self }
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    pub fn emac_dmaris_rps(&mut self) -> _EMAC_DMARIS_RPSW {
        _EMAC_DMARIS_RPSW { w: self }
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn emac_dmaris_rwt(&mut self) -> _EMAC_DMARIS_RWTW {
        _EMAC_DMARIS_RWTW { w: self }
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_eti(&mut self) -> _EMAC_DMARIS_ETIW {
        _EMAC_DMARIS_ETIW { w: self }
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_fbi(&mut self) -> _EMAC_DMARIS_FBIW {
        _EMAC_DMARIS_FBIW { w: self }
    }
    #[doc = "Bit 14 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_eri(&mut self) -> _EMAC_DMARIS_ERIW {
        _EMAC_DMARIS_ERIW { w: self }
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn emac_dmaris_ais(&mut self) -> _EMAC_DMARIS_AISW {
        _EMAC_DMARIS_AISW { w: self }
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn emac_dmaris_nis(&mut self) -> _EMAC_DMARIS_NISW {
        _EMAC_DMARIS_NISW { w: self }
    }
    #[doc = "Bits 17:19 - Received Process State"]
    #[inline(always)]
    pub fn emac_dmaris_rs(&mut self) -> _EMAC_DMARIS_RSW {
        _EMAC_DMARIS_RSW { w: self }
    }
    #[doc = "Bits 20:22 - Transmit Process State"]
    #[inline(always)]
    pub fn emac_dmaris_ts(&mut self) -> _EMAC_DMARIS_TSW {
        _EMAC_DMARIS_TSW { w: self }
    }
    #[doc = "Bits 23:25 - Access Error"]
    #[inline(always)]
    pub fn emac_dmaris_ae(&mut self) -> _EMAC_DMARIS_AEW {
        _EMAC_DMARIS_AEW { w: self }
    }
    #[doc = "Bit 27 - MAC MMC Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_mmc(&mut self) -> _EMAC_DMARIS_MMCW {
        _EMAC_DMARIS_MMCW { w: self }
    }
    #[doc = "Bit 28 - MAC PMT Interrupt Status"]
    #[inline(always)]
    pub fn emac_dmaris_pmt(&mut self) -> _EMAC_DMARIS_PMTW {
        _EMAC_DMARIS_PMTW { w: self }
    }
    #[doc = "Bit 29 - Timestamp Trigger Interrupt Status"]
    #[inline(always)]
    pub fn emac_dmaris_tt(&mut self) -> _EMAC_DMARIS_TTW {
        _EMAC_DMARIS_TTW { w: self }
    }
}
