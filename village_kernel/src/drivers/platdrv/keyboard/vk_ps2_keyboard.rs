//###########################################################################
// vk_ps2_keyboard.rs
// The specific implementation of functions related to ps2 keyboard
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_plat_driver;
use crate::traits::vk_callback::Callback;
use crate::traits::vk_driver::{Driver, PlatDevWrapper, PlatDriver};
use crate::traits::vk_event_codes::{EventCode, KeyStatus};
use crate::vendor::ia32legacy::core::i686::*;
use crate::village::kernel;
use alloc::boxed::Box;

// Scan code
const KEY_CODES: [u8; 0x59] = [
    /* 0x00 */ EventCode::KEY_RESERVED,   EventCode::KEY_ESC,        EventCode::KEY_1,           EventCode::KEY_2,
               EventCode::KEY_3,          EventCode::KEY_4,          EventCode::KEY_5,           EventCode::KEY_6,
    /* 0x08 */ EventCode::KEY_7,          EventCode::KEY_8,          EventCode::KEY_9,           EventCode::KEY_0,
               EventCode::KEY_MINUS,      EventCode::KEY_EQUAL,      EventCode::KEY_BACK_SPACE,  EventCode::KEY_TAB,
    /* 0x10 */ EventCode::KEY_Q,          EventCode::KEY_W,          EventCode::KEY_E,           EventCode::KEY_R,
               EventCode::KEY_T,          EventCode::KEY_Y,          EventCode::KEY_U,           EventCode::KEY_I,
    /* 0x18 */ EventCode::KEY_O,          EventCode::KEY_P,          EventCode::KEY_LEFT_BRACE,  EventCode::KEY_RIGHT_BRACE,
               EventCode::KEY_ENTER,      EventCode::KEY_LEFT_CTRL,  EventCode::KEY_A,           EventCode::KEY_S,
    /* 0x20 */ EventCode::KEY_D,          EventCode::KEY_F,          EventCode::KEY_G,           EventCode::KEY_H,
               EventCode::KEY_J,          EventCode::KEY_K,          EventCode::KEY_L,           EventCode::KEY_SEMICOLON,
    /* 0x28 */ EventCode::KEY_APOSTROPHE, EventCode::KEY_GRAVE,      EventCode::KEY_LEFT_SHIFT,  EventCode::KEY_BACK_SLASH,
               EventCode::KEY_Z,          EventCode::KEY_X,          EventCode::KEY_C,           EventCode::KEY_V,
    /* 0x30 */ EventCode::KEY_B,          EventCode::KEY_N,          EventCode::KEY_M,           EventCode::KEY_COMMA,
               EventCode::KEY_DOT,        EventCode::KEY_SLASH,      EventCode::KEY_RIGHT_SHIFT, EventCode::KEY_KPASTERISK,
    /* 0x38 */ EventCode::KEY_LEFT_ALT,   EventCode::KEY_SPACE,      EventCode::KEY_CAPS_LOCK,   EventCode::KEY_1,
               EventCode::KEY_2,          EventCode::KEY_3,          EventCode::KEY_4,           EventCode::KEY_5,
    /* 0x40 */ EventCode::KEY_6,          EventCode::KEY_7,          EventCode::KEY_8,           EventCode::KEY_9,
               EventCode::KEY_F10,        EventCode::KEY_NUM_LOCK,   EventCode::KEY_SCROLL_LOCK, EventCode::KEY_KP_7,
    /* 0x48 */ EventCode::KEY_KP_8,       EventCode::KEY_KP_9,       EventCode::KEY_KP_MINUS,    EventCode::KEY_KP_4,
               EventCode::KEY_KP_5,       EventCode::KEY_KP_6,       EventCode::KEY_KP_PLUS,     EventCode::KEY_KP_1,
    /* 0x50 */ EventCode::KEY_KP_2,       EventCode::KEY_KP_3,       EventCode::KEY_KP_0,        EventCode::KEY_KP_DOT,
               EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,    EventCode::KEY_F11,
    /* 0x58 */ EventCode::KEY_F12
];

