#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYNC {
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
#[doc = "Possible values of the field `TIMER_SYNC_SYNCT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_SYNC_SYNCT0R {
    #[doc = "GPTM0 is not affected"]
    TIMER_SYNC_SYNCT0_NONE,
    #[doc = "A timeout event for Timer A of GPTM0 is triggered"]
    TIMER_SYNC_SYNCT0_TA,
    #[doc = "A timeout event for Timer B of GPTM0 is triggered"]
    TIMER_SYNC_SYNCT0_TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM0 is triggered"]
    TIMER_SYNC_SYNCT0_TATB,
}
impl TIMER_SYNC_SYNCT0R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER_SYNC_SYNCT0R::TIMER_SYNC_SYNCT0_NONE => 0,
            TIMER_SYNC_SYNCT0R::TIMER_SYNC_SYNCT0_TA => 1,
            TIMER_SYNC_SYNCT0R::TIMER_SYNC_SYNCT0_TB => 2,
            TIMER_SYNC_SYNCT0R::TIMER_SYNC_SYNCT0_TATB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> TIMER_SYNC_SYNCT0R {
        match value {
            0 => TIMER_SYNC_SYNCT0R::TIMER_SYNC_SYNCT0_NONE,
            1 => TIMER_SYNC_SYNCT0R::TIMER_SYNC_SYNCT0_TA,
            2 => TIMER_SYNC_SYNCT0R::TIMER_SYNC_SYNCT0_TB,
            3 => TIMER_SYNC_SYNCT0R::TIMER_SYNC_SYNCT0_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT0_NONE`"]
    #[inline(always)]
    pub fn is_timer_sync_synct0_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT0R::TIMER_SYNC_SYNCT0_NONE
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT0_TA`"]
    #[inline(always)]
    pub fn is_timer_sync_synct0_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT0R::TIMER_SYNC_SYNCT0_TA
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT0_TB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct0_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT0R::TIMER_SYNC_SYNCT0_TB
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT0_TATB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct0_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT0R::TIMER_SYNC_SYNCT0_TATB
    }
}
#[doc = "Values that can be written to the field `TIMER_SYNC_SYNCT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_SYNC_SYNCT0W {
    #[doc = "GPTM0 is not affected"]
    TIMER_SYNC_SYNCT0_NONE,
    #[doc = "A timeout event for Timer A of GPTM0 is triggered"]
    TIMER_SYNC_SYNCT0_TA,
    #[doc = "A timeout event for Timer B of GPTM0 is triggered"]
    TIMER_SYNC_SYNCT0_TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM0 is triggered"]
    TIMER_SYNC_SYNCT0_TATB,
}
impl TIMER_SYNC_SYNCT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMER_SYNC_SYNCT0W::TIMER_SYNC_SYNCT0_NONE => 0,
            TIMER_SYNC_SYNCT0W::TIMER_SYNC_SYNCT0_TA => 1,
            TIMER_SYNC_SYNCT0W::TIMER_SYNC_SYNCT0_TB => 2,
            TIMER_SYNC_SYNCT0W::TIMER_SYNC_SYNCT0_TATB => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TIMER_SYNC_SYNCT0W<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_SYNC_SYNCT0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_SYNC_SYNCT0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPTM0 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct0_none(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT0W::TIMER_SYNC_SYNCT0_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM0 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct0_ta(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT0W::TIMER_SYNC_SYNCT0_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM0 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct0_tb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT0W::TIMER_SYNC_SYNCT0_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM0 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct0_tatb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT0W::TIMER_SYNC_SYNCT0_TATB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `TIMER_SYNC_SYNCT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_SYNC_SYNCT1R {
    #[doc = "GPTM1 is not affected"]
    TIMER_SYNC_SYNCT1_NONE,
    #[doc = "A timeout event for Timer A of GPTM1 is triggered"]
    TIMER_SYNC_SYNCT1_TA,
    #[doc = "A timeout event for Timer B of GPTM1 is triggered"]
    TIMER_SYNC_SYNCT1_TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM1 is triggered"]
    TIMER_SYNC_SYNCT1_TATB,
}
impl TIMER_SYNC_SYNCT1R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER_SYNC_SYNCT1R::TIMER_SYNC_SYNCT1_NONE => 0,
            TIMER_SYNC_SYNCT1R::TIMER_SYNC_SYNCT1_TA => 1,
            TIMER_SYNC_SYNCT1R::TIMER_SYNC_SYNCT1_TB => 2,
            TIMER_SYNC_SYNCT1R::TIMER_SYNC_SYNCT1_TATB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> TIMER_SYNC_SYNCT1R {
        match value {
            0 => TIMER_SYNC_SYNCT1R::TIMER_SYNC_SYNCT1_NONE,
            1 => TIMER_SYNC_SYNCT1R::TIMER_SYNC_SYNCT1_TA,
            2 => TIMER_SYNC_SYNCT1R::TIMER_SYNC_SYNCT1_TB,
            3 => TIMER_SYNC_SYNCT1R::TIMER_SYNC_SYNCT1_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT1_NONE`"]
    #[inline(always)]
    pub fn is_timer_sync_synct1_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT1R::TIMER_SYNC_SYNCT1_NONE
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT1_TA`"]
    #[inline(always)]
    pub fn is_timer_sync_synct1_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT1R::TIMER_SYNC_SYNCT1_TA
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT1_TB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct1_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT1R::TIMER_SYNC_SYNCT1_TB
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT1_TATB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct1_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT1R::TIMER_SYNC_SYNCT1_TATB
    }
}
#[doc = "Values that can be written to the field `TIMER_SYNC_SYNCT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_SYNC_SYNCT1W {
    #[doc = "GPTM1 is not affected"]
    TIMER_SYNC_SYNCT1_NONE,
    #[doc = "A timeout event for Timer A of GPTM1 is triggered"]
    TIMER_SYNC_SYNCT1_TA,
    #[doc = "A timeout event for Timer B of GPTM1 is triggered"]
    TIMER_SYNC_SYNCT1_TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM1 is triggered"]
    TIMER_SYNC_SYNCT1_TATB,
}
impl TIMER_SYNC_SYNCT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMER_SYNC_SYNCT1W::TIMER_SYNC_SYNCT1_NONE => 0,
            TIMER_SYNC_SYNCT1W::TIMER_SYNC_SYNCT1_TA => 1,
            TIMER_SYNC_SYNCT1W::TIMER_SYNC_SYNCT1_TB => 2,
            TIMER_SYNC_SYNCT1W::TIMER_SYNC_SYNCT1_TATB => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TIMER_SYNC_SYNCT1W<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_SYNC_SYNCT1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_SYNC_SYNCT1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPTM1 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct1_none(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT1W::TIMER_SYNC_SYNCT1_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM1 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct1_ta(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT1W::TIMER_SYNC_SYNCT1_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM1 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct1_tb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT1W::TIMER_SYNC_SYNCT1_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM1 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct1_tatb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT1W::TIMER_SYNC_SYNCT1_TATB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 2);
        self.w.bits |= ((value as u32) & 3) << 2;
        self.w
    }
}
#[doc = "Possible values of the field `TIMER_SYNC_SYNCT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_SYNC_SYNCT2R {
    #[doc = "GPTM2 is not affected"]
    TIMER_SYNC_SYNCT2_NONE,
    #[doc = "A timeout event for Timer A of GPTM2 is triggered"]
    TIMER_SYNC_SYNCT2_TA,
    #[doc = "A timeout event for Timer B of GPTM2 is triggered"]
    TIMER_SYNC_SYNCT2_TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM2 is triggered"]
    TIMER_SYNC_SYNCT2_TATB,
}
impl TIMER_SYNC_SYNCT2R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER_SYNC_SYNCT2R::TIMER_SYNC_SYNCT2_NONE => 0,
            TIMER_SYNC_SYNCT2R::TIMER_SYNC_SYNCT2_TA => 1,
            TIMER_SYNC_SYNCT2R::TIMER_SYNC_SYNCT2_TB => 2,
            TIMER_SYNC_SYNCT2R::TIMER_SYNC_SYNCT2_TATB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> TIMER_SYNC_SYNCT2R {
        match value {
            0 => TIMER_SYNC_SYNCT2R::TIMER_SYNC_SYNCT2_NONE,
            1 => TIMER_SYNC_SYNCT2R::TIMER_SYNC_SYNCT2_TA,
            2 => TIMER_SYNC_SYNCT2R::TIMER_SYNC_SYNCT2_TB,
            3 => TIMER_SYNC_SYNCT2R::TIMER_SYNC_SYNCT2_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT2_NONE`"]
    #[inline(always)]
    pub fn is_timer_sync_synct2_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT2R::TIMER_SYNC_SYNCT2_NONE
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT2_TA`"]
    #[inline(always)]
    pub fn is_timer_sync_synct2_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT2R::TIMER_SYNC_SYNCT2_TA
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT2_TB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct2_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT2R::TIMER_SYNC_SYNCT2_TB
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT2_TATB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct2_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT2R::TIMER_SYNC_SYNCT2_TATB
    }
}
#[doc = "Values that can be written to the field `TIMER_SYNC_SYNCT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_SYNC_SYNCT2W {
    #[doc = "GPTM2 is not affected"]
    TIMER_SYNC_SYNCT2_NONE,
    #[doc = "A timeout event for Timer A of GPTM2 is triggered"]
    TIMER_SYNC_SYNCT2_TA,
    #[doc = "A timeout event for Timer B of GPTM2 is triggered"]
    TIMER_SYNC_SYNCT2_TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM2 is triggered"]
    TIMER_SYNC_SYNCT2_TATB,
}
impl TIMER_SYNC_SYNCT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMER_SYNC_SYNCT2W::TIMER_SYNC_SYNCT2_NONE => 0,
            TIMER_SYNC_SYNCT2W::TIMER_SYNC_SYNCT2_TA => 1,
            TIMER_SYNC_SYNCT2W::TIMER_SYNC_SYNCT2_TB => 2,
            TIMER_SYNC_SYNCT2W::TIMER_SYNC_SYNCT2_TATB => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TIMER_SYNC_SYNCT2W<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_SYNC_SYNCT2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_SYNC_SYNCT2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPTM2 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct2_none(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT2W::TIMER_SYNC_SYNCT2_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM2 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct2_ta(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT2W::TIMER_SYNC_SYNCT2_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM2 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct2_tb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT2W::TIMER_SYNC_SYNCT2_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM2 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct2_tatb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT2W::TIMER_SYNC_SYNCT2_TATB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = "Possible values of the field `TIMER_SYNC_SYNCT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_SYNC_SYNCT3R {
    #[doc = "GPTM3 is not affected"]
    TIMER_SYNC_SYNCT3_NONE,
    #[doc = "A timeout event for Timer A of GPTM3 is triggered"]
    TIMER_SYNC_SYNCT3_TA,
    #[doc = "A timeout event for Timer B of GPTM3 is triggered"]
    TIMER_SYNC_SYNCT3_TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM3 is triggered"]
    TIMER_SYNC_SYNCT3_TATB,
}
impl TIMER_SYNC_SYNCT3R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER_SYNC_SYNCT3R::TIMER_SYNC_SYNCT3_NONE => 0,
            TIMER_SYNC_SYNCT3R::TIMER_SYNC_SYNCT3_TA => 1,
            TIMER_SYNC_SYNCT3R::TIMER_SYNC_SYNCT3_TB => 2,
            TIMER_SYNC_SYNCT3R::TIMER_SYNC_SYNCT3_TATB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> TIMER_SYNC_SYNCT3R {
        match value {
            0 => TIMER_SYNC_SYNCT3R::TIMER_SYNC_SYNCT3_NONE,
            1 => TIMER_SYNC_SYNCT3R::TIMER_SYNC_SYNCT3_TA,
            2 => TIMER_SYNC_SYNCT3R::TIMER_SYNC_SYNCT3_TB,
            3 => TIMER_SYNC_SYNCT3R::TIMER_SYNC_SYNCT3_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT3_NONE`"]
    #[inline(always)]
    pub fn is_timer_sync_synct3_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT3R::TIMER_SYNC_SYNCT3_NONE
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT3_TA`"]
    #[inline(always)]
    pub fn is_timer_sync_synct3_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT3R::TIMER_SYNC_SYNCT3_TA
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT3_TB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct3_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT3R::TIMER_SYNC_SYNCT3_TB
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT3_TATB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct3_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT3R::TIMER_SYNC_SYNCT3_TATB
    }
}
#[doc = "Values that can be written to the field `TIMER_SYNC_SYNCT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_SYNC_SYNCT3W {
    #[doc = "GPTM3 is not affected"]
    TIMER_SYNC_SYNCT3_NONE,
    #[doc = "A timeout event for Timer A of GPTM3 is triggered"]
    TIMER_SYNC_SYNCT3_TA,
    #[doc = "A timeout event for Timer B of GPTM3 is triggered"]
    TIMER_SYNC_SYNCT3_TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM3 is triggered"]
    TIMER_SYNC_SYNCT3_TATB,
}
impl TIMER_SYNC_SYNCT3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMER_SYNC_SYNCT3W::TIMER_SYNC_SYNCT3_NONE => 0,
            TIMER_SYNC_SYNCT3W::TIMER_SYNC_SYNCT3_TA => 1,
            TIMER_SYNC_SYNCT3W::TIMER_SYNC_SYNCT3_TB => 2,
            TIMER_SYNC_SYNCT3W::TIMER_SYNC_SYNCT3_TATB => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TIMER_SYNC_SYNCT3W<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_SYNC_SYNCT3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_SYNC_SYNCT3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPTM3 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct3_none(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT3W::TIMER_SYNC_SYNCT3_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM3 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct3_ta(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT3W::TIMER_SYNC_SYNCT3_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM3 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct3_tb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT3W::TIMER_SYNC_SYNCT3_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM3 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct3_tatb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT3W::TIMER_SYNC_SYNCT3_TATB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 6);
        self.w.bits |= ((value as u32) & 3) << 6;
        self.w
    }
}
#[doc = "Possible values of the field `TIMER_SYNC_SYNCT4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_SYNC_SYNCT4R {
    #[doc = "GPTM4 is not affected"]
    TIMER_SYNC_SYNCT4_NONE,
    #[doc = "A timeout event for Timer A of GPTM4 is triggered"]
    TIMER_SYNC_SYNCT4_TA,
    #[doc = "A timeout event for Timer B of GPTM4 is triggered"]
    TIMER_SYNC_SYNCT4_TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM4 is triggered"]
    TIMER_SYNC_SYNCT4_TATB,
}
impl TIMER_SYNC_SYNCT4R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER_SYNC_SYNCT4R::TIMER_SYNC_SYNCT4_NONE => 0,
            TIMER_SYNC_SYNCT4R::TIMER_SYNC_SYNCT4_TA => 1,
            TIMER_SYNC_SYNCT4R::TIMER_SYNC_SYNCT4_TB => 2,
            TIMER_SYNC_SYNCT4R::TIMER_SYNC_SYNCT4_TATB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> TIMER_SYNC_SYNCT4R {
        match value {
            0 => TIMER_SYNC_SYNCT4R::TIMER_SYNC_SYNCT4_NONE,
            1 => TIMER_SYNC_SYNCT4R::TIMER_SYNC_SYNCT4_TA,
            2 => TIMER_SYNC_SYNCT4R::TIMER_SYNC_SYNCT4_TB,
            3 => TIMER_SYNC_SYNCT4R::TIMER_SYNC_SYNCT4_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT4_NONE`"]
    #[inline(always)]
    pub fn is_timer_sync_synct4_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT4R::TIMER_SYNC_SYNCT4_NONE
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT4_TA`"]
    #[inline(always)]
    pub fn is_timer_sync_synct4_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT4R::TIMER_SYNC_SYNCT4_TA
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT4_TB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct4_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT4R::TIMER_SYNC_SYNCT4_TB
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT4_TATB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct4_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT4R::TIMER_SYNC_SYNCT4_TATB
    }
}
#[doc = "Values that can be written to the field `TIMER_SYNC_SYNCT4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_SYNC_SYNCT4W {
    #[doc = "GPTM4 is not affected"]
    TIMER_SYNC_SYNCT4_NONE,
    #[doc = "A timeout event for Timer A of GPTM4 is triggered"]
    TIMER_SYNC_SYNCT4_TA,
    #[doc = "A timeout event for Timer B of GPTM4 is triggered"]
    TIMER_SYNC_SYNCT4_TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM4 is triggered"]
    TIMER_SYNC_SYNCT4_TATB,
}
impl TIMER_SYNC_SYNCT4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMER_SYNC_SYNCT4W::TIMER_SYNC_SYNCT4_NONE => 0,
            TIMER_SYNC_SYNCT4W::TIMER_SYNC_SYNCT4_TA => 1,
            TIMER_SYNC_SYNCT4W::TIMER_SYNC_SYNCT4_TB => 2,
            TIMER_SYNC_SYNCT4W::TIMER_SYNC_SYNCT4_TATB => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TIMER_SYNC_SYNCT4W<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_SYNC_SYNCT4W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_SYNC_SYNCT4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPTM4 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct4_none(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT4W::TIMER_SYNC_SYNCT4_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM4 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct4_ta(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT4W::TIMER_SYNC_SYNCT4_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM4 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct4_tb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT4W::TIMER_SYNC_SYNCT4_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM4 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct4_tatb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT4W::TIMER_SYNC_SYNCT4_TATB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 8);
        self.w.bits |= ((value as u32) & 3) << 8;
        self.w
    }
}
#[doc = "Possible values of the field `TIMER_SYNC_SYNCT5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_SYNC_SYNCT5R {
    #[doc = "GPTM5 is not affected"]
    TIMER_SYNC_SYNCT5_NONE,
    #[doc = "A timeout event for Timer A of GPTM5 is triggered"]
    TIMER_SYNC_SYNCT5_TA,
    #[doc = "A timeout event for Timer B of GPTM5 is triggered"]
    TIMER_SYNC_SYNCT5_TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM5 is triggered"]
    TIMER_SYNC_SYNCT5_TATB,
}
impl TIMER_SYNC_SYNCT5R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER_SYNC_SYNCT5R::TIMER_SYNC_SYNCT5_NONE => 0,
            TIMER_SYNC_SYNCT5R::TIMER_SYNC_SYNCT5_TA => 1,
            TIMER_SYNC_SYNCT5R::TIMER_SYNC_SYNCT5_TB => 2,
            TIMER_SYNC_SYNCT5R::TIMER_SYNC_SYNCT5_TATB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> TIMER_SYNC_SYNCT5R {
        match value {
            0 => TIMER_SYNC_SYNCT5R::TIMER_SYNC_SYNCT5_NONE,
            1 => TIMER_SYNC_SYNCT5R::TIMER_SYNC_SYNCT5_TA,
            2 => TIMER_SYNC_SYNCT5R::TIMER_SYNC_SYNCT5_TB,
            3 => TIMER_SYNC_SYNCT5R::TIMER_SYNC_SYNCT5_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT5_NONE`"]
    #[inline(always)]
    pub fn is_timer_sync_synct5_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT5R::TIMER_SYNC_SYNCT5_NONE
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT5_TA`"]
    #[inline(always)]
    pub fn is_timer_sync_synct5_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT5R::TIMER_SYNC_SYNCT5_TA
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT5_TB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct5_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT5R::TIMER_SYNC_SYNCT5_TB
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT5_TATB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct5_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT5R::TIMER_SYNC_SYNCT5_TATB
    }
}
#[doc = "Values that can be written to the field `TIMER_SYNC_SYNCT5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_SYNC_SYNCT5W {
    #[doc = "GPTM5 is not affected"]
    TIMER_SYNC_SYNCT5_NONE,
    #[doc = "A timeout event for Timer A of GPTM5 is triggered"]
    TIMER_SYNC_SYNCT5_TA,
    #[doc = "A timeout event for Timer B of GPTM5 is triggered"]
    TIMER_SYNC_SYNCT5_TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM5 is triggered"]
    TIMER_SYNC_SYNCT5_TATB,
}
impl TIMER_SYNC_SYNCT5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMER_SYNC_SYNCT5W::TIMER_SYNC_SYNCT5_NONE => 0,
            TIMER_SYNC_SYNCT5W::TIMER_SYNC_SYNCT5_TA => 1,
            TIMER_SYNC_SYNCT5W::TIMER_SYNC_SYNCT5_TB => 2,
            TIMER_SYNC_SYNCT5W::TIMER_SYNC_SYNCT5_TATB => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TIMER_SYNC_SYNCT5W<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_SYNC_SYNCT5W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_SYNC_SYNCT5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPTM5 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct5_none(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT5W::TIMER_SYNC_SYNCT5_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM5 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct5_ta(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT5W::TIMER_SYNC_SYNCT5_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM5 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct5_tb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT5W::TIMER_SYNC_SYNCT5_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM5 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct5_tatb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT5W::TIMER_SYNC_SYNCT5_TATB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 10);
        self.w.bits |= ((value as u32) & 3) << 10;
        self.w
    }
}
#[doc = "Possible values of the field `TIMER_SYNC_SYNCT6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_SYNC_SYNCT6R {
    #[doc = "GPTM6 is not affected"]
    TIMER_SYNC_SYNCT6_NONE,
    #[doc = "A timeout event for Timer A of GPTM6 is triggered"]
    TIMER_SYNC_SYNCT6_TA,
    #[doc = "A timeout event for Timer B of GPTM6 is triggered"]
    TIMER_SYNC_SYNCT6_TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM6 is triggered"]
    TIMER_SYNC_SYNCT6_TATB,
}
impl TIMER_SYNC_SYNCT6R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER_SYNC_SYNCT6R::TIMER_SYNC_SYNCT6_NONE => 0,
            TIMER_SYNC_SYNCT6R::TIMER_SYNC_SYNCT6_TA => 1,
            TIMER_SYNC_SYNCT6R::TIMER_SYNC_SYNCT6_TB => 2,
            TIMER_SYNC_SYNCT6R::TIMER_SYNC_SYNCT6_TATB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> TIMER_SYNC_SYNCT6R {
        match value {
            0 => TIMER_SYNC_SYNCT6R::TIMER_SYNC_SYNCT6_NONE,
            1 => TIMER_SYNC_SYNCT6R::TIMER_SYNC_SYNCT6_TA,
            2 => TIMER_SYNC_SYNCT6R::TIMER_SYNC_SYNCT6_TB,
            3 => TIMER_SYNC_SYNCT6R::TIMER_SYNC_SYNCT6_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT6_NONE`"]
    #[inline(always)]
    pub fn is_timer_sync_synct6_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT6R::TIMER_SYNC_SYNCT6_NONE
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT6_TA`"]
    #[inline(always)]
    pub fn is_timer_sync_synct6_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT6R::TIMER_SYNC_SYNCT6_TA
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT6_TB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct6_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT6R::TIMER_SYNC_SYNCT6_TB
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT6_TATB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct6_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT6R::TIMER_SYNC_SYNCT6_TATB
    }
}
#[doc = "Values that can be written to the field `TIMER_SYNC_SYNCT6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_SYNC_SYNCT6W {
    #[doc = "GPTM6 is not affected"]
    TIMER_SYNC_SYNCT6_NONE,
    #[doc = "A timeout event for Timer A of GPTM6 is triggered"]
    TIMER_SYNC_SYNCT6_TA,
    #[doc = "A timeout event for Timer B of GPTM6 is triggered"]
    TIMER_SYNC_SYNCT6_TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM6 is triggered"]
    TIMER_SYNC_SYNCT6_TATB,
}
impl TIMER_SYNC_SYNCT6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMER_SYNC_SYNCT6W::TIMER_SYNC_SYNCT6_NONE => 0,
            TIMER_SYNC_SYNCT6W::TIMER_SYNC_SYNCT6_TA => 1,
            TIMER_SYNC_SYNCT6W::TIMER_SYNC_SYNCT6_TB => 2,
            TIMER_SYNC_SYNCT6W::TIMER_SYNC_SYNCT6_TATB => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TIMER_SYNC_SYNCT6W<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_SYNC_SYNCT6W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_SYNC_SYNCT6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPTM6 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct6_none(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT6W::TIMER_SYNC_SYNCT6_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM6 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct6_ta(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT6W::TIMER_SYNC_SYNCT6_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM6 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct6_tb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT6W::TIMER_SYNC_SYNCT6_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM6 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct6_tatb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT6W::TIMER_SYNC_SYNCT6_TATB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 12);
        self.w.bits |= ((value as u32) & 3) << 12;
        self.w
    }
}
#[doc = "Possible values of the field `TIMER_SYNC_SYNCT7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_SYNC_SYNCT7R {
    #[doc = "GPT7 is not affected"]
    TIMER_SYNC_SYNCT7_NONE,
    #[doc = "A timeout event for Timer A of GPTM7 is triggered"]
    TIMER_SYNC_SYNCT7_TA,
    #[doc = "A timeout event for Timer B of GPTM7 is triggered"]
    TIMER_SYNC_SYNCT7_TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM7 is triggered"]
    TIMER_SYNC_SYNCT7_TATB,
}
impl TIMER_SYNC_SYNCT7R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER_SYNC_SYNCT7R::TIMER_SYNC_SYNCT7_NONE => 0,
            TIMER_SYNC_SYNCT7R::TIMER_SYNC_SYNCT7_TA => 1,
            TIMER_SYNC_SYNCT7R::TIMER_SYNC_SYNCT7_TB => 2,
            TIMER_SYNC_SYNCT7R::TIMER_SYNC_SYNCT7_TATB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> TIMER_SYNC_SYNCT7R {
        match value {
            0 => TIMER_SYNC_SYNCT7R::TIMER_SYNC_SYNCT7_NONE,
            1 => TIMER_SYNC_SYNCT7R::TIMER_SYNC_SYNCT7_TA,
            2 => TIMER_SYNC_SYNCT7R::TIMER_SYNC_SYNCT7_TB,
            3 => TIMER_SYNC_SYNCT7R::TIMER_SYNC_SYNCT7_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT7_NONE`"]
    #[inline(always)]
    pub fn is_timer_sync_synct7_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT7R::TIMER_SYNC_SYNCT7_NONE
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT7_TA`"]
    #[inline(always)]
    pub fn is_timer_sync_synct7_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT7R::TIMER_SYNC_SYNCT7_TA
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT7_TB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct7_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT7R::TIMER_SYNC_SYNCT7_TB
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT7_TATB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct7_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT7R::TIMER_SYNC_SYNCT7_TATB
    }
}
#[doc = "Values that can be written to the field `TIMER_SYNC_SYNCT7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_SYNC_SYNCT7W {
    #[doc = "GPT7 is not affected"]
    TIMER_SYNC_SYNCT7_NONE,
    #[doc = "A timeout event for Timer A of GPTM7 is triggered"]
    TIMER_SYNC_SYNCT7_TA,
    #[doc = "A timeout event for Timer B of GPTM7 is triggered"]
    TIMER_SYNC_SYNCT7_TB,
    #[doc = "A timeout event for both Timer A and Timer B of GPTM7 is triggered"]
    TIMER_SYNC_SYNCT7_TATB,
}
impl TIMER_SYNC_SYNCT7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMER_SYNC_SYNCT7W::TIMER_SYNC_SYNCT7_NONE => 0,
            TIMER_SYNC_SYNCT7W::TIMER_SYNC_SYNCT7_TA => 1,
            TIMER_SYNC_SYNCT7W::TIMER_SYNC_SYNCT7_TB => 2,
            TIMER_SYNC_SYNCT7W::TIMER_SYNC_SYNCT7_TATB => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TIMER_SYNC_SYNCT7W<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_SYNC_SYNCT7W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_SYNC_SYNCT7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPT7 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct7_none(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT7W::TIMER_SYNC_SYNCT7_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM7 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct7_ta(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT7W::TIMER_SYNC_SYNCT7_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM7 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct7_tb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT7W::TIMER_SYNC_SYNCT7_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM7 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct7_tatb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT7W::TIMER_SYNC_SYNCT7_TATB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 14);
        self.w.bits |= ((value as u32) & 3) << 14;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Synchronize GPTM Timer 0"]
    #[inline(always)]
    pub fn timer_sync_synct0(&self) -> TIMER_SYNC_SYNCT0R {
        TIMER_SYNC_SYNCT0R::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bits 2:3 - Synchronize GPTM Timer 1"]
    #[inline(always)]
    pub fn timer_sync_synct1(&self) -> TIMER_SYNC_SYNCT1R {
        TIMER_SYNC_SYNCT1R::_from(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Synchronize GPTM Timer 2"]
    #[inline(always)]
    pub fn timer_sync_synct2(&self) -> TIMER_SYNC_SYNCT2R {
        TIMER_SYNC_SYNCT2R::_from(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Synchronize GPTM Timer 3"]
    #[inline(always)]
    pub fn timer_sync_synct3(&self) -> TIMER_SYNC_SYNCT3R {
        TIMER_SYNC_SYNCT3R::_from(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Synchronize GPTM Timer 4"]
    #[inline(always)]
    pub fn timer_sync_synct4(&self) -> TIMER_SYNC_SYNCT4R {
        TIMER_SYNC_SYNCT4R::_from(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Synchronize GPTM Timer 5"]
    #[inline(always)]
    pub fn timer_sync_synct5(&self) -> TIMER_SYNC_SYNCT5R {
        TIMER_SYNC_SYNCT5R::_from(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Synchronize GPTM Timer 6"]
    #[inline(always)]
    pub fn timer_sync_synct6(&self) -> TIMER_SYNC_SYNCT6R {
        TIMER_SYNC_SYNCT6R::_from(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Synchronize GPTM Timer 7"]
    #[inline(always)]
    pub fn timer_sync_synct7(&self) -> TIMER_SYNC_SYNCT7R {
        TIMER_SYNC_SYNCT7R::_from(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Synchronize GPTM Timer 0"]
    #[inline(always)]
    pub fn timer_sync_synct0(&mut self) -> _TIMER_SYNC_SYNCT0W {
        _TIMER_SYNC_SYNCT0W { w: self }
    }
    #[doc = "Bits 2:3 - Synchronize GPTM Timer 1"]
    #[inline(always)]
    pub fn timer_sync_synct1(&mut self) -> _TIMER_SYNC_SYNCT1W {
        _TIMER_SYNC_SYNCT1W { w: self }
    }
    #[doc = "Bits 4:5 - Synchronize GPTM Timer 2"]
    #[inline(always)]
    pub fn timer_sync_synct2(&mut self) -> _TIMER_SYNC_SYNCT2W {
        _TIMER_SYNC_SYNCT2W { w: self }
    }
    #[doc = "Bits 6:7 - Synchronize GPTM Timer 3"]
    #[inline(always)]
    pub fn timer_sync_synct3(&mut self) -> _TIMER_SYNC_SYNCT3W {
        _TIMER_SYNC_SYNCT3W { w: self }
    }
    #[doc = "Bits 8:9 - Synchronize GPTM Timer 4"]
    #[inline(always)]
    pub fn timer_sync_synct4(&mut self) -> _TIMER_SYNC_SYNCT4W {
        _TIMER_SYNC_SYNCT4W { w: self }
    }
    #[doc = "Bits 10:11 - Synchronize GPTM Timer 5"]
    #[inline(always)]
    pub fn timer_sync_synct5(&mut self) -> _TIMER_SYNC_SYNCT5W {
        _TIMER_SYNC_SYNCT5W { w: self }
    }
    #[doc = "Bits 12:13 - Synchronize GPTM Timer 6"]
    #[inline(always)]
    pub fn timer_sync_synct6(&mut self) -> _TIMER_SYNC_SYNCT6W {
        _TIMER_SYNC_SYNCT6W { w: self }
    }
    #[doc = "Bits 14:15 - Synchronize GPTM Timer 7"]
    #[inline(always)]
    pub fn timer_sync_synct7(&mut self) -> _TIMER_SYNC_SYNCT7W {
        _TIMER_SYNC_SYNCT7W { w: self }
    }
}
