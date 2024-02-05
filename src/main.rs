use chess::game::play_game;
use chess::strategy::*;



fn main() {
    /*
    let mut g = ChessGame::new();
    
    g.board.set((6, 7).into(), Empty);
    g.board.set((6, 1).into(), Empty);
    g.board.set((6, 3).into(), Empty);
    g.board.set((7, 6).into(), Empty);
    g.board.set((7, 5).into(), Empty);
    println!("{}", g.board);
    let moves = g.possible_moves(g.turn.toggle());
    for m in moves {
        println!("{:?}", m);
    }
    println!("In check: {}", g.in_check(g.turn));
   
    */
    /* 
    let mut g = ChessGame::blank();
    g.board.set((3,4).into(), Pawn(0, Black));
    g.board.set((4,5).into(), Rook(1, White));
    println!("{}", g);
   
    println!("Available moves:");
    let moves = g.available_moves(Black);
    for m in moves {
        println!("{:?}", m);
    }
    //let moves = g.available_moves(Black);
    //for m in moves {
    //    println!("{:?}", m);
    //}
    */
     
    let state = play_game(&equal_opportunity, &random_player);
    println!("Final state: {:?}", state);
    
}
