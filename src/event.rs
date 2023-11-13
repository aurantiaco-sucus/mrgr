
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Key {
    Up,
    Down,
    Left,
    Right,
    North,
    South,
    West,
    East,
    Select,
    Start,
    LeftBumper,
    RightBumper,
    LeftTrigger,
    RightTrigger,
    LeftThumb,
    RightThumb,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Mouse {
    Main,
    Secondary,
    Tertiary,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Axis {
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
    LeftTrigger,
    RightTrigger,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Event {
    ExitRequested,
    KeyPressed(Key),
    KeyReleased(Key),
    MousePressed(Mouse),
    MouseReleased(Mouse),
    MouseMoved(u32, u32),
    AxisChanged(Axis, f32),
}