use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::app::{App, ConfirmAction, InputMode, View};
use crate::dotfile::{GitStatus, LinkStatus};

/// Render the entire UI
pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Actions bar (2 lines)
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
    let (line1_actions, line2_actions): (Vec<_>, Vec<_>) = if matches!(app.view, View::List) {
        (
            vec![
                ("a", "add"),
                ("l", "link"),
                ("u", "unlink"),
                ("s", "sync links"),
                ("U", "unmanage"),
                ("d", "delete"),
                ("E", "edit file"),
                ("e", "edit dest"),
            ],
            vec![
                ("F2", "rename"),
                ("q", "quit"),
                ("r", "refresh"),
                ("/", "search"),
                ("?", "help"),
                ("", ""),
                ("", ""),
                ("", ""),
            ],
        )
    } else {
        (vec![("Esc", "cancel")], vec![])
    };

    const CELL_WIDTH: usize = 14;

    let make_line = |actions: &[(&str, &str)]| -> Line {
        let spans: Vec<Span> = actions
            .iter()
            .flat_map(|(key, desc)| {
                if key.is_empty() {
                    vec![Span::styled(" ".repeat(CELL_WIDTH), Style::default())]
                } else {
                    let key_part = format!(" [{}]", key);
                    let desc_part = format!(" {}", desc);
                    let used = key_part.len() + desc_part.len();
                    let padding = CELL_WIDTH.saturating_sub(used);
                    vec![
                        Span::styled(key_part, Style::default().fg(Color::Yellow)),
                        Span::styled(desc_part, Style::default().fg(Color::Gray)),
                        Span::styled(" ".repeat(padding), Style::default()),
                    ]
                }
            })
            .collect();
        Line::from(spans)
    };

    let lines = vec![make_line(&line1_actions), make_line(&line2_actions)];
    let bar = Paragraph::new(lines).style(Style::default().bg(Color::DarkGray));
    frame.render_widget(bar, area);
}

fn render_main_content(frame: &mut Frame, area: Rect, app: &App) {
    // Table header
    let header = Row::new(vec![
        Cell::from("Name").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Status").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Destination").style(Style::default().add_modifier(Modifier::BOLD)),
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

            let git_style = match dotfile.git_status {
                GitStatus::Modified => Style::default().fg(Color::Yellow),
                GitStatus::Staged => Style::default().fg(Color::Green),
                GitStatus::Clean => Style::default(),
            };

            let name = format!(
                "{}{}",
                if dotfile.is_directory { "🗀 " } else { "" },
                dotfile.name,
            );

            let dest_display = dotfile.dest_raw.as_str();

            let row_style = if display_idx == app.selected {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            };

            Row::new(vec![
                Cell::from(name),
                Cell::from(dotfile.link_status.symbol()).style(status_style),
                Cell::from(dest_display),
                Cell::from(dotfile.git_status.symbol()).style(git_style),
            ])
            .style(row_style)
        })
        .collect();

    let widths = [
        Constraint::Length(20),
        Constraint::Length(8),
        Constraint::Min(30),
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
                "Confirm Delete",
                format!(
                    "Permanently delete '{}' from repo?\n\nThe file will no longer exist on your system.",
                    name
                ),
            )
        }
        ConfirmAction::Unmanage(idx) => {
            let dotfile = app.dotfiles.get(*idx);
            let name = dotfile.map(|d| d.name.as_str()).unwrap_or("?");
            let dest = dotfile.map(|d| d.dest_raw.as_str()).unwrap_or("?");
            (
                "Confirm Unmanage",
                format!(
                    "Move '{}' from repo to\n{}?\n\nThe file will no longer be tracked.",
                    name, dest
                ),
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
        InputMode::Rename(_) => ("Rename Dotfile", "New name (alphanumeric, -, _, . only):"),
    };

    // Build the content with autocomplete preview
    let should_show_completion = matches!(
        mode,
        InputMode::AddDotfileSource | InputMode::EditDestination(_)
    );

    let mut lines = vec![Line::from(prompt)];

    // Build input line with completion preview
    let has_completion = should_show_completion
        && app.completion_index.is_some()
        && !app.completion_candidates.is_empty();

    if has_completion {
        let idx = app.completion_index.unwrap(); // Safe because we checked is_some()
        if let Some(candidate) = app.completion_candidates.get(idx) {
            let home = std::env::var("HOME").unwrap_or_default();
            let path_str = candidate.to_string_lossy();
            let display_path = if !home.is_empty() && path_str.starts_with(&home) {
                path_str.replacen(&home, "$HOME", 1)
            } else {
                path_str.to_string()
            };

            // Add trailing slash for directories
            let suggestion = if candidate.is_dir() {
                format!("{}/", display_path)
            } else {
                display_path
            };

            // Show what part is being autocompleted (grayed out)
            let completion_part = if suggestion.len() > app.input.len() {
                suggestion[app.input.len()..].to_string()
            } else {
                String::new()
            };

            let input_line = Line::from(vec![
                Span::styled("> ", Style::default().fg(Color::White)),
                Span::styled(&app.input, Style::default().fg(Color::White)),
                Span::styled(completion_part, Style::default().fg(Color::DarkGray)),
                Span::styled("_", Style::default().fg(Color::White)),
            ]);
            lines.push(input_line);
        } else {
            // Fallback if index is out of bounds
            lines.push(Line::from(format!("> {}_", app.input)));
        }
    } else {
        // No completion - show normal input
        lines.push(Line::from(format!("> {}_", app.input)));
    }

    // Add spacing
    lines.push(Line::from(""));

    // Add instructions with Tab hint for path completion modes
    if should_show_completion {
        lines.push(Line::from(
            "[Tab] Complete path  [Enter] Confirm  [Esc] Cancel",
        ));
    } else {
        lines.push(Line::from("[Enter] Confirm  [Esc] Cancel"));
    }

    let dialog = Paragraph::new(lines)
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
    let area = centered_rect(60, 80, frame.area());
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
   E         Open in $EDITOR
   l         Link selected
   s         Sync links (bulk)
   u         Unlink selected
   U         Unmanage (move to dest)
   d         Delete from repo
   a         Add new dotfile
   e         Edit destination
   F2        Rename dotfile
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

    // Build list items (show all dotfiles)
    let items: Vec<Line> = app
        .distribute_indices
        .iter()
        .enumerate()
        .map(|(display_idx, &dotfile_idx)| {
            let dotfile = &app.dotfiles[dotfile_idx];
            let selected = app.distribute_selected.contains(&dotfile_idx);
            let checkbox = if selected { "[x]" } else { "[ ]" };
            let cursor = if display_idx == app.distribute_cursor {
                ">"
            } else {
                " "
            };

            let (status_indicator, status_color) = match dotfile.link_status {
                LinkStatus::Conflict => (" [conflict]", Color::Yellow),
                LinkStatus::Broken => (" [broken]", Color::Red),
                _ => ("", Color::Gray),
            };

            let style = if display_idx == app.distribute_cursor {
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
                Span::styled(status_indicator, Style::default().fg(status_color)),
            ])
        })
        .collect();

    let list = Paragraph::new(items).block(
        Block::default()
            .title(" Sync Links - Select dotfiles to be (un)linked ")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(Style::default().bg(Color::Black)),
    );

    frame.render_widget(list, chunks[0]);

    // Help text
    let help = Paragraph::new(" [Space] toggle  [a] all  [n] none  [Enter] apply  [Esc] cancel")
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
