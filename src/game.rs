use std::{fmt::Display, cmp::min};
use action::{Action, Move, Take, Promote, PromoteTake, Passant, Castle};
use coord::Coord;
use final_state::FinalState;
use piece::{Piece, PieceData};
use player::Player;

use Player::*;
use Action::*;
use Piece::*;
use FinalState::*;

use crate::{action, coord, final_state, piece, player};

#[derive(Debug, Clone, Copy)]
pub struct ChessBoard {
    pub board: [Piece; 64],
}

#[derive(Debug, Clone)]
pub struct ChessGame {
    pub turn_number: u32,
    pub turn: Player,
    pub board: ChessBoard,
    pub moves: Vec<Action>,
    pub removed: Vec<Piece>,
}

fn starting_board() -> ChessBoard {
    ChessBoard { board: 
        [
            Rook(0, Black), Knight(1, Black), Bishop(2, Black), Queen(3, Black), King(4, Black), Bishop(5, Black), Knight(6, Black), Rook(7, Black),
            Pawn(8, Black), Pawn(9, Black), Pawn(10, Black), Pawn(11, Black), Pawn(12, Black), Pawn(13, Black), Pawn(14, Black), Pawn(15, Black),
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, 
            Pawn(16, White), Pawn(17, White), Pawn(18, White), Pawn(19, White), Pawn(20, White), Pawn(21, White), Pawn(22, White), Pawn(23, White),
            Rook(24, White), Knight(25, White), Bishop(26, White), Queen(27, White), King(28, White), Bishop(29, White), Knight(30, White), Rook(31, White),
        ]
    }
}

impl ChessBoard {
    fn default() -> ChessBoard {
        starting_board()
    }

    pub fn get(&self, coord: Coord) -> Piece {
        self.board[((8*coord.row)+(coord.col)) as usize]
    }

    pub fn set(&mut self, coord: Coord, p: Piece) {
        self.board[((8*coord.row)+(coord.col)) as usize] = p;
    }

    fn pieces(&self) -> Vec<PieceData> {
        Coord::all_coords().map(|coord| {
            let piece = self.get(coord);
            PieceData::new(piece, coord)
        }).collect()
    }

    fn pieces_for(&self, player: Player) -> Vec<PieceData> {
        self.pieces().into_iter()
            .filter(|p| {
                p.piece.owner().map_or(false, |x| x == player)
            }).collect()
    }
}

impl ChessGame {
    pub fn new() -> Self {
        ChessGame { turn_number: 0, turn: White, board: ChessBoard::default(), moves: Vec::new() , removed: Vec::new() }
    }

    // for troubleshooting
    pub fn blank() -> Self {
        ChessGame { turn_number: 0, turn: White, board: ChessBoard { board: [Empty; 64] }, moves: vec![AMove(Move::new(Empty, (0,0).into(), (0,0).into()))], removed: Vec::new() }
    }

    // do the given action
    // want to not mutate the original board since doing immutable operations 
    // makes the search tree easier
    
    pub fn step(&self, act: Action) -> Self {
        let mut game = self.clone();
        // always push the action, increase turn number, and change the turn
        game.moves.push(act);
        game.turn_number += 1;
        game.turn = game.turn.toggle();
        match act {
            AMove(m) => {
                game.board.set(m.to, m.piece);
                game.board.set(m.from, Empty);
            }
            ATake(m) => {
                game.board.set(m.to, m.piece);
                game.board.set(m.from, Empty);
                game.removed.push(m.removed);
            }
            ACastle(m) => {
                game.board.set(m.king_to, m.king);
                game.board.set(m.king_from, Empty);
                game.board.set(m.rook_to, m.rook);
                game.board.set(m.rook_from, Empty);
            }
            APromote(m) => {
                game.board.set(m.to, m.end);
                game.board.set(m.from, Empty);
            }
            APromoteTake(m) => {
                game.board.set(m.to, m.end);
                game.board.set(m.from, Empty);
                game.removed.push(m.removed);
            }
            APassant(m) => {
                game.board.set(m.to, m.piece);
                game.board.set(m.from, Empty);
                game.board.set(m.removed_from, Empty);
                game.removed.push(m.removed);
            }
        }
        game
    }

