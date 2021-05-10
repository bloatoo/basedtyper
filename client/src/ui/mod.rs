use crate::app::{State, App};
use std::cmp::Ordering;
use tui::{ backend::Backend, layout::{ Alignment, Rect
    },
    style::{
        Color,
        Modifier,
        Style
    },
    terminal::Frame,
    text::{
        Span,
        Spans
    }, 
    widgets::Paragraph
};

pub mod utils;
pub mod wordlist;

use utils::{center, spans};

pub fn draw_end_screen<T: Backend>(f: &mut Frame<T>, chunk: Rect, app: &App) {
    let wpm = (app.wordlist.to_string().len() as f64 / 5_f64)
        / ((app.time_taken as f64 / 1000_f64) / 60_f64);

    let blue = Style::default().fg(Color::Blue);

    let mut spans: Vec<Spans> = spans(chunk.height);

    for _ in 0..chunk.height / 2 - 3 {
        spans.push(Spans::default());
    }

    let text = vec![
        Spans::from(vec![Span::styled("WPM", blue.add_modifier(Modifier::BOLD)),
            Span::styled(format!(": {:.2}", wpm), blue)]),

        Spans::from(vec![Span::styled("Time used", blue.add_modifier(Modifier::BOLD)),
            Span::styled(format!(": {:.1}s", app.time_taken as f64 / 1000_f64), blue)]),

        Spans::from(vec![Span::raw("")]),

        Spans::from(vec![Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to quit")]),
        Spans::from(vec![Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to restart")]),

        Spans::from(vec![Span::styled("m", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to go to the main menu")])
    ];

    for index in 0..text.len() {
        spans[chunk.height as usize / 2 - 3 + index] = text[index].clone();
    }

    f.render_widget(center(spans), chunk);
}

pub fn draw_main_menu<T: Backend>(f: &mut Frame<T>, chunk: Rect, app: &App) {
    let mut main_menu = spans(chunk.height);

    main_menu[chunk.height as usize / 2 - 2] = Spans::from(Span::styled("basedtyper", Style::default()
        .fg(Color::Magenta)
        .add_modifier(Modifier::BOLD)));

    let menu_items: Vec<String> = vec![
        String::from("wordlist"),
        String::from("multiplayer (UNSTABLE)"),
        String::from("quotes")
    ];

    for idx in 0..3 {
        let span = if app.current_index - 1 == idx {
            Spans::from(Span::styled(menu_items[idx].clone(), Style::default().fg(Color::Green)))
        } else {
            Spans::from(Span::raw(menu_items[idx].clone()))
        };

        main_menu[chunk.height as usize / 2 + idx] = span;
    }

    main_menu[chunk.height as usize - 1] = Spans::from(format!("wordlist directory: {}", app.config.general.wordlist_directory.clone()));

    f.render_widget(center(main_menu), chunk);
}

pub fn draw_waiting_screen<T: Backend>(f: &mut Frame<T>, chunk: Rect, _app: &App) {
    let mut spans = spans(chunk.height);
    spans[chunk.height as usize / 2] = Spans::from(Span::styled("waiting for server...", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)));

    f.render_widget(center(spans), chunk);
}

pub fn draw_host_prompt<T: Backend>(f: &mut Frame<T>, chunk: Rect, app: &App) {
    let mut spans = spans(chunk.height);

    spans[chunk.height as usize / 2] = Spans::from(vec![
        Span::styled("hostname (ip:port): ", Style::default()
                     .fg(Color::Blue)
                     .add_modifier(Modifier::BOLD)), 
        Span::raw(app.input_string.clone())
    ]);

    if !app.current_error.is_empty() {
        spans[chunk.height as usize / 2 + 1] = Spans::from(Span::styled(app.current_error.clone(), Style::default().fg(Color::Red)));
    }

    f.render_widget(center(spans), chunk);
}

pub fn draw_wordlist_prompt<T: Backend>(f: &mut Frame<T>, chunk: Rect, app: &App) {
    let mut spans = spans(chunk.height);

    spans[chunk.height as usize / 2] = Spans::from(vec![
        Span::styled("wordlist name: ", Style::default()
                     .fg(Color::Blue)
                     .add_modifier(Modifier::BOLD)), 
        Span::raw(app.input_string.clone())
    ]);

    if !app.current_error.is_empty() {
        spans[chunk.height as usize / 2 + 1] = Spans::from(Span::styled(app.current_error.clone(), Style::default().fg(Color::Red)));
    }

    f.render_widget(center(spans), chunk);
}

pub fn draw_multiplayer_end_screen<T: Backend>(f: &mut Frame<T>, chunk: Rect, _app: &App) {
    let mut ui_text = spans(chunk.height);

    let spans = vec![
        Spans::from(Span::styled("waiting for server...", Style::default().add_modifier(Modifier::BOLD).fg(Color::Blue))),
        Spans::default(),
        Spans::default(),
        Spans::default(),
        Spans::from(vec![
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to quit")
        ])
    ];

    for (idx, span) in spans.iter().enumerate() {
        ui_text[chunk.height as usize / 2 + idx] = span.clone(); 
    }

    f.render_widget(center(ui_text), chunk);

}

pub fn draw_typing_game<T: Backend>(f: &mut Frame<T>, chunk: Rect, app: &App) {
    let mut ui_text = spans(chunk.height);
    let mut wordlist_string: Vec<Span> = Vec::new();
    
    let mut words: String = app.wordlist.to_string();

    if words.len() > app.chunks[0].width as usize {
        words = String::from(&words[..app.chunks[0].width as usize]);
    }

    let words = words
        .split("")
        .collect::<Vec<&str>>();

    for (index, c) in words.iter().enumerate() {
        match index.cmp(&app.current_index) {
            Ordering::Less => {
                if words[index] != app.input_string.split("").nth(index).unwrap() {
                     wordlist_string.push(Span::styled(*c, Style::default().bg(Color::Red)));
                } else {
                     wordlist_string.push(Span::styled(*c, Style::default().fg(Color::DarkGray)));
                }
            }

            _ => wordlist_string.push(Span::styled(*c, Style::default())),
        }
    }

    f.set_cursor(
        chunk.x + (chunk.width as f32 / 2.0).ceil() as u16 + app.current_index as u16
            - (words.len() as f32 / 2.0).ceil() as u16,
        chunk.y + chunk.height / 2
    );

    let time_elapsed = if app.timer.is_some() {
        (app.timer.unwrap().elapsed().as_millis() as f64 / 1000.0) / 60.0
    } else {
        0.0
    };

    let wpm = (app.input_string.len() as f64 / 5.0) / time_elapsed;

    let index = app.input_string.split(' ').count() - 1;

    let defs: Vec<String> = app.wordlist.defs();

    let def = if defs.len() > index {
        defs[index].clone()
    } else {
        String::new()
    };

    ui_text[chunk.height as usize / 4] = Spans::from(Span::raw(format!("WPM: {:.2}", wpm)));
    ui_text[chunk.height as usize / 3] = Spans::from(def);
    ui_text[chunk.height as usize / 2] = Spans::from(wordlist_string);

    if app.connection.enabled {
        app.connection.players.iter().enumerate().for_each(|(idx, player)| {
            ui_text[chunk.height as usize / 2 + idx] = Spans::from(Span::raw(player.username.clone()));
            ui_text[chunk.height as usize / 2 + idx * 2] = Spans::from(Span::raw("-".repeat(player.pos)));
        });
    }

    f.render_widget(Paragraph::new(ui_text).alignment(Alignment::Center), chunk);
}

pub fn draw_ui<T: Backend>(f: &mut Frame<T>, app: &App) {
    match app.state {
        State::MainMenu => draw_main_menu(f, app.chunks[0], app),
        State::TypingGame => draw_typing_game(f, app.chunks[0], app),
        State::EndScreen => draw_end_screen(f, app.chunks[0], app),
        State::Waiting => draw_waiting_screen(f, app.chunks[0], app),
        State::HostPrompt => draw_host_prompt(f, app.chunks[0], app),
        State::WordlistPrompt => draw_wordlist_prompt(f, app.chunks[0], app),
        State::MultiplayerEndScreen => draw_multiplayer_end_screen(f, app.chunks[0], app),
    }
}
