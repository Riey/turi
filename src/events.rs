#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClickEvent;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CharEvent(pub char);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EnterEvent;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackspaceEvent;

