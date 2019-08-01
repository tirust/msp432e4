#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STAT {
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
pub struct UDMA_STAT_MASTENR {
    bits: bool,
}
impl UDMA_STAT_MASTENR {
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
pub struct _UDMA_STAT_MASTENW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_STAT_MASTENW<'a> {
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
#[doc = "Possible values of the field `UDMA_STAT_STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDMA_STAT_STATER {
    #[doc = "Idle"]
    UDMA_STAT_STATE_IDLE,
    #[doc = "Reading channel controller data"]
    UDMA_STAT_STATE_RD_CTRL,
    #[doc = "Reading source end pointer"]
    UDMA_STAT_STATE_RD_SRCENDP,
    #[doc = "Reading destination end pointer"]
    UDMA_STAT_STATE_RD_DSTENDP,
    #[doc = "Reading source data"]
    UDMA_STAT_STATE_RD_SRCDAT,
    #[doc = "Writing destination data"]
    UDMA_STAT_STATE_WR_DSTDAT,
    #[doc = "Waiting for uDMA request to clear"]
    UDMA_STAT_STATE_WAIT,
    #[doc = "Writing channel controller data"]
    UDMA_STAT_STATE_WR_CTRL,
    #[doc = "Stalled"]
    UDMA_STAT_STATE_STALL,
    #[doc = "Done"]
    UDMA_STAT_STATE_DONE,
    #[doc = "Undefined"]
    UDMA_STAT_STATE_UNDEF,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl UDMA_STAT_STATER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            UDMA_STAT_STATER::UDMA_STAT_STATE_IDLE => 0,
            UDMA_STAT_STATER::UDMA_STAT_STATE_RD_CTRL => 1,
            UDMA_STAT_STATER::UDMA_STAT_STATE_RD_SRCENDP => 2,
            UDMA_STAT_STATER::UDMA_STAT_STATE_RD_DSTENDP => 3,
            UDMA_STAT_STATER::UDMA_STAT_STATE_RD_SRCDAT => 4,
            UDMA_STAT_STATER::UDMA_STAT_STATE_WR_DSTDAT => 5,
            UDMA_STAT_STATER::UDMA_STAT_STATE_WAIT => 6,
            UDMA_STAT_STATER::UDMA_STAT_STATE_WR_CTRL => 7,
            UDMA_STAT_STATER::UDMA_STAT_STATE_STALL => 8,
            UDMA_STAT_STATER::UDMA_STAT_STATE_DONE => 9,
            UDMA_STAT_STATER::UDMA_STAT_STATE_UNDEF => 10,
            UDMA_STAT_STATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> UDMA_STAT_STATER {
        match value {
            0 => UDMA_STAT_STATER::UDMA_STAT_STATE_IDLE,
            1 => UDMA_STAT_STATER::UDMA_STAT_STATE_RD_CTRL,
            2 => UDMA_STAT_STATER::UDMA_STAT_STATE_RD_SRCENDP,
            3 => UDMA_STAT_STATER::UDMA_STAT_STATE_RD_DSTENDP,
            4 => UDMA_STAT_STATER::UDMA_STAT_STATE_RD_SRCDAT,
            5 => UDMA_STAT_STATER::UDMA_STAT_STATE_WR_DSTDAT,
            6 => UDMA_STAT_STATER::UDMA_STAT_STATE_WAIT,
            7 => UDMA_STAT_STATER::UDMA_STAT_STATE_WR_CTRL,
            8 => UDMA_STAT_STATER::UDMA_STAT_STATE_STALL,
            9 => UDMA_STAT_STATER::UDMA_STAT_STATE_DONE,
            10 => UDMA_STAT_STATER::UDMA_STAT_STATE_UNDEF,
            i => UDMA_STAT_STATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_IDLE`"]
    #[inline(always)]
    pub fn is_udma_stat_state_idle(&self) -> bool {
        *self == UDMA_STAT_STATER::UDMA_STAT_STATE_IDLE
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_RD_CTRL`"]
    #[inline(always)]
    pub fn is_udma_stat_state_rd_ctrl(&self) -> bool {
        *self == UDMA_STAT_STATER::UDMA_STAT_STATE_RD_CTRL
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_RD_SRCENDP`"]
    #[inline(always)]
    pub fn is_udma_stat_state_rd_srcendp(&self) -> bool {
        *self == UDMA_STAT_STATER::UDMA_STAT_STATE_RD_SRCENDP
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_RD_DSTENDP`"]
    #[inline(always)]
    pub fn is_udma_stat_state_rd_dstendp(&self) -> bool {
        *self == UDMA_STAT_STATER::UDMA_STAT_STATE_RD_DSTENDP
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_RD_SRCDAT`"]
    #[inline(always)]
    pub fn is_udma_stat_state_rd_srcdat(&self) -> bool {
        *self == UDMA_STAT_STATER::UDMA_STAT_STATE_RD_SRCDAT
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_WR_DSTDAT`"]
    #[inline(always)]
    pub fn is_udma_stat_state_wr_dstdat(&self) -> bool {
        *self == UDMA_STAT_STATER::UDMA_STAT_STATE_WR_DSTDAT
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_WAIT`"]
    #[inline(always)]
    pub fn is_udma_stat_state_wait(&self) -> bool {
        *self == UDMA_STAT_STATER::UDMA_STAT_STATE_WAIT
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_WR_CTRL`"]
    #[inline(always)]
    pub fn is_udma_stat_state_wr_ctrl(&self) -> bool {
        *self == UDMA_STAT_STATER::UDMA_STAT_STATE_WR_CTRL
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_STALL`"]
    #[inline(always)]
    pub fn is_udma_stat_state_stall(&self) -> bool {
        *self == UDMA_STAT_STATER::UDMA_STAT_STATE_STALL
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_DONE`"]
    #[inline(always)]
    pub fn is_udma_stat_state_done(&self) -> bool {
        *self == UDMA_STAT_STATER::UDMA_STAT_STATE_DONE
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_UNDEF`"]
    #[inline(always)]
    pub fn is_udma_stat_state_undef(&self) -> bool {
        *self == UDMA_STAT_STATER::UDMA_STAT_STATE_UNDEF
    }
}
#[doc = "Values that can be written to the field `UDMA_STAT_STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDMA_STAT_STATEW {
    #[doc = "Idle"]
    UDMA_STAT_STATE_IDLE,
    #[doc = "Reading channel controller data"]
    UDMA_STAT_STATE_RD_CTRL,
    #[doc = "Reading source end pointer"]
    UDMA_STAT_STATE_RD_SRCENDP,
    #[doc = "Reading destination end pointer"]
    UDMA_STAT_STATE_RD_DSTENDP,
    #[doc = "Reading source data"]
    UDMA_STAT_STATE_RD_SRCDAT,
    #[doc = "Writing destination data"]
    UDMA_STAT_STATE_WR_DSTDAT,
    #[doc = "Waiting for uDMA request to clear"]
    UDMA_STAT_STATE_WAIT,
    #[doc = "Writing channel controller data"]
    UDMA_STAT_STATE_WR_CTRL,
    #[doc = "Stalled"]
    UDMA_STAT_STATE_STALL,
    #[doc = "Done"]
    UDMA_STAT_STATE_DONE,
    #[doc = "Undefined"]
    UDMA_STAT_STATE_UNDEF,
}
impl UDMA_STAT_STATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            UDMA_STAT_STATEW::UDMA_STAT_STATE_IDLE => 0,
            UDMA_STAT_STATEW::UDMA_STAT_STATE_RD_CTRL => 1,
            UDMA_STAT_STATEW::UDMA_STAT_STATE_RD_SRCENDP => 2,
            UDMA_STAT_STATEW::UDMA_STAT_STATE_RD_DSTENDP => 3,
            UDMA_STAT_STATEW::UDMA_STAT_STATE_RD_SRCDAT => 4,
            UDMA_STAT_STATEW::UDMA_STAT_STATE_WR_DSTDAT => 5,
            UDMA_STAT_STATEW::UDMA_STAT_STATE_WAIT => 6,
            UDMA_STAT_STATEW::UDMA_STAT_STATE_WR_CTRL => 7,
            UDMA_STAT_STATEW::UDMA_STAT_STATE_STALL => 8,
            UDMA_STAT_STATEW::UDMA_STAT_STATE_DONE => 9,
            UDMA_STAT_STATEW::UDMA_STAT_STATE_UNDEF => 10,
        }
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_STAT_STATEW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_STAT_STATEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UDMA_STAT_STATEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn udma_stat_state_idle(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATEW::UDMA_STAT_STATE_IDLE)
    }
    #[doc = "Reading channel controller data"]
    #[inline(always)]
    pub fn udma_stat_state_rd_ctrl(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATEW::UDMA_STAT_STATE_RD_CTRL)
    }
    #[doc = "Reading source end pointer"]
    #[inline(always)]
    pub fn udma_stat_state_rd_srcendp(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATEW::UDMA_STAT_STATE_RD_SRCENDP)
    }
    #[doc = "Reading destination end pointer"]
    #[inline(always)]
    pub fn udma_stat_state_rd_dstendp(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATEW::UDMA_STAT_STATE_RD_DSTENDP)
    }
    #[doc = "Reading source data"]
    #[inline(always)]
    pub fn udma_stat_state_rd_srcdat(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATEW::UDMA_STAT_STATE_RD_SRCDAT)
    }
    #[doc = "Writing destination data"]
    #[inline(always)]
    pub fn udma_stat_state_wr_dstdat(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATEW::UDMA_STAT_STATE_WR_DSTDAT)
    }
    #[doc = "Waiting for uDMA request to clear"]
    #[inline(always)]
    pub fn udma_stat_state_wait(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATEW::UDMA_STAT_STATE_WAIT)
    }
    #[doc = "Writing channel controller data"]
    #[inline(always)]
    pub fn udma_stat_state_wr_ctrl(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATEW::UDMA_STAT_STATE_WR_CTRL)
    }
    #[doc = "Stalled"]
    #[inline(always)]
    pub fn udma_stat_state_stall(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATEW::UDMA_STAT_STATE_STALL)
    }
    #[doc = "Done"]
    #[inline(always)]
    pub fn udma_stat_state_done(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATEW::UDMA_STAT_STATE_DONE)
    }
    #[doc = "Undefined"]
    #[inline(always)]
    pub fn udma_stat_state_undef(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATEW::UDMA_STAT_STATE_UNDEF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 4);
        self.w.bits |= ((value as u32) & 15) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_STAT_DMACHANSR {
    bits: u8,
}
impl UDMA_STAT_DMACHANSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_STAT_DMACHANSW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_STAT_DMACHANSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(31 << 16);
        self.w.bits |= ((value as u32) & 31) << 16;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Master Enable Status"]
    #[inline(always)]
    pub fn udma_stat_masten(&self) -> UDMA_STAT_MASTENR {
        let bits = ((self.bits >> 0) & 1) != 0;
        UDMA_STAT_MASTENR { bits }
    }
    #[doc = "Bits 4:7 - Control State Machine Status"]
    #[inline(always)]
    pub fn udma_stat_state(&self) -> UDMA_STAT_STATER {
        UDMA_STAT_STATER::_from(((self.bits >> 4) & 15) as u8)
    }
    #[doc = "Bits 16:20 - Available uDMA Channels Minus 1"]
    #[inline(always)]
    pub fn udma_stat_dmachans(&self) -> UDMA_STAT_DMACHANSR {
        let bits = ((self.bits >> 16) & 31) as u8;
        UDMA_STAT_DMACHANSR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Master Enable Status"]
    #[inline(always)]
    pub fn udma_stat_masten(&mut self) -> _UDMA_STAT_MASTENW {
        _UDMA_STAT_MASTENW { w: self }
    }
    #[doc = "Bits 4:7 - Control State Machine Status"]
    #[inline(always)]
    pub fn udma_stat_state(&mut self) -> _UDMA_STAT_STATEW {
        _UDMA_STAT_STATEW { w: self }
    }
    #[doc = "Bits 16:20 - Available uDMA Channels Minus 1"]
    #[inline(always)]
    pub fn udma_stat_dmachans(&mut self) -> _UDMA_STAT_DMACHANSW {
        _UDMA_STAT_DMACHANSW { w: self }
    }
}
