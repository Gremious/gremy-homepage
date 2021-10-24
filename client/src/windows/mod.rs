use super::*;

pub mod main;

pub type TabState = Mutable<Tab>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SmartDefault)]
pub enum Tab {
    #[default]
    Main,
}
