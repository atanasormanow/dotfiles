use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::app::{App, ConfirmAction, InputMode, View};
use crate::dotfile::LinkStatus;

/// Render the entire UI
pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Actions bar
            Constraint::Min(5),    // Main content
            Constraint::Length(1), // Status bar
        ])
        .split(frame.area());

    render_actions_bar(frame, chunks[0], app);
    render_main_content(frame, chunks[1], app);
    render_status_bar(frame, chunks[2], app);

    // Render overlays (dialogs)
    match &app.view {
        View::Confirm(action) => render_confirm_dialog(frame, action, app),
        View::Input(mode) => render_input_dialog(frame, mode, app),
        View::Help => render_help_dialog(frame),
        View::Message(msg) => render_message_dialog(frame, msg),
        View::Distribute => render_distribute_dialog(frame, app),
        View::List => {}
    }
}

fn render_actions_bar(frame: &mut Frame, area: Rect, app: &App) {
    let actions = if matches!(app.view, View::List) {
        vec![
            ("q", "quit"),
            ("a", "add"),
            ("l", "link"),
            ("L", "link all"),
            ("u", "unlink"),
            ("d", "delete"),
            ("e", "edit"),
            ("r", "refresh"),
            ("/", "search"),
            ("?", "help"),
        ]
    } else {
        vec![("Esc", "back")]
    };

    let spans: Vec<Span> = actions
        .iter()
        .flat_map(|(key, desc)| {
            vec![
                Span::styled(format!(" [{}]", key), Style::default().fg(Color::Yellow)),
                Span::styled(format!("{} ", desc), Style::default().fg(Color::Gray)),
            ]
        })
        .collect();

    let bar = Paragraph::new(Line::from(spans)).style(Style::default().bg(Color::DarkGray));
    frame.render_widget(bar, area);
}

fn render_main_content(frame: &mut Frame, area: Rect, app: &App) {
    // Table header
    let header = Row::new(vec![
        Cell::from("Name").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Status").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Destination").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Root").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Git").style(Style::default().add_modifier(Modifier::BOLD)),
    ])
    .height(1)
    .style(Style::default().fg(Color::Cyan));

    // Table rows
    let rows: Vec<Row> = app
        .filtered_indices
        .iter()
        .enumerate()
        .map(|(display_idx, &actual_idx)| {
            let dotfile = &app.dotfiles[actual_idx];

            let status_style = match dotfile.link_status {
                LinkStatus::Linked => Style::default().fg(Color::Green),
                LinkStatus::Broken => Style::default().fg(Color::Red),
                LinkStatus::Conflict => Style::default().fg(Color::Yellow),
                LinkStatus::Unlinked => Style::default().fg(Color::Gray),
                LinkStatus::Unknown(_) => Style::default().fg(Color::Magenta),
            };

            let name = format!(
                "{}{}",
                if dotfile.is_directory { "🗀 " } else { "" },
                dotfile.name,
            );

            let dest_display = dotfile.dest_raw.as_str();
            let root_display = if dotfile.needs_sudo { "[x]" } else { "[ ]" };

            let row_style = if display_idx == app.selected {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            };

            Row::new(vec![
                Cell::from(name),
                Cell::from(dotfile.link_status.symbol()).style(status_style),
                Cell::from(dest_display),
                Cell::from(root_display),
                Cell::from(dotfile.git_status.symbol()),
            ])
            .style(row_style)
        })
        .collect();

    let widths = [
        Constraint::Length(20),
        Constraint::Length(8),
        Constraint::Min(30),
        Constraint::Length(6),
        Constraint::Length(5),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(if app.filter.is_empty() {
                    " Dotfiles ".to_string()
                } else {
                    format!(" Dotfiles (filter: {}) ", app.filter)
                }),
        )
        .row_highlight_style(Style::default().add_modifier(Modifier::BOLD));

    let mut state = TableState::default();
    state.select(Some(app.selected));

    frame.render_stateful_widget(table, area, &mut state);
}

fn render_status_bar(frame: &mut Frame, area: Rect, app: &App) {
    let message = if let Some(msg) = &app.status_message {
        msg.clone()
    } else if let Some(dotfile) = app.selected_dotfile() {
        format!(
            "{}: {} -> {}",
            dotfile.name,
            dotfile.link_status.symbol(),
            dotfile.dest_expanded.display()
        )
    } else {
        "No dotfiles found".to_string()
    };

    let status =
        Paragraph::new(message).style(Style::default().fg(Color::White).bg(Color::DarkGray));
    frame.render_widget(status, area);
}

