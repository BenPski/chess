use std::fmt::Display;
use coord::Coord;
use Player::*;
use Piece::*;

use crate::{player::Player, coord};

pub type ID = u8;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    Empty,
    Pawn(ID, Player),
    Rook(ID, Player),
    Knight(ID, Player),
    Bishop(ID, Player),
    Queen(ID, Player),
    King(ID, Player),
}

impl Piece {
    pub fn owner(&self) -> Option<Player> {
        match self {
            Empty => None,
            Pawn(_, p)   => Some(*p),
            Rook(_, p)   => Some(*p),
            Knight(_, p) => Some(*p),
            Bishop(_, p) => Some(*p),
            Queen(_, p)  => Some(*p),
            King(_, p)   => Some(*p),
        }
    }

    pub fn id(&self) -> Option<ID> {
        match self {
            Empty => None,
            Pawn(id, _)   => Some(*id),
            Rook(id, _)   => Some(*id),
            Knight(id, _) => Some(*id),
            Bishop(id, _) => Some(*id),
            Queen(id, _)  => Some(*id),
            King(id, _)   => Some(*id),
        }
    }

    pub fn owned_by(&self, player: Player) -> bool {
        if let Some(p) = self.owner() {
            p == player
        } else {
            false
        }
    }

    pub fn empty(&self) -> bool {
        match self {
            Empty => true,
            _ => false,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PieceData {
    pub piece: Piece,
    pub coord: Coord,
}

impl PieceData {
    pub fn new(piece: Piece, coord: Coord) -> Self {
        PieceData { piece , coord }
    }

    // can this piece reach the coordinate in a single step as an attack
    // for pawns en passant needs to be verified as realistic separately
    pub fn can_reach(&self, coord: Coord) -> bool {
        match self.piece {
            Pawn(_, owner) => {
                let dir = owner.pawn_dir().row; //if owner == Black { 1 } else { -1 };
                let row_diff = coord.row - self.coord.row;
                let col_diff = coord.col - self.coord.col;
                (row_diff, col_diff) == (dir, 1) || (row_diff, col_diff) == (dir, -1)
            }
            Bishop(_, _) => {
                let row_diff = (self.coord.row - coord.row).abs();
                let col_diff = (self.coord.col - coord.col).abs();
                row_diff == col_diff
            }
            Rook(_, _) => {
                let row_diff = (self.coord.row - coord.row).abs();
                let col_diff = (self.coord.col - coord.col).abs();
                row_diff == 0 || col_diff == 0
            }
            Knight(_, _) => {
                let row_diff = (self.coord.row - coord.row).abs();
                let col_diff = (self.coord.col - coord.col).abs();
                (row_diff, col_diff) == (2,1) || (row_diff, col_diff) == (1,2)
            }
            Queen(_, _) => {
                let row_diff = (self.coord.row - coord.row).abs();
                let col_diff = (self.coord.col - coord.col).abs();
                row_diff == col_diff || row_diff == 0 || col_diff == 0
            }
            King(_, _) => {
                let row_diff = (self.coord.row - coord.row).abs();
                let col_diff = (self.coord.col - coord.col).abs();
                (row_diff, col_diff) == (1,1) || (row_diff, col_diff) == (1,0) || (row_diff, col_diff) == (0, 1)
            }
            _ => false
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, " "),
            Self::Pawn(_,White)   => write!(f, "♟︎"),
            Self::Pawn(_,Black)   => write!(f, "♙"),
            Self::Rook(_,White)   => write!(f, "♜"),
            Self::Rook(_,Black)   => write!(f, "♖"),
            Self::Knight(_,White) => write!(f, "♞"),
            Self::Knight(_,Black) => write!(f, "♘"),
            Self::Bishop(_,White) => write!(f, "♝"),
            Self::Bishop(_,Black) => write!(f, "♗"),
            Self::Queen(_,White)  => write!(f, "♛"),
            Self::Queen(_,Black)  => write!(f, "♕"),
            Self::King(_,White)   => write!(f, "♚"),
            Self::King(_,Black)   => write!(f, "♔"),
        }
    }
}
