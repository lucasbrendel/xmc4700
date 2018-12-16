#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RXICMP_ERROR_OCTETS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RXICMPERROCTR {
    bits: u32,
}
impl RXICMPERROCTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Number of bytes received in an ICMP segment with checksum errors"]
    #[inline]
    pub fn rxicmperroct(&self) -> RXICMPERROCTR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RXICMPERROCTR { bits }
    }
}