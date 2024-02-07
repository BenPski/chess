use core::f32;
use std::collections::HashMap;

use enum_iterator::{Sequence, all};
use rand::{thread_rng, seq::SliceRandom};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::piece::{PieceData, ID};
use crate::player::{Player};
use crate::{game::ChessGame, action::Action};
use crate::piece::Piece::{*};

use Action::*;
// general implementation is
// try to make an action
// no action -> lost the game
// so Fn(&ChessGame) -> Option<Action>

// quite a few of these could have mulitple steps of lookahead, but for now just
// going to do specific amounts of lookahead for each thing or one step

trait EvalGame {
    // evaluate the action and the resulting game
    fn eval(&self, action: Action, game: &ChessGame) -> f32;

    // often just 0, but maybe a reason to have something different
    fn no_moves(&self) -> f32 {
        0.0
    }
}

trait ChooseMove {
    // given the values computed for the top level actions pick the actions
    // examples would be the max value or min value
    fn choose_move(&self, left: (f32, Action), right: (f32, Action)) -> (f32, Action);
}

trait CombineValues {
    // given all computed values for the lower level actions combine them
    // examples would be the max, min, average, deviation
    fn combine_values(&self, values: Vec<f32>) -> f32;
}


// a simple lookahead strategy that only considers the given players moves for 
// simplicities sake and because the other player's strategy is unknown
fn strategy(depth: u8, player: Player, game: &ChessGame, eval: &impl EvalGame, choose: &impl ChooseMove, combine: &impl CombineValues) -> Option<Action> {
    let mut moves: Vec<Action> = game.possible_moves(player).collect();
    moves.shuffle(&mut thread_rng());
    if moves.is_empty() {
        None
    } else {
        let mut choice = (strategy_lookahead(depth-1, player, moves[0], game, eval, combine), moves[0]);
        for m in &moves[1..] {
            let res = (strategy_lookahead(depth-1, player, *m, game, eval, combine), *m);
            
            choice = choose.choose_move(choice, res);
        }
        Some(choice.1)
    }
}


