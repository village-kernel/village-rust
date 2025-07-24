//###########################################################################
// vk_cmdmsg.rs
// The specific implementation of functions related to cmdmsg
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::misc::fopts::vk_dev_fopt::DevFopt;
use alloc::string::{String, ToString};

// Static const
const CMD_HISTORY_SIZE: usize = 10;
const ARG_BUFFER_SIZE: usize = 256;

// Enum input mode
enum InputMode {
    Edit = 0,
    Insert,
    Ansi,
    Dcs,
    Csi,
    Osc,
}

// Struct cmd msg
pub struct CmdMsg {
    pub cmd: String,
    pub args: String,
}

// Impl cmd msg
impl CmdMsg {
    pub const fn new() -> Self {
        Self {
            cmd: String::new(),
            args: String::new(),
        }
    }
}

// Struct cmd msg mgr
pub struct CmdMsgMgr {
    transceiver: DevFopt,
    input_mode: InputMode,
    has_message: bool,
    rx_msg: CmdMsg,
    rx_pos: usize,
    history: usize,
    tx_buf: String,
    rx_buf: String,
    cmd_history: [Option<String>; CMD_HISTORY_SIZE],
}

// Impl CmdMsgMgr
impl CmdMsgMgr {
    // New
    pub const fn new() -> Self {
        Self {
            transceiver: DevFopt::new(),
            input_mode: InputMode::Edit,
            has_message: false,
            rx_msg: CmdMsg::new(),
            rx_pos: 0,
            history: 0,
            tx_buf: String::new(),
            rx_buf: String::new(),
            cmd_history: [const { None }; CMD_HISTORY_SIZE],
        }
    }

    // Setup
    pub fn setup(&mut self, driver: &str) {
        // Open transceiver
        if self.transceiver.open(driver) {
            let msg = "\r\nPlease press Enter to activate this console.\r\n";
            let msglen = msg.len();

            //Output msg
            let mut sent = 0;
            while sent != msglen {
                sent = self.transceiver.write(msg.as_bytes(), msglen, 0);
            }

            //Wait for Enter
            let mut key = [0u8; 1];
            loop {
                let size = self.transceiver.read(&mut key, 1, 0);
                if size > 0 && key[0] == 0x0d {
                    break;
                }
            }
        }
    }

    // Execute
    pub fn execute(&mut self) -> bool {
        // Sent data when txbuffer not empty
        self.sending();

        // Received data and decode
        self.receiving()
    }

    // Exit
    pub fn exit(&mut self) {
        // Close transceiver
        self.transceiver.close();
    }
}

// Impl cmd msg mgr
impl CmdMsgMgr {
    // Write
    pub fn write(&mut self, data: &str) {
        self.tx_buf.push_str(data);
        self.sending();
    }

    // Sending
    fn sending(&mut self) {
        if !self.tx_buf.is_empty() {
            let bytes = self.tx_buf.as_bytes();
            while self.transceiver.write(bytes, bytes.len(), 0) != bytes.len() {}
            self.tx_buf.clear();
        }
    }
}

// Impl cmd msg mgr
impl CmdMsgMgr {
    // Receiving
    fn receiving(&mut self) -> bool {
        const BR_BUF_SIZE: u8 = 20;
        let mut br_buff: [u8; BR_BUF_SIZE as usize] = [0; BR_BUF_SIZE as usize];
        let mut br_size: u8;

        while {
            br_size = self.transceiver.read(&mut br_buff, BR_BUF_SIZE as usize, 0) as u8;
            br_size > 0
        } {
            for i in 0..br_size {
                let byte = br_buff[i as usize];

                // Control not to exceed the maximum length.
                if self.rx_buf.len() >= ARG_BUFFER_SIZE {
                    self.rx_buf.clear();
                    self.rx_pos = 0;
                    return false;
                }

                match self.input_mode {
                    InputMode::Insert => {
                        if !self.insert_mode(byte) {
                            self.edit_mode(byte);
                        }
                    }
                    InputMode::Edit => self.edit_mode(byte),
                    InputMode::Ansi => self.ansi_mode(byte),
                    InputMode::Dcs => self.dcs_mode(byte),
                    InputMode::Csi => self.csi_mode(byte),
                    InputMode::Osc => self.osc_mode(byte),
                }

                if self.has_message {
                    self.has_message = false;
                    return true;
                }
            }
        }

        false
    }

