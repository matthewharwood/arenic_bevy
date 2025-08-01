use crate::utils::AudioClips;

pub mod alchemist;
pub mod bard;
pub mod cardinal;
pub mod forager;
pub mod hunter;
pub mod merchant;
pub mod thief;
pub mod warrior;

pub trait Character {
    const CLASS_NAME: &'static str;
    const AUDIO: AudioClips<4>;
    const ICON: (&'static str, &'static str);
    const PORTRAIT: &'static str;
}
