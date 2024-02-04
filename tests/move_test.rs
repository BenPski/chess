use chess::game::ChessGame;
use chess::piece::Piece::*;
use chess::player::Player::*;
use chess::action::Action::*;
use chess::action::*;

#[test]
fn king_moves() {
    let mut game = ChessGame::blank();
    let king = King(0, Black);
    let orig = (4,4).into();
    game.board.set(orig, king);
    let mut moves = game.available_moves(Black);
    let mut expected = vec![
        AMove(Move::new(king, orig, orig + (0,1).into())),
        AMove(Move::new(king, orig, orig + (0,-1).into())),
        AMove(Move::new(king, orig, orig + (1,0).into())),
        AMove(Move::new(king, orig, orig + (-1,0).into())),
        AMove(Move::new(king, orig, orig + (1,1).into())),
        AMove(Move::new(king, orig, orig + (1,-1).into())),
        AMove(Move::new(king, orig, orig + (-1,1).into())),
        AMove(Move::new(king, orig, orig + (-1,-1).into())),
    ];

    moves.sort();
    expected.sort();
    assert_eq!(moves, expected);
}

#[test]
fn knight_moves() {
    let mut game = ChessGame::blank();
    let knight = Knight(0, Black);
    let orig = (4,4).into();
    game.board.set(orig, knight);
    let mut moves = game.available_moves(Black);
    let mut expected = vec![
        AMove(Move::new(knight, orig, orig + (1,2).into())),
        AMove(Move::new(knight, orig, orig + (2,1).into())),
        AMove(Move::new(knight, orig, orig + (-1,2).into())),
        AMove(Move::new(knight, orig, orig + (-2,1).into())),
        AMove(Move::new(knight, orig, orig + (1,-2).into())),
        AMove(Move::new(knight, orig, orig + (2,-1).into())),
        AMove(Move::new(knight, orig, orig + (-1,-2).into())),
        AMove(Move::new(knight, orig, orig + (-2,-1).into())),
    ];

    moves.sort();
    expected.sort();
    assert_eq!(moves, expected);
}

#[test]
fn bishop_moves() {
    let mut game = ChessGame::blank();
    let bishop = Bishop(0, Black);
    let orig = (4,4).into();
    game.board.set(orig, bishop);
    let mut moves = game.available_moves(Black);
    let mut expected = vec![
        AMove(Move::new(bishop, orig, orig + (1,1).into())),
        AMove(Move::new(bishop, orig, orig + (2,2).into())),
        AMove(Move::new(bishop, orig, orig + (3,3).into())),
        AMove(Move::new(bishop, orig, orig + (-1,-1).into())),
        AMove(Move::new(bishop, orig, orig + (-2,-2).into())),
        AMove(Move::new(bishop, orig, orig + (-3,-3).into())),
        AMove(Move::new(bishop, orig, orig + (-4,-4).into())),
        AMove(Move::new(bishop, orig, orig + (-1,1).into())),
        AMove(Move::new(bishop, orig, orig + (-2,2).into())),
        AMove(Move::new(bishop, orig, orig + (-3,3).into())),
        AMove(Move::new(bishop, orig, orig + (1,-1).into())),
        AMove(Move::new(bishop, orig, orig + (2,-2).into())),
        AMove(Move::new(bishop, orig, orig + (3,-3).into())),
    ];

    moves.sort();
    expected.sort();
    assert_eq!(moves, expected);
}

#[test]
fn rook_moves() {
    let mut game = ChessGame::blank();
    let rook = Rook(0, Black);
    let orig = (4,4).into();
    game.board.set(orig, rook);
    let mut moves = game.available_moves(Black);
    let mut expected = vec![
        AMove(Move::new(rook, orig, orig + (1,0).into())),
        AMove(Move::new(rook, orig, orig + (2,0).into())),
        AMove(Move::new(rook, orig, orig + (3,0).into())),
        AMove(Move::new(rook, orig, orig + (-1,0).into())),
        AMove(Move::new(rook, orig, orig + (-2,0).into())),
        AMove(Move::new(rook, orig, orig + (-3,0).into())),
        AMove(Move::new(rook, orig, orig + (-4,0).into())),
        AMove(Move::new(rook, orig, orig + (0,1).into())),
        AMove(Move::new(rook, orig, orig + (0,2).into())),
        AMove(Move::new(rook, orig, orig + (0,3).into())),
        AMove(Move::new(rook, orig, orig + (0,-1).into())),
        AMove(Move::new(rook, orig, orig + (0,-2).into())),
        AMove(Move::new(rook, orig, orig + (0,-3).into())),
        AMove(Move::new(rook, orig, orig + (0,-4).into())),
    ];

    moves.sort();
    expected.sort();
    assert_eq!(moves, expected);
}

