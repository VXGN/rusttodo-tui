use crate::app::{App, InputMode, Tab};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(3),  // Tabs
            Constraint::Min(0),     // Content
            Constraint::Length(3),  // Status/Input
            Constraint::Length(3),  // Help
        ])
        .split(f.area());

    render_header(f, chunks[0]);
    render_tabs(f, chunks[1], app);
    render_content(f, chunks[2], app);
    render_input(f, chunks[3], app);
    render_help(f, chunks[4], app);
}

fn render_header(f: &mut Frame, area: Rect) {
    let header = Paragraph::new("todo-tui")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, area);
}

fn render_tabs(f: &mut Frame, area: Rect, app: &App) {
    let titles = vec!["All", "Active", "Completed"];
    let selected = match app.tab {
        Tab::All => 0,
        Tab::Active => 1,
        Tab::Completed => 2,
    };

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Filter"))
        .select(selected)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(tabs, area);
}

fn render_content(f: &mut Frame, area: Rect, app: &App) {
    if app.input_mode == InputMode::ViewingDetails {
        render_details(f, area, app);
    } else {
        render_list(f, area, app);
    }
}

fn render_list(f: &mut Frame, area: Rect, app: &App) {
    let filtered = app.filtered_todos();

    let items: Vec<ListItem> = filtered
        .iter()
        .enumerate()
        .map(|(i, todo)| {
            let status = if todo.completed { "✓" } else { " " };
            let priority_indicator = match todo.priority {
                crate::todo::Priority::High => "●",
                crate::todo::Priority::Medium => "○",
                crate::todo::Priority::Low => "·",
            };

            let content = Line::from(vec![
                Span::styled(
                    format!("[{}] ", status),
                    Style::default().fg(if todo.completed {
                        Color::Green
                    } else {
                        Color::Gray
                    }),
                ),
                Span::styled(
                    priority_indicator,
                    Style::default().fg(todo.priority.color()),
                ),
                Span::raw(" "),
                Span::styled(
                    &todo.title,
                    Style::default()
                        .fg(if todo.completed { Color::Gray } else { Color::White })
                        .add_modifier(if todo.completed {
                            Modifier::CROSSED_OUT
                        } else {
                            Modifier::empty()
                        }),
                ),
            ]);

            let style = if i == app.selected {
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("Tasks ({}/{})", filtered.len(), app.todos.len())),
    );

    f.render_widget(list, area);
}

fn render_details(f: &mut Frame, area: Rect, app: &App) {
    let filtered = app.filtered_todos();
    if let Some(todo) = filtered.get(app.selected) {
        let text = vec![
            Line::from(vec![
                Span::styled("Title: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&todo.title),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Priority: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(
                    todo.priority.as_str(),
                    Style::default().fg(todo.priority.color()),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(
                    if todo.completed { "Completed" } else { "Active" },
                    Style::default().fg(if todo.completed {
                        Color::Green
                    } else {
                        Color::Yellow
                    }),
                ),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Description:",
                Style::default().add_modifier(Modifier::BOLD),
            )]),
            Line::from(if todo.description.is_empty() {
                "No description"
            } else {
                &todo.description
            }),
            Line::from(""),
            Line::from(vec![
                Span::styled("Created: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(todo.created_at.format("%Y-%m-%d %H:%M").to_string()),
            ]),
        ];

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Details"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }
}

fn render_input(f: &mut Frame, area: Rect, app: &App) {
    match app.input_mode {
        InputMode::Adding => {
            let priorities = ["Low", "Medium", "High"];
            let input_text = format!(
                "Title: {} | Desc: {} | Priority: {} (↑↓ to change)",
                app.input_buffer,
                if app.description_buffer.is_empty() {
                    "<none>"
                } else {
                    &app.description_buffer
                },
                priorities[app.priority_index]
            );

            let input = Paragraph::new(input_text)
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Add Todo (Press Tab to switch fields, Enter to save, Esc to cancel)"));
            f.render_widget(input, area);
        }
        _ => {
            let status = Paragraph::new(format!(
                "Total: {} | Active: {} | Completed: {}",
                app.todos.len(),
                app.todos.iter().filter(|t| !t.completed).count(),
                app.todos.iter().filter(|t| t.completed).count()
            ))
            .block(Block::default().borders(Borders::ALL).title("Status"));
            f.render_widget(status, area);
        }
    }
}

fn render_help(f: &mut Frame, area: Rect, app: &App) {
    let help_text = match app.input_mode {
        InputMode::Normal => {
            "[a]dd [d]elete [Space]toggle [Enter]details [Tab]switch-tab [q]uit"
        }
        InputMode::ViewingDetails => "[Esc]back",
        InputMode::Adding => "[Enter]save [Esc]cancel [Tab]next-field [↑↓]priority",
        _ => "",
    };

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, area);
}