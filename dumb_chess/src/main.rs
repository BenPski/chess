use clap::Parser;
use dumb_chess::game::play_game;
use dumb_chess::strategy::*;

use enum_iterator::all;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long, default_value="Random")]
    white_player: String,
    #[arg(short, long, default_value="Random")]
    black_player: String,
    #[arg(short, long)]
    list: bool
}


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
    
    let cli = Cli::parse();
    
    if cli.list {
        println!("Available strategies:");
        for s in all::<Strategy>() {
            println!("{}", s.name());
        }
    } else {
        let white = cli.white_player;
        let black = cli.black_player;

        if let (Some(white_strat), Some(black_strat)) = (strategy_map().get(&white), strategy_map().get(&black)) {
            let state = play_game(*black_strat, *white_strat);
            println!("Final state: {:?}", state);
        } else {
            println!("Could not find one of the given strategies: {} + {}", white, black);
            println!("Options for strategies are:");
            for s in all::<Strategy>() {
                println!("{}", s.name());
            }
        }
    }
}