    // Insert mode
    fn insert_mode(&mut self, byte: u8) -> bool {
        // ASCII 32(space) ~ 126(~)
        if byte >= 0x20 && byte <= 0x7e {
            if self.rx_pos <= self.rx_buf.len() {
                // Insert new char in rx_pos
                let char_to_insert = byte as char;
                self.rx_buf.insert(self.rx_pos, char_to_insert);
                self.rx_pos += 1;

                // Sent new string
                let mut back = 0;
                self.tx_buf.push(char_to_insert);
                for i in self.rx_pos..self.rx_buf.len() {
                    self.tx_buf.push(self.rx_buf.chars().nth(i).unwrap());
                    back += 1;
                }

                // Move cursor back
                for _ in 0..back {
                    self.tx_buf.push('\x08');
                }
            }
            return true;
        }
        // ASCII DEL
        else if 0x7f == byte || b'\x08' == byte {
            if self.rx_pos > 0 && !self.rx_buf.is_empty() {
                // Del char[rxBufPos] and move char[rxBufPos] to char[rxBufPos-1]
                if self.rx_pos <= self.rx_buf.len() {
                    self.rx_buf.remove(self.rx_pos - 1);
                    self.rx_pos -= 1;
                }

                // Sent new string
                let mut back = 0;
                self.tx_buf.push('\x08');
                for i in self.rx_pos..self.rx_buf.len() {
                    self.tx_buf.push(self.rx_buf.chars().nth(i).unwrap());
                    back += 1;
                }
                self.tx_buf.push(' ');
                back += 1;

                // Move cursor back
                for _ in 0..back {
                    self.tx_buf.push('\x08');
                }
            }
            return true;
        }

        self.input_mode = InputMode::Edit;
        return false;
    }

    // Edit mode
    fn edit_mode(&mut self, byte: u8) {
        // ANSI ESC
        if b'\x1b' == byte {
            self.input_mode = InputMode::Ansi;
        }
        // ASCII 32(space) ~ 126(~)
        else if byte >= 0x20 && byte <= 0x7e {
            // Control not to exceed the maximum length.
            if self.rx_buf.len() < ARG_BUFFER_SIZE {
                self.tx_buf.push(byte as char);
                self.rx_buf.push(byte as char);
                self.rx_pos += 1;
            }
        }
        // ASCII DEL
        else if 0x7f == byte || b'\x08' == byte {
            if self.rx_pos > 0 {
                // Backspace one character
                self.rx_pos -= 1;
                if !self.rx_buf.is_empty() {
                    // Remove the last character
                    self.rx_buf.pop();
                }

                // Backspace character on terminal
                self.tx_buf.push_str("\x08 \x08");
            }
        }
        // ASCII CR
        else if 0x0d == byte {
            // Check is null cmd
            if self.rx_buf.is_empty() {
                self.rx_msg.cmd = "null".to_string();
                self.has_message = true;
                return;
            }

            // Set command
            let mut cmd_str = String::new();
            for c in self.rx_buf.chars() {
                if c == ' ' || c == '\r' || c == '\0' {
                    break;
                }
                cmd_str.push(c);
            }
            self.rx_msg.cmd = cmd_str;

            // Record history
            self.record_history();

            // Set args
            if let Some(ref history_cmd) = self.cmd_history[CMD_HISTORY_SIZE - 2] {
                self.rx_msg.args = history_cmd.clone();
            } else {
                self.rx_msg.args = String::new();
            }

            // Reset rxBufPos
            self.rx_pos = 0;
            self.rx_buf.clear();
            self.has_message = true;
            return;
        }
    }

