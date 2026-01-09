use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, BorderType},
    Frame,
};
use crate::state::AppState;

const SKY_BLUE: Color = Color::Rgb(189, 231, 244);
const SLATE: Color = Color::Rgb(111, 123, 145);
const CHARCOAL: Color = Color::Rgb(28, 28, 28);
const NEON_GREEN: Color = Color::Rgb(0, 255, 127);
const DIM_GRAY: Color = Color::Rgb(60, 60, 60);

// Braille Art (Hero) - 88 chars wide
const HERO_ART: &[&str] = &[
    "⣿⣿⣿⣿⣿⣷⣿⣿⣿⡅⡹⢿⠆⠙⠋⠉⠻⠿⣿⣿⣿⣿⣿⣿⣮⠻⣦⡙⢷⡑⠘⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣌⠡⠌⠂⣙⠻⣛⠻⠷⠐⠈⠛⢱⣮⣷⣽⣿",
    "⣿⣿⣿⣿⡇⢿⢹⣿⣶⠐⠁⠀⣀⣠⣤⠄⠀⠀⠈⠙⠻⣿⣿⣿⣦⣵⣌⠻⣷⢝⠦⠚⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢟⣻⣿⣊⡃⠀⣙⠿⣿⣿⣿⣎⢮⡀⢮⣽⣿⣿",
    "⢿⣿⣿⣿⣧⡸⡎⡛⡩⠖⠀⣴⣿⣿⣿⠀⠀⠀⠀⠸⠇⠀⠙⢿⣿⣿⣿⣷⣌⢷⣑⢷⣄⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⣫⠶⠛⠉⠀⠁⠀⠈⠈⠀⠠⠜⠻⣿⣆⢿⣼⣿⣿⣿",
    "⢐⣿⣿⣿⣿⣧⢧⣧⢻⣦⢀⣹⣿⣿⣿⣇⠀⠄⠀⠀⠀⡀⠀⠈⢻⣿⣿⣿⣿⣷⣝⢦⡹⠷⡙⢿⣿⣿⣿⣿⣿⣿⣿⣿⠈⠁⠀⠀⠀⠁⠀⠀⠀⠱⣶⣄⡀⠀⠈⠛⠜⣿⣿⣿⣿",
    "⠀⠊⢫⣿⣏⣿⡌⣼⣄⢫⡌⣿⣿⣿⣿⣿⣦⡈⠲⣄⣤⣤⡡⢀⣠⣿⣿⣿⣿⣿⣿⣷⣼⣍⢬⣦⡙⣿⣿⣿⣿⣿⣯⢁⡄⠀⡀⡀⠀⠄⢈⣠⢪⠀⣿⣿⣿⣦⠀⢉⢂⠹⡿⣿⣿",
    "⠀⠀⠄⢹⢃⢻⣟⠙⣿⣦⠱⢻⣿⣿⣿⣿⣿⣿⣷⣬⣍⣭⣥⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⡙⢿⣼⡿⣿⣿⣿⣿⣿⣷⣄⠘⣱⢦⣤⡴⡿⢈⣼⣿⣿⣿⣇⣴⣶⣮⣅⢻⣿⡏",
    "⠀⠀⠈⠹⣇⢡⢿⡆⠻⣿⣷⠀⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣍⡻⣿⣟⣻⣿⣿⣿⣿⣷⣦⣥⣬⣤⣴⣾⣿⣿⣿⣿⣷⣿⣿⣿⣿⣷⡜⠃",
    "⠀⠀⠀⢀⣘⠈⢂⠃⣧⡹⣿⣷⡄⠙⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣮⣅⡙⢿⣟⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠋⡕⠂",
    "⠀⠀⠀⠀⠀⠀⠛⢷⣜⢷⡌⠻⣿⣿⣦⣝⣻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣯⣹⣷⣦⣹⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠉⠃⠀",
];