fn strategy_lookahead(depth: u8, player: Player, act: Action, game: &ChessGame, eval: &impl EvalGame, combine: &impl CombineValues) -> f32 {
    let g = game.step(act);
    if depth == 0 {
        eval.eval(act, &g)
    } else {
        let mut moves: Vec<Action> = g.possible_moves(player).collect();
        moves.shuffle(&mut thread_rng());
        if moves.is_empty() {
            eval.no_moves()
        } else {
            let values : Vec<f32> = moves.into_iter()
                .map(|m| strategy_lookahead(depth-1, player, m, &g, eval, combine)).collect();
            combine.combine_values(values)
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Sequence)]
pub enum Strategy {
    Random,
    GiveUp,
    Swarm,
    Huddle,
    SmellyKing,
    Intimidated,
    Ape,
    Sleepy,
    Pacifist,
    EqualOpportunity,
    Momentum,
    Prepared,
    Lawyer,
    Criminal,
    Paralegal,
    UndercoverCop,
    // can't do these two very well right now, counting pieces without the opponents
    // moves just means all the game states are the same
    //Hoarder,
    //Monk,
    DrunkKing,
    Polite,
    ElderlyKing,
    Shutdown,
    LadiesFirst,
    Offensive,
    Defensive,
}

use Strategy::*;

pub fn strategy_map() -> HashMap<String, Strategy> {
    let mut map = HashMap::new();
    for strategy in all::<Strategy>() {
        map.insert(strategy.name().to_string(), strategy);
    }
    map
}

impl Strategy {
    pub fn name(&self) -> &'static str {
        match self {
            Random           => "Random",
            Swarm            => "Swarm",
            GiveUp           => "Give up",
            Huddle           => "Huddle",
            SmellyKing       => "Smelly King",
            Intimidated      => "Intimidated",
            Ape              => "Ape",
            Sleepy           => "Sleepy",
            Pacifist         => "Pacifist",
            EqualOpportunity => "Equal opportunity",
            Momentum         => "Momentum",
            Prepared         => "Prepared",
            Lawyer           => "Lawyer",
            Criminal         => "Criminal",
            Paralegal        => "Paralegal",
            UndercoverCop    => "Undercover cop",
            //Hoarder          => "Hoarder",
            //Monk             => "Monk",
            DrunkKing        => "Drunk King",
            Polite           => "Polite",
            ElderlyKing      => "Elderly King",
            Shutdown         => "Shutdown",
            LadiesFirst      => "Ladies first",
            Offensive        => "Offensive",
            Defensive        => "Defensive",
        }
    }
    pub fn description(&self) -> &'static str {
        match self {
            Random           => "Choose moves randomly.",
            Swarm            => "Minimizes the total distance between all of their pieces and the enemy king.",
            GiveUp           => "Chess isn't really their thing",
            Huddle           => "Keep your pieces as close to your king as possible",
            SmellyKing       => "Your king stinks, all your pieces run away",
            Intimidated      => "The other king is quite scary, all your pieces run away from him",
            Ape              => "Big moves are the only way to get to the moon.",
            Sleepy           => "Too sleepy to move very far.",
            Pacifist         => "If I don't attack maybe they won't either.",
            EqualOpportunity => "Everyone needs a chance to succeed, keeps the move count for each piece relatively balanced.",
            Momentum         => "Stick to the choices you have made, the piece that has moved the most keeps getting chosen",
            Prepared         => "Always be ready to attack. Note, attacking makes you less ready to attack.",
            Lawyer           => "Maximize your options",
            Criminal         => "Minimize your options",
            Paralegal        => "Maximize the opponents options",
            UndercoverCop    => "Minimize the opponents options",
            //Hoarder          => "You never know when a piece will be important.",
            //Monk             => "Detach from your pieces to achieve oneness with everything.",
            DrunkKing        => "The King breaks loose and stumbles about.",
            Polite           => "Let the other King move as much as possible.",
            ElderlyKing      => "The King can hardly move without his walker.",
            Shutdown         => "Keep the other king from moving.",
            LadiesFirst      => "Let the queen do what she wants.",
            Offensive        => "Try to put the opponent in check often.",
            Defensive        => "Avoid being in check.",
        }
    }
    pub fn run(&self, game: &ChessGame) -> Option<Action> {
        match self {
            Random           => {
                let moves: Vec<Action> = game.possible_moves(game.turn).collect();
                if !moves.is_empty() {
                    Some(*moves.choose(&mut thread_rng()).unwrap())
                } else {
                    None
                }
            },
            Swarm => {
                let player = game.turn;
                strategy(1, player, game, &KingDistance(player, player.toggle()), &MinChoose, &MinCombine)
            },
            GiveUp => {
                None
            },
            Huddle => {
                let player = game.turn;
                strategy(1, player, game, &KingDistance(player, player), &MinChoose, &MinCombine)
            },
            SmellyKing => {
                let player = game.turn;
                strategy(1, player, game, &KingDistance(player, player), &MaxChoose, &MaxCombine)
            },
            Intimidated => {
                let player = game.turn;
                strategy(1, player, game, &KingDistance(player, player.toggle()), &MaxChoose, &MaxCombine)
            },
            Ape => {
                strategy(1, game.turn, game, &BigPlays, &MaxChoose, &MaxCombine)
            },
            Sleepy => {
                strategy(1, game.turn, game, &BigPlays, &MinChoose, &MinCombine)
            },
            Pacifist => {
                strategy(1, game.turn, game, &Attacks, &MinChoose, &SumCombine)
            },
            EqualOpportunity => {
                strategy(1, game.turn, game, &CountMoves, &MinChoose, &MinCombine)
            },
            Momentum => {
                strategy(1, game.turn, game, &CountMoves, &MaxChoose, &MaxCombine)
            },
            Prepared => {
                strategy(1, game.turn, game, &Attacks, &MaxChoose, &SumCombine)
            },
            Lawyer => {
                let player = game.turn;
                strategy(1, player, game, &MoveAmount(player), &MaxChoose, &MaxCombine)
            }
            Criminal => {
                let player = game.turn;
                strategy(1, player, game, &MoveAmount(player), &MinChoose, &MinCombine)
            }
            Paralegal => {
                let player = game.turn;
                strategy(1, player, game, &MoveAmount(player.toggle()), &MaxChoose, &MaxCombine)
            }
            UndercoverCop => {
                let player = game.turn;
                strategy(1, player, game, &MoveAmount(player.toggle()), &MinChoose, &MinCombine)
            }
            DrunkKing => {
                strategy(1, game.turn, game, &KingMoves(game.turn), &MaxChoose, &SumCombine)
            }
            Polite => {
                strategy(1, game.turn, game, &KingMoves(game.turn.toggle()), &MaxChoose, &SumCombine)
            }
            ElderlyKing => {
                strategy(1, game.turn, game, &KingMoves(game.turn), &MinChoose, &SumCombine)
            }
            Shutdown => {
                strategy(1, game.turn, game, &KingMoves(game.turn.toggle()), &MinChoose, &SumCombine)
            }
            LadiesFirst => {
                strategy(1, game.turn, game, &QueenMoves, &MaxChoose, &SumCombine)
            }
            Offensive => {
                strategy(2, game.turn, game, &InCheck(game.turn.toggle()), &MaxChoose, &MaxCombine)
            }
            Defensive => {
                strategy(2, game.turn, game, &InCheck(game.turn), &MinChoose, &MinCombine)
            }
        }
    }
}

