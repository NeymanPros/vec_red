use iced::{keyboard, Subscription, Vector};
use iced::keyboard::Key;
use iced::keyboard::key::Named;
use crate::{Message, VecRed};

impl VecRed<'_> {
    pub(crate) fn subscription(&self) -> Subscription<Message> {
        let keyboard_events = keyboard::on_key_press(|a, b| {
            Self::shortcuts(a, b)
        });
        let resize_event = iced::window::resize_events().map(|(_id, size)|{
            Message::WindowResized(size)
        });
        Subscription::batch(vec![keyboard_events, resize_event])
    }

    fn shortcuts (key: Key, modifiers: keyboard::Modifiers) -> Option<Message> {
        if modifiers.is_empty() {
            return match key {
                Key::Named(Named::Delete) => {
                    Some(Message::DeletePoint)
                }
                Key::Named(Named::ArrowLeft) => {
                    Some(Message::ZoomShift(Vector::new(-100.0, 0.0)))
                }
                Key::Named(Named::ArrowRight) => {
                    Some(Message::ZoomShift(Vector::new(100.0, 0.0)))
                }
                Key::Named(Named::ArrowUp) => {
                    Some(Message::ZoomShift(Vector::new(0.0, -100.0)))
                }
                Key::Named(Named::ArrowDown) => {
                    Some(Message::ZoomShift(Vector::new(0.0, 100.0)))
                }
                _ => { None }
            }
        }
        match modifiers {
            keyboard::Modifiers::SHIFT => {
                match key {
                    Key::Named(Named::ArrowLeft) => {
                        Some(Message::ZoomShift(Vector::new(-10.0, 0.0)))
                    }
                    Key::Named(Named::ArrowRight) => {
                        Some(Message::ZoomShift(Vector::new(10.0, 0.0)))
                    }
                    Key::Named(Named::ArrowUp) => {
                        Some(Message::ZoomShift(Vector::new(0.0, -10.0)))
                    }
                    Key::Named(Named::ArrowDown) => {
                        Some(Message::ZoomShift(Vector::new(0.0, 10.0)))
                    }
                    _ => { None }
                }
            }
            keyboard::Modifiers::CTRL => {
                match key.as_ref() {
                    Key::Character("z") => {
                        Some(Message::Undo)
                    }
                    Key::Character("=") => {
                        Some(Message::ZoomScale(1.1))
                    }
                    Key::Character("-") => {
                        Some(Message::ZoomScale(0.9))
                    }
                    _ => { None }
                }
            }
            _ => { None }
        }
    }
}
