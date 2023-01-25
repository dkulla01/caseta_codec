use anyhow::anyhow;
use std::fmt::{Display, Formatter};

pub type RemoteId = u8;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum ButtonId {
    PowerOn,
    Up,
    Favorite,
    Down,
    PowerOff,
}

impl Display for ButtonId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<u8> for ButtonId {
    type Error = anyhow::Error;

    fn try_from(id: u8) -> Result<Self, anyhow::Error> {
        match id {
            2 => Ok(ButtonId::PowerOn),
            5 => Ok(ButtonId::Up),
            3 => Ok(ButtonId::Favorite),
            6 => Ok(ButtonId::Down),
            4 => Ok(ButtonId::PowerOff),
            _ => Err(anyhow!("{} is not a valid button id", id)),
        }
    }
}

#[derive(Debug)]
pub enum ButtonAction {
    Press,
    Release,
}

impl Display for ButtonAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<u8> for ButtonAction {
    type Error = anyhow::Error;
    fn try_from(id: u8) -> Result<Self, Self::Error> {
        match id {
            3 => Ok(ButtonAction::Press),
            4 => Ok(ButtonAction::Release),
            _ => Err(anyhow!("{} is not a valid button action", id)),
        }
    }
}