// since there is a default sort, just use that sort
pub fn sorted_player(game: &ChessGame) -> Option<Action> {
    let mut moves: Vec<Action> = game.possible_moves(game.turn).collect();
    if !moves.is_empty() {
        moves.sort();
        Some(moves[0])
    } else {
        None
    }
}

pub fn rev_sorted_player(game: &ChessGame) -> Option<Action> {
    let mut moves: Vec<Action> = game.possible_moves(game.turn).collect();
    if !moves.is_empty() {
        moves.sort();
        moves.reverse();
        Some(moves[0])
    } else {
        None
    }
}

// cheat
// always promote
// randomly swap pieces
// switch with opponent pieces
// do illegal moves

struct KingDistance(Player, Player);
struct BigPlays;
struct Attacks;
struct CountMoves;
struct MoveAmount(Player);
struct KingMoves(Player);
struct QueenMoves;
struct InCheck(Player);

impl EvalGame for KingDistance {
    fn eval(&self, _action: Action, game: &ChessGame) -> f32 {
        let player = self.0;
        let interested = self.1;
        king(game, interested)
            .map_or(0.0, |k| total_distance(game, player, k) as f32)
    }
}

impl EvalGame for BigPlays {
    fn eval(&self, action: Action, _game: &ChessGame) -> f32 {
        match action {
            AMove(m) => {
                m.from.man_dist(m.to) as f32
            }
            ATake(m) => {
                m.from.man_dist(m.to) as f32
            }
            ACastle(m) => {
                (m.king_from.man_dist(m.king_to) + m.rook_from.man_dist(m.rook_to)) as f32
            }
            APromote(m) => {
                m.from.man_dist(m.to) as f32
            }
            APromoteTake(m) => {
                m.from.man_dist(m.to) as f32
            }
            APassant(m) => {
                m.from.man_dist(m.to) as f32
            }
        }
    }
}

impl EvalGame for Attacks {
    fn eval(&self, action: Action, _game: &ChessGame) -> f32 {
        match action {
            ATake(_) => 1.0,
            APromoteTake(_) => 1.0,
            _ => 0.0
        }
    }
}

