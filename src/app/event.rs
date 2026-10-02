pub(super) struct EventPump {
    events: sdl3::EventPump,
}

impl EventPump {
    pub(super) fn init(sdl: &sdl3::Sdl) -> Result<Self, sdl3::Error> {
        let events = sdl.event_pump()?;
        Ok(Self { events })
    }

    pub(super) fn iter(&mut self) -> sdl3::event::EventPollIterator<'_> {
        self.events.poll_iter()
    }
}