#[test]
fn queen_moves() {
    let mut game = ChessGame::blank();
    let queen = Queen(0, Black);
    let orig = (4,4).into();
    game.board.set(orig, queen);
    let mut moves = game.available_moves(Black);
    let mut expected = vec![
        AMove(Move::new(queen, orig, orig + (1,0).into())),
        AMove(Move::new(queen, orig, orig + (2,0).into())),
        AMove(Move::new(queen, orig, orig + (3,0).into())),
        AMove(Move::new(queen, orig, orig + (-1,0).into())),
        AMove(Move::new(queen, orig, orig + (-2,0).into())),
        AMove(Move::new(queen, orig, orig + (-3,0).into())),
        AMove(Move::new(queen, orig, orig + (-4,0).into())),
        AMove(Move::new(queen, orig, orig + (0,1).into())),
        AMove(Move::new(queen, orig, orig + (0,2).into())),
        AMove(Move::new(queen, orig, orig + (0,3).into())),
        AMove(Move::new(queen, orig, orig + (0,-1).into())),
        AMove(Move::new(queen, orig, orig + (0,-2).into())),
        AMove(Move::new(queen, orig, orig + (0,-3).into())),
        AMove(Move::new(queen, orig, orig + (0,-4).into())),
        AMove(Move::new(queen, orig, orig + (1,1).into())),
        AMove(Move::new(queen, orig, orig + (2,2).into())),
        AMove(Move::new(queen, orig, orig + (3,3).into())),
        AMove(Move::new(queen, orig, orig + (-1,-1).into())),
        AMove(Move::new(queen, orig, orig + (-2,-2).into())),
        AMove(Move::new(queen, orig, orig + (-3,-3).into())),
        AMove(Move::new(queen, orig, orig + (-4,-4).into())),
        AMove(Move::new(queen, orig, orig + (-1,1).into())),
        AMove(Move::new(queen, orig, orig + (-2,2).into())),
        AMove(Move::new(queen, orig, orig + (-3,3).into())),
        AMove(Move::new(queen, orig, orig + (1,-1).into())),
        AMove(Move::new(queen, orig, orig + (2,-2).into())),
        AMove(Move::new(queen, orig, orig + (3,-3).into())),
    ];

    moves.sort();
    expected.sort();
    assert_eq!(moves, expected);
}

#[test]
fn pawn_moves() {
    let mut game = ChessGame::blank();
    let pawn = Pawn(0, Black);
    let orig = (4,4).into();
    game.board.set(orig, pawn);
    let mut moves = game.available_moves(Black);
    // hasn't moved before
    let mut expected = vec![
        AMove(Move::new(pawn, orig, orig + Black.pawn_dir())),
        AMove(Move::new(pawn, orig, orig + Black.pawn_dir() + Black.pawn_dir())),
    ];

    moves.sort();
    expected.sort();
    assert_eq!(moves, expected);
}

#[test]
fn castling() {
    let mut game = ChessGame::blank();
    let rook = Rook(0, Black);
    let rook_orig = (0,0).into();
    let king = King(1, Black);
    let king_orig = (0,3).into();
    game.board.set(rook_orig, rook);
    game.board.set(king_orig, king);
    let moves = game.available_moves(Black);
    let castle = ACastle(Castle::new(king, rook, king_orig, king_orig + (0, -2).into(), rook_orig, king_orig + (0, -1).into()));
    assert!(moves.contains(&castle));
}

#[test]
fn no_castling() {
    let mut game = ChessGame::blank();
    let rook = Rook(0, Black);
    let rook_orig = (0,0).into();
    let king = King(1, Black);
    let king_orig = (0,3).into();
    let op_rook = Rook(2, White);
    game.board.set((7, 2).into(), op_rook);
    game.board.set(rook_orig, rook);
    game.board.set(king_orig, king);
    let moves = game.available_moves(Black);
    let castle = ACastle(Castle::new(king, rook, king_orig, king_orig + (0, -2).into(), rook_orig, king_orig + (0, -1).into()));
    assert!(!moves.contains(&castle));
}

