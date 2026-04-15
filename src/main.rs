mod actions;
mod app;
mod dotfile;
mod git;
mod ui;

use std::io;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;

use app::{App, ConfirmAction, View};

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    // Create app
    let app = App::new();

    // Run the app
    let result = match app {
        Ok(mut app) => run(&mut terminal, &mut app),
        Err(e) => {
            // Restore terminal before showing error
            disable_raw_mode()?;
            io::stdout().execute(LeaveAlternateScreen)?;
            return Err(e);
        }
    };

    // Restore terminal
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        // Render
        terminal.draw(|frame| ui::render(frame, app))?;

        // Handle pending editor open
        if app.pending_editor {
            // Leave alternate screen for editor
            disable_raw_mode()?;
            io::stdout().execute(LeaveAlternateScreen)?;

            // Run the editor
            app.do_open_editor();

            // Re-enter alternate screen
            enable_raw_mode()?;
            io::stdout().execute(EnterAlternateScreen)?;
            terminal.clear()?;
            continue;
        }

        // Handle input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                // Clear status message on any key press
                app.status_message = None;

                match &app.view {
                    View::List => handle_list_input(app, key.code, key.modifiers),
                    View::Help => {
                        app.view = View::List;
                    }
                    View::Message(_) => {
                        app.view = View::List;
                    }
                    View::Confirm(action) => {
                        handle_confirm_input(app, key.code, action.clone());
                    }
                    View::Input(_) => {
                        handle_input_mode(app, key.code);
                    }
                    View::Distribute => {
                        handle_distribute_input(app, key.code);
                    }
                }

                if app.should_quit {
                    break;
                }
            }
        }
    }

    Ok(())
}

fn handle_list_input(app: &mut App, code: KeyCode, modifiers: KeyModifiers) {
    match code {
        // Quit
        KeyCode::Char('q') => app.should_quit = true,

        // Navigation
        KeyCode::Char('j') | KeyCode::Down => app.select_next(),
        KeyCode::Char('k') | KeyCode::Up => app.select_prev(),
        KeyCode::Char('g') | KeyCode::Home => app.select_first(),
        KeyCode::Char('G') | KeyCode::End => app.select_last(),

        // Actions
        KeyCode::Char('E') => app.open_in_editor(),
        KeyCode::Char('l') => app.link_selected(),
        KeyCode::Char('S') => app.start_distribute(),
        KeyCode::Char('u') => app.unlink_selected(),
        KeyCode::Char('U') => app.unmanage_selected(),
        KeyCode::Char('a') => app.start_add(),
        KeyCode::Char('e') => app.start_edit_dest(),
        KeyCode::Char('d') => app.remove_selected(),
        KeyCode::Char('r') => {
            let _ = app.refresh();
        }

        // Search
        KeyCode::Char('/') => app.start_search(),
        KeyCode::Esc => app.clear_filter(),

        // Rename
        KeyCode::F(2) => app.start_rename(),

        // Help
        KeyCode::Char('?') => app.show_help(),

        // Ctrl+C to quit
        KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }

        _ => {}
    }
}

fn handle_confirm_input(app: &mut App, code: KeyCode, action: ConfirmAction) {
    match code {
        KeyCode::Char('y') | KeyCode::Char('Y') => match action {
            ConfirmAction::Unlink(idx) => app.confirm_unlink(idx),
            ConfirmAction::ForceLink(idx) => app.confirm_force_link(idx),
            ConfirmAction::Remove(idx) => app.confirm_remove(idx),
            ConfirmAction::Unmanage(idx) => app.confirm_unmanage(idx),
            ConfirmAction::ReplaceAdd { source, name } => app.confirm_replace_add(source, name),
            ConfirmAction::DistributeConflicts { .. } => {
                app.distribute_execute_with_force(true);
            }
        },
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            // For distribute conflicts, go back to distribute view instead of list
            if matches!(action, ConfirmAction::DistributeConflicts { .. }) {
                app.view = View::Distribute;
            } else {
                app.view = View::List;
            }
        }
        _ => {}
    }
}

fn handle_input_mode(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Enter => app.submit_input(),
        KeyCode::Esc => app.cancel(),
        KeyCode::Backspace => app.input_backspace(),
        KeyCode::Tab => app.trigger_completion(),
        KeyCode::Char(c) => app.input_char(c),
        _ => {}
    }
}

fn handle_distribute_input(app: &mut App, code: KeyCode) {
    match code {
        // Navigation
        KeyCode::Char('j') | KeyCode::Down => app.distribute_next(),
        KeyCode::Char('k') | KeyCode::Up => app.distribute_prev(),

        // Toggle selection
        KeyCode::Char(' ') => app.distribute_toggle(),

        // Select all / none
        KeyCode::Char('a') => app.distribute_select_all(),
        KeyCode::Char('n') => app.distribute_select_none(),

        // Execute
        KeyCode::Enter => app.distribute_execute(),

        // Cancel
        KeyCode::Esc | KeyCode::Char('q') => app.distribute_cancel(),

        _ => {}
    }
}
