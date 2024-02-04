use crate::{piece::Piece, coord::Coord};

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Action {
    ACastle(Castle),
    AMove(Move),
    ATake(Take),
    APromote(Promote),
    APromoteTake(PromoteTake),
    APassant(Passant),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Castle {
    pub king: Piece,
    pub rook: Piece,
    pub king_from: Coord,
    pub king_to: Coord,
    pub rook_from: Coord,
    pub rook_to: Coord,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Move {
    pub piece: Piece,
    pub from: Coord,
    pub to: Coord,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Take {
    pub piece: Piece,
    pub removed: Piece,
    pub from: Coord,
    pub to: Coord,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Promote {
    pub piece: Piece,
    pub end: Piece,
    pub from: Coord,
    pub to: Coord,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct PromoteTake {
    pub piece: Piece,
    pub end: Piece,
    pub removed: Piece,
    pub from: Coord,
    pub to: Coord,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Passant {
    pub piece: Piece,
    pub removed: Piece,
    pub from: Coord,
    pub to: Coord,
    pub removed_from: Coord,
}

impl Castle {
    pub fn new(king: Piece, rook: Piece, king_from: Coord, king_to: Coord, rook_from: Coord, rook_to: Coord) -> Self {
        Castle { king, rook, king_from, king_to, rook_from, rook_to }
    }
}

impl Move {
    pub fn new(piece: Piece, from: Coord, to: Coord) -> Self {
        Move { piece, from, to }
    }
}

impl Take {
    pub fn new(piece: Piece, removed: Piece, from: Coord, to: Coord) -> Self {
        Take { piece, removed, from, to }
    }
}

impl Promote {
    pub fn new(piece: Piece, end: Piece, from: Coord, to: Coord) -> Self {
        Promote { piece, end, from, to }
    }
}

impl PromoteTake {
    pub fn new(piece: Piece, end: Piece, removed: Piece, from: Coord, to: Coord) -> Self {
        PromoteTake { piece, end, removed, from, to }
    }
}

impl Passant {
    pub fn new(piece: Piece, removed: Piece, from: Coord, to: Coord, removed_from: Coord) -> Self {
        Passant { piece, removed, from, to, removed_from }
    }
}