// OCC Cassette Art (Logo) - ~38 chars wide
const SIDEBAR_LOGO: &[&str] = &[
    "⠀⠀⠀⠀⢀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⡀⠀⠀⠀⠀",
    "⠀⠀⠀⠀⣿⡿⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⢿⣿⠀⠀⠀⠀",
    "⠀⠀⠀⠀⣿⣿⡏⠀⠤⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⠀⠀⠀⠀",
    "⠀⠀⠀⠀⣿⣿⡇⠀⠀⢀⣤⡄⠀⠀⣤⣤⠀⠀⢠⣤⡀⠀⠀⢸⣿⣿⠀⠀⠀⠀",
    "⠀⠀⠀⠀⣿⣿⡇⠀⢸⡇⠀⠀⡇⣿⠀⠀⠉⢸⡇⠀⠈⠁⠀⢸⣿⣿⠀⠀⠀⠀",
    "⠀⠀⠀⠀⣿⣷⡇⠀⠸⢇⣀⣀⠇⠿⣀⣀⠤⠸⢇⣀⡠⠄⠀⢸⣿⣿⠀⠀⠀⠀",
    "⠀⠀⠀⠀⣿⣷⡇⠀⠀⠈⠉⠁⠀⠀⠉⠉⠀⠀⠈⠉⠁⠀⠀⢸⣿⣿⠀⠀⠀⠀",
    "⠀⠀⠀⠀⣿⡿⣧⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣼⣻⣿⠀⠀⠀⠀",
    "⠀⠀⠀⠀⣿⣿⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣿⣿⠀⠀⠀⠀",
    "⠀⠀⠀⠀⣿⣿⠯⠭⠭⠭⠭⠭⠽⣿⣿⣿⣿⣿⣿⡿⠿⣿⢟⡻⣿⣿⠀⠀⠀⠀",
    "⠀⠀⠀⠀⣿⣿⣭⣭⣭⣭⣭⣭⣭⣿⣿⣿⣿⣿⣿⣿⣶⣿⣮⣵⣿⣿⠀⠀⠀⠀",
    "⠀⠀⠀⠀⠈⠉⠉⣭⣭⣭⣭⣭⣭⣭⣭⣭⣭⣭⣭⣭⣭⣭⣭⠉⠉⠁⠀⠀⠀⠀",
    "⠀⠀⠀⠀⠀⠀⠀⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠀⠀⠀⠀⠀⠀⠀",
];

fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let x = if r.width > width { r.x + (r.width - width) / 2 } else { r.x };
    let y = if r.height > height { r.y + (r.height - height) / 2 } else { r.y };
    let w = width.min(r.width);
    let h = height.min(r.height);
    Rect::new(x, y, w, h)
}

fn draw_bar(label: &str, value: u16, max: u16, color: Color) -> Line {
    let width = 20;
    let filled = (value as f32 / max as f32 * width as f32) as usize;
    let bar = format!("{}{}", "█".repeat(filled), "░".repeat(width - filled));
    Line::from(vec![
        Span::styled(format!("{:<8}", label), Style::default().fg(SLATE)),
        Span::styled(bar, Style::default().fg(color)),
        Span::styled(format!(" {:>3}%", value), Style::default().fg(color)),
    ])
}

