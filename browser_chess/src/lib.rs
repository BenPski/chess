mod utils;

use js_sys::JsString;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::console_log;
use std::{f64, cell::RefCell, rc::Rc};
use enum_iterator::all;

use dumb_chess::{strategy::{*, self}, game::ChessGame, action::Action, player::Player, final_state::FinalState};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    alert(&format!("sup {}", s));
}

/*
 * Instead of re-exporting and binding those use websys to handle all the interactions
 * that were inevitably going to be handled in javascript anyways
 *
 * Idea is to have a canvas that renders the chess board, then the controls are:
 * - selecting the algorithms for the different players
 * - play the game with some default speed (maybe configurable)
 * - maybe have some control over stepping
 * - reset the board
 */

use Player::*;
use FinalState::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct GameState {
    turn: Player,
    canvas: web_sys::HtmlCanvasElement,
    status: web_sys::HtmlElement,
    white_select: web_sys::HtmlSelectElement,
    black_select: web_sys::HtmlSelectElement,
    white: Strategy,
    black: Strategy,
    game: ChessGame,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum State {
    Playing,
    Draw,
    WinWhite,
    WinBlack,
}

impl From<Option<FinalState>> for State {
    fn from(value: Option<FinalState>) -> Self {
        match value {
            None => State::Playing,
            Some(Draw) => State::Draw,
            Some(Win(White)) => State::WinWhite,
            Some(Win(Black)) => State::WinBlack,
        }       
    }
}

#[wasm_bindgen]
impl GameState {
    pub fn new(canvas: web_sys::HtmlCanvasElement,
               status: web_sys::HtmlElement,
               white_select: web_sys::HtmlSelectElement,
               black_select: web_sys::HtmlSelectElement,
               black: Strategy,
               white: Strategy)
        -> Self {
        GameState { turn: Player::White, canvas, status, white_select, black_select, white, black , game: ChessGame::new() }
    }
    
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        let white = get_selection(&self.white_select);
        let black = get_selection(&self.black_select);
        self.white = white;
        self.black = black;
        self.turn = White;
        self.game = ChessGame::new();
    }

    pub fn step(&mut self) -> State {
        if let Some(act) = 
            match self.turn {
                White => {
                    self.white.run(&self.game).into()
                }
                Black => {
                    self.black.run(&self.game).into()
                }
            }
        {
            self.game = self.game.step(act);
            self.turn = self.turn.toggle();
            self.game.check_state().into()
        } else {
            Some(Draw).into()
        }
    }

    pub fn render(&self) {
        render_board(&self.canvas, &self.game)
    }

    #[wasm_bindgen]
    pub fn loop_step(&mut self) -> bool {
        let state = self.step();
        self.render();
        match state {
            State::Playing => {
                if self.turn == White {
                    self.status.set_text_content(Some("White's turn"));
                } else { 
                    self.status.set_text_content(Some("Black's turn"));
                }
                false
            }
            State::Draw => {
                self.status.set_text_content(Some("Game over: Draw"));
                true
            },
            State::WinWhite => {
                self.status.set_text_content(Some("White wins"));
                true
            },
            State::WinBlack => {
                self.status.set_text_content(Some("Black wins"));
                true
            },
        }
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

#[wasm_bindgen]
pub fn setup(canvas: web_sys::HtmlCanvasElement,
           status: web_sys::HtmlElement,
           white_select: web_sys::HtmlSelectElement,
           black_select: web_sys::HtmlSelectElement,
           descriptions: web_sys::HtmlElement)
    -> GameState {
    
    let _ = setup_descriptions(descriptions);
    let _ = setup_select(&white_select);
    let _ = setup_select(&black_select);
    let game = GameState::new(canvas,
                              status,
                              white_select,
                              black_select,
                              Strategy::Random,
                              Strategy::Random);
   game 
}


fn setup_select(select: &web_sys::HtmlSelectElement) -> Result<(), JsValue> {
    let window = web_sys::window().expect("No global window");
    let document = window.document().expect("Windows should have a document");

    for strategy in all::<Strategy>() {
        let option = document.create_element("option")?;
        option.set_text_content(Some(strategy.name()));
        let _ = select.append_child(&option);
    }
    Ok(())
}

fn get_selection(select: &web_sys::HtmlSelectElement) -> Strategy {
    let name = select.value();
    strategy_map().get(&name).unwrap().clone()
}

fn setup_descriptions(descriptions: web_sys::HtmlElement) -> Result<(), JsValue> {
    let document = window().document().expect("Windows should have a document");

    let table = document.create_element("table")?;
    for strategy in all::<Strategy>() {
        let row = document.create_element("tr")?;
        let name_cell = document.create_element("td")?;
        let desc_cell = document.create_element("td")?;
        name_cell.set_text_content(Some(strategy.name()));
        desc_cell.set_text_content(Some(strategy.description()));
        let _ = row.append_child(&name_cell);
        let _ = row.append_child(&desc_cell);
        let _ = table.append_child(&row);
    }
    let _ = descriptions.append_child(&table);
    Ok(())
}

fn render_board(canvas: &web_sys::HtmlCanvasElement, game: &ChessGame) {
    let grid_size = 50;
    canvas.set_width(8*grid_size);
    canvas.set_height(8*grid_size);
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    for row in 0..8 {
        for col in 0..8 {
            let color = if (row + col) % 2 == 0 {
                JsValue::from_str("#FFCE9E")
            } else {
                JsValue::from_str("#D18B47")
            };
            context.set_fill_style(&color);
            context.fill_rect((row*grid_size) as f64, (col*grid_size) as f64, grid_size as f64, grid_size as f64);
            context.fill();
        }
    }

    context.set_font("50px Arial");
    context.set_fill_style(&JsValue::from_str("black"));
    for row in 0..8 {
        for col in 0..8 {
            let piece = game.board.get((row as i32, col as i32).into());
            let _ = context.fill_text(&format!("{}", piece), ((col)*grid_size) as f64, ((row+1)*grid_size-5) as f64);
        }
    }
}
