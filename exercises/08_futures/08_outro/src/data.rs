use crate::store::TicketId;

use serde::{Serialize};
use crate::description::TicketDescription;
use crate::title::TicketTitle;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}
