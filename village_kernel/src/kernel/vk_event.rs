//###########################################################################
// vk_event.rs
// The specific implementation of functions related to event
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::misc::fopts::vk_dev_fopt::DevFopt;
use crate::misc::model::vk_observer::ObserverModel;
use crate::traits::vk_callback::Callback;
use crate::traits::vk_driver::DriverID;
use crate::traits::vk_kernel::{Event, EventOutFormat, EventType};
use crate::traits::vk_kernel::{EventInputAxis, EventInputKey, EventOutputAxis, EventOutputText};
use crate::traits::vk_linkedlist::LinkedList;
use crate::village::kernel;

// Struct concrete event
pub struct ConcreteEvent {
    input_key: EventInputKey,
    input_axis: EventInputAxis,

    output_text: EventOutputText,
    output_axis: EventOutputAxis,
    output_format: EventOutFormat,

    in_devs: LinkedList<DevFopt>,
    observers: [ObserverModel; EventType::AllSizes as usize],
}

// Impl concrete event
impl ConcreteEvent {
    pub const fn new() -> Self {
        Self {
            input_key: EventInputKey::new(),
            input_axis: EventInputAxis::new(),

            output_text: EventOutputText::new(),
            output_axis: EventOutputAxis::new(),
            output_format: EventOutFormat::Noraml,

            in_devs: LinkedList::new(),
            observers: [const { ObserverModel::new() }; EventType::AllSizes as usize],
        }
    }
}

// impl concrete event
impl ConcreteEvent {
    // Setup
    pub fn setup(&mut self) {
        // Init all input devices
        for device in kernel().device().get_drivers().iter_mut() {
            if device.info().get_id() == DriverID::Input {
                self.init_input_device(device.info().get_name());
            }
        }

        // Set default output format
        self.output_format = EventOutFormat::Noraml;

        // Output debug info
        kernel().debug().info("Input event setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {
        // Exit all input devices
        for device in kernel().device().get_drivers().rev_iter_mut() {
            if device.info().get_id() == DriverID::Input {
                self.exit_input_device(device.info().get_name());
            }
        }

        // Clear all input devices
        self.in_devs.clear();

        // Clear all observers
        for i in 0..EventType::AllSizes as usize {
            self.observers[i].clear();
        }
    }
}

// Impl event for concrete event
impl Event for ConcreteEvent {
    // Init input device
    fn init_input_device(&mut self, input: &str) {
        // Create an input device object
        let mut device = DevFopt::new();

        // Open and add into in_devs list
        if device.open(input) {
            self.in_devs.push(device);
        }
    }

    // Exit input device
    fn exit_input_device(&mut self, input: &str) {
        self.in_devs
            .retain_mut(|device| !(device.get_name() == input));
    }

    // Attach
    fn attach(&mut self, etype: EventType, callback: Callback) {
        self.observers[etype as usize].attach(callback);
    }

    // Detach
    fn detach(&mut self, etype: EventType, callback: Callback) {
        self.observers[etype as usize].detach(callback);
    }

    // Report key
    fn report_key(&mut self, code: isize, status: isize) {
        self.input_key.code = code;
        self.input_key.status = status;
        self.observers[EventType::InputKey as usize].notify(&mut self.input_key);
    }

    // Report axis
    fn report_axis(&mut self, axis_x: isize, axis_y: isize, axis_z: isize) {
        self.input_axis.axis_x = axis_x;
        self.input_axis.axis_y = axis_y;
        self.input_axis.axis_z = axis_z;
        self.observers[EventType::InputAxis as usize].notify(&mut self.input_axis);
    }

    // Push char
    fn push_char(&mut self, ch: char) {
        self.output_text.data.clear();
        self.output_text.data.push(ch);
        self.observers[EventType::OutputText as usize].notify(&mut self.input_axis);
    }

    // Push str
    fn push_str(&mut self, str: &str) {
        self.output_text.data.clear();
        self.output_text.data.push_str(str);
        self.observers[EventType::OutputText as usize].notify(&mut self.output_text);
    }

    // Push axis
    fn push_axis(&mut self, axis_x: isize, axis_y: isize, axis_z: isize) {
        self.output_axis.axis_x = axis_x;
        self.output_axis.axis_y = axis_y;
        self.output_axis.axis_z = axis_z;
        self.observers[EventType::OutputAxis as usize].notify(&mut self.output_axis);
    }

    // Set out format
    fn set_out_format(&mut self, format: EventOutFormat) {
        self.output_format = format;
    }

    // Get out format
    fn get_out_format(&mut self) -> EventOutFormat {
        self.output_format.clone()
    }
}