impl EvalGame for CountMoves {
    fn eval(&self, action: Action, game: &ChessGame) -> f32 {
        let ids = action_ids(action);
        let mut total = 0.0;
        for m in &game.moves {
            let move_ids = action_ids(*m);
            for i in move_ids {
                if ids.contains(&i) { total += 1.0 }
            }
        }
        total
    }
}

impl EvalGame for MoveAmount {
    fn eval(&self, _action: Action, game: &ChessGame) -> f32 {
        game.possible_moves(self.0).count() as f32
    }
}

impl EvalGame for KingMoves {
    fn eval(&self, action: Action, _game: &ChessGame) -> f32 {
        match action {
            ATake(m) => {
                if matches!(m.piece, King(_, p) if p == self.0) { 1.0 } else { 0.0 }
            }
            AMove(m) => {
                if matches!(m.piece, King(_, p) if p == self.0) { 1.0 } else { 0.0 }
            }
            ACastle(_m) => 1.0,
            _ => 0.0
        }
    }
}

impl EvalGame for QueenMoves {
    fn eval(&self, action: Action, _game: &ChessGame) -> f32 {
        match action {
            ATake(m) => {
                if matches!(m.piece, Queen(_, _)) { 1.0 } else { 0.0 }
            }
            AMove(m) => {
                if matches!(m.piece, Queen(_, _)) { 1.0 } else { 0.0 }
            }
            APromote(m) => {
                if matches!(m.end, Queen(_, _)) { 1.0 } else { 0.0 }
            }
            APromoteTake(m) => {
                if matches!(m.end, Queen(_, _)) { 1.0 } else { 0.0 }
            }
            _ => 0.0
        }
    }
}

impl EvalGame for InCheck {
    fn eval(&self, _action: Action, game: &ChessGame) -> f32 {
        if game.in_check(self.0) {
            1.0
        } else {
            0.0
        }
    }
}

struct MaxChoose;
struct MinChoose;
struct FirstChoose;
struct LastChoose;

impl ChooseMove for MaxChoose {
    fn choose_move(&self, left: (f32, Action), right: (f32, Action)) -> (f32, Action) {
        if left.0 < right.0 {
            right
        } else {
            left
        }
    }
}

impl ChooseMove for MinChoose {
    fn choose_move(&self, left: (f32, Action), right: (f32, Action)) -> (f32, Action) {
        if left.0 < right.0 {
            left
        } else {
            right
        }
    }
}

impl ChooseMove for FirstChoose {
    fn choose_move(&self, left: (f32, Action), _right: (f32, Action)) -> (f32, Action) {
        left
    }
}

impl ChooseMove for LastChoose {
    fn choose_move(&self, _left: (f32, Action), right: (f32, Action)) -> (f32, Action) {
        right
    }
}

struct MaxCombine;
struct MinCombine;
struct AverageCombine;
struct MedianCombine;
struct VarianceCombine;
struct SumCombine;
struct ProductCombine;
struct ConstantCombine(f32);

impl CombineValues for MaxCombine {
    fn combine_values(&self, values: Vec<f32>) -> f32 {
        values.into_iter().reduce(f32::max).unwrap()
    }
}

impl CombineValues for MinCombine {
    fn combine_values(&self, values: Vec<f32>) -> f32 {
        values.into_iter().reduce(f32::min).unwrap()
    }
}

impl CombineValues for AverageCombine {
    fn combine_values(&self, values: Vec<f32>) -> f32 {
        average(&values)
    }
}

impl CombineValues for MedianCombine {
    fn combine_values(&self, values: Vec<f32>) -> f32 {
        let mut vals = values.clone();
        vals.sort_by(|a,b| a.partial_cmp(b).unwrap());
        vals[vals.len()/2]
    }
}

