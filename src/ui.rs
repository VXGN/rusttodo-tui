use crate::app::{AddingField, App, InputMode, Tab};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap, Padding},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // > Header
            Constraint::Length(3),  // > Tabs
            Constraint::Min(0),     // > Content
            Constraint::Length(3),  // > Status/Input
            Constraint::Length(2),  // > Help
        ])
        .split(f.area());

    render_header(f, chunks[0]);
    render_tabs(f, chunks[1], app);
    render_content(f, chunks[2], app);
    render_input(f, chunks[3], app);
    render_help(f, chunks[4], app);
}

fn render_header(f: &mut Frame, area: Rect) {
    let header = Paragraph::new(" TODO TUI ")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .style(Style::default().fg(Color::DarkGray)),
        );
    f.render_widget(header, area);
}

fn render_tabs(f: &mut Frame, area: Rect, app: &App) {
    let titles = [" All ", " Active ", " Completed "];

    let selected = match app.tab {
        Tab::All => 0,
        Tab::Active => 1,
        Tab::Completed => 2,
    };

    let tabs = Tabs::new(
        titles
            .iter()
            .map(|t| Line::styled(*t, Style::default().fg(Color::White)))
            .collect::<Vec<_>>(),
    )
    .select(selected)
    .style(Style::default().fg(Color::Gray))
    .highlight_style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD | Modifier::REVERSED),
    )
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .title(" Filters ")
            .style(Style::default().fg(Color::DarkGray)),
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
            let status = if todo.completed { "✓" } else { "·" };

            let priority_icon = match todo.priority {
                crate::todo::Priority::High => "⬤",
                crate::todo::Priority::Medium => "◉",
                crate::todo::Priority::Low => "○",
            };

            let text = Line::from(vec![
                Span::styled(
                    format!(" {} ", status),
                    Style::default().fg(if todo.completed {
                        Color::Green
                    } else {
                        Color::DarkGray
                    }),
                ),
                Span::styled(
                    priority_icon,
                    Style::default().fg(todo.priority.color()),
                ),
                Span::raw("  "),
                Span::styled(
                    &todo.title,
                    Style::default()
                        .fg(if todo.completed { Color::Gray } else { Color::White })
                        .add_modifier(if todo.completed {
                            Modifier::DIM
                        } else {
                            Modifier::empty()
                        }),
                ),
            ]);

            let style = if i == app.selected {
                Style::default()
                    .bg(Color::Gray)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            ListItem::new(text).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Tasks ({}/{}) ", filtered.len(), app.todos.len()))
                .padding(Padding::horizontal(1)),
        )
        .highlight_symbol("▶ ");

    f.render_widget(list, area);
}

fn render_details(f: &mut Frame, area: Rect, app: &App) {
    let filtered = app.filtered_todos();
    if let Some(todo) = filtered.get(app.selected) {
        let text = vec![
            Line::from(vec![Span::styled(
                &todo.title,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Priority: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(todo.priority.as_str(), Style::default().fg(todo.priority.color())),
            ]),
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
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Details ")
                    .padding(Padding::horizontal(2)),
            )
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }
}

fn render_input(f: &mut Frame, area: Rect, app: &App) {
    match &app.input_mode {
        InputMode::Adding(field) => {
            let priorities = ["Low", "Medium", "High"];

            let (title_style, desc_style) = match field {
                AddingField::Title => (
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                    Style::default().fg(Color::DarkGray),
                ),
                AddingField::Description => (
                    Style::default().fg(Color::DarkGray),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
            };

            let input_lines = vec![
                Line::from(vec![
                    Span::styled("Title:  ", title_style),
                    Span::styled(&app.input_buffer, title_style),
                    Span::styled(if matches!(field, AddingField::Title) { "█" } else { "" }, title_style),
                ]),
                Line::from(vec![
                    Span::styled("Desc:   ", desc_style),
                    Span::styled(&app.description_buffer, desc_style),
                    Span::styled(if matches!(field, AddingField::Description) { "█" } else { "" }, desc_style),
                ]),
                Line::from(vec![
                    Span::styled("Priority: ", Style::default().fg(Color::White)),
                    Span::styled(
                        priorities[app.priority_index],
                        Style::default().fg(match app.priority_index {
                            0 => Color::Green,
                            1 => Color::Yellow,
                            _ => Color::Red,
                        }),
                    ),
                ]),
            ];

            let block = Block::default()
                .borders(Borders::ALL)
                .title(" Add Todo ")
                .padding(Padding::vertical(1));

            f.render_widget(Paragraph::new(input_lines).block(block), area);
        }

        _ => {
            let status = Paragraph::new(format!(
                "Total: {}   Active: {}   Completed: {}",
                app.todos.len(),
                app.todos.iter().filter(|t| !t.completed).count(),
                app.todos.iter().filter(|t| t.completed).count()
            ))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Status ")
                    .padding(Padding::horizontal(1)),
            );
            f.render_widget(status, area);
        }
    }
}

fn render_help(f: &mut Frame, area: Rect, app: &App) {
    let help = match &app.input_mode {
        InputMode::Normal =>
            " a:Add   d:Delete   Space:Toggle   Enter:Details   Tab:Filter   q:Quit ",
        InputMode::ViewingDetails => " Esc:Back ",
        InputMode::Adding(_) => " Tab:Switch   ↑↓:Priority   Enter:Save   Esc:Cancel ",
        _ => "",
    };

    let help_widget = Paragraph::new(help)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::TOP).style(
            Style::default().fg(Color::DarkGray),
        ));

    f.render_widget(help_widget, area);
}
