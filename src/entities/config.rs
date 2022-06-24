use sfml::window::Key;

#[derive(Clone, Copy)]
pub struct MoveKeyboardConfig {
    pub top_left: Key,
    pub top: Key,
    pub top_right: Key,
    pub bottom_left: Key,
    pub bottom: Key,
    pub bottom_right: Key,
}

#[derive(Clone, Copy)]
pub struct OrderKeyboardConfig {
    pub build_city: Key,
    pub build_farm_field: Key,
    pub build_mine: Key,
    pub transform: Key,
}

pub struct MapKeyboardConfig {
    pub new_turn: Key,
}
