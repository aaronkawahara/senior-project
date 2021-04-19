#[doc = "Register `STAR` reader"]
pub struct R(crate::R<STAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<STAR_SPEC>> for R {
    fn from(reader: crate::R<STAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDMABTC` reader - IDMA buffer transfer complete"]
pub struct IDMABTC_R(crate::FieldReader<bool, bool>);
impl IDMABTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDMABTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDMABTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDMATE` reader - IDMA transfer error"]
pub struct IDMATE_R(crate::FieldReader<bool, bool>);
impl IDMATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDMATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDMATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKSTOP` reader - SDMMC_CK stopped in Voltage switch procedure"]
pub struct CKSTOP_R(crate::FieldReader<bool, bool>);
impl CKSTOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKSTOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKSTOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSWEND` reader - Voltage switch critical timing section completion"]
pub struct VSWEND_R(crate::FieldReader<bool, bool>);
impl VSWEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        VSWEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSWEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKTIMEOUT` reader - Boot acknowledgment timeout"]
pub struct ACKTIMEOUT_R(crate::FieldReader<bool, bool>);
impl ACKTIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKTIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACKTIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKFAIL` reader - Boot acknowledgment received (boot acknowledgment check fail)"]
pub struct ACKFAIL_R(crate::FieldReader<bool, bool>);
impl ACKFAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKFAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACKFAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIOIT` reader - SDIO interrupt received"]
pub struct SDIOIT_R(crate::FieldReader<bool, bool>);
impl SDIOIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIOIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIOIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSYD0END` reader - end of SDMMC_D0 Busy following a CMD response detected"]
pub struct BUSYD0END_R(crate::FieldReader<bool, bool>);
impl BUSYD0END_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSYD0END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSYD0END_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSYD0` reader - Inverted value of SDMMC_D0 line (Busy), sampled at the end of a CMD response and a second time 2 SDMMC_CK cycles after the CMD response"]
pub struct BUSYD0_R(crate::FieldReader<bool, bool>);
impl BUSYD0_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSYD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSYD0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFOE` reader - Receive FIFO empty"]
pub struct RXFIFOE_R(crate::FieldReader<bool, bool>);
impl RXFIFOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFOE` reader - Transmit FIFO empty"]
pub struct TXFIFOE_R(crate::FieldReader<bool, bool>);
impl TXFIFOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFOF` reader - Receive FIFO full"]
pub struct RXFIFOF_R(crate::FieldReader<bool, bool>);
impl RXFIFOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFOF` reader - Transmit FIFO full"]
pub struct TXFIFOF_R(crate::FieldReader<bool, bool>);
impl TXFIFOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFOHF` reader - Receive FIFO half full: there are at least 8 words in the FIFO"]
pub struct RXFIFOHF_R(crate::FieldReader<bool, bool>);
impl RXFIFOHF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFOHF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFOHF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFOHE` reader - Transmit FIFO half empty: at least 8 words can be written into the FIFO"]
pub struct TXFIFOHE_R(crate::FieldReader<bool, bool>);
impl TXFIFOHE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFOHE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFOHE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPSMACT` reader - Command path state machine active, i.e. not in Idle state"]
pub struct CPSMACT_R(crate::FieldReader<bool, bool>);
impl CPSMACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPSMACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPSMACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPSMACT` reader - Data path state machine active, i.e. not in Idle state"]
pub struct DPSMACT_R(crate::FieldReader<bool, bool>);
impl DPSMACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPSMACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPSMACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DABORT` reader - Data transfer aborted by CMD12"]
pub struct DABORT_R(crate::FieldReader<bool, bool>);
impl DABORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DABORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DABORT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBCKEND` reader - Data block sent/received"]
pub struct DBCKEND_R(crate::FieldReader<bool, bool>);
impl DBCKEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBCKEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBCKEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DHOLD` reader - Data transfer Hold"]
pub struct DHOLD_R(crate::FieldReader<bool, bool>);
impl DHOLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DHOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAEND` reader - Data end (data counter, SDIDCOUNT, is zero)"]
pub struct DATAEND_R(crate::FieldReader<bool, bool>);
impl DATAEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATAEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDSENT` reader - Command sent (no response required)"]
pub struct CMDSENT_R(crate::FieldReader<bool, bool>);
impl CMDSENT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDSENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDSENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDREND` reader - Command response received (CRC check passed)"]
pub struct CMDREND_R(crate::FieldReader<bool, bool>);
impl CMDREND_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDREND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDREND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVERR` reader - Received FIFO overrun error"]
pub struct RXOVERR_R(crate::FieldReader<bool, bool>);
impl RXOVERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUNDERR` reader - Transmit FIFO underrun error"]
pub struct TXUNDERR_R(crate::FieldReader<bool, bool>);
impl TXUNDERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUNDERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUNDERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTIMEOUT` reader - Data timeout"]
pub struct DTIMEOUT_R(crate::FieldReader<bool, bool>);
impl DTIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMEOUT` reader - Command response timeout"]
pub struct CTIMEOUT_R(crate::FieldReader<bool, bool>);
impl CTIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCRCFAIL` reader - Data block sent/received (CRC check failed)"]
pub struct DCRCFAIL_R(crate::FieldReader<bool, bool>);
impl DCRCFAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCRCFAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCRCFAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCRCFAIL` reader - Command response received (CRC check failed)"]
pub struct CCRCFAIL_R(crate::FieldReader<bool, bool>);
impl CCRCFAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCRCFAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCRCFAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 28 - IDMA buffer transfer complete"]
    #[inline(always)]
    pub fn idmabtc(&self) -> IDMABTC_R {
        IDMABTC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - IDMA transfer error"]
    #[inline(always)]
    pub fn idmate(&self) -> IDMATE_R {
        IDMATE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - SDMMC_CK stopped in Voltage switch procedure"]
    #[inline(always)]
    pub fn ckstop(&self) -> CKSTOP_R {
        CKSTOP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Voltage switch critical timing section completion"]
    #[inline(always)]
    pub fn vswend(&self) -> VSWEND_R {
        VSWEND_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Boot acknowledgment timeout"]
    #[inline(always)]
    pub fn acktimeout(&self) -> ACKTIMEOUT_R {
        ACKTIMEOUT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Boot acknowledgment received (boot acknowledgment check fail)"]
    #[inline(always)]
    pub fn ackfail(&self) -> ACKFAIL_R {
        ACKFAIL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SDIO interrupt received"]
    #[inline(always)]
    pub fn sdioit(&self) -> SDIOIT_R {
        SDIOIT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - end of SDMMC_D0 Busy following a CMD response detected"]
    #[inline(always)]
    pub fn busyd0end(&self) -> BUSYD0END_R {
        BUSYD0END_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Inverted value of SDMMC_D0 line (Busy), sampled at the end of a CMD response and a second time 2 SDMMC_CK cycles after the CMD response"]
    #[inline(always)]
    pub fn busyd0(&self) -> BUSYD0_R {
        BUSYD0_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO empty"]
    #[inline(always)]
    pub fn rxfifoe(&self) -> RXFIFOE_R {
        RXFIFOE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn txfifoe(&self) -> TXFIFOE_R {
        TXFIFOE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Receive FIFO full"]
    #[inline(always)]
    pub fn rxfifof(&self) -> RXFIFOF_R {
        RXFIFOF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transmit FIFO full"]
    #[inline(always)]
    pub fn txfifof(&self) -> TXFIFOF_R {
        TXFIFOF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO half full: there are at least 8 words in the FIFO"]
    #[inline(always)]
    pub fn rxfifohf(&self) -> RXFIFOHF_R {
        RXFIFOHF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Transmit FIFO half empty: at least 8 words can be written into the FIFO"]
    #[inline(always)]
    pub fn txfifohe(&self) -> TXFIFOHE_R {
        TXFIFOHE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Command path state machine active, i.e. not in Idle state"]
    #[inline(always)]
    pub fn cpsmact(&self) -> CPSMACT_R {
        CPSMACT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Data path state machine active, i.e. not in Idle state"]
    #[inline(always)]
    pub fn dpsmact(&self) -> DPSMACT_R {
        DPSMACT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Data transfer aborted by CMD12"]
    #[inline(always)]
    pub fn dabort(&self) -> DABORT_R {
        DABORT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data block sent/received"]
    #[inline(always)]
    pub fn dbckend(&self) -> DBCKEND_R {
        DBCKEND_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Data transfer Hold"]
    #[inline(always)]
    pub fn dhold(&self) -> DHOLD_R {
        DHOLD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Data end (data counter, SDIDCOUNT, is zero)"]
    #[inline(always)]
    pub fn dataend(&self) -> DATAEND_R {
        DATAEND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Command sent (no response required)"]
    #[inline(always)]
    pub fn cmdsent(&self) -> CMDSENT_R {
        CMDSENT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Command response received (CRC check passed)"]
    #[inline(always)]
    pub fn cmdrend(&self) -> CMDREND_R {
        CMDREND_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Received FIFO overrun error"]
    #[inline(always)]
    pub fn rxoverr(&self) -> RXOVERR_R {
        RXOVERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO underrun error"]
    #[inline(always)]
    pub fn txunderr(&self) -> TXUNDERR_R {
        TXUNDERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data timeout"]
    #[inline(always)]
    pub fn dtimeout(&self) -> DTIMEOUT_R {
        DTIMEOUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command response timeout"]
    #[inline(always)]
    pub fn ctimeout(&self) -> CTIMEOUT_R {
        CTIMEOUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data block sent/received (CRC check failed)"]
    #[inline(always)]
    pub fn dcrcfail(&self) -> DCRCFAIL_R {
        DCRCFAIL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Command response received (CRC check failed)"]
    #[inline(always)]
    pub fn ccrcfail(&self) -> CCRCFAIL_R {
        CCRCFAIL_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [star](index.html) module"]
pub struct STAR_SPEC;
impl crate::RegisterSpec for STAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [star::R](R) reader structure"]
impl crate::Readable for STAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAR to value 0"]
impl crate::Resettable for STAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