fn render_confirm_dialog(frame: &mut Frame, action: &ConfirmAction, app: &App) {
    let area = centered_rect(50, 30, frame.area());
    frame.render_widget(Clear, area);

    let (title, message) = match action {
        ConfirmAction::Unlink(idx) => {
            let name = app
                .dotfiles
                .get(*idx)
                .map(|d| d.name.as_str())
                .unwrap_or("?");
            ("Confirm Unlink", format!("Remove symlink for '{}'?", name))
        }
        ConfirmAction::Remove(idx) => {
            let name = app
                .dotfiles
                .get(*idx)
                .map(|d| d.name.as_str())
                .unwrap_or("?");
            (
                "Confirm Remove",
                format!("Permanently delete '{}' from repo?", name),
            )
        }
        ConfirmAction::ForceLink(idx) => {
            let name = app
                .dotfiles
                .get(*idx)
                .map(|d| d.name.as_str())
                .unwrap_or("?");
            (
                "Confirm Force Link",
                format!(
                    "A file exists at destination.\nReplace it with symlink for '{}'?",
                    name
                ),
            )
        }
        ConfirmAction::ReplaceAdd { name, .. } => (
            "Confirm Replace",
            format!(
                "Dotfile '{}' already exists.\nReplace it with the new file?",
                name
            ),
        ),
        ConfirmAction::DistributeConflicts { conflict_count } => (
            "Confirm Overwrite",
            format!(
                "{} selected dotfile(s) have conflicts.\nOverwrite existing files?",
                conflict_count
            ),
        ),
    };

    let text = format!("{}\n\n[y] Yes  [n] No", message);
    let dialog = Paragraph::new(text)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .style(Style::default().bg(Color::Black)),
        )
        .style(Style::default().fg(Color::White));

    frame.render_widget(dialog, area);
}

fn render_input_dialog(frame: &mut Frame, mode: &InputMode, app: &App) {
    let area = centered_rect(60, 25, frame.area());
    frame.render_widget(Clear, area);

    let (title, prompt) = match mode {
        InputMode::AddDotfileSource => ("Add Dotfile", "Source file path:"),
        InputMode::AddDotfileName { .. } => ("Add Dotfile", "Dotfile name:"),
        InputMode::EditDestination(_) => ("Edit Destination", "New destination:"),
        InputMode::Search => ("Search", "Filter by name:"),
    };

    let text = format!(
        "{}\n> {}_\n\n[Enter] Confirm  [Esc] Cancel",
        prompt, app.input
    );
    let dialog = Paragraph::new(text)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .style(Style::default().bg(Color::Black)),
        )
        .style(Style::default().fg(Color::White));

    frame.render_widget(dialog, area);
}

fn render_help_dialog(frame: &mut Frame) {
    let area = centered_rect(50, 70, frame.area());
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().bg(Color::Black));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Split into two columns
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner);

    let left_text = r#"
 Navigation
 ----------
 j/Down    Move down
 k/Up      Move up
 g/Home    Go to first
 G/End     Go to last

 Actions
 -------
 l         Link selected
 L         Link multiple
 u         Unlink selected
 d         Delete from repo
 a         Add new dotfile
 e         Edit destination
 r         Refresh list
 /         Search/filter
 Esc       Clear / Cancel
 ?         Show this help
 q         Quit
"#;

    let right_text = r#"
 Status Column
 -------------
 [L] Linked   - symlink active
 [-] Unlinked - not linked
 [C] Conflict - file exists
 [X] Broken   - bad symlink
 [?] Unknown  - read error

 Root Column
 -----------
 [x] = outside $HOME

 Git Column
 ----------
 [ ] Clean
 [M] Modified
 [+] Staged



 Press any key to close
"#;

    let left = Paragraph::new(left_text).style(Style::default().fg(Color::White));
    let right = Paragraph::new(right_text).style(Style::default().fg(Color::White));

    frame.render_widget(left, columns[0]);
    frame.render_widget(right, columns[1]);
}

fn render_message_dialog(frame: &mut Frame, message: &str) {
    let area = centered_rect(50, 20, frame.area());
    frame.render_widget(Clear, area);

    let text = format!("{}\n\nPress any key to close", message);
    let dialog = Paragraph::new(text)
        .block(
            Block::default()
                .title(" Message ")
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .style(Style::default().bg(Color::Black)),
        )
        .style(Style::default().fg(Color::White));

    frame.render_widget(dialog, area);
}

fn render_distribute_dialog(frame: &mut Frame, app: &App) {
    let area = centered_rect(70, 80, frame.area());
    frame.render_widget(Clear, area);

    // Split into list area and help text
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(area);

    // Build list items
    let items: Vec<Line> = app
        .dotfiles
        .iter()
        .enumerate()
        .map(|(idx, dotfile)| {
            let selected = app.distribute_selected.contains(&idx);
            let checkbox = if selected { "[x]" } else { "[ ]" };
            let cursor = if idx == app.distribute_cursor {
                ">"
            } else {
                " "
            };

            let status_indicator = match dotfile.link_status {
                LinkStatus::Linked => " (linked)",
                LinkStatus::Conflict => " (conflict)",
                _ => "",
            };

            let style = if idx == app.distribute_cursor {
                Style::default().bg(Color::DarkGray)
            } else if selected {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            };

            Line::from(vec![
                Span::styled(format!("{} {} ", cursor, checkbox), style),
                Span::styled(
                    format!(
                        "{}{}",
                        if dotfile.is_directory { "🗀 " } else { "" },
                        dotfile.name
                    ),
                    style,
                ),
                Span::styled(status_indicator, Style::default().fg(Color::Gray)),
            ])
        })
        .collect();

    let list = Paragraph::new(items).block(
        Block::default()
            .title(" Distribute - Select dotfiles to link ")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(Style::default().bg(Color::Black)),
    );

    frame.render_widget(list, chunks[0]);

    // Help text
    let help = Paragraph::new(" [Space] toggle  [a] all  [n] none  [Enter] link  [Esc] cancel")
        .style(Style::default().fg(Color::Gray));
    frame.render_widget(help, chunks[1]);
}

/// Helper to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
