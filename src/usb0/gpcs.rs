#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPCS {
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
#[doc = "Possible values of the field `USB_GPCS_DEVMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_GPCS_DEVMODR {
    #[doc = "Use USB0VBUS and USB0ID pin"]
    USB_GPCS_DEVMOD_OTG,
    #[doc = "Force USB0VBUS and USB0ID low"]
    USB_GPCS_DEVMOD_HOST,
    #[doc = "Force USB0VBUS and USB0ID high"]
    USB_GPCS_DEVMOD_DEV,
    #[doc = "Use USB0VBUS and force USB0ID low"]
    USB_GPCS_DEVMOD_HOSTVBUS,
    #[doc = "Use USB0VBUS and force USB0ID high"]
    USB_GPCS_DEVMOD_DEVVBUS,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl USB_GPCS_DEVMODR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            USB_GPCS_DEVMODR::USB_GPCS_DEVMOD_OTG => 0,
            USB_GPCS_DEVMODR::USB_GPCS_DEVMOD_HOST => 2,
            USB_GPCS_DEVMODR::USB_GPCS_DEVMOD_DEV => 3,
            USB_GPCS_DEVMODR::USB_GPCS_DEVMOD_HOSTVBUS => 4,
            USB_GPCS_DEVMODR::USB_GPCS_DEVMOD_DEVVBUS => 5,
            USB_GPCS_DEVMODR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> USB_GPCS_DEVMODR {
        match value {
            0 => USB_GPCS_DEVMODR::USB_GPCS_DEVMOD_OTG,
            2 => USB_GPCS_DEVMODR::USB_GPCS_DEVMOD_HOST,
            3 => USB_GPCS_DEVMODR::USB_GPCS_DEVMOD_DEV,
            4 => USB_GPCS_DEVMODR::USB_GPCS_DEVMOD_HOSTVBUS,
            5 => USB_GPCS_DEVMODR::USB_GPCS_DEVMOD_DEVVBUS,
            i => USB_GPCS_DEVMODR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `USB_GPCS_DEVMOD_OTG`"]
    #[inline(always)]
    pub fn is_usb_gpcs_devmod_otg(&self) -> bool {
        *self == USB_GPCS_DEVMODR::USB_GPCS_DEVMOD_OTG
    }
    #[doc = "Checks if the value of the field is `USB_GPCS_DEVMOD_HOST`"]
    #[inline(always)]
    pub fn is_usb_gpcs_devmod_host(&self) -> bool {
        *self == USB_GPCS_DEVMODR::USB_GPCS_DEVMOD_HOST
    }
    #[doc = "Checks if the value of the field is `USB_GPCS_DEVMOD_DEV`"]
    #[inline(always)]
    pub fn is_usb_gpcs_devmod_dev(&self) -> bool {
        *self == USB_GPCS_DEVMODR::USB_GPCS_DEVMOD_DEV
    }
    #[doc = "Checks if the value of the field is `USB_GPCS_DEVMOD_HOSTVBUS`"]
    #[inline(always)]
    pub fn is_usb_gpcs_devmod_hostvbus(&self) -> bool {
        *self == USB_GPCS_DEVMODR::USB_GPCS_DEVMOD_HOSTVBUS
    }
    #[doc = "Checks if the value of the field is `USB_GPCS_DEVMOD_DEVVBUS`"]
    #[inline(always)]
    pub fn is_usb_gpcs_devmod_devvbus(&self) -> bool {
        *self == USB_GPCS_DEVMODR::USB_GPCS_DEVMOD_DEVVBUS
    }
}
#[doc = "Values that can be written to the field `USB_GPCS_DEVMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_GPCS_DEVMODW {
    #[doc = "Use USB0VBUS and USB0ID pin"]
    USB_GPCS_DEVMOD_OTG,
    #[doc = "Force USB0VBUS and USB0ID low"]
    USB_GPCS_DEVMOD_HOST,
    #[doc = "Force USB0VBUS and USB0ID high"]
    USB_GPCS_DEVMOD_DEV,
    #[doc = "Use USB0VBUS and force USB0ID low"]
    USB_GPCS_DEVMOD_HOSTVBUS,
    #[doc = "Use USB0VBUS and force USB0ID high"]
    USB_GPCS_DEVMOD_DEVVBUS,
}
impl USB_GPCS_DEVMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            USB_GPCS_DEVMODW::USB_GPCS_DEVMOD_OTG => 0,
            USB_GPCS_DEVMODW::USB_GPCS_DEVMOD_HOST => 2,
            USB_GPCS_DEVMODW::USB_GPCS_DEVMOD_DEV => 3,
            USB_GPCS_DEVMODW::USB_GPCS_DEVMOD_HOSTVBUS => 4,
            USB_GPCS_DEVMODW::USB_GPCS_DEVMOD_DEVVBUS => 5,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USB_GPCS_DEVMODW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_GPCS_DEVMODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_GPCS_DEVMODW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Use USB0VBUS and USB0ID pin"]
    #[inline(always)]
    pub fn usb_gpcs_devmod_otg(self) -> &'a mut W {
        self.variant(USB_GPCS_DEVMODW::USB_GPCS_DEVMOD_OTG)
    }
    #[doc = "Force USB0VBUS and USB0ID low"]
    #[inline(always)]
    pub fn usb_gpcs_devmod_host(self) -> &'a mut W {
        self.variant(USB_GPCS_DEVMODW::USB_GPCS_DEVMOD_HOST)
    }
    #[doc = "Force USB0VBUS and USB0ID high"]
    #[inline(always)]
    pub fn usb_gpcs_devmod_dev(self) -> &'a mut W {
        self.variant(USB_GPCS_DEVMODW::USB_GPCS_DEVMOD_DEV)
    }
    #[doc = "Use USB0VBUS and force USB0ID low"]
    #[inline(always)]
    pub fn usb_gpcs_devmod_hostvbus(self) -> &'a mut W {
        self.variant(USB_GPCS_DEVMODW::USB_GPCS_DEVMOD_HOSTVBUS)
    }
    #[doc = "Use USB0VBUS and force USB0ID high"]
    #[inline(always)]
    pub fn usb_gpcs_devmod_devvbus(self) -> &'a mut W {
        self.variant(USB_GPCS_DEVMODW::USB_GPCS_DEVMOD_DEVVBUS)
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
    #[doc = "Bits 0:2 - Device Mode"]
    #[inline(always)]
    pub fn usb_gpcs_devmod(&self) -> USB_GPCS_DEVMODR {
        USB_GPCS_DEVMODR::_from(((self.bits >> 0) & 7) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Device Mode"]
    #[inline(always)]
    pub fn usb_gpcs_devmod(&mut self) -> _USB_GPCS_DEVMODW {
        _USB_GPCS_DEVMODW { w: self }
    }
}