// Extende scan code
const EXTENDE_KEY_CODES: [u8; 0x59] = [
    /* 0x00 */ EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,    EventCode::KEY_RESERVED,
               EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,    EventCode::KEY_RESERVED,
    /* 0x08 */ EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,    EventCode::KEY_RESERVED,
               EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,    EventCode::KEY_RESERVED,
    /* 0x10 */ EventCode::KEY_BACK,       EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,    EventCode::KEY_RESERVED,
               EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,    EventCode::KEY_RESERVED,
    /* 0x18 */ EventCode::KEY_RESERVED,   EventCode::KEY_FORWARD,    EventCode::KEY_RESERVED,    EventCode::KEY_RESERVED,
               EventCode::KEY_KP_ENTER,   EventCode::KEY_RIGHT_CTRL, EventCode::KEY_RESERVED,    EventCode::KEY_RESERVED,
    /* 0x20 */ EventCode::KEY_MUTE,       EventCode::KEY_RESERVED,   EventCode::KEY_PLAY,        EventCode::KEY_RESERVED,
               EventCode::KEY_STOP,       EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,    EventCode::KEY_RESERVED,
    /* 0x28 */ EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,    EventCode::KEY_RESERVED,
               EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,    EventCode::KEY_VOLUM_EDOWN,
    /* 0x30 */ EventCode::KEY_VOLUME_UP,  EventCode::KEY_RESERVED,   EventCode::KEY_WWW,         EventCode::KEY_RESERVED,
               EventCode::KEY_RESERVED,   EventCode::KEY_KP_SLASH,   EventCode::KEY_RESERVED,    EventCode::KEY_RESERVED,
    /* 0x38 */ EventCode::KEY_RIGHT_ALT,  EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,    EventCode::KEY_RESERVED,
               EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,    EventCode::KEY_RESERVED,
    /* 0x40 */ EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,    EventCode::KEY_RESERVED,
               EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,    EventCode::KEY_HOME,
    /* 0x48 */ EventCode::KEY_P,          EventCode::KEY_PAGE_UP,    EventCode::KEY_RESERVED,    EventCode::KEY_LEFT,
               EventCode::KEY_RESERVED,   EventCode::KEY_RIGHT,      EventCode::KEY_RESERVED,    EventCode::KEY_END,
    /* 0x50 */ EventCode::KEY_DOWN,       EventCode::KEY_PAGE_DOWN,  EventCode::KEY_INSERT,      EventCode::KEY_DELETE,
               EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,   EventCode::KEY_RESERVED,    EventCode::KEY_RESERVED,
    /* 0x58 */ EventCode::KEY_RESERVED,
];

// Struct PS2KeyboardConfig
#[derive(Clone)]
pub struct PS2KeyboardConfig {
    pub irq: isize,
}

// Impl PS2KeyboardConfig
impl PS2KeyboardConfig {
    pub const fn new() -> Self {
        Self { irq: 0 }
    }
}

// Struct PS2Keyboard
pub struct PS2Keyboard {
    config: PS2KeyboardConfig,
    is_extended: bool,
}

// Impl PS2Keyboard
impl PS2Keyboard {
    pub const fn new() -> Self {
        Self {
            config: PS2KeyboardConfig::new(),
            is_extended: false,
        }
    }

    // Set config
    fn set_config(&mut self, data: *mut ()) {
        if !data.is_null() {
            self.config = unsafe { (*(data as *mut PS2KeyboardConfig)).clone() }
        }
    }

    // Input handler
    fn input_handler(&mut self) {
        if (port_byte_in(PS2_READ_STATUS) & PS2_STATUS_OUTPUT_BUFFER_MSK) != 0 {
            // Get the raw scancode
            let scancode = port_byte_in(PS2_READ_DATA);

            // Set the is extended flag and return when scancode is 0xE0
            if scancode == 0xE0 { self.is_extended = true; return; }

            // Select keycodes table
            let p_key_codes = if self.is_extended {
                EXTENDE_KEY_CODES
            } else {
                KEY_CODES
            };

            // Report key event
            if scancode > 0 && scancode <= 0x58 {
                let code = p_key_codes[scancode as usize];
                kernel().event().report_key(code.into(), KeyStatus::KEY_PRESSED.into());
            } else if scancode > 0x80 && scancode <= 0xd0 {
                let code = p_key_codes[(scancode - 0x80) as usize];
                kernel().event().report_key(code.into(), KeyStatus::KEY_RELEASED.into());
            }

            // Clear the is extended flag
            self.is_extended = false; 
        }
    }
}

// Impl PS2Keyboard
impl Driver for PS2Keyboard {
    // Open
    fn open(&mut self, data: *mut ()) -> bool {
        // Get config
        self.set_config(data);

        // Create input handler
        let input_cb = Callback::new(Self::input_handler as u32).with_instance(self);
        kernel().interrupt().set_isr_cb(self.config.irq, input_cb);

        true
    }

    // Close
    fn close(&mut self) {
        // Delete input handler
        let input_cb = Callback::new(Self::input_handler as u32).with_instance(self);
        kernel().interrupt().del_isr_cb(self.config.irq, input_cb);
    }
}

// Struct ps2 keyboard drv
struct PS2KeyboardDrv;

// Impl plat driver for ps2 keyboard driver
impl PlatDriver for PS2KeyboardDrv {
    // Probe
    fn probe(&mut self, device: &mut PlatDevWrapper) -> bool {
        device.plat().attach(Box::new(PS2Keyboard::new()));
        true
    }

    // Remove
    fn remove(&mut self, device: &mut PlatDevWrapper) -> bool {
        device.plat().detach();
        true
    }
}

// Register plat driver
register_plat_driver!(PS2KeyboardDrv, ps2keyboard, ps2_keyboard_drv);
