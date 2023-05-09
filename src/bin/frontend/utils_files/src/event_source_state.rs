use gloo_events::EventListener;
use js_sys::Function;
use web_sys::Event;
use web_sys::EventSource;


#[allow(dead_code)]
pub struct EventSourceState {
    event_source: EventSource,
    event_listener: Vec<EventListener>,
}

impl EventSourceState {
    pub fn new<U, E>(
        url: &str,
        js_function: Option<(String, String)>,
        callback_update: U,
        callback_end: E,
    ) -> Self
    where
        U: FnMut(&Event) + 'static,
        E: FnMut(&Event) + 'static,
    {
        let event_source: EventSource = EventSource::new(url).unwrap();
        if let Some(js_function_unwrapped) = js_function {
            event_source.set_onmessage(Some(&Function::new_with_args(
                &js_function_unwrapped.0,
                &js_function_unwrapped.1,
            )));
        } else {
            event_source.set_onopen(Some(&Function::new_no_args(
                "console.log('Event Source Initialized');",
            )));
            event_source.set_onmessage(Some(&Function::new_no_args(
                "console.log('Received Links Request');",
            )));
            event_source.set_onerror(Some(&Function::new_no_args(
                "console.log('Event Source Error');",
            )));
        }
        let event_listener: Vec<EventListener> = vec![
            EventListener::new(&event_source, "message", callback_update),
            EventListener::new(&event_source, "error", callback_end),
        ];
        Self {
            event_source,
            event_listener,
        }
    }

    pub fn close_connection(&mut self) {
        self.event_source.close();
        for listener in self.event_listener.iter_mut() {
            drop(listener);
        }
    }
}
