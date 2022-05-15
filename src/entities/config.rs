use sfml::window::Key;

pub struct MoveKeyboardConfig {
    pub top_left: Key,
    pub top: Key,
    pub top_right: Key,
    pub bottom_left: Key,
    pub bottom: Key,
    pub bottom_right: Key,
}

pub struct OrderKeyboardConfig {
    pub build_city: Key,
}