    fn has_moved(&self, piece: Piece) -> bool {
        for m in &self.moves {
            match m {
                AMove(mv) => {
                    if mv.piece == piece {
                        return true;
                    }
                }
                ACastle(c) => {
                    if c.king == piece || c.rook == piece {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
    
    // can the given piece be attacked
    // a decent bit of duplicate logic from generating moves, but I don't think
    // they quite overlap enough since generating moves is "give me all the moves
    // you can make" and this is "can you make this specific move" which are 
    // different from a computational standpoint (can very easily get infinite
    // or very deep recursion if you do: give me all moves -> is one an attack,
    // since moves can rely on moving into an attacked position)
    pub fn can_attack(&self, piece: PieceData, attacker: Player) -> bool {
        let coord = piece.coord;
        let (row, col) = (coord.row, coord.col);

        // en passant checks
        // this piece is a pawn, the last move was this pawn with a 2 step move
        // there is an attacker's pawn to the left or right
        if matches!(piece.piece, Pawn(_, _)) {
            if let Some(AMove(m)) = self.moves.last() {
                if m.piece == piece.piece && (m.from.row - m.to.row).abs() == 2 {
                    if coord.col > 0 {
                        let coord_adj = coord + (0, -1).into();
                        let adj = self.board.get(coord_adj);
                        if matches!(adj, Pawn(_, _)) && adj.owned_by(attacker) {
                            return true;
                        }
                    }
                    if coord.col < 7 {
                        let coord_adj = coord + (0, 1).into();
                        let adj = self.board.get(coord_adj);
                        if matches!(adj, Pawn(_, _)) && adj.owned_by(attacker) {
                            return true;
                        }
                    }
                }
            }
        }

        // search along diagonals and straights
        // iterate through the upper bounds of the ranges (the max number of steps
        // to take before hitting an edge) and the stepping directions
        let check = [
            (min(7-row, col)  , ( 1, -1).into()),
            (min(row, col)    , (-1, -1).into()),
            (min(row, 7-col)  , (-1,  1).into()),
            (min(7-row, 7-col), ( 1,  1).into()),
            (7-row            , ( 1,  0).into()),
            (col              , ( 0, -1).into()),
            (row              , (-1,  0).into()),
            (7-col            , ( 0,  1).into()),
        ].into_iter()
            .any(|(limit, step)| {
                if let Some(other) = self.search(coord, limit, step) {
                    if other.piece.owned_by(attacker) && other.can_reach(coord) {
                        return true;
                    }
                }
                false
            });

        if check { return true };

        // knight checks
        let check = [
            coord + (-2, -1).into(),
            coord + (-1, -2).into(),
            coord + (1, -2).into(),
            coord + (2, -1).into(),
            coord + (2, 1).into(),
            coord + (1, 2).into(),
            coord + (-1, 2).into(),
            coord + (-2, 1).into(),
        ].into_iter()
            .filter(|x| !(x.row < 0 || x.row > 7 || x.col < 0 || x.col > 7))
            .any(|c| { 
                let p = self.board.get(c);
                p.owned_by(attacker) && matches!(p, Knight(_, _))
            });
        if check { return true };
        

        false
    }

    // all the moves available to the player
    // typically the player should be the current player
    // if the king is in check then the only available moves are the ones that
    // remove check
    // also cannot make a move that leaves the player in check
    pub fn possible_moves(&self, player: Player) -> Vec<Action> {
        let moves = self.available_moves(player);
        // filter out moves that leave the player in check
        moves.into_iter().filter(|m| {
            let g = self.step(*m);
            !g.in_check(player)
        }).collect()
    }

    // usually don't want to call this
    pub fn available_moves(&self, player: Player) -> Vec<Action> {
        let mut moves = Vec::new();
        let pieces = self.board.pieces_for(player);
        for piece in pieces {
            let all_moves = self.calc_moves(piece);
            moves.extend(all_moves);
        }
        moves
    }
    
    
    fn calc_moves(&self, piece: PieceData) -> Vec<Action> {
        match piece.piece {
            Pawn(_, _)   => self.pawn_moves(piece),
            Rook(_, _)   => self.rook_moves(piece),
            Bishop(_, _) => self.bishop_moves(piece),
            Knight(_, _) => self.knight_moves(piece),
            Queen(_, _)  => self.queen_moves(piece),
            King(_, _)   => self.king_moves(piece),
            Empty => {
                println!("Tried to compute the moves for an empty piece");
                Vec::new()
            }
        }
    }

    // pawn moves are sort of complicated
    // if they haven't moved yet they can move two spaces
    // otherwise it is one space
    // if a pawn reaches the edge it is promoted to a choice of a queen, biship, knight, or rook
    // if the pawn is adjacent to a pawn that previously moved two spaces then it
    //   can be captured "en passant" or diagonally
    // a pawn takes pieces diagonally
    
    fn pawn_moves(&self, orig: PieceData) -> Vec<Action> {
        let owner = orig.piece.owner().unwrap();
        let coord = orig.coord;
        //let (row, col) = (coord.row, coord.col);
        let mut moves = Vec::new();
        // for now black is always fixed on top and white to the bottom so
        // can just assume the direction things go
        let dir = owner.pawn_dir(); //if orig.piece.owned_by(Black) { 1 } else { -1 };

        // two steps
        if !self.has_moved(orig.piece) {
            let coord_skip = coord + dir;
            let coord_dest = coord_skip + dir;
            let skip = self.board.get(coord_skip);
            let dest = self.board.get(coord_dest);
            if dest.empty() && skip.empty() {
                moves.push(AMove(Move::new(orig.piece, coord, coord_dest)));
            } 
        }

        // one step or promote
        let coord_dest = coord + dir;
        if coord_dest.row == 0 || coord_dest.row == 7 {
            let dest = self.board.get(coord_dest);
            if dest.empty() {
                let id = orig.piece.id().unwrap();
                moves.push(APromote(Promote::new(orig.piece, Queen(id, owner), coord, coord_dest)));
                moves.push(APromote(Promote::new(orig.piece, Bishop(id, owner), coord, coord_dest)));
                moves.push(APromote(Promote::new(orig.piece, Knight(id, owner), coord, coord_dest)));
                moves.push(APromote(Promote::new(orig.piece, Rook(id, owner), coord, coord_dest)));
            }
        } else {
            let dest = self.board.get(coord_dest);
            if dest.empty() {
                moves.push(AMove(Move::new(orig.piece, coord, coord_dest)));
            }
        }

        // take or take and promote
        if coord_dest.row == 0 || coord_dest.row == 7 {
            if coord_dest.col > 0 {
                let coord_dest = coord_dest + (0, -1).into();
                let dest = self.board.get(coord_dest);
                if dest.owned_by(owner.toggle()) {
                    let id = orig.piece.id().unwrap();
                    moves.push(APromoteTake(PromoteTake::new(orig.piece, Queen(id, owner), dest, coord, coord_dest)));
                    moves.push(APromoteTake(PromoteTake::new(orig.piece, Bishop(id, owner), dest, coord, coord_dest)));
                    moves.push(APromoteTake(PromoteTake::new(orig.piece, Knight(id, owner), dest, coord, coord_dest)));
                    moves.push(APromoteTake(PromoteTake::new(orig.piece, Rook(id, owner), dest, coord, coord_dest)));
                }
            }
            if coord_dest.col < 7 {
                let coord_dest = coord_dest + (0, 1).into();
                let dest = self.board.get(coord_dest);
                if dest.owned_by(owner.toggle()) {
                    let id = orig.piece.id().unwrap();
                    moves.push(APromoteTake(PromoteTake::new(orig.piece, Queen(id, owner), dest, coord, coord_dest)));
                    moves.push(APromoteTake(PromoteTake::new(orig.piece, Bishop(id, owner), dest, coord, coord_dest)));
                    moves.push(APromoteTake(PromoteTake::new(orig.piece, Knight(id, owner), dest, coord, coord_dest)));
                    moves.push(APromoteTake(PromoteTake::new(orig.piece, Rook(id, owner), dest, coord, coord_dest)));
                }
            }
        } else {
            if coord_dest.col > 0 {
                let coord_dest = coord_dest + (0, -1).into();
                let dest = self.board.get(coord_dest);
                if dest.owned_by(owner.toggle()) {
                    moves.push(ATake(Take::new(orig.piece, dest, coord, coord_dest)));
                }
            }
            if coord_dest.col < 7 {
                let coord_dest = coord_dest + (0, 1).into();
                let dest = self.board.get(coord_dest);
                if dest.owned_by(owner.toggle()) {
                    moves.push(ATake(Take::new(orig.piece, dest, coord, coord_dest)));
                }
            }
        }

        // en passant
        if coord.col > 0 {
            let coord_adj = coord + (0, -1).into();
            let coord_dest = coord_adj + dir;
            let adj = self.board.get(coord_adj);
            let dest = self.board.get(coord_dest);
            if adj.owned_by(owner.toggle()) && dest.empty() {
                // last move was a pawn that moved to the adjacent square and moved two spaces
                if let Some(AMove(m)) = self.moves.last() {
                    if matches!(m.piece, Pawn(_, _)) && m.to == coord_adj && (m.from.row - m.to.row).abs() == 2 {
                        moves.push(APassant(Passant::new(orig.piece, adj, coord, coord_dest, coord_adj)));
                    }
                }
            }
        }
        if coord.col < 0 {
            let coord_adj = coord + (0, 1).into();
            let coord_dest = coord_adj + dir;
            let adj = self.board.get(coord_adj);
            let dest = self.board.get(coord_dest);
            if adj.owned_by(owner.toggle()) && dest.empty() {
                // last move was a pawn that moved to the adjacent square and moved two spaces
                if let Some(AMove(m)) = self.moves.last() {
                    if matches!(m.piece, Pawn(_, _)) && m.to == coord_adj && (m.from.row - m.to.row).abs() == 2 {
                        moves.push(APassant(Passant::new(orig.piece, adj, coord, coord_dest, coord_adj)));
                    }
                }
            }
        }

        moves

    }

    // probably the easiest to compute, just don't go off the edge
    
    fn knight_moves(&self, orig: PieceData) -> Vec<Action> {
        let owner = orig.piece.owner().unwrap();
        let coord = orig.coord;
        let possible = [
            coord + (-2, -1).into(),
            coord + (-1, -2).into(),
            coord + (1, -2).into(),
            coord + (2, -1).into(),
            coord + (2, 1).into(),
            coord + (1, 2).into(),
            coord + (-1, 2).into(),
            coord + (-2, 1).into(),
        ];

        possible.into_iter()
            .filter(|x| !(x.row < 0 || x.row > 7 || x.col < 0 || x.col > 7))
            .filter(|x| !self.board.get(*x).owned_by(owner))
            .map(|x| {
                let piece = self.board.get(x);
                if piece.owned_by(owner.toggle()) {
                    ATake(Take::new(orig.piece, piece, coord, x))
                } else {
                    AMove(Move::new(orig.piece, coord, x))
                }
            })
            .collect()
    }

    
    fn king_moves(&self, orig: PieceData) -> Vec<Action> {
        let owner = orig.piece.owner().unwrap();
        let coord = orig.coord;
        let possible = [
            coord + (-1, -1).into(),
            coord + (-1, 0).into(),
            coord + (-1, 1).into(),
            coord + (0, 1).into(),
            coord + (1, 1).into(),
            coord + (1, 0).into(),
            coord + (1, -1).into(),
            coord + (0, -1).into(),
        ];

        let mut moves: Vec<Action> = possible.into_iter()
            .filter(|x| !(x.row < 0 || x.row > 7 || x.col < 0 || x.col > 7))
            .filter(|x| !self.board.get(*x).owned_by(owner))
            .map(|x| {
                let piece = self.board.get(x);
                if piece.owned_by(owner.toggle()) {
                    ATake(Take::new(orig.piece, piece, coord, x))
                } else {
                    AMove(Move::new(orig.piece, coord, x))
                }
            })
            .collect();

        // need to do castling, but that requires understanding attacking positions
        // whether a piece moved in the past
        
        // rules of castling
        // both the king and the rook must not have moved previously
        // no pieces between the king and rook
        // king not under attack and neither are the spaces the king would pass through
        // the move would be
        //   king goes 2 spaces in the direction of the rook
        //   rook ends up on the inner edge of the king
        if !self.has_moved(orig.piece) {
            let rooks: Vec<_> = self.board.pieces_for(owner).into_iter()
                .filter(|x| matches!(x.piece, Rook(_, _)))
                .filter(|x| !self.has_moved(x.piece)).collect();
            for piece in rooks {
                let rook = piece.piece;
                let coord_dest = piece.coord;
                let mut can_castle = true;
                // check for things in the way
                // since king-rook must be aligned on the row to not have moved
                // only look across columns
                let dir = (coord_dest.col-coord.col).abs()/(coord_dest.col-coord.col);
                for i in 1..(coord_dest.col-coord.col).abs() {
                    let coord_int = coord + (0, dir*i).into();
                    if !self.board.get(coord_int).empty() {
                        can_castle = false;
                        break;
                    }
                }
                // king not under attack
                // aka opponent can not make any moves that land onto one of the
                // concerning spaces
                if can_castle {
                    let spaces = (0..=2).map(|i| (coord.row, coord.col+dir*i).into());
                    for space in spaces {
                        let p = self.board.get(space);
                        let pd = PieceData::new(p, space);
                        if self.can_attack(pd, owner.toggle()) {
                            can_castle = false;
                            break;
                        }
                    }
                    
                }
                if can_castle {
                    moves.push(ACastle(Castle::new(orig.piece, rook, coord, coord + (0, 2*dir).into(), coord_dest, coord + (0, dir).into())));
                }
            }
        }

        moves
    }


    // move diagonally until
    // - moving off board
    // - moving into own piece
    // - moving onto opponent piece
    
    fn bishop_moves(&self, orig: PieceData) -> Vec<Action> {
        let (row, col) = orig.coord.into();
        let info = vec![
            (min(7-row, col), (1, -1).into()),
            (min(row, col), (-1, -1).into()),
            (min(row, 7-col), (-1, 1).into()),
            (min(7-row, 7-col), (1, 1).into()),
        ];
        self.stepper_wrapper(orig, info)
    }

    
    fn queen_moves(&self, orig: PieceData) -> Vec<Action> {
        let (row, col) = orig.coord.into();
        let info = vec![
            (min(7-row, col), (1, -1).into()),
            (min(row, col), (-1, -1).into()),
            (min(row, 7-col), (-1, 1).into()),
            (min(7-row, 7-col), (1, 1).into()),
            (7-row, (1, 0).into()),
            (col, (0, -1).into()),
            (row, (-1, 0).into()),
            (7-col, (0, 1).into()),
        ];
        self.stepper_wrapper(orig, info)
    }

    
    fn rook_moves(&self, orig: PieceData) -> Vec<Action> {
        let (row, col) = orig.coord.into();
        let info = vec![
            (7-row, (1, 0).into()),
            (col, (0, -1).into()),
            (row, (-1, 0).into()),
            (7-col, (0, 1).into()),
        ];
        self.stepper_wrapper(orig, info)
    }

    // since it is common enough to step in a direction until vacancy
    fn stepper_wrapper(&self, orig: PieceData, info: Vec<(i32, Coord)>) -> Vec<Action> {
        let mut moves = Vec::new();
        for (limit, step) in info {
            let res = self.stepper(orig, limit, step);
            moves.extend(res);
        }
        moves
    }

    fn stepper(&self, orig: PieceData, limit: i32, step: Coord) -> Vec<Action> {
        let mut moves = Vec::new();
        let owner = orig.piece.owner().unwrap();
        let mut coord_ = orig.coord;
        for _ in 0..limit {
            coord_ += step;
            let piece = self.board.get(coord_);
            if piece.empty() {
                moves.push(AMove(Move::new(orig.piece, orig.coord, coord_)));
            } else if piece.owned_by(owner) {
                break;
            } else {
                moves.push(ATake(Take::new(orig.piece, piece, orig.coord, coord_)));
                break;
            }
        }
        moves
    }


    // find the first piece by searching in a direction from a starting point
    fn search(&self, start: Coord, limit: i32, step: Coord) -> Option<PieceData> {
        let mut coord_ = start;
        for _ in 0..limit {
            coord_ += step;
            let piece = self.board.get(coord_);
            if !piece.empty() {
                return Some(PieceData::new(piece, coord_));
            }
        }
        None
    }

    // can any of the opponents possible moves do a take on the given player's king?
    pub fn in_check(&self, player: Player) -> bool {
        let kings = self.board.pieces_for(player).into_iter()
            .filter(|x| matches!(x.piece, King(_, _))).collect::<Vec<_>>();
        match kings.len() {
            1 => self.can_attack(kings[0], player.toggle()),
            0 => panic!("There are no kings somehow"),
            _ => panic!("There is more than 1 king somehow"),
        }
    }

    // is the game done and what is the status if so
    // general rules that should be implemented
    // draws:
    //   stalemate - no moves left and not in check
    //   3 repeat - one players game state repeated 3 times, don't have a good setup for this
    //   50 turn - 50 turns without a take and no pawns moved
    // winning means the player has no moves and is in check -> opponent wins
    // this is intended to be used after stepping as a check
    
    fn check_state(&self) -> Option<FinalState> {
        let acts = self.possible_moves(self.turn);
        if acts.is_empty() {
            if self.in_check(self.turn) {
                return Some(Win(self.turn.toggle()));
            } else {
                return Some(Draw);
            }
        } else if self.moves.len() >= 50 {
            // I imagine this could be better
            let mut uninteresting = true;
            for act in &self.moves[self.moves.len()-50..] {
                match act {
                    ATake(_) => {
                        uninteresting = false;
                        break;
                    }
                    APromoteTake(_) => {
                        uninteresting = false;
                        break;
                    }
                    APromote(_) => {
                        uninteresting = false;
                        break;
                    }
                    APassant(_) => {
                        uninteresting = false;
                        break;
                    }
                    AMove(m) => {
                        if matches!(m.piece, Pawn(_, _)) {
                            uninteresting = false;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if uninteresting {
                return Some(Draw)
            }
        }
        None
    }
}

impl Display for ChessBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        /*
         * Goal
         *  |abcdefgh
         * ----------
         * 1|########
         * 2|
         * 3|
         * 4|
         * 5|
         * 6|
         * 7|
         * 8|
         */
        let mut lines = Vec::new();
        lines.push(" |abcdefgh".to_string());
        lines.push("----------".to_string());
        for row in 0..8 {
            let mut line = format!("{}|", row+1);
            for col in 0..8 {
                line.push_str(&format!("{}", self.get((row, col).into())));
            }
            lines.push(line);
        }
        for line in lines {
            writeln!(f, "{}", line)?
        }
        Ok(())
    }
}

impl Display for ChessGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Turn: {:?}", self.turn)?;
        self.board.fmt(f)
    }
}

impl Default for ChessGame {
    fn default() -> Self {
        Self::new()
    }
}

pub fn play_game(black_player: &dyn Fn(&ChessGame) -> Option<Action>, white_player: &dyn Fn(&ChessGame) -> Option<Action>) -> FinalState {
    let mut game = ChessGame::new();
    let mut active = white_player;
    let mut inactive = black_player;
    loop {
        println!("{}", game);
        if let Some(state) = game.check_state() {
            return state;
        } else if let Some(act) = active(&game) {
            println!("{:?} chose: {:?}", game.turn, act);
            game = game.step(act);
            (active, inactive) = (inactive, active);
        } else {
            println!("Couldn't make a move, but couldn't determine that ahead of time for some reason");
            return Draw;
        }
    }
}