pub fn ui(frame: &mut Frame, app: &AppState) {
    let area = frame.area();
    let qr_height = if let Some(ref m) = app.qr_matrix { (m.len() + 1) / 2 } else { 0 };

    frame.render_widget(Block::default().style(Style::default().bg(CHARCOAL)), area);

    // Centered 125x60 canvas
    let size = centered_rect(125, 60, area);

    // Main Vertical: [ HERO | CONTENT | STATUS ]
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(HERO_ART.len() as u16 + 2),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(size);

    // --- TOP: HERO ART ---
    let hero_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(SKY_BLUE))
        .title(Span::styled(" OPEN COMPUTING CLUB // NEURAL_LINK ", Style::default().fg(CHARCOAL).bg(SKY_BLUE).add_modifier(Modifier::BOLD)));
    let mut hero_content = vec![];
    for line in HERO_ART {
        hero_content.push(Line::from(Span::styled(*line, Style::default().fg(SLATE))));
    }
    let hero_inner = hero_block.inner(main_layout[0]);
    frame.render_widget(hero_block, main_layout[0]);
    frame.render_widget(Paragraph::new(hero_content).alignment(Alignment::Center), hero_inner);

    // --- MIDDLE: SIDEBAR & MAIN POSTER ---
    let content_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(46), // Sidebar width
        ])
        .split(main_layout[1]);

    // --- LEFT: MAIN POSTER AREA (QR + STATS) ---
    let poster_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),    // QR Code
            Constraint::Length(12), // System Stats (btop style)
        ])
        .split(content_split[0]);

    // 1. QR BOX
    let qr_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(SKY_BLUE))
        .title(Span::styled(" JAN 2026 RECRUIT HUB ", Style::default().bg(SKY_BLUE).fg(CHARCOAL).add_modifier(Modifier::BOLD)));
    let qr_inner = qr_block.inner(poster_layout[0]);
    frame.render_widget(qr_block, poster_layout[0]);

    let qr_v_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(qr_height as u16),
            Constraint::Fill(1),
            Constraint::Length(4),
        ])
        .split(qr_inner);

    let mut qr_content = vec![];
    if let Some(ref matrix) = app.qr_matrix {
        for y in (0..matrix.len()).step_by(2) {
            let mut line = String::new();
            for x in 0..matrix.len() {
                let top = matrix[y][x];
                let bottom = if y + 1 < matrix.len() { matrix[y + 1][x] } else { false };
                let block = match (top, bottom) {
                    (true, true) => '█', (true, false) => '▀', (false, true) => '▄', (false, false) => ' ',
                };
                line.push(block);
            }
            qr_content.push(Line::from(Span::styled(line, Style::default().fg(SKY_BLUE))));
        }
    }
    frame.render_widget(Paragraph::new(qr_content).alignment(Alignment::Center), qr_v_layout[1]);

    let cta_text = vec![
        Line::from("JAN 2026 RECRUIT IS OPEN!"),
        Line::from(""),
        Line::from(Span::styled(">> Join to take AI's job. <<", Style::default().fg(NEON_GREEN).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(vec![
            Span::styled("TARGET: ", Style::default().fg(SLATE)),
            Span::styled(app.url.to_uppercase(), Style::default().fg(SKY_BLUE).add_modifier(Modifier::UNDERLINED)),
        ]),
    ];
    frame.render_widget(Paragraph::new(cta_text).alignment(Alignment::Center), qr_v_layout[3]);

    // 2. BTOP STATS BOX
    let stats_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(SLATE))
        .title(Span::styled(" SYSTEM_DIAGNOSTICS ", Style::default().fg(SLATE)));
    let stats_inner = stats_block.inner(poster_layout[1]);
    frame.render_widget(stats_block, poster_layout[1]);

    let stats_text = vec![
        Line::from(""),
        draw_bar("CPU_LOAD", 84, 100, SKY_BLUE),
        draw_bar("MEM_USAGE", 62, 100, NEON_GREEN),
        draw_bar("NET_LINK", 99, 100, SKY_BLUE),
        Line::from(""),
        Line::from(vec![
            Span::styled("KERNEL: ", Style::default().fg(SLATE)),
            Span::styled("RUST_OS_V4.2 ", Style::default().fg(SKY_BLUE)),
            Span::styled("THREADS: ", Style::default().fg(SLATE)),
            Span::styled("128_ACTIVE", Style::default().fg(NEON_GREEN)),
        ]),
    ];
    frame.render_widget(Paragraph::new(stats_text).alignment(Alignment::Center), stats_inner);

    // --- RIGHT: SIDEBAR (SEGREGATED BOXES) ---
    let sidebar_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(15), // Logo Box
            Constraint::Length(14), // Recruitment Details Box
            Constraint::Min(0),     // Missions Box
        ])
        .split(content_split[1]);

    // 1. LOGO BOX
    let logo_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(SKY_BLUE))
        .title(Span::styled(" OCC_BRAND ", Style::default().fg(SKY_BLUE)));
    let mut logo_content = vec![];
    for line in SIDEBAR_LOGO {
        logo_content.push(Line::from(Span::styled(*line, Style::default().fg(SKY_BLUE))));
    }
    frame.render_widget(Paragraph::new(logo_content).alignment(Alignment::Center).block(logo_block), sidebar_layout[0]);

    // 2. RECRUITMENT DETAILS (Left Aligned)
    let details_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(SLATE))
        .title(Span::styled(" OBJECTIVES ", Style::default().fg(SLATE)))
        .padding(ratatui::widgets::Padding::new(2, 1, 0, 0));
    let details_text = vec![
        Line::from(""),
        Line::from(Span::styled("WHO WE ARE", Style::default().fg(SKY_BLUE).add_modifier(Modifier::BOLD))),
        Line::from("-------"),
        Line::from("• Level 10 Gyatt Coders"),
        Line::from("• Rizzing the Borrow Checker"),
        Line::from(""),
        Line::from(Span::styled("VIBE_CHECK", Style::default().fg(SKY_BLUE).add_modifier(Modifier::BOLD))),
        Line::from("-------"),
        Line::from("• Error 500 internal server error"),
        Line::from("• [Message has been blocked due to spam]"),
    ];
    frame.render_widget(Paragraph::new(details_text).alignment(Alignment::Left).block(details_block), sidebar_layout[1]);

    // 3. ZZZ MISSIONS BOX
    let missions_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(SLATE))
        .title(Span::styled(" ACTIVE_COMMISSIONS ", Style::default().fg(SLATE)))
        .padding(ratatui::widgets::Padding::new(2, 1, 0, 0));
    let missions_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(" [!] ", Style::default().fg(NEON_GREEN)),
            Span::styled("SHOW_OFF_YOUR_SHI-[REDACTED]", Style::default().fg(SKY_BLUE)),
        ]),
        Line::from(vec![
            Span::styled(" [!] ", Style::default().fg(NEON_GREEN)),
            Span::styled("BUILD_DEEZ_NUTZ", Style::default().fg(SKY_BLUE)),
        ]),
        Line::from(vec![
            Span::styled(" [!] ", Style::default().fg(NEON_GREEN)),
            Span::styled("HAVE_A_CHANCE_AT_HOMELABBING!", Style::default().fg(SKY_BLUE)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(" PROGRESS: ", Style::default().fg(SLATE)),
            Span::styled("A progress bar", Style::default().fg(SKY_BLUE)),
        ]),
    ];
    frame.render_widget(Paragraph::new(missions_text).alignment(Alignment::Left).block(missions_block), sidebar_layout[2]);

    // --- BOTTOM: STATUS ---
    let status_text = Line::from(vec![
        Span::styled(" q ", Style::default().bg(SLATE).fg(CHARCOAL).add_modifier(Modifier::BOLD)),
        Span::styled(" quit ", Style::default().fg(SLATE)),
        Span::raw(" "),
        Span::styled(" r ", Style::default().bg(SLATE).fg(CHARCOAL).add_modifier(Modifier::BOLD)),
        Span::styled(" refresh ", Style::default().fg(SLATE)),
        Span::raw(" | "),
        Span::styled(" [KERNEL_READY] ", Style::default().fg(NEON_GREEN)),
        Span::raw(" | "),
        Span::styled(" JAN 2026 RECRUIT ", Style::default().fg(SKY_BLUE).add_modifier(Modifier::BOLD)),
    ]);
    frame.render_widget(Paragraph::new(status_text).alignment(Alignment::Right), main_layout[2]);
}