impl CombineValues for VarianceCombine {
    fn combine_values(&self, values: Vec<f32>) -> f32 {
        variance(&values)
    }
}

impl CombineValues for SumCombine {
    fn combine_values(&self, values: Vec<f32>) -> f32 {
        values.into_iter().sum()
    }
}

impl CombineValues for ProductCombine {
    fn combine_values(&self, values: Vec<f32>) -> f32 {
        values.into_iter().product()
    }
}

impl CombineValues for ConstantCombine {
    fn combine_values(&self, _values: Vec<f32>) -> f32 {
        self.0
    }
}


// some utilities
fn king(game: &ChessGame, player: Player) -> Option<PieceData> {
    let kings = game.board.pieces_for(player).into_iter()
        .filter(|x| matches!(x.piece, King(_,_)))
        .collect::<Vec<_>>();
    if kings.len() == 1 {
        Some(kings[0])
    } else {
        None
    }
}

fn total_distance(game: &ChessGame, player: Player, piece: PieceData) -> i32 {
    let pieces = game.board.pieces_for(player);
    pieces.into_iter().map(|p| p.coord.man_dist(piece.coord)).sum()
}

fn action_ids(action: Action) -> Vec<ID> {
    match action {
        AMove(m) => vec![m.piece.id().unwrap()],
        ATake(m) => vec![m.piece.id().unwrap()],
        ACastle(m) => vec![m.king.id().unwrap(), m.rook.id().unwrap()],
        APromote(m) => vec![m.piece.id().unwrap()],
        APromoteTake(m) => vec![m.piece.id().unwrap()],
        APassant(m) => vec![m.piece.id().unwrap()],
    }
}

fn average(values: &[f32]) -> f32 {
    let mut count = 0.0;
    let mut sum = 0.0;
    for v in values {
        count += 1.0;
        sum += v;
    }
    sum/count
}

fn variance(values: &[f32]) -> f32 {
    let mut var = 0.0;
    let mut count = 0.0;
    let avg = average(values);
    for v in values {
        count += 1.0;
        var += (v - avg).powf(2.0);
    }
    var/count
}

