use std::fmt::Debug;

use std::sync::mpsc;

use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AbsInfo, AbsoluteAxisType, AttributeSet, EventType, InputEvent, Key, UinputAbsSetup,
};

const RIGHT_START: i32 = 780;
const LEFT_START: i32 = 10;

const SLIDER_AXES: [(AbsoluteAxisType, &str, i32, i32); 8] = [
    (AbsoluteAxisType::ABS_X, "Joystick X", LEFT_START, 160),
    (AbsoluteAxisType::ABS_Y, "Joystick Y", LEFT_START, 200),
    (AbsoluteAxisType::ABS_RX, "Joystick X", RIGHT_START, 160),
    (AbsoluteAxisType::ABS_RY, "Joystick Y", RIGHT_START, 200),
    (AbsoluteAxisType::ABS_HAT0X, "Hat X", LEFT_START, 280),
    (AbsoluteAxisType::ABS_HAT0Y, "Hat Y", LEFT_START, 320),
    (AbsoluteAxisType::ABS_HAT1X, "Index", LEFT_START, 0),
    (AbsoluteAxisType::ABS_HAT1Y, "Index", RIGHT_START, 0),
];

const BUTTONS: [(Key, &str, i32, i32); 13] = [
    (Key::BTN_SOUTH, "A", RIGHT_START, 280),     
    (Key::BTN_EAST, "B", RIGHT_START, 320),
    (Key::BTN_TL, "LT", LEFT_START, 120),
    (Key::BTN_TR, "RT", RIGHT_START, 120),
    (Key::BTN_TL2, "LB", LEFT_START, 40),
    (Key::BTN_TR2, "RB", RIGHT_START, 40),
    (Key::BTN_THUMBL, "Hat Press", LEFT_START, 360),

    (Key::BTN_WEST, "Y", 200, 440),
    (Key::BTN_NORTH, "X", 200, 480),
    (Key::BTN_SELECT, "Select / View", 400, 440),
    (Key::BTN_START, "Menu / Start", 400, 480),
    (Key::BTN_THUMBR, "Right Thumbstick", 600, 440),
    (Key::BTN_MODE, "Mode", 600, 480),
];

pub type AnalogAxis = Control<i8>;
pub type Button = Control<bool>;

pub type EventCode = u16;

pub fn build_uninput() -> anyhow::Result<(Box<[AnalogAxis]>, Box<[Button]>)> {
    let mut device = VirtualDeviceBuilder::new()?.name("Linux Virtual Joystick");

    let (event_sender, event_recv) = mpsc::channel();

    let abs_setup = AbsInfo::new(0, -100, 100, 0, 0, 1);
    let mut axes = Vec::with_capacity(SLIDER_AXES.len());
    for (axis, name, pos_x, pos_y) in SLIDER_AXES {
        let axis = UinputAbsSetup::new(axis, abs_setup);
        device = device.with_absolute_axis(&axis)?;
        axes.push(AnalogAxis::new(axis.code(), event_sender.clone(), name, pos_x, pos_y))
    }

    let mut buttons = Vec::with_capacity(SLIDER_AXES.len());
    let mut keys = AttributeSet::<Key>::new();
    for (button, name, pos_x, pos_y) in BUTTONS {
        keys.insert(button);
        buttons.push(Button::new(button.code(), event_sender.clone(), name, pos_x, pos_y))
    }
    let device = device.with_keys(&keys)?;

    let device = device.build()?;

    std::thread::spawn(|| device_thread(device, event_recv));

    Ok((axes.into_boxed_slice(), buttons.into_boxed_slice()))
}

type EventValue = i32;

#[derive(Debug)]
pub struct Control<T: Default + PartialEq + Clone + ControllerValue + Debug> {
    event_code: EventCode,
    old_value: T,
    pub new_value: T,
    event_sender: mpsc::Sender<InputEvent>,
    name: &'static str,
    pub pos_x: i32,
    pub pos_y: i32,
}

impl<T: Default + PartialEq + ControllerValue + Clone + Debug> Control<T> {
    pub fn new(
        event_code: EventCode,
        event_sender: mpsc::Sender<InputEvent>,
        name: &'static str,
        pos_x: i32,
        pos_y: i32,
    ) -> Self {
        Self {
            event_code,
            old_value: T::default(),
            new_value: T::default(),
            event_sender,
            name,
            pos_x,
            pos_y,
        }
    }

    pub fn new_value(&mut self) {
        if self.new_value == self.old_value {
            return;
        }
        let control_value = InputEvent::new(
            T::controller_type(),
            self.event_code,
            self.new_value.controller_value(),
        );
        self.event_sender.send(control_value).unwrap();
        self.old_value = self.new_value.clone();
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

pub trait ControllerValue {
    fn controller_type() -> EventType;
    fn controller_value(&self) -> EventValue;
}

impl ControllerValue for i8 {
    fn controller_type() -> EventType {
        EventType::ABSOLUTE
    }
    fn controller_value(&self) -> EventValue {
        i32::from(*self)
    }
}

impl ControllerValue for bool {
    fn controller_type() -> EventType {
        EventType::KEY
    }

    fn controller_value(&self) -> EventValue {
        i32::from(*self)
    }
}

// This could also be done using async rust, but that seemed to be overkill for this project.
fn device_thread(mut device: VirtualDevice, events: mpsc::Receiver<InputEvent>) {
    loop {
        let Ok(new_event) = events.recv() else {
            //Sender dropped, so app is being closed.
            break;
        };
        device.emit(&[new_event]).unwrap()
    }
}
