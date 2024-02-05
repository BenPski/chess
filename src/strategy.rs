use core::f32;

use rand::{thread_rng, seq::SliceRandom};

use crate::piece::{PieceData, ID};
use crate::player::Player;
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
    let mut moves = game.possible_moves(player);
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
        let mut moves = g.possible_moves(player);
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


// just pick a random move from the possible moves
pub fn random_player(game: &ChessGame) -> Option<Action> {
    let moves = game.possible_moves(game.turn);
    if !moves.is_empty() {
        Some(*moves.choose(&mut thread_rng()).unwrap())
    } else {
        None
    }
}

// since there is a default sort, just use that sort
pub fn sorted_player(game: &ChessGame) -> Option<Action> {
    let mut moves = game.possible_moves(game.turn);
    if !moves.is_empty() {
        moves.sort();
        Some(moves[0])
    } else {
        None
    }
}

pub fn rev_sorted_player(game: &ChessGame) -> Option<Action> {
    let mut moves = game.possible_moves(game.turn);
    if !moves.is_empty() {
        moves.sort();
        moves.reverse();
        Some(moves[0])
    } else {
        None
    }
}

pub fn give_up(_game: &ChessGame) -> Option<Action> {
    None
}

pub fn swarm(game: &ChessGame) -> Option<Action> {
    let player = game.turn;
    strategy(1, player, game, &KingDistance(player, player.toggle()), &MinChoose, &MinCombine)
}

pub fn huddle(game: &ChessGame) -> Option<Action> {
    let player = game.turn;
    strategy(1, player, game, &KingDistance(player, player), &MinChoose, &MinCombine)
}

pub fn stinky_king(game: &ChessGame) -> Option<Action> {
    let player = game.turn;
    strategy(1, player, game, &KingDistance(player, player), &MaxChoose, &MaxCombine)
}

pub fn intimidated(game: &ChessGame) -> Option<Action> {
    let player = game.turn;
    strategy(1, player, game, &KingDistance(player, player.toggle()), &MaxChoose, &MaxCombine)
}

pub fn ape(game: &ChessGame) -> Option<Action> {
    strategy(1, game.turn, game, &BigPlays, &MaxChoose, &MaxCombine)
}

pub fn tired(game: &ChessGame) -> Option<Action> {
    strategy(1, game.turn, game, &BigPlays, &MinChoose, &MinCombine)
}

pub fn pacifist(game: &ChessGame) -> Option<Action> {
    strategy(1, game.turn, game, &Attacks, &MinChoose, &SumCombine)
}

pub fn aggressive(game: &ChessGame) -> Option<Action> {
    strategy(1, game.turn, game, &Attacks, &MaxChoose, &SumCombine)
}

pub fn equal_opportunity(game: &ChessGame) -> Option<Action> {
    strategy(1, game.turn, game, &CountMoves, &MinChoose, &MinCombine)
}

pub fn focused(game: &ChessGame) -> Option<Action> {
    strategy(1, game.turn, game, &CountMoves, &MaxChoose, &MaxCombine)
}

// move king into check
// pick move that maximizes most moves
// pick move that limits most moves
// pick move that limits opponents moves
// pick move that maximizes opponents moves
// move that maximizes number of possible takes
// move that limits number of takes
// maximize promotions
// swap sides
// maximize opponents attacks on own king
// protect your queen
// look only at your own moves with some lookahead
// always take
// piece that has moved the least
// try to draw
// maximize distance between pieces and own king
// suicide king
// drunk king
// big plays
//
// sorting things
//

// cheat
// always promote
// randomly swap pieces
// switch with opponent pieces
// do illegal moves

struct KingDistance(Player, Player);
struct BigPlays;
struct Attacks;
struct CountMoves;

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
