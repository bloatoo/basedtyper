use crate::app::{State, App};
use std::cmp::Ordering;

use tui::{
    backend::Backend,
    layout::{
        Alignment,
        Rect
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
    let wpm = (app.word_string.len() as f64 / 5_f64)
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
        String::from("multiplayer (VERY UNSTABLE)"),
        String::from("quote (UNSTABLE)")
    ];

    for idx in 0..3 {
        let span = if app.current_index - 1 == idx {
            Spans::from(Span::styled(menu_items[idx].clone(), Style::default().fg(Color::Green)))
        } else {
            Spans::from(Span::raw(menu_items[idx].clone()))
        };

        main_menu[chunk.height as usize / 2 + idx] = span;
    }

    

    if app.wordlist.0 {
        main_menu[chunk.height as usize - 2] = Spans::from(Span::raw(format!("wordlist name: {}", app.wordlist.1)));
    }

    if app.host.0 {
        main_menu[chunk.height as usize - 2] = Spans::from(Span::raw(format!("host ip and port: {}", app.host.1)));
    }

    f.render_widget(center(main_menu), chunk);
}

pub fn draw_typing_game<T: Backend>(f: &mut Frame<T>, chunk: Rect, app: &App) {
    let mut ui_text = spans(chunk.height);
    let mut wordlist_string: Vec<Span> = Vec::new();
    
    let mut words: String = app.words
        .iter()
        .map(|word| word.get_word().clone())
        .collect::<Vec<String>>()
        .join(" ");

    if words.len() > app.chunks[0].width as usize {
        words = String::from(&words[..app.chunks[0].width as usize]);
    }

    words = String::from(words.trim_start());


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
        chunk.x + chunk.width / 2 + app.current_index as u16
            - words.len() as u16 / 2,
        chunk.y + chunk.height / 2
    );

    let time_elapsed = if app.timer.is_some() {
        (app.timer.unwrap().elapsed().as_millis() as f64 / 1000.0) / 60.0
    } else {
        0.0
    };

    let wpm = (app.input_string.len() as f64 / 5.0) / time_elapsed;

    let index = app.input_string.split(' ').count() - 1;

    let defs: Vec<&String> = app.words.iter().map(|elem| elem.get_definition()).collect();

    let def = if defs.len() > index {
        defs[index].clone()
    } else {
        String::new()
    };

    ui_text[chunk.height as usize / 4] = Spans::from(Span::raw(format!("WPM: {:.2}", wpm)));
    ui_text[chunk.height as usize / 3] = Spans::from(def);
    ui_text[chunk.height as usize / 2] = Spans::from(wordlist_string);

    f.render_widget(Paragraph::new(ui_text).alignment(Alignment::Center), chunk);
}

pub fn draw_ui<T: Backend>(f: &mut Frame<T>, app: &App) {
    match app.state {
        State::MainMenu => {
            draw_main_menu(f, app.chunks[0].clone(), app);
        }

        State::TypingGame => {
            draw_typing_game(f, app.chunks[0].clone(), app);
        }

        State::EndScreen => {
            draw_end_screen(f, app.chunks[0].clone(), app);
        }

        _ => ()
    }
}
