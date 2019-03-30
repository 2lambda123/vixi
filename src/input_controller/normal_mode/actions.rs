use std::collections::HashMap;

use crate::devices::keyboard::KeyStroke;

use failure::Error;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    SwitchToInsertMode,
    SwitchToVisualMode,
    Exit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    PageUp,
    PageDown,
    Paste,
}

pub type Config = HashMap<String, String>;

pub struct Actions(HashMap<KeyStroke, Action>);

impl Default for Actions {
    fn default() -> Self {
        let mut actions = HashMap::with_capacity(1);

        actions.insert(KeyStroke::Char('i'), Action::SwitchToInsertMode);
        actions.insert(KeyStroke::Char('v'), Action::SwitchToVisualMode);
        actions.insert(KeyStroke::Char('q'), Action::Exit);
        actions.insert(KeyStroke::Char('p'), Action::Paste);

        // The classic arrow keys.
        actions.insert(KeyStroke::KeyUp, Action::MoveUp);
        actions.insert(KeyStroke::KeyDown, Action::MoveDown);
        actions.insert(KeyStroke::KeyLeft, Action::MoveLeft);
        actions.insert(KeyStroke::KeyRight, Action::MoveRight);
        actions.insert(KeyStroke::KeyPreviousPage, Action::PageUp);
        actions.insert(KeyStroke::KeyNextPage, Action::PageDown);

        // The "vim like" keys.
        actions.insert(KeyStroke::Char('k'), Action::MoveUp);
        actions.insert(KeyStroke::Char('j'), Action::MoveDown);
        actions.insert(KeyStroke::Char('h'), Action::MoveLeft);
        actions.insert(KeyStroke::Char('l'), Action::MoveRight);

        Self(actions)
    }
}

impl Actions {
    #[allow(dead_code)]
    pub fn from_config(config_map: &Config) -> Result<Self, Error> {
        let mut actions = HashMap::with_capacity(config_map.len());

        for (key_desc, action_name) in config_map.iter() {
            let keystroke = KeyStroke::from_description(&key_desc)
                .ok_or_else(|| format_err!("failed to parse the key {}", key_desc))?;

            let action = match action_name.as_str() {
                "swtich_to_insert_mode" => Action::SwitchToInsertMode,
                "move_up" => Action::MoveUp,
                "move_down" => Action::MoveDown,
                "move_left" => Action::MoveLeft,
                "move_right" => Action::MoveRight,
                "exit" => Action::Exit,
                "page_up" => Action::PageUp,
                "page_down" => Action::PageDown,
                "paste" => Action::Paste,
                _ => return Err(format_err!("unknown action {}", action_name)),
            };

            actions.insert(keystroke, action);
        }

        Ok(Self(actions))
    }

    pub fn get(&self, key: KeyStroke) -> Option<Action> {
        if let Some(key) = self.0.get(&key) {
            Some(*key)
        } else {
            None
        }
    }
}