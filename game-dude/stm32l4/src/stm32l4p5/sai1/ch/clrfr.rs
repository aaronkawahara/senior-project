# [ doc = "Register `CLRFR` reader" ] pub struct R ( crate :: R < CLRFR_SPEC > ) ; impl core :: ops :: Deref for R { type Target = crate :: R < CLRFR_SPEC > ; # [ inline ( always ) ] fn deref ( & self ) -> & Self :: Target { & self . 0 } } impl core :: convert :: From < crate :: R < CLRFR_SPEC > > for R { fn from ( reader : crate :: R < CLRFR_SPEC > ) -> Self { R ( reader ) } } # [ doc = "Register `CLRFR` writer" ] pub struct W ( crate :: W < CLRFR_SPEC > ) ; impl core :: ops :: Deref for W { type Target = crate :: W < CLRFR_SPEC > ; # [ inline ( always ) ] fn deref ( & self ) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [ inline ( always ) ] fn deref_mut ( & mut self ) -> & mut Self :: Target { & mut self . 0 } } impl core :: convert :: From < crate :: W < CLRFR_SPEC > > for W { fn from ( writer : crate :: W < CLRFR_SPEC > ) -> Self { W ( writer ) } } # [ doc = "Field `CLFSDET` reader - Clear late frame synchronization detection flag" ] pub struct CLFSDET_R ( crate :: FieldReader < bool , bool > ) ; impl CLFSDET_R { pub ( crate ) fn new ( bits : bool ) -> Self { CLFSDET_R ( crate :: FieldReader :: new ( bits ) ) } } impl core :: ops :: Deref for CLFSDET_R { type Target = crate :: FieldReader < bool , bool > ; # [ inline ( always ) ] fn deref ( & self ) -> & Self :: Target { & self . 0 } } # [ doc = "Field `CLFSDET` writer - Clear late frame synchronization detection flag" ] pub struct CLFSDET_W < 'a > { w : & 'a mut W , } impl < 'a > CLFSDET_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 6 ) ) | ( ( value as u32 & 0x01 ) << 6 ) ; self . w } } # [ doc = "Field `CAFSDET` reader - Clear anticipated frame synchronization detection flag" ] pub struct CAFSDET_R ( crate :: FieldReader < bool , bool > ) ; impl CAFSDET_R { pub ( crate ) fn new ( bits : bool ) -> Self { CAFSDET_R ( crate :: FieldReader :: new ( bits ) ) } } impl core :: ops :: Deref for CAFSDET_R { type Target = crate :: FieldReader < bool , bool > ; # [ inline ( always ) ] fn deref ( & self ) -> & Self :: Target { & self . 0 } } # [ doc = "Field `CAFSDET` writer - Clear anticipated frame synchronization detection flag" ] pub struct CAFSDET_W < 'a > { w : & 'a mut W , } impl < 'a > CAFSDET_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 5 ) ) | ( ( value as u32 & 0x01 ) << 5 ) ; self . w } } # [ doc = "Field `CCNRDY` reader - Clear codec not ready flag" ] pub struct CCNRDY_R ( crate :: FieldReader < bool , bool > ) ; impl CCNRDY_R { pub ( crate ) fn new ( bits : bool ) -> Self { CCNRDY_R ( crate :: FieldReader :: new ( bits ) ) } } impl core :: ops :: Deref for CCNRDY_R { type Target = crate :: FieldReader < bool , bool > ; # [ inline ( always ) ] fn deref ( & self ) -> & Self :: Target { & self . 0 } } # [ doc = "Field `CCNRDY` writer - Clear codec not ready flag" ] pub struct CCNRDY_W < 'a > { w : & 'a mut W , } impl < 'a > CCNRDY_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 4 ) ) | ( ( value as u32 & 0x01 ) << 4 ) ; self . w } } # [ doc = "Field `CWCKCFG` reader - Clear wrong clock configuration flag" ] pub struct CWCKCFG_R ( crate :: FieldReader < bool , bool > ) ; impl CWCKCFG_R { pub ( crate ) fn new ( bits : bool ) -> Self { CWCKCFG_R ( crate :: FieldReader :: new ( bits ) ) } } impl core :: ops :: Deref for CWCKCFG_R { type Target = crate :: FieldReader < bool , bool > ; # [ inline ( always ) ] fn deref ( & self ) -> & Self :: Target { & self . 0 } } # [ doc = "Field `CWCKCFG` writer - Clear wrong clock configuration flag" ] pub struct CWCKCFG_W < 'a > { w : & 'a mut W , } impl < 'a > CWCKCFG_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 2 ) ) | ( ( value as u32 & 0x01 ) << 2 ) ; self . w } } # [ doc = "Field `CMUTEDET` reader - Mute detection flag" ] pub struct CMUTEDET_R ( crate :: FieldReader < bool , bool > ) ; impl CMUTEDET_R { pub ( crate ) fn new ( bits : bool ) -> Self { CMUTEDET_R ( crate :: FieldReader :: new ( bits ) ) } } impl core :: ops :: Deref for CMUTEDET_R { type Target = crate :: FieldReader < bool , bool > ; # [ inline ( always ) ] fn deref ( & self ) -> & Self :: Target { & self . 0 } } # [ doc = "Field `CMUTEDET` writer - Mute detection flag" ] pub struct CMUTEDET_W < 'a > { w : & 'a mut W , } impl < 'a > CMUTEDET_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! ( 0x01 << 1 ) ) | ( ( value as u32 & 0x01 ) << 1 ) ; self . w } } # [ doc = "Field `COVRUDR` reader - Clear overrun / underrun" ] pub struct COVRUDR_R ( crate :: FieldReader < bool , bool > ) ; impl COVRUDR_R { pub ( crate ) fn new ( bits : bool ) -> Self { COVRUDR_R ( crate :: FieldReader :: new ( bits ) ) } } impl core :: ops :: Deref for COVRUDR_R { type Target = crate :: FieldReader < bool , bool > ; # [ inline ( always ) ] fn deref ( & self ) -> & Self :: Target { & self . 0 } } # [ doc = "Field `COVRUDR` writer - Clear overrun / underrun" ] pub struct COVRUDR_W < 'a > { w : & 'a mut W , } impl < 'a > COVRUDR_W < 'a > { # [ doc = r"Sets the field bit" ] # [ inline ( always ) ] pub fn set_bit ( self ) -> & 'a mut W { self . bit ( true ) } # [ doc = r"Clears the field bit" ] # [ inline ( always ) ] pub fn clear_bit ( self ) -> & 'a mut W { self . bit ( false ) } # [ doc = r"Writes raw bits to the field" ] # [ inline ( always ) ] pub fn bit ( self , value : bool ) -> & 'a mut W { self . w . bits = ( self . w . bits & ! 0x01 ) | ( value as u32 & 0x01 ) ; self . w } } impl R { # [ doc = "Bit 6 - Clear late frame synchronization detection flag" ] # [ inline ( always ) ] pub fn clfsdet ( & self ) -> CLFSDET_R { CLFSDET_R :: new ( ( ( self . bits >> 6 ) & 0x01 ) != 0 ) } # [ doc = "Bit 5 - Clear anticipated frame synchronization detection flag" ] # [ inline ( always ) ] pub fn cafsdet ( & self ) -> CAFSDET_R { CAFSDET_R :: new ( ( ( self . bits >> 5 ) & 0x01 ) != 0 ) } # [ doc = "Bit 4 - Clear codec not ready flag" ] # [ inline ( always ) ] pub fn ccnrdy ( & self ) -> CCNRDY_R { CCNRDY_R :: new ( ( ( self . bits >> 4 ) & 0x01 ) != 0 ) } # [ doc = "Bit 2 - Clear wrong clock configuration flag" ] # [ inline ( always ) ] pub fn cwckcfg ( & self ) -> CWCKCFG_R { CWCKCFG_R :: new ( ( ( self . bits >> 2 ) & 0x01 ) != 0 ) } # [ doc = "Bit 1 - Mute detection flag" ] # [ inline ( always ) ] pub fn cmutedet ( & self ) -> CMUTEDET_R { CMUTEDET_R :: new ( ( ( self . bits >> 1 ) & 0x01 ) != 0 ) } # [ doc = "Bit 0 - Clear overrun / underrun" ] # [ inline ( always ) ] pub fn covrudr ( & self ) -> COVRUDR_R { COVRUDR_R :: new ( ( self . bits & 0x01 ) != 0 ) } } impl W { # [ doc = "Bit 6 - Clear late frame synchronization detection flag" ] # [ inline ( always ) ] pub fn clfsdet ( & mut self ) -> CLFSDET_W { CLFSDET_W { w : self } } # [ doc = "Bit 5 - Clear anticipated frame synchronization detection flag" ] # [ inline ( always ) ] pub fn cafsdet ( & mut self ) -> CAFSDET_W { CAFSDET_W { w : self } } # [ doc = "Bit 4 - Clear codec not ready flag" ] # [ inline ( always ) ] pub fn ccnrdy ( & mut self ) -> CCNRDY_W { CCNRDY_W { w : self } } # [ doc = "Bit 2 - Clear wrong clock configuration flag" ] # [ inline ( always ) ] pub fn cwckcfg ( & mut self ) -> CWCKCFG_W { CWCKCFG_W { w : self } } # [ doc = "Bit 1 - Mute detection flag" ] # [ inline ( always ) ] pub fn cmutedet ( & mut self ) -> CMUTEDET_W { CMUTEDET_W { w : self } } # [ doc = "Bit 0 - Clear overrun / underrun" ] # [ inline ( always ) ] pub fn covrudr ( & mut self ) -> COVRUDR_W { COVRUDR_W { w : self } } # [ doc = "Writes raw bits to the register." ] pub unsafe fn bits ( & mut self , bits : u32 ) -> & mut Self { self . 0 . bits ( bits ) ; self } } # [ doc = "AClear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrfr](index.html) module" ] pub struct CLRFR_SPEC ; impl crate :: RegisterSpec for CLRFR_SPEC { type Ux = u32 ; } # [ doc = "`read()` method returns [clrfr::R](R) reader structure" ] impl crate :: Readable for CLRFR_SPEC { type Reader = R ; } # [ doc = "`write(|w| ..)` method takes [clrfr::W](W) writer structure" ] impl crate :: Writable for CLRFR_SPEC { type Writer = W ; } # [ doc = "`reset()` method sets CLRFR to value 0" ] impl crate :: Resettable for CLRFR_SPEC { # [ inline ( always ) ] fn reset_value ( ) -> Self :: Ux { 0 } }