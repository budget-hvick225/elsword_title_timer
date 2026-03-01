use rdev::Key;
use strum_macros::{EnumIter, EnumString, Display};
use std::str::FromStr;
use eframe::egui;
use eframe::egui::Key as EguiKey;

#[derive(Debug, Clone, Copy, EnumIter, EnumString, PartialEq, Eq, Hash, Display)]
pub enum MyKey {
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    Alt,
    AltGr,
    MetaLeft,
    MetaRight,
    Escape,
    Tab,
    CapsLock,
    Return,
    Backspace,
    Space,
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
    // Numbers
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    // Letters
    KeyA, KeyB, KeyC, KeyD, KeyE, KeyF, KeyG, KeyH, KeyI, KeyJ, KeyK,
    KeyL, KeyM, KeyN, KeyO, KeyP, KeyQ, KeyR, KeyS, KeyT, KeyU, KeyV,
    KeyW, KeyX, KeyY, KeyZ,
    // Misc
    Minus, Equal, LeftBracket, RightBracket, BackSlash, Semicolon, Quote, Comma, Dot, Slash,
    Grave,
    End,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
}

impl MyKey {
    /// Convert MyKey -> rdev::Key
    pub fn parse_key(s: &str) -> Option<MyKey> {
        MyKey::from_str(s).ok()
    }
    pub fn to_rdev(self) -> Key {
        use MyKey::*;
        match self {
            ShiftLeft => Key::ShiftLeft,
            ShiftRight => Key::ShiftRight,
            ControlLeft => Key::ControlLeft,
            ControlRight => Key::ControlRight,
            Alt => Key::Alt,
            AltGr => Key::AltGr,
            MetaLeft => Key::MetaLeft,
            MetaRight => Key::MetaRight,
            Escape => Key::Escape,
            Tab => Key::Tab,
            CapsLock => Key::CapsLock,
            Return => Key::Return,
            Backspace => Key::Backspace,
            Space => Key::Space,
            LeftArrow => Key::LeftArrow,
            RightArrow => Key::RightArrow,
            UpArrow => Key::UpArrow,
            DownArrow => Key::DownArrow,
            Num0 => Key::Num0,
            Num1 => Key::Num1,
            Num2 => Key::Num2,
            Num3 => Key::Num3,
            Num4 => Key::Num4,
            Num5 => Key::Num5,
            Num6 => Key::Num6,
            Num7 => Key::Num7,
            Num8 => Key::Num8,
            Num9 => Key::Num9,
            KeyA => Key::KeyA,
            KeyB => Key::KeyB,
            KeyC => Key::KeyC,
            KeyD => Key::KeyD,
            KeyE => Key::KeyE,
            KeyF => Key::KeyF,
            KeyG => Key::KeyG,
            KeyH => Key::KeyH,
            KeyI => Key::KeyI,
            KeyJ => Key::KeyJ,
            KeyK => Key::KeyK,
            KeyL => Key::KeyL,
            KeyM => Key::KeyM,
            KeyN => Key::KeyN,
            KeyO => Key::KeyO,
            KeyP => Key::KeyP,
            KeyQ => Key::KeyQ,
            KeyR => Key::KeyR,
            KeyS => Key::KeyS,
            KeyT => Key::KeyT,
            KeyU => Key::KeyU,
            KeyV => Key::KeyV,
            KeyW => Key::KeyW,
            KeyX => Key::KeyX,
            KeyY => Key::KeyY,
            KeyZ => Key::KeyZ,
            Minus => Key::Minus,
            Equal => Key::Equal,
            LeftBracket => Key::LeftBracket,
            RightBracket => Key::RightBracket,
            BackSlash => Key::BackSlash,
            Semicolon => Key::SemiColon,
            Quote => Key::Quote,
            Comma => Key::Comma,
            Dot => Key::Dot,
            Slash => Key::Slash,
            End => Key::End,
            F1 => Key::F1,
            F2 => Key::F2,
            F3 => Key::F3,
            F4 => Key::F4,
            F5 => Key::F5,
            F6 => Key::F6,
            F7 => Key::F7,
            F8 => Key::F8,
            F9 => Key::F9,
            F10 => Key::F10,
            F11 => Key::F11,
            F12 => Key::F12,
            Grave => Key::BackQuote,
        }
    }
    // once those work, I can use them for my to_rdev() func. therefore achieving egui to rdev functionality.
    pub fn egui_to_mykey(arg: &EguiKey) -> Option<MyKey> {
        use MyKey::*;
        match arg {
            //Right now lacking alt Left/right shift, left/right control, left/right meta. will add a distinguish method for that.

            // Arrows
            EguiKey::ArrowUp => Some(UpArrow),
            EguiKey::ArrowDown => Some(DownArrow),
            EguiKey::ArrowLeft => Some(LeftArrow),
            EguiKey::ArrowRight => Some(RightArrow),

            // Basic control keys
            EguiKey::Escape => Some(Escape),
            EguiKey::Tab => Some(Tab),
            EguiKey::Backspace => Some(Backspace),
            EguiKey::Enter => Some(Return),
            EguiKey::Space => Some(Space),
            EguiKey::End => Some(End),

            // Letters
            EguiKey::A => Some(KeyA),
            EguiKey::B => Some(KeyB),
            EguiKey::C => Some(KeyC),
            EguiKey::D => Some(KeyD),
            EguiKey::E => Some(KeyE),
            EguiKey::F => Some(KeyF),
            EguiKey::G => Some(KeyG),
            EguiKey::H => Some(KeyH),
            EguiKey::I => Some(KeyI),
            EguiKey::J => Some(KeyJ),
            EguiKey::K => Some(KeyK),
            EguiKey::L => Some(KeyL),
            EguiKey::M => Some(KeyM),
            EguiKey::N => Some(KeyN),
            EguiKey::O => Some(KeyO),
            EguiKey::P => Some(KeyP),
            EguiKey::Q => Some(KeyQ),
            EguiKey::R => Some(KeyR),
            EguiKey::S => Some(KeyS),
            EguiKey::T => Some(KeyT),
            EguiKey::U => Some(KeyU),
            EguiKey::V => Some(KeyV),
            EguiKey::W => Some(KeyW),
            EguiKey::X => Some(KeyX),
            EguiKey::Y => Some(KeyY),
            EguiKey::Z => Some(KeyZ),

            // Numbers
            EguiKey::Num0 => Some(Num0),
            EguiKey::Num1 => Some(Num1),
            EguiKey::Num2 => Some(Num2),
            EguiKey::Num3 => Some(Num3),
            EguiKey::Num4 => Some(Num4),
            EguiKey::Num5 => Some(Num5),
            EguiKey::Num6 => Some(Num6),
            EguiKey::Num7 => Some(Num7),
            EguiKey::Num8 => Some(Num8),
            EguiKey::Num9 => Some(Num9),

            // Function keys
            EguiKey::F1 => Some(F1),
            EguiKey::F2 => Some(F2),
            EguiKey::F3 => Some(F3),
            EguiKey::F4 => Some(F4),
            EguiKey::F5 => Some(F5),
            EguiKey::F6 => Some(F6),
            EguiKey::F7 => Some(F7),
            EguiKey::F8 => Some(F8),
            EguiKey::F9 => Some(F9),
            EguiKey::F10 => Some(F10),
            EguiKey::F11 => Some(F11),
            EguiKey::F12 => Some(F12),

            // Misc symbols
            EguiKey::Minus => Some(Minus),
            EguiKey::Equals => Some(Equal),
            EguiKey::Semicolon => Some(Semicolon),
            EguiKey::Quote => Some(Quote),
            EguiKey::Comma => Some(Comma),
            EguiKey::Period => Some(Dot),
            EguiKey::Slash => Some(Slash),
            EguiKey::Backslash => Some(BackSlash),
            EguiKey::OpenBracket => Some(LeftBracket),
            EguiKey::CloseBracket => Some(RightBracket),
            EguiKey::Backtick => Some(Grave),

            _ => None,
        }
    }
    pub fn modifiers_to_mykey(mods: egui::Modifiers) -> Option<MyKey> {
        None // will do something... 
    }
}