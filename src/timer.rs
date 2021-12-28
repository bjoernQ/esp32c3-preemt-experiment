use void::Void;

extern "C" {
    // ROM functions, see esp32c3-link.x
    pub fn ets_delay_us(us: u32);
}

pub struct EtsTimer {
    delay: u32,
}

impl EtsTimer {
    pub fn new(delay_us: u32) -> Self {
        Self { delay: delay_us }
    }
}

impl embedded_hal::timer::CountDown for EtsTimer {
    type Time = u32;

    fn start<T>(&mut self, count: T)
    where
        T: Into<Self::Time>,
    {
        self.delay = count.into();
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        Ok(unsafe { ets_delay_us(self.delay) })
    }
}

impl embedded_hal::timer::Periodic for EtsTimer {}
