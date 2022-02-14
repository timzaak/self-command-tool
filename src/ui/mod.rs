use std::time::Duration;
use crossterm:: {
    execute,
    event::{self, Event},
    terminal::enable_raw_mode
};
use crossterm::event::KeyCode;
use crossterm::terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tokio::time::Instant;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use tui::text::Span;
use tui::widgets::{List, ListItem, ListState};
use crate::v2fly::VMessConfig;

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    #[allow(dead_code)]
    fn unselect(&mut self) {
        self.state.select(None);
    }

    fn get_select(&self) -> Option<&T>{
        self.state.selected().map(|x| self.items.get(x)).flatten()
    }
}
/*
fn ui<B:Backend>(f: &mut Frame<B>, data:&mut StatefulList<VMessConfig>)  {
    let list_items:Vec<ListItem> = data.items.iter().map(|c| {
        let i = Span::from(c.ps.as_str());
        ListItem::new(i)
    }).collect();
    let list = List::new(list_items).highlight_symbol(">>");
    f.render_stateful_widget(list, f.size(), &mut data.state);
}*/

pub async fn run_v2fly_ui(configs: Vec<VMessConfig>) -> anyhow::Result<Option<String>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let tick_rate = Duration::from_millis(250);
    let last_tick = Instant::now();

    let mut data = StatefulList::with_items(configs);
    loop {
        //terminal.draw(|f|{ ui(f, &mut data) })?;
        terminal.draw(|f|{
            let list_items:Vec<ListItem> = data.items.iter().map(|c| {
                let i = Span::from(c.ps.as_str());
                ListItem::new(i)
            }).collect();
            let list = List::new(list_items).highlight_symbol(">>");
            f.render_stateful_widget(list, f.size(), &mut data.state);
        })?;
        let timeout = tick_rate.checked_sub(last_tick.elapsed()).unwrap_or_else(||Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => data.previous(),
                    KeyCode::Down => data.next(),
                    KeyCode::Enter => break,
                    _ => {}
                }
            }
        }
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    match data.get_select() {
        Some(r) => println!("{}", r.ps),
        _ => {}
    }

    Ok(data.get_select().map(|x|{
        serde_json::to_string_pretty(&(x.to_v2fly_outbounds_json())).unwrap()
    }))
}