#[test]
fn en_passant() {
    let mut game = ChessGame::blank();
    let pawn1 = Pawn(0, Black);
    let pawn2 = Pawn(1, White);
    let orig = (1,3).into();
    game.board.set(orig, pawn1);
    game = game.step(AMove(Move::new(pawn1, orig, orig + (2, 0).into())));
    game.board.set((3,4).into(), pawn2);
    let moves = game.available_moves(White);
    let passant = APassant(Passant::new(pawn2, pawn1, (3,4).into(), (2,3).into(), (3,3).into()));

    assert!(moves.contains(&passant));
}

#[test]
fn knight_jump() {
    let mut game = ChessGame::blank();
    let pawn1 = Pawn(0, Black);
    let pawn2 = Pawn(1, White);
    let knight = Knight(2, Black);
    game.board.set((0,0).into(), knight);
    game.board.set((1,0).into(), pawn1);
    game.board.set((0,1).into(), pawn2);
    let moves = game.available_moves(Black);

    let move1 = AMove(Move::new(knight, (0,0).into(), (1,2).into()));
    let move2 = AMove(Move::new(knight, (0,0).into(), (2,1).into()));

    assert!(moves.contains(&move1));
    assert!(moves.contains(&move2));
}

#[test]
fn rook_take() {
    let mut game = ChessGame::blank();
    let pawn = Pawn(0, White);
    let rook = Rook(1, Black);
    game.board.set((1,2).into(), pawn);
    game.board.set((4,2).into(), rook);
    let moves = game.available_moves(Black);

    let take = ATake(Take::new(rook, pawn, (4,2).into(), (1,2).into()));

    assert!(moves.contains(&take));
    assert_eq!(moves.into_iter().filter(|x| matches!(x, ATake(_))).count(), 1);
}

#[test]
fn rook_take1() {
    let mut game = ChessGame::blank();
    let pawn1 = Pawn(0, White);
    let pawn2 = Pawn(3, White);
    let rook = Rook(1, Black);
    game.board.set((1,2).into(), pawn1);
    game.board.set((0,2).into(), pawn2);
    game.board.set((4,2).into(), rook);
    let moves = game.available_moves(Black);

    let take1 = ATake(Take::new(rook, pawn1, (4,2).into(), (1,2).into()));
    let take2 = ATake(Take::new(rook, pawn1, (4,2).into(), (0,2).into()));

    assert!(moves.contains(&take1));
    assert!(!moves.contains(&take2));

}

#[test]
fn rook_blocked() {
    let mut game = ChessGame::blank();
    let pawn1 = Pawn(0, Black);
    let pawn2 = Pawn(3, White);
    let rook = Rook(1, Black);
    game.board.set((1,2).into(), pawn1);
    game.board.set((0,2).into(), pawn2);
    game.board.set((4,2).into(), rook);
    let moves = game.available_moves(Black);

    assert_eq!(moves.into_iter().filter(|x| matches!(x, ATake(_))).count(), 0);
}

#[test]
fn queen_take() {
    let mut game = ChessGame::blank();
    let pawn = Pawn(0, White);
    let queen = Queen(1, Black);
    game.board.set((1,2).into(), pawn);
    game.board.set((4,2).into(), queen);
    let moves = game.available_moves(Black);

    let take = ATake(Take::new(queen, pawn, (4,2).into(), (1,2).into()));

    assert!(moves.contains(&take));
    assert_eq!(moves.into_iter().filter(|x| matches!(x, ATake(_))).count(), 1);
}

#[test]
fn queen_take1() {
    let mut game = ChessGame::blank();
    let pawn1 = Pawn(0, White);
    let pawn2 = Pawn(3, White);
    let queen = Queen(1, Black);
    game.board.set((1,2).into(), pawn1);
    game.board.set((0,2).into(), pawn2);
    game.board.set((4,2).into(), queen);
    let moves = game.available_moves(Black);

    let take1 = ATake(Take::new(queen, pawn1, (4,2).into(), (1,2).into()));
    let take2 = ATake(Take::new(queen, pawn1, (4,2).into(), (0,2).into()));

    assert!(moves.contains(&take1));
    assert!(!moves.contains(&take2));

}

#[test]
fn queen_blocked() {
    let mut game = ChessGame::blank();
    let pawn1 = Pawn(0, Black);
    let pawn2 = Pawn(3, White);
    let queen = Queen(1, Black);
    game.board.set((1,2).into(), pawn1);
    game.board.set((0,2).into(), pawn2);
    game.board.set((4,2).into(), queen);
    let moves = game.available_moves(Black);

    assert_eq!(moves.into_iter().filter(|x| matches!(x, ATake(_))).count(), 0);
}

