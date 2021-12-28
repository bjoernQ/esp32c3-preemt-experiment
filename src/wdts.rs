pub fn disable_wdts() {
    unsafe {
        // super wdt
        core::ptr::write_volatile(0x600080B0 as *mut _, 0x8F1D312Au32); // disable write protect
        core::ptr::write_volatile(
            0x600080AC as *mut _,
            core::ptr::read_volatile(0x600080AC as *const u32) | 1 << 31,
        ); // set RTC_CNTL_SWD_AUTO_FEED_EN
        core::ptr::write_volatile(0x600080B0 as *mut _, 0u32); // enable write protect

        // tg0 wdg
        core::ptr::write_volatile(0x6001f064 as *mut _, 0x50D83AA1u32); // disable write protect
        core::ptr::write_volatile(0x6001F048 as *mut _, 0u32);
        core::ptr::write_volatile(0x6001f064 as *mut _, 0u32); // enable write protect

        // tg1 wdg
        core::ptr::write_volatile(0x60020064 as *mut _, 0x50D83AA1u32); // disable write protect
        core::ptr::write_volatile(0x60020048 as *mut _, 0u32);
        core::ptr::write_volatile(0x60020064 as *mut _, 0u32); // enable write protect

        // rtc wdg
        core::ptr::write_volatile(0x600080a8 as *mut _, 0x50D83AA1u32); // disable write protect
        core::ptr::write_volatile(0x60008090 as *mut _, 0u32);
        core::ptr::write_volatile(0x600080a8 as *mut _, 0u32); // enable write protect
    }
}
