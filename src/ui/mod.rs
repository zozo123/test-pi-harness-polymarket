pub mod dashboard;
pub mod markets;
pub mod events;
pub mod spreads;
pub mod help;
pub mod theme;

use ratatui::Frame;
use crate::app::{App, View};

/// Main draw dispatcher — routes to the active view.
pub fn draw(frame: &mut Frame, app: &App) {
    match app.view {
        View::Dashboard => dashboard::draw(frame, app),
        View::Markets => markets::draw(frame, app),
        View::Events => events::draw(frame, app),
        View::Spreads => spreads::draw(frame, app),
        View::Help => help::draw(frame, app),
    }
}