    // ANSI mode
    fn ansi_mode(&mut self, byte: u8) {
        match byte {
            b'N' => {}                                //ESC N | 0x8e | SS2 – Single Shift Two
            b'O' => {}                                //ESC O | 0x8f | SS3 – Single Shift Three
            b'P' => self.input_mode = InputMode::Dcs, //ESC P | 0x90 | DCS – Device Control String
            b'[' => self.input_mode = InputMode::Csi, //ESC [ | 0x9b | CSI - Control Sequence Introducer
            b'\\' => {}                               //ESC \ | 0x9c | ST – String Terminator
            b']' => self.input_mode = InputMode::Osc, //ESC ] | 0x9d | OSC – Operating System Command
            b'X' => {}                                //ESC X | 0x98 | SOS – Start of String
            b'^' => {}                                //ESC ^ | 0x9e | PM – Privacy Message
            b'_' => {} //ESC _ | 0x9f | APC – Application Program Command
            b'c' => {} //ESC c |      | RIS – Reset to Initial State
            _ => {}
        }
    }

    // DCS mode
    fn dcs_mode(&mut self, _byte: u8) {
        // Not implemented
    }

    // CSI mode
    fn csi_mode(&mut self, byte: u8) {
        // Param byte
        if byte >= 0x30 && byte <= 0x3f {
            // Not implemented
        }

        // Middle byte
        if byte >= 0x20 && byte <= 0x2f {
            // Not implemented
        }

        // Final byte
        if byte >= 0x40 && byte <= 0x7e {
            match byte {
                b'A' => {
                    // up
                    if self.history > 0 && self.cmd_history[self.history - 1].is_some() {
                        self.record_temp_cmd();
                        self.history -= 1;
                        self.restored_history();
                    }
                }
                b'B' => {
                    // down
                    if self.history < CMD_HISTORY_SIZE - 1
                        && self.cmd_history[self.history + 1].is_some()
                    {
                        self.history += 1;
                        self.restored_history();
                    }
                }
                b'C' => {
                    // right
                    if self.rx_pos < self.rx_buf.len() {
                        self.tx_buf
                            .push(self.rx_buf.chars().nth(self.rx_pos).unwrap());
                        self.rx_pos += 1;
                    }
                    self.input_mode = InputMode::Insert;
                    return;
                }
                b'D' => {
                    // left
                    if self.rx_pos > 0 {
                        self.rx_pos -= 1;
                        self.tx_buf.push('\x08');
                    }
                    self.input_mode = InputMode::Insert;
                    return;
                }
                b'm' => {} // SGR
                _ => {}
            }

            self.input_mode = InputMode::Edit;
        }
    }

    // OSC mode
    fn osc_mode(&mut self, byte: u8) {
        if byte == 0 {
            self.input_mode = InputMode::Edit;
        }
    }

    // Read
    pub fn read(&mut self) -> CmdMsg {
        CmdMsg {
            cmd: self.rx_msg.cmd.clone(),
            args: self.rx_msg.args.clone(),
        }
    }
}

// Impl cmd msg mgr
impl CmdMsgMgr {
    /// Record temp cmd
    fn record_temp_cmd(&mut self) {
        // Return when history is not last
        if CMD_HISTORY_SIZE - 1 != self.history {
            return;
        }

        // Copy rx_buf string to history cmd
        let history_cmd = self.rx_buf.clone();

        // Set the history cmd as the last history
        self.cmd_history[CMD_HISTORY_SIZE - 1] = Some(history_cmd);
    }

    /// Record history
    fn record_history(&mut self) {
        // Copy rx_buf string to historyCmd
        let history_cmd = self.rx_buf.clone();

        // Move cmd_history[i+1] to cmd_history[i]
        for i in 0..(CMD_HISTORY_SIZE - 2) {
            self.cmd_history[i] = self.cmd_history[i + 1].take();
        }

        // Set the history cmd as the second to last history
        self.cmd_history[CMD_HISTORY_SIZE - 2] = Some(history_cmd);

        // Reset history position
        self.history = CMD_HISTORY_SIZE - 1;
    }

    /// Restored history
    fn restored_history(&mut self) {
        if let Some(ref history_cmd) = self.cmd_history[self.history] {
            // Clear display string
            let current_len = self.rx_buf.len();
            for _ in 0..current_len {
                self.tx_buf.push_str("\x08 \x08");
            }

            // Reset history cmd to rx_buf
            self.rx_buf = history_cmd.clone();
            self.rx_pos = self.rx_buf.len();

            // Display new cmd
            for c in self.rx_buf.chars() {
                self.tx_buf.push(c);
            }

            self.sending();
        }
    }
}
