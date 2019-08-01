#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
#[doc = "Possible values of the field `TIMER_CFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_CFGR {
    #[doc = "For a 16/32-bit timer, this value selects the 32-bit timer configuration"]
    TIMER_CFG_32_BIT_TIMER,
    #[doc = "For a 16/32-bit timer, this value selects the 32-bit real-time clock (RTC) counter configuration"]
    TIMER_CFG_32_BIT_RTC,
    #[doc = "For a 16/32-bit timer, this value selects the 16-bit timer configuration"]
    TIMER_CFG_16_BIT,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl TIMER_CFGR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER_CFGR::TIMER_CFG_32_BIT_TIMER => 0,
            TIMER_CFGR::TIMER_CFG_32_BIT_RTC => 1,
            TIMER_CFGR::TIMER_CFG_16_BIT => 4,
            TIMER_CFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> TIMER_CFGR {
        match value {
            0 => TIMER_CFGR::TIMER_CFG_32_BIT_TIMER,
            1 => TIMER_CFGR::TIMER_CFG_32_BIT_RTC,
            4 => TIMER_CFGR::TIMER_CFG_16_BIT,
            i => TIMER_CFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_CFG_32_BIT_TIMER`"]
    #[inline(always)]
    pub fn is_timer_cfg_32_bit_timer(&self) -> bool {
        *self == TIMER_CFGR::TIMER_CFG_32_BIT_TIMER
    }
    #[doc = "Checks if the value of the field is `TIMER_CFG_32_BIT_RTC`"]
    #[inline(always)]
    pub fn is_timer_cfg_32_bit_rtc(&self) -> bool {
        *self == TIMER_CFGR::TIMER_CFG_32_BIT_RTC
    }
    #[doc = "Checks if the value of the field is `TIMER_CFG_16_BIT`"]
    #[inline(always)]
    pub fn is_timer_cfg_16_bit(&self) -> bool {
        *self == TIMER_CFGR::TIMER_CFG_16_BIT
    }
}
#[doc = "Values that can be written to the field `TIMER_CFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_CFGW {
    #[doc = "For a 16/32-bit timer, this value selects the 32-bit timer configuration"]
    TIMER_CFG_32_BIT_TIMER,
    #[doc = "For a 16/32-bit timer, this value selects the 32-bit real-time clock (RTC) counter configuration"]
    TIMER_CFG_32_BIT_RTC,
    #[doc = "For a 16/32-bit timer, this value selects the 16-bit timer configuration"]
    TIMER_CFG_16_BIT,
}
impl TIMER_CFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMER_CFGW::TIMER_CFG_32_BIT_TIMER => 0,
            TIMER_CFGW::TIMER_CFG_32_BIT_RTC => 1,
            TIMER_CFGW::TIMER_CFG_16_BIT => 4,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TIMER_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_CFGW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_CFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "For a 16/32-bit timer, this value selects the 32-bit timer configuration"]
    #[inline(always)]
    pub fn timer_cfg_32_bit_timer(self) -> &'a mut W {
        self.variant(TIMER_CFGW::TIMER_CFG_32_BIT_TIMER)
    }
    #[doc = "For a 16/32-bit timer, this value selects the 32-bit real-time clock (RTC) counter configuration"]
    #[inline(always)]
    pub fn timer_cfg_32_bit_rtc(self) -> &'a mut W {
        self.variant(TIMER_CFGW::TIMER_CFG_32_BIT_RTC)
    }
    #[doc = "For a 16/32-bit timer, this value selects the 16-bit timer configuration"]
    #[inline(always)]
    pub fn timer_cfg_16_bit(self) -> &'a mut W {
        self.variant(TIMER_CFGW::TIMER_CFG_16_BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 0);
        self.w.bits |= ((value as u32) & 7) << 0;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - GPTM Configuration"]
    #[inline(always)]
    pub fn timer_cfg(&self) -> TIMER_CFGR {
        TIMER_CFGR::_from(((self.bits >> 0) & 7) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - GPTM Configuration"]
    #[inline(always)]
    pub fn timer_cfg(&mut self) -> _TIMER_CFGW {
        _TIMER_CFGW { w: self }
    }
}
