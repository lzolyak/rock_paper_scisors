extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;


#[derive(Copy, Clone, PartialEq)]
enum Move{
    Rock,
    Paper,
    Scisors,
    Quit,
    Invalid
}

struct Score{
   games : i32,
   wins : i32,
   losses : i32,
   ties : i32,
   rocks : i32,
   papers: i32,
   scisors : i32
}

fn main() {
    let mut player = String::new();
    println!("Please enter a player name: ");
    io::stdin().read_line(&mut player);

    let mut score_board : Score = Score{games : 0, wins : 0, losses : 0, ties : 0, rocks: 0, papers: 0, scisors: 0};

  
loop{
    score_board.games = score_board.games + 1;
    let compMove = computer_move();
   
    let mut playerMove = String::new();
    println!("Enter choice (r,p,s) or q to quit > ");
    io::stdin().read_line(&mut playerMove);

    //check if our input was one char and a terminator
    if(playerMove.len() == 2){
 
	//trim our input       
	playerMove.truncate(1);
    	
	if(playerMove != "q".to_string()){
	//figure out what the player entered
	let mv = parse_input(playerMove, &mut score_board);

	print_moves(compMove, mv);

	let mut win = calculate_winner(compMove, mv, &mut score_board);

	if(win == 1){ println!("You win!");}
	else if(win == 2){ println!("You lose!");}
	if(win == 3){ println!("You tied!");}
   	}
	else{
		println!("Player Stats: ");
		let mut w : f32 = (score_board.wins as f32/ score_board.games as f32) *100.0;
		println!("Wins: {} ({:.2} %)", score_board.wins, w);

		let mut l : f32 = (score_board.losses as f32/ score_board.games as f32) * 100.0;
		println!("Losses: {} ({:.2} %)", score_board.losses, l);

		let mut t : f32 = (score_board.ties as f32 / score_board.games as f32) * 100.0;
		println!("Ties: {} ({:.2} %)", score_board.ties, t);

		println!("Rocks: {}", score_board.rocks);
		println!("Papers: {}", score_board.papers);
		println!("Scisors: {}", score_board.scisors);
        break;
	}
}
}

}

fn computer_move()-> Move{
     let rand_num = rand::thread_rng().gen_range(1, 4);

     if(rand_num == 1){
        return Move::Rock;
     }
     else if(rand_num == 2){
        return Move::Paper;
     }
     else if(rand_num == 3){
        return Move::Scisors;
     }
return Move::Invalid;
}

fn parse_input(input: String, mut current_move: &mut Score )-> Move{
    if input == "r".to_string(){
	current_move.rocks = current_move.rocks + 1;
        return Move::Rock;
    }
    else if input == "p".to_string(){
	current_move.papers = current_move.papers + 1;
        return Move::Paper;
    }
    else if input == "s".to_string(){
	current_move.scisors = current_move.scisors +1;
        return Move::Scisors;
    }
    else if input == "q".to_string(){
	return Move::Quit;
    }
return Move::Invalid;
}

fn print_moves(comp: Move, human: Move) -> (){

        match human{
            Move::Rock => println!("Player chose: Rock"),
            Move::Paper => println!("Player chose: Paper"),
            Move::Scisors => println!("Player chose: Scisors"),
            Move::Quit => println!("Player chose: Quit"),
            Move::Invalid => println!("Player chose: Invalid")
	}


        match comp{
            Move::Rock => println!("Opponent chose: Rock"),
            Move::Paper => println!("Opponent chose: Paper"),
            Move::Scisors => println!("Opponent chose: Scisors"),
            Move::Quit => println!("Opponent chose: Quit"),
            Move::Invalid => println!("Opponent chose: Invalid")
	}
}


//returns 1 for win, 2 for loss, 3 for tie, 0 if there was a problem
fn calculate_winner(comp: Move, human: Move, mut current_move: &mut Score) -> i32{
    if(comp == human){
	current_move.ties = current_move.ties +1;
        return 3;
    }
    let mut human_num = 0;

        match human{
            Move::Rock => human_num = 1,
            Move::Paper => human_num = 2,
            Move::Scisors =>  human_num = 3,
            Move::Quit =>  human_num = 4,
            Move::Invalid => human_num = 5,
	}

    let mut comp_num = 0;

     match comp{
            Move::Rock => comp_num = 1,
            Move::Paper => comp_num = 2,
            Move::Scisors =>  comp_num = 3,
            Move::Quit =>  comp_num = 4,
            Move::Invalid => comp_num = 5,
    }

    //rock beaten by paper
    if(human_num == 1 && comp_num == 2){
	current_move.losses = current_move.losses + 1;
	return 2;
    }
    //rock beats scisors
    else if(human_num == 1 && comp_num == 3){
	current_move.wins = current_move.wins + 1;
	return 1;
    }
    //paper beats rock
    else if(human_num == 2 && comp_num == 1){
	current_move.wins = current_move.wins + 1;
	return 1;
    }
    //paper beaten by scisors
    else if(human_num == 2 && comp_num == 3){
	current_move.losses = current_move.losses + 1;
	return 2;
    }
    //Scisors beaten by rock
    else if(human_num == 3 && comp_num == 1){
	current_move.losses = current_move.losses + 1;
	return 2;
    }
    //scisors beat paper
    else if(human_num == 2 && comp_num == 3){
	current_move.wins = current_move.wins + 1;
	return 1;
    }

return 0;
}
