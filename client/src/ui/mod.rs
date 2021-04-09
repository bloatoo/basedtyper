use crate::app::{State, App};
use std::cmp::Ordering;

use tui::{
    backend::Backend,
    layout::{
        Alignment,
        Constraint,
        Direction,
        Layout,
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

pub fn spans(size: u16) -> Vec<Spans<'static>> {
    let mut v = Vec::new();

    for _ in 0..size {
        v.push(Spans::default());    
    }

    v
}

pub fn center(spans: Vec<Spans<'static>>) -> Paragraph<'static> {
    Paragraph::new(spans).alignment(Alignment::Center)
}

pub fn draw_main_menu<T: Backend>(f: &mut Frame<T>, chunk: Rect, app: &App) {
    let mut main_menu = spans(chunk.height);

    main_menu[chunk.height as usize / 2 - 5] = Spans::from(Span::styled("basedtyper", Style::default()
        .fg(Color::Magenta)
        .add_modifier(Modifier::BOLD)));


    let menu_items: Vec<String> = vec![
        String::from("           wordlist          "),
        String::from(" multiplayer (VERY UNSTABLE) "),
        String::from("       quote (UNSTABLE)      ")
    ];

    for idx in 0..3 {
        let span = if app.current_index - 1 == idx {
            Spans::from(Span::styled(menu_items[idx].clone(), Style::default().fg(Color::Green)))
        } else {
            Spans::from(Span::raw(menu_items[idx].clone()))
        };

        main_menu[chunk.height as usize / 2 + idx] = span;
    }

    f.render_widget(center(main_menu), chunk);
}

pub fn draw_typing_game<T: Backend>(f: &mut Frame<T>, chunk: Rect, app: &App) {
    let mut ui_text = spans(chunk.height);
    let mut wordlist_string: Vec<Span> = Vec::new();
    
    let words: String = app.words
        .iter()
        .map(|word| word.get_word().clone())
        .collect::<Vec<String>>()
        .join(" ");

    let words = &words[..app.chunks[0].width as usize];

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
            - wordlist_string.len() as u16 / 2,
        chunk.y + chunk.height / 2);

    let wpm = (app.input_string.len() as f64 / 5_f64)
        / (if app.timer.is_some() { (app.timer.unwrap().elapsed().as_millis() as f64 / 1000_f64) / 60_f64 } else { 0_f64 });

    ui_text[chunk.height as usize / 2 - chunk.height as usize / 3] = Spans::from(Span::raw(format!("WPM: {:.2}", wpm)));
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

        _ => ()
    }
}