#[test]
fn bishop_take() {
    let mut game = ChessGame::blank();
    let pawn = Pawn(0, White);
    let bishop = Bishop(1, Black);
    game.board.set((1,1).into(), pawn);
    game.board.set((4,4).into(), bishop);
    let moves = game.available_moves(Black);

    let take = ATake(Take::new(bishop, pawn, (4,4).into(), (1,1).into()));

    assert!(moves.contains(&take));
    assert_eq!(moves.into_iter().filter(|x| matches!(x, ATake(_))).count(), 1);
}

#[test]
fn bishop_take1() {
    let mut game = ChessGame::blank();
    let pawn1 = Pawn(0, White);
    let pawn2 = Pawn(3, White);
    let bishop = Bishop(1, Black);
    game.board.set((1,1).into(), pawn1);
    game.board.set((0,0).into(), pawn2);
    game.board.set((4,4).into(), bishop);
    let moves = game.available_moves(Black);

    let take1 = ATake(Take::new(bishop, pawn1, (4,4).into(), (1,1).into()));
    let take2 = ATake(Take::new(bishop, pawn1, (4,4).into(), (0,0).into()));

    assert!(moves.contains(&take1));
    assert!(!moves.contains(&take2));

}

#[test]
fn bishop_blocked() {
    let mut game = ChessGame::blank();
    let pawn1 = Pawn(0, Black);
    let pawn2 = Pawn(3, White);
    let bishop = Bishop(1, Black);
    game.board.set((1,1).into(), pawn1);
    game.board.set((0,0).into(), pawn2);
    game.board.set((4,4).into(), bishop);
    let moves = game.available_moves(Black);

    assert_eq!(moves.into_iter().filter(|x| matches!(x, ATake(_))).count(), 0);
}

#[test]
fn pawn_take() {
    let mut game = ChessGame::blank();
    let pawn = Pawn(0, White);
    let rook = Rook(2, Black);
    game.board.set((4,5).into(), pawn);
    game.board.set((3,4).into(), rook);
    let moves = game.available_moves(White);

    let take = ATake(Take::new(pawn, rook, (4,5).into(), (3,4).into()));

    assert!(moves.contains(&take));
}

#[test]
fn all_promote() {
    let mut game = ChessGame::blank();
    let pawn = Pawn(0, White);
    game.board.set((2,0).into(), pawn);
    game = game.step(AMove(Move::new(pawn, (2,0).into(), (1,0).into())));
    let moves = game.available_moves(White);

    let promote_queen = APromote(Promote::new(pawn, Queen(0, White), (1,0).into(), (0,0).into()));
    let promote_rook = APromote(Promote::new(pawn, Rook(0, White), (1,0).into(), (0,0).into()));
    let promote_bishop = APromote(Promote::new(pawn, Bishop(0, White), (1,0).into(), (0,0).into()));
    let promote_knight = APromote(Promote::new(pawn, Knight(0, White), (1,0).into(), (0,0).into()));

    assert!(moves.contains(&promote_queen));
    assert!(moves.contains(&promote_rook));
    assert!(moves.contains(&promote_bishop));
    assert!(moves.contains(&promote_knight));
}

#[test]
fn all_promote_take() {
    let mut game = ChessGame::blank();
    let pawn = Pawn(0, White);
    let rook = Rook(1, Black);
    game.board.set((2,0).into(), pawn);
    game = game.step(AMove(Move::new(pawn, (2,0).into(), (1,0).into())));
    game.board.set((0,1).into(), rook);
    let moves = game.available_moves(White);

    let promote_queen = APromoteTake(PromoteTake::new(pawn, Queen(0, White), rook, (1,0).into(), (0,1).into()));
    let promote_rook = APromoteTake(PromoteTake::new(pawn, Rook(0, White), rook, (1,0).into(), (0,1).into()));
    let promote_bishop = APromoteTake(PromoteTake::new(pawn, Bishop(0, White), rook, (1,0).into(), (0,1).into()));
    let promote_knight = APromoteTake(PromoteTake::new(pawn, Knight(0, White), rook, (1,0).into(), (0,1).into()));

    assert!(moves.contains(&promote_queen));
    assert!(moves.contains(&promote_rook));
    assert!(moves.contains(&promote_bishop));
    assert!(moves.contains(&promote_knight));

}
