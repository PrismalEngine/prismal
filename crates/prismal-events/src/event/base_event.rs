use thiserror::Error;

use crate::event::keyboard_event_data::KeyboardEventData;
use crate::event::mouse_button_event_data::MouseButtonEventData;
use crate::event::mouse_motion_event_data::MouseMotionEventData;
use crate::event::mouse_wheel_event_data::MouseWheelEventData;
use crate::event::window_focus_event_data::WindowFocusEventData;
use crate::event::window_move_event_data::WindowMoveEventData;

pub use prismal_window::prelude::WinitDeviceEvent;
pub use prismal_window::prelude::WinitEvent;
pub use prismal_window::prelude::WinitWindowEvent;

/// An even that occurred in a Prismal Engine application
#[derive(Debug, Clone)]
pub enum Event {
    /// An event from the keyboard has been received.
    Keyboard(KeyboardEventData),

    /// A mouse button has been pressed or released.
    MouseButton(MouseButtonEventData),

    /// Change in physical position of a pointing device.
    MouseMotion(MouseMotionEventData),

    /// A mouse wheel movement or touchpad scroll occurred.
    MouseWheel(MouseWheelEventData),

    /// The window gained or lost focus.
    WindowFocus(WindowFocusEventData),

    /// The position of the window has changed.
    WindowMove(WindowMoveEventData),
}

impl Event {
    pub fn try_from_winit<'a>(event: &WinitEvent<'a, ()>) -> Result<Self, TryEventFromWinitError> {
        match event {
            WinitEvent::WindowEvent { event, .. } => match event {
                WinitWindowEvent::Moved(position) => {
                    Ok(Event::WindowMove(WindowMoveEventData::new(*position)))
                }
                WinitWindowEvent::Focused(gained) => {
                    Ok(Event::WindowFocus(WindowFocusEventData::new(*gained)))
                }
                WinitWindowEvent::KeyboardInput { input, .. } => {
                    Ok(Event::Keyboard(KeyboardEventData::new(*input)))
                }
                WinitWindowEvent::MouseInput { state, button, .. } => Ok(Event::MouseButton(
                    MouseButtonEventData::new(*state, *button),
                )),
                WinitWindowEvent::MouseWheel { delta, .. } => {
                    Ok(Event::MouseWheel(MouseWheelEventData::new(*delta)))
                }
                _ => Err(TryEventFromWinitError::InvalidSourceEvent),
            },
            WinitEvent::DeviceEvent { event, .. } => {
                if let WinitDeviceEvent::MouseMotion { delta } = event {
                    Ok(Event::MouseMotion(MouseMotionEventData::new(*delta)))
                } else {
                    Err(TryEventFromWinitError::InvalidSourceEvent)
                }
            }
            _ => Err(TryEventFromWinitError::InvalidSourceEvent),
        }
    }

    pub fn as_keyboard(&self) -> Option<&KeyboardEventData> {
        if let Self::Keyboard(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_mouse_button(&self) -> Option<&MouseButtonEventData> {
        if let Self::MouseButton(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_mouse_motion(&self) -> Option<&MouseMotionEventData> {
        if let Self::MouseMotion(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_mouse_wheel(&self) -> Option<&MouseWheelEventData> {
        if let Self::MouseWheel(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_window_focus(&self) -> Option<&WindowFocusEventData> {
        if let Self::WindowFocus(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_window_move(&self) -> Option<&WindowMoveEventData> {
        if let Self::WindowMove(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[derive(Debug, Error)]
pub enum TryEventFromWinitError {
    #[error("Unsupported winit source event")]
    InvalidSourceEvent,
}