// dynamic dispatch version
// not really sure which is "better"
// at this point in time the enum/switch pattern and this version seem
// pretty similar
// other alternative is maybe a message passing style where there is a handler
// and the "message" branches between name/description/run
// trait Strategy {
//   fn call(message: Message, res: &mut _) {
//   }
// }
// enum Message {
//   Name,
//   Description,
//   Run,
// }
// message passing seems unpleasant at first glance, but I could be wrong
// 
// the main performance bottleneck was the vector allocation, so the difference
// in behaviors isn't obvious yet
/*
pub trait Strategy {
    fn name() -> &'static str;
    fn description() -> &'static str;
    fn run(&self, game: &ChessGame) -> Option<Action>;
}
pub struct Random;
pub struct Swarm;
pub struct GiveUp;
pub struct Huddle;
pub struct SmellyKing;
pub struct Intimidated;
pub struct Ape;
pub struct Sleepy;
pub struct Pacifist;
pub struct EqualOpportunity;
pub struct Determined;
pub struct Prepared;

impl Strategy for Random {
    fn name() -> &'static str {
        "Random"
    }

    fn description() -> &'static str {
        "Choose moves randomly."
    }

    fn run(&self, game: &ChessGame) -> Option<Action> {
        let moves = game.possible_moves(game.turn);
        if !moves.is_empty() {
            Some(*moves.choose(&mut thread_rng()).unwrap())
        } else {
            None
        }   
    }
}

impl Strategy for Swarm {
    fn name() -> &'static str {
        "Swarm"
    }

    fn description() -> &'static str {
        "Minimizes the total distance between all of their pieces and the enemy king."
    }

    fn run(&self, game: &ChessGame) -> Option<Action> {
        let player = game.turn;
        strategy(1, player, game, &KingDistance(player, player.toggle()), &MinChoose, &MinCombine)
    }
}

impl Strategy for GiveUp {
    fn name() -> &'static str {
        "Give up"
    }

    fn description() -> &'static str {
        "Chess isn't really their thing"
    }

    fn run(&self, game: &ChessGame) -> Option<Action> {
        None
    }
}

impl Strategy for Huddle {
    fn name() -> &'static str {
        "Huddle"
    }

    fn description() -> &'static str {
        "Keep your pieces as close to your king as possible"
    }

    fn run(&self, game: &ChessGame) -> Option<Action> {
        let player = game.turn;
        strategy(1, player, game, &KingDistance(player, player), &MinChoose, &MinCombine)
    }
}

impl Strategy for SmellyKing {
    fn name() -> &'static str {
        "Smelly King"
    }
    fn description() -> &'static str {
        "Your king stinks, all your pieces run away"
    }
    fn run(&self, game: &ChessGame) -> Option<Action> {
        let player = game.turn;
        strategy(1, player, game, &KingDistance(player, player), &MaxChoose, &MaxCombine)
    }
}

impl Strategy for Intimidated {
    fn name() -> &'static str {
        "Intimidated"
    }
    fn description() -> &'static str {
        "The other king is quite scary, all your pieces run away from him"
    }
    fn run(&self, game: &ChessGame) -> Option<Action> {
        let player = game.turn;
        strategy(1, player, game, &KingDistance(player, player.toggle()), &MaxChoose, &MaxCombine)
    }
}

impl Strategy for Ape {
    fn name() -> &'static str {
        "Ape"
    }
    fn description() -> &'static str {
        "Big moves are the only way to get to the moon."
    }
    fn run(&self, game: &ChessGame) -> Option<Action> {
        strategy(1, game.turn, game, &BigPlays, &MaxChoose, &MaxCombine)
    }
}

impl Strategy for Sleepy {
    fn name() -> &'static str {
        "Sleepy"
    }
    fn description() -> &'static str {
        "Too sleepy to move very far."
    }
    fn run(&self, game: &ChessGame) -> Option<Action> {
        strategy(1, game.turn, game, &BigPlays, &MinChoose, &MinCombine)
    }
}

impl Strategy for Pacifist {
    fn name() -> &'static str {
        "Pacifist"
    }
    fn description() -> &'static str {
        "Minimizes the number of attacks it has on the opponent."
    }
    fn run(&self, game: &ChessGame) -> Option<Action> {
        strategy(1, game.turn, game, &Attacks, &MinChoose, &SumCombine)
    }
}

impl Strategy for Prepared {
    fn name() -> &'static str {
        "Prepared"
    }
    fn description() -> &'static str {
        "Maximizes the number of attacks it has on the opponent."
    }
    fn run(&self, game: &ChessGame) -> Option<Action> {
        strategy(1, game.turn, game, &Attacks, &MaxChoose, &SumCombine)
    }
}

impl Strategy for EqualOpportunity {
    fn name() -> &'static str {
        "Equal Opportunity"
    }
    fn description() -> &'static str {
        "Everyone needs a chance to succeed, keeps the move count for each piece relatively balanced."
    }
    fn run(&self, game: &ChessGame) -> Option<Action> {
        strategy(1, game.turn, game, &CountMoves, &MinChoose, &MinCombine)
    }
}

impl Strategy for Determined {
    fn name() -> &'static str {
        "Determined"
    }
    fn description() -> &'static str {
        "Sticks to the choices that have already been made, the piece that has moved the most keeps getting chosen"
    }
    fn run(&self, game: &ChessGame) -> Option<Action> {
        strategy(1, game.turn, game, &CountMoves, &MaxChoose, &MaxCombine)
    }
}

pub fn strategy_map() -> HashMap<String, impl Strategy> {
    let mut map = HashMap::new();
    map.insert(Random::name().to_string(), Random);
    
    map
}
*/
