use gdnative::api::GlobalConstants;

pub fn scancode_to_egui(scancode: i64) -> Option<egui::Key> {

    let key = match scancode {
        GlobalConstants::KEY_DOWN => Some(egui::Key::ArrowDown),
        GlobalConstants::KEY_LEFT => Some(egui::Key::ArrowLeft),
        GlobalConstants::KEY_RIGHT => Some(egui::Key::ArrowRight),
        GlobalConstants::KEY_UP => Some(egui::Key::ArrowUp),
        GlobalConstants::KEY_ESCAPE => Some(egui::Key::Escape),
        GlobalConstants::KEY_TAB => Some(egui::Key::Tab),
        GlobalConstants::KEY_BACKSPACE => Some(egui::Key::Backspace),
        GlobalConstants::KEY_ENTER => Some(egui::Key::Enter),
        GlobalConstants::KEY_SPACE => Some(egui::Key::Space),
        GlobalConstants::KEY_INSERT => Some(egui::Key::Insert),
        GlobalConstants::KEY_DELETE => Some(egui::Key::Delete),
        GlobalConstants::KEY_HOME => Some(egui::Key::Home),
        GlobalConstants::KEY_END => Some(egui::Key::End),
        GlobalConstants::KEY_PAGEUP => Some(egui::Key::PageUp),
        GlobalConstants::KEY_PAGEDOWN => Some(egui::Key::PageDown),
        GlobalConstants::KEY_0 | GlobalConstants::KEY_KP_0 => Some(egui::Key::Num0),
        GlobalConstants::KEY_1 | GlobalConstants::KEY_KP_1 => Some(egui::Key::Num1),
        GlobalConstants::KEY_2 | GlobalConstants::KEY_KP_2 => Some(egui::Key::Num2),
        GlobalConstants::KEY_3 | GlobalConstants::KEY_KP_3 => Some(egui::Key::Num3),
        GlobalConstants::KEY_4 | GlobalConstants::KEY_KP_4 => Some(egui::Key::Num4),
        GlobalConstants::KEY_5 | GlobalConstants::KEY_KP_5 => Some(egui::Key::Num5),
        GlobalConstants::KEY_6 | GlobalConstants::KEY_KP_6 => Some(egui::Key::Num6),
        GlobalConstants::KEY_7 | GlobalConstants::KEY_KP_7 => Some(egui::Key::Num7),
        GlobalConstants::KEY_8 | GlobalConstants::KEY_KP_8 => Some(egui::Key::Num8),
        GlobalConstants::KEY_9 | GlobalConstants::KEY_KP_9 => Some(egui::Key::Num9),
        GlobalConstants::KEY_A => Some(egui::Key::A),
        GlobalConstants::KEY_B => Some(egui::Key::B),
        GlobalConstants::KEY_C => Some(egui::Key::C),
        GlobalConstants::KEY_D => Some(egui::Key::D),
        GlobalConstants::KEY_E => Some(egui::Key::E),
        GlobalConstants::KEY_F => Some(egui::Key::F),
        GlobalConstants::KEY_G => Some(egui::Key::G),
        GlobalConstants::KEY_H => Some(egui::Key::H),
        GlobalConstants::KEY_I => Some(egui::Key::I),
        GlobalConstants::KEY_J => Some(egui::Key::J),
        GlobalConstants::KEY_K => Some(egui::Key::K),
        GlobalConstants::KEY_L => Some(egui::Key::L),
        GlobalConstants::KEY_M => Some(egui::Key::M),
        GlobalConstants::KEY_N => Some(egui::Key::N),
        GlobalConstants::KEY_O => Some(egui::Key::O),
        GlobalConstants::KEY_P => Some(egui::Key::P),
        GlobalConstants::KEY_Q => Some(egui::Key::Q),
        GlobalConstants::KEY_R => Some(egui::Key::R),
        GlobalConstants::KEY_S => Some(egui::Key::S),
        GlobalConstants::KEY_T => Some(egui::Key::T),
        GlobalConstants::KEY_U => Some(egui::Key::U),
        GlobalConstants::KEY_V => Some(egui::Key::V),
        GlobalConstants::KEY_W => Some(egui::Key::W),
        GlobalConstants::KEY_X => Some(egui::Key::X),
        GlobalConstants::KEY_Y => Some(egui::Key::Y),
        GlobalConstants::KEY_Z => Some(egui::Key::Z),
        _ => None,
    };

    key
}

pub fn mouse_button_index_to_egui(button_index: i64) -> Option<egui::PointerButton> {
    match button_index {
        GlobalConstants::BUTTON_LEFT => Some(egui::PointerButton::Primary),
        GlobalConstants::BUTTON_RIGHT => Some(egui::PointerButton::Secondary),
        GlobalConstants::BUTTON_MIDDLE => Some(egui::PointerButton::Middle),
        _ => None,
    }
}