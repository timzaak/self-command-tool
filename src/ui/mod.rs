use std::io::Stdout;
use std::time::Duration;
use crossterm:: {
    execute,
    event::{self, Event},
    terminal::enable_raw_mode
};
use crossterm::event::{EnableMouseCapture, KeyCode};
use crossterm::terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tokio::time::Instant;
use tui::backend::CrosstermBackend;
use tui::{Frame, Terminal};
use tui::layout::{Direction, Layout};
use tui::text::Span;
use tui::widgets::{List, ListItem, ListState};
use crate::v2fly::VMessConfig;

struct V2FlyUI {
    state:ListState,
}


async fn run_v2fly_ui(configs: Vec<VMessConfig>) -> anyhow::Result<()> {

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    let mut state = ListState::default();

    loop {
        let list_items:Vec<ListItem> = configs.iter().map(|c| {
            let i = Span::from(c.ps.as_str());
            ListItem::new(i)
        }).collect();
        let list_items = List::new(list_items);

        terminal.draw(|f|{
            f.render_stateful_widget(list_items, f.size(), &mut state);
        });
        let timeout = tick_rate.checked_sub(last_tick.elapsed()).unwrap_or_else(||Duration::from_secs(0));
        if(crossterm::event::poll(timeout)?) {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') =>  break,
                    _ => {}
                }
            }
        }
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}