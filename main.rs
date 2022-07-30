
// Salvo 
// Written by John R. Bruhling	
// https://github.com/autotunafish

 /////.   //|\\   |||     \\\     ///    ,|||||,
//,,     //  \\   |||      \\\   ///   /||'  '||\
   //,  //===\\   |||       \\\ ///    |||,  ,|||
/////  //    \\   |||||||    \\///      \\|||//'  

		
extern crate rand;
use std::io;
use rand::prelude::*;
use std::convert::TryInto;

fn main() {
 
 //We instantiate mut vecs for each the user and comp board
 //and pieces; A 10x10 grid is represented by 100 elements 
 //(x4), 2 for the players' pieces and 2 to mark the shots. 
 //The 5 smaller vectors' (x2) inner elements will
 //be consumed into the comp and user boards respectively.
 //The catk and uatk boards mark the attempted shots and are
 //basically mirrors of the other players main board minus the 
 //ship locations.
 
 let mut comp_board = Vec::new();
 let mut user_board = Vec::new();
 let mut catk_board = Vec::new();
 let mut uatk_board = Vec::new();
 
 //The char Z represents an open spot. These vecs handle chars 
 //while others handle Strings...
 for _i in 0..100 {
    user_board.push('Z');
    comp_board.push('Z');
    catk_board.push('Z');
    uatk_board.push('Z');
    }
    
 //A - E chars show the ships listed in the pieces array below.
 //These are called to label the ship when it is  time to be 
 //placed. Different chars will mark hits and misses and when 
 //none of a particular letter matched on anymore, it is sunk.
 //The first letter of the ship names conveniently correlate.
let user_acc = vec!('A', 'A', 'A', 'A', 'A');
let comp_acc = vec!('A', 'A', 'A', 'A', 'A');
let user_bts = vec!('B', 'B', 'B', 'B', 'B');
let comp_bts = vec!('B', 'B', 'B', 'B', 'B');
let user_crs = vec!('C', 'C', 'C', 'C');
let comp_crs = vec!('C', 'C', 'C', 'C');
let user_dst = vec!('D', 'D', 'D');
let comp_dst = vec!('D', 'D', 'D');
let user_esc = vec!('E', 'E');
let comp_esc = vec!('E', 'E');

//The number String in the pieces array indicates the ship length.
let pieces = [ 
	["Aircraft Carrier ", "5"],
	["Battleship       ", "5"],
	["Cruiser          ", "4"],
	["Destroyer        ", "3"],
	["Escort           ", "2"]
		];
		
let directions = ["E", "W", "S", "N"];

//The value i represents the length of the pieces array. It's
//incremented to the array length and breaks the loop once 
//all 5 pieces are set. It's also called to correlate the piece
//lengths as well.
let mut i = 0;

//prints the vector to see initially.
	thead();
	makescreen(&uatk_board);	
	phead();
	makescreen(&user_board);
	println!("\x1b[48;5;125m\x1b[38;5;118m            New Game!            \x1b[0m");
	let input3 = String::new();
	
loop {
	//Begin the USER game setup.
	//The user first sets the piece position and then 
	//direction. 
	
	println!("\x1b[48;5;112m\x1b[38;5;21m Enter position {}\x1b[0m", &pieces[i][0]);
    let input = String::new();
    let mut input2 = String::new();
    let mut allowed = Vec::new();
    
    //The get_input fn takes user io and formats the input.
    let mut input = get_input(input);
		
	//The input should now be reduced to only 2 elements 
	//after being passed into the function; if not then try again.
	//First a header is printed followed by both boards.
	//invleng and invchar print the error messages.
	if input.len() != 2 {
		thead();
		makescreen(&uatk_board);
		phead();
		makescreen(&user_board);
		invleng();

		continue;
		}
		
	//The String is halved yielding the 2 values as Strings.
	let second = input.split_off(1);
	
	//The check_valid fn only checks whether the input values 
	//will match to a valid character (of &String) of 
	//A-J and 0-9. It does not check yet that one is a letter 
	//and the other a number.
	if check_valid(&input) == false {
		thead();
		makescreen(&uatk_board);
		phead();
		makescreen(&user_board);
		invchar();
		continue;
		}
	if check_valid(&second) == false {
		thead();
		makescreen(&uatk_board);
		phead();
		makescreen(&user_board);
		invchar();
		continue;
		}
	
	//The fn check_correct checks that one of the inputs is
	//a letter and the other a number in order to get the 
	//correct position from the cartesian plane system. This
	//fn is almost identical to the last except that it checks 
	//for only a letter A-J twice looking for both a true and 
	//false.
	if check_correct(&input) == check_correct(&second) {
		thead();
		makescreen(&uatk_board);
		phead();
		makescreen(&user_board);
		invchar();
		continue;
	}
		
	//We use this to convert to the index.
	let dval = convert_pos(&input, &second);
	
	//The check_position fn takes the index of &inputs and 
	//the &user_board vector and quantifies the values into a 
	//location in the vec as a len(). It then checks whether or 
	//not this location is already occupied by another piece.
	if check_position(&dval, &user_board) == false {
		thead();
		makescreen(&uatk_board);
		phead();
		makescreen(&user_board);
		println!("\x1b[48;5;130m\x1b[38;5;21m  Position occupied, try again   \x1b[0m");
		continue;
	}
	
	//The check_direction_#'s are for calculating valid directions, 
	//takes same references as the first check along with a ship len.
	//The fn checks whether the piece will a. overrun the total len 
	//of the board vector, b. overrun the virtual borders of the 
	//10x10 grid and c. if it crosses another ship.
	//The allowed directions are pushed into the allowed vec.
	//h is the length associated with each ship called through the 
	//i incrementor.
	let h = (&pieces[i][1]).to_string();
	
	if check_direction_e(&dval, &user_board, &h) == true {
		allowed.push("E");
	}
	if check_direction_w(&dval, &user_board, &h) == true {
		allowed.push("W");
	}
	if check_direction_s(&dval, &user_board, &h) == true {
		allowed.push("S");
	}
	if check_direction_n(&dval, &user_board, &h) == true {
		allowed.push("N");
	}
	
	//The allowed directions, if any, are displayed ..
	if allowed.len() == 0 {
		thead();
		makescreen(&uatk_board);
		phead();
		makescreen(&user_board);
		println!("\x1b[48;5;130m\x1b[38;5;21m    No directions, try again     \x1b[0m");
		continue;
	}
	thead();
	makescreen(&uatk_board);
	phead();
	makescreen(&user_board);
	println!("\x1b[48;5;112m\x1b[38;5;21m  Enter Direction, X to return   \x1b[0m");
	
	let mut dirind = false;
	loop {
		print!("\x1b[48;5;112m\x1b[38;5;21m Allowed Directions -> \x1b[0m");
		match allowed.len() {
		1 => print!("\x1b[48;5;112m\x1b[38;5;52m.{}      . \x1b[0m\n", &allowed[0]),
		2 => print!("\x1b[48;5;112m\x1b[38;5;52m.{} {}    . \x1b[0m\n", &allowed[0], &allowed[1]),
		3 => print!("\x1b[48;5;112m\x1b[38;5;52m.{} {} {}  . \x1b[0m\n", &allowed[0], allowed[1], allowed[2]),
		4 => print!("\x1b[48;5;112m\x1b[38;5;52m.{} {} {} {}. \x1b[0m\n", &allowed[0], allowed[1], allowed[2], allowed[3]),
		_ => ()
		}
		
	//..and matched against the input to check for validity.
	//OK sets dirind to true and breaks the loop, else retry.
	//"X" returns to the position input, weird input is discarded.
		input2 = get_input(input2);
		
		if input2 == "X" {
			thead();
			makescreen(&uatk_board);
			phead();
			makescreen(&user_board);
			println!("\x1b[48;5;130m\x1b[38;5;21m    X -> Return                  \x1b[0m");
			break;
			}
		if input2.len() != 1 {
			thead();
			makescreen(&uatk_board);
			phead();
			makescreen(&user_board);
			invleng();
			input2.clear();
			continue;
			} 
			for a in &allowed {
				if &input2.as_str() == a {
					dirind = true;
					break;
					}
				}
			if dirind != true {
			thead();
			makescreen(&uatk_board);
			phead();
			makescreen(&user_board);
			invchar();
			input2.clear();
			continue;
			} else {
				break;
			}
		}
	if input2 == "X" {
		input2.clear();
		continue;
	} 
	
	//The ship vec is selected according to i and cloned as 
	//is the user_board.
		let w = user_board.clone();
		let mut q = Vec::new();
		match i {
		0 => { q = user_acc.clone();},
		1 => { q = user_bts.clone();},
		2 => { q = user_crs.clone();},
		3 => { q = user_dst.clone();},
		4 => { q = user_esc.clone();},
		_ => ()
		}
	
	//The piece is now placed in the selected direction.
	//The user_board is ='d as the mutated return of it's 
	//clone w. 
	match input2.as_str() {
		"S" => { user_board = set_direction_s(&dval, w, &h, q) },
		"N" => { user_board = set_direction_n(&dval, w, &h, q) },
		"E" => { user_board = set_direction_e(&dval, w, &h, q) },
		"W" => { user_board = set_direction_w(&dval, w, &h, q) },
		_ => ()
	}
	
	//The input values are cleared for re-use (as any 
	//other instance prior).
	
	if dirind == true {
				thead();
				makescreen(&uatk_board);
				phead();
				makescreen(&user_board);
				println!("\x1b[48;5;112m\x1b[38;5;52m Direction {} -> OK               \x1b[0m", &input2);
			}
			
	input.clear();
	input2.clear();
	
	//The var i controls the loop according to the fixed
	//len of the pieces array. 
	i += 1;
	if i == 5 {
		break;
		}
	}//end user setup loop
		
		
		/////////////////////////////////////////////////////////
		////////////////////////////////////////////////////////
		////////////////////////////////////////////////////////
		
//incrementor is reset	
	i = 0;
	
		//The computer now places its pieces in a basically 
		//identical manner.
		
		//The fn get_compinput mimics the user input fn and gens the 
		//letter-number combo as one string and then splits off as
		//before. The comp is limited to valid input only and so the
		//unecessary validity checks are skipped.
loop {	
		let mut compinp = String::new(); 
		let mut allowed = Vec::new();
		let mut selallow = String::new();
		compinp = get_compinput(compinp);
		let mut compsec = compinp.split_off(1);
		let dval = convert_pos(&compinp, &compsec);
		if check_position(&dval, &comp_board) == false {
			continue;
		}
	
	let h = (&pieces[i][1]).to_string();
	//println!("{}", &h);
	
	if check_direction_e(&dval, &comp_board, &h) == true {
		allowed.push("E");
	}
	if check_direction_w(&dval, &comp_board, &h) == true {
		allowed.push("W");
	}
	if check_direction_s(&dval, &comp_board, &h) == true {
		allowed.push("S");
	}
	if check_direction_n(&dval, &comp_board, &h) == true {
		allowed.push("N");
	}

	//The allowed directions, if any, are displayed ..
	if allowed.len() == 0 {
		continue;
	}
	
	//The computer can select any value of NSWE (no X) whether 
	//or not it's selectable in which case it will re-choose until
	//a (or the) valid direction is selected. 
		let mut dirind = false;
	loop {
		let compdir = rand::thread_rng().gen_range(0..=3);
			for a in &allowed {
				if &directions[compdir] == a {
					dirind = true;
					selallow = directions[compdir].to_string().clone();
					break;
					}
				}
			if dirind != true {
			continue;
			} else {
				break;
			}
		}
		
		let w = comp_board.clone();
		let mut q = Vec::new();
		match i {
		0 => { q = comp_acc.clone();},
		1 => { q = comp_bts.clone();},
		2 => { q = comp_crs.clone();},
		3 => { q = comp_dst.clone();},
		4 => { q = comp_esc.clone();},
		_ => ()
		}
	
	match selallow.as_str() {
	"S" => { comp_board = set_direction_s(&dval, w, &h, q) },
	"N" => { comp_board = set_direction_n(&dval, w, &h, q) },
	"E" => { comp_board = set_direction_e(&dval, w, &h, q) },
	"W" => { comp_board = set_direction_w(&dval, w, &h, q) },
		_ => ()
	}

	compinp.clear();
	compsec.clear();

	/////////////////////////////////////
	//Cheat Mode! 
	//Uncomment the two lines below to see the
	//computer board.
	////////////////////////////////////////
	
	//println!("\x1b[48;5;125m\x1b[38;5;118m          CMPTER BOARD           \x1b[0m");
	
	//makescreen(&comp_board);
	
	////////////////////////////////////////
	i += 1;
	if i == 5 {
		break;
			}
		
	}//end computer board setup loop
	
	///////////////////////////////////////////////////////////////////
	///////////////////////////////////////////////////////////////////
	///////////////////////////////////////////////////////////////////
	
	//Diceroll to determine who goes first of a random number % 2
	//The user picks 0 or 1, checked for validity and 'que' 
	//holds the reference.
	//////////////////////////////////////////////////
	//The compiler says cturn isn't read idk but it get's used for
	//for our purposes, idk...
	/////////////////////////////////////////////////
	let cturn;
	let turn = rand::thread_rng().gen_range(0..=100);
	if turn % 2 == 0 {
		cturn = "1".to_string();
	} else {
		cturn = "0".to_string();
	}
	let mut cointoss = String::new();
	loop {
		
	println!("\x1b[48;5;112m\x1b[38;5;52m   Heads of Tails! Enter 0 or 1  \x1b[0m");
	cointoss = get_input(cointoss);
	if cointoss.len() != 1 {
			thead();
			makescreen(&uatk_board);
			phead();
			makescreen(&user_board);
			invleng();
			cointoss.clear();
			continue;
		} 
	match cointoss.as_str() {
		"0" | "1" => { if &cturn == "0" {
					thead();
					makescreen(&uatk_board);
					phead();
					makescreen(&user_board);
					print!("\x1b[48;5;112m\x1b[38;5;52m It is Heads {}, \x1b[0m", cturn);
							} else {
					thead();
					makescreen(&uatk_board);
					phead();
					makescreen(&user_board);
					print!("\x1b[48;5;112m\x1b[38;5;52m It is Tails {}, \x1b[0m", cturn);
					}	break; 
				},
		_ => { 
			thead();
			makescreen(&uatk_board);
			phead();
			makescreen(&user_board);
			invchar();
			cointoss.clear();
			continue; }
		}
	}//end cointoss loop
	
	//que sets true/false for first or second turn.
	let mut que = false;
	if cointoss != cturn {
		print!("\x1b[48;5;112m\x1b[38;5;52myou go second    \x1b[0m\n");
	} else {
		print!("\x1b[48;5;112m\x1b[38;5;52myou go first     \x1b[0m\n");
		que = true;
	}
	
	println!("\x1b[48;5;112m\x1b[38;5;52m   Press Enter to begin game     \x1b[0m");
	
	get_input(input3);
	
	
	let mut gameover = false;
	//let mut comptrack = Vec::new();
	
	loop {//Begin turn loop
	
		if que == true{//User turn
	
	//Look through the board for any floating ship bits.
	//is_afloat corresponds to each ship in order A-E.
	//We iter and match over the user board and set true
	//if any and all letters that are found.
			//i = 0;
			gameover = true;
			let mut isafloat = [false, false, false, false, false];
			for v in &user_board {
				match v {
					'A' => {isafloat[0] = true},
					'B' => {isafloat[1] = true},
					'C' => {isafloat[2] = true},
					'D' => {isafloat[3] = true},
					'E' => {isafloat[4] = true},
					_ => ()
					}
				}
			let mut shotcount = 0;
			
	//We match on the existence of true, else the previous
	//turn has sent us to Davy Jones' Locker!
			for b in &isafloat {
				match b {
				true => {gameover = false;
						 shotcount += 1;},
				_ => ()
				}
			}
			
			if gameover == true {
				println!("\x1b[48;5;125m\x1b[38;5;118m         COMPUTER BOARD          \x1b[0m");
				makescreen(&comp_board);
				phead();
				makescreen(&user_board);
				println!("\x1b[48;5;130m\x1b[38;5;21m    Game Over! Computer Wins!    \x1b[0m");
				break
			}
			
		//We start the targeting process.
			thead();
			makescreen(&uatk_board);
			phead();
			makescreen(&user_board);
			println!("\x1b[48;5;112m\x1b[38;5;52m                                 \x1b[0m");
			
			let mut lowed = Vec::new();
			let mut sl = 1;
			let mut xm = 1;
			loop {
			//xm controls the reprint of the Enter Shot line so
			//it all stays pretty : )
				if xm == sl {
				println!("\x1b[48;5;130m\x1b[38;5;21m    Enter shot {} of {}            \x1b[0m", &sl, &shotcount);
					xm += 1;
					}
				let input = String::new();
				let mut input = get_input(input);
				let lpc = input.clone();
				
			//Check the input for validity and stuff.
				if input.len() != 2 {
					thead();
					makescreen(&uatk_board);
					phead();
					makescreen(&user_board);
					mtch4(&lowed);
					invleng();
					continue;
					}
				let second = input.split_off(1);
				if check_valid(&input) == false {
					thead();
					makescreen(&uatk_board);
					phead();
					makescreen(&user_board);
					mtch4(&lowed);
					invchar();
					continue;
					}
				if check_valid(&second) == false {
					thead();
					makescreen(&uatk_board);
					phead();
					makescreen(&user_board);
					mtch4(&lowed);
					invchar();
					continue;
					}
				if check_correct(&input) == check_correct(&second) {
					thead();
					makescreen(&uatk_board);
					phead();
					makescreen(&user_board);
					mtch4(&lowed);
					invchar();
					continue;
				}
				
				let dval = convert_pos(&input, &second);

				if check_position(&dval, &uatk_board) == false {
					thead();
					makescreen(&uatk_board);
					phead();
					makescreen(&user_board);
					mtch4(&lowed);
					println!("\x1b[48;5;130m\x1b[38;5;21m  Already attempted, try again   \x1b[0m");
					continue;
				}
			//lowed is now verified valid input and pushed.
				lowed.push(lpc);
		
			//We enter the targeting solution and prepare to fire!
				let w = uatk_board.clone();
				uatk_board = set_shot(&dval, w);
				thead();
				makescreen(&uatk_board);
				phead();
				makescreen(&user_board);
				mtch4(&lowed);
				sl += 1;
				if sl == shotcount + 1 {
					break
				}
			}
			
			println!("\x1b[48;5;112m\x1b[38;5;52m   Press Enter to Fire           \x1b[0m");
			let input5 = String::new();
			get_input(input5);
			
		//We clone lowed for an unused copy of the shots later.
			let relow = lowed.clone();
			
		//Fire!
		//...by checking the lowed contents against the comp_board
		//and then inserting a H or M at the locations like above.
		//The shots are placed on both comp_board and uatk_board.
			let mut report = Vec::new();
			for s in lowed {
					let mut bl = s.clone();
					let kc = bl.split_off(1);
					let dval = convert_pos(&bl, &kc);

					let mrk = check_position(&dval, &comp_board);
					if mrk == true {
						report.push('M');
						let w = comp_board.clone();
						comp_board = take_shot(&dval, w);
						let u = uatk_board.clone();
						let hit = false;
						uatk_board = ref_shot(&dval, u, hit);
					} else {
						report.push('H');
						let w = comp_board.clone();
						comp_board = take_shot(&dval, w);
						let u = uatk_board.clone();
						let hit = true;
						uatk_board = ref_shot(&dval, u, hit);
					}
				}
			
			//Displays the results of the last shot.
			//Check if an enemy ship is sunk otherwise the user
		//won't know what ship they've sunk.
			let mut themafloat = [false, false, false, false, false];
			for e in &comp_board {
				match e {
					'A' => {themafloat[0] = true},
					'B' => {themafloat[1] = true},
					'C' => {themafloat[2] = true},
					'D' => {themafloat[3] = true},
					'E' => {themafloat[4] = true},
					_ => ()
					}
				}
			
			thead();
			makescreen(&uatk_board);
			println!("\x1b[48;5;112m\x1b[38;5;21m ------------------------------- \x1b[0m");
			println!("\x1b[48;5;112m\x1b[38;5;21m       FRIENDLY SHOT REPORT      \x1b[0m");
			
			//Displays the results of the last shot.
			i = 0;
			loop {
				println!("\x1b[48;5;112m\x1b[38;5;21m           {}  is a  {}           \x1b[0m", &relow[i], &report[i]);
				i += 1;
				if i == relow.len() {
					break
				}
			}
			let mut k = 5 - i;
			while k != 0 {
				println!("\x1b[48;5;112m\x1b[38;5;21m ------------------------------- \x1b[0m");
				k -= 1;
			}
			i = 0;
			loop {
				if themafloat[i] == false {
					println!("\x1b[48;5;112m\x1b[38;5;52m Enemy {}is sunk  \x1b[0m", &pieces[i][0]);
					
				
					} else {
					println!("\x1b[48;5;112m\x1b[38;5;52m Enemy {}remains  \x1b[0m", &pieces[i][0]);
					
					}
				i += 1;
				if i == 5 {
					break
				}
			}
			println!("\x1b[48;5;112m\x1b[38;5;21m ------------------------------- \x1b[0m");
			
			
			println!("\x1b[48;5;112m\x1b[38;5;52m   Press Enter to continue       \x1b[0m");
			let input4 = String::new();
			get_input(input4);
			
			//Set que to false to initiate the next turn.
			que = false;
			}//End user turn	
			
			if gameover == true {
		break
		} 
		
	////////////////////////////////////////////////////////////////////////
	///////////////////////////////////////////////////////////////////////
	///////////////////////////////////////////////////////////////////////
	
		if que == false {//Begin Comp turn
			gameover = true;
			let mut isafloat = [false, false, false, false, false];
			for v in &comp_board {
				match v {
					'A' => {isafloat[0] = true},
					'B' => {isafloat[1] = true},
					'C' => {isafloat[2] = true},
					'D' => {isafloat[3] = true},
					'E' => {isafloat[4] = true},
					_ => ()
					}
				}
			let mut shotcount = 0;
	//We match on the existence of true, else the previous
	//turn has sent them to Davy Jones' Locker!
			for b in &isafloat {
				match b {
				true => {gameover = false;
						 shotcount += 1;},
				_ => ()
				}
			}
			if gameover == true {
				println!("\x1b[48;5;125m\x1b[38;5;118m         COMPUTER BOARD          \x1b[0m");
				makescreen(&comp_board);
				phead();
				makescreen(&user_board);
		println!("\x1b[48;5;130m\x1b[38;5;21m      Game Over! You Win!        \x1b[0m");
				break
			}
			
		//We find the targets.
			let mut callowed = Vec::new();
			
			loop {
				let mut compinp = String::new();
				let mut darg = 0;
				let clb = catk_board.clone();

			//Match on previous hits.
			if shotcount >= 1 {
				for j in &clb {
				
					match j {
						'H' => {
						
						//If all false through all of the iteration j
						//then do random fire below						
						let hspec = (&pieces[4][1]).to_string();
						
						if check_direction_e(&darg, &catk_board, &hspec) == true {
							let blc = catk_board.clone();
							catk_board = set_shot_e(&darg, blc);
							let mut ths = darg.clone();
							ths += 1;
							let tht = convert_ind(&ths);
							callowed.push(tht);
							shotcount -= 1;
							if shotcount <= 1 {
								break
							}
							
						} 
						//For some reason set shot w was faulty, idk
						let mut barg = darg.clone();
						if barg % 10 != 0 {
							barg -= 1;
						if check_position(&barg, &catk_board) == true {
							let blc = catk_board.clone();
							catk_board = set_shot(&barg, blc);
							let ths = barg.clone();
							let tht = convert_ind(&ths);
							callowed.push(tht);
							shotcount -= 1;
							if shotcount <= 1 {
								break
								}
							} 
						}
						if check_direction_s(&darg, &catk_board, &hspec) == true {
							let blc = catk_board.clone();
							catk_board = set_shot_s(&darg, blc);
							let mut ths = darg.clone();
							ths += 10;
							let tht = convert_ind(&ths);
							callowed.push(tht);
							shotcount -= 1;
							if shotcount <= 1 {
								break
							}
						} 
						if check_direction_n(&darg, &catk_board, &hspec) == true {
							let blc = catk_board.clone();
							catk_board = set_shot_n(&darg, blc);
							let mut ths = darg.clone();
							ths -= 10;
							let tht = convert_ind(&ths);
							callowed.push(tht);
							shotcount -= 1;
							if shotcount <= 1 {
								break
							}
						}
							},
						_ => ()
						}
					darg += 1;
					}
				}
				if shotcount == 0 {
					break
				}
				
				//else do the random selection below
				//with get_compinput
				compinp = get_compinput(compinp);
				let lpc = compinp.clone();
				let compsec = compinp.split_off(1);
				let dval = convert_pos(&compinp, &compsec);
				if check_position(&dval, &catk_board) == false {
					continue;
				}
				callowed.push(lpc);
				let w = catk_board.clone();
				catk_board = set_shot(&dval, w);
				if shotcount >= 1 {
				shotcount -= 1;
				}
				if shotcount == 0 {
					break
				}
			}//end loop
	
	//Computer Fires.
		let arelow = callowed.clone();
		let mut report = Vec::new();
		for s in callowed {
					let mut bl = s.clone();
					let kc = bl.split_off(1);
					let dval = convert_pos(&bl, &kc);
					let mrk = check_position(&dval, &user_board);
					if mrk == true {
						report.push('M');
						let w = user_board.clone();
						user_board = take_shot(&dval, w);
						let u = catk_board.clone();
						let hit = false;
						catk_board = ref_shot(&dval, u, hit);
					} else {
						report.push('H');
						let w = user_board.clone();
						user_board = take_shot(&dval, w);
						let u = catk_board.clone();
						let hit = true;
						catk_board = ref_shot(&dval, u, hit);
					}
				}
			let mut ayemafloat = [false, false, false, false, false];
			for e in &user_board {
				match e {
					'A' => {ayemafloat[0] = true},
					'B' => {ayemafloat[1] = true},
					'C' => {ayemafloat[2] = true},
					'D' => {ayemafloat[3] = true},
					'E' => {ayemafloat[4] = true},
					_ => ()
					}
				}
			
			phead();
			makescreen(&user_board);
			println!("\x1b[48;5;130m\x1b[38;5;21m ------------------------------- \x1b[0m");
			println!("\x1b[48;5;130m\x1b[38;5;21m        ENEMY SHOT REPORT        \x1b[0m");
			i = 0;
			loop {
				println!("\x1b[48;5;130m\x1b[38;5;21m           {}  is a  {}           \x1b[0m", &arelow[i], &report[i]);
				i += 1;
				if i == arelow.len() {
					break
				}
			}
			let mut k = 5 - i;
			while k != 0 {
				println!("\x1b[48;5;130m\x1b[38;5;21m ------------------------------- \x1b[0m");
				k -= 1;
			}
			i = 0;
			loop {
				if ayemafloat[i] == false {
					println!("\x1b[48;5;130m\x1b[38;5;21mFriendly {}is sunk\x1b[0m", &pieces[i][0]);
					} else {
					println!("\x1b[48;5;130m\x1b[38;5;21mFriendly {}remains\x1b[0m", &pieces[i][0]);
					}
				i += 1;
				if i == 5 {
					break
				}
			}
			println!("\x1b[48;5;130m\x1b[38;5;21m ------------------------------- \x1b[0m");
			println!("\x1b[48;5;112m\x1b[38;5;52m   Press Enter to continue       \x1b[0m");
	let input3 = String::new();
	get_input(input3);
		
		//////////////////////////////////////////////
		//Currently unused, line 485
		//Was considering vecs to hold all the shots
		//of a particular ship, idk
		//for x in arelow {
		//	comptrack.push(x);
		//}
		//println!("COMPTRACK {:?}", &comptrack);
		////////////////////////////////////////

			que = true;
		}//comp turn
		
	if gameover == true {
		break
		} 
		
	}//Turn
	println!("\x1b[48;5;130m\x1b[38;5;21m --------------FIN-------------- \x1b[0m");
}//game

	
fn get_compinput(mut v: String) -> String {
		let complet = rand::thread_rng().gen_range('A'..='J') as char;
		let compnum = rand::thread_rng().gen_range('0'..='9') as char;
		//println!("letter: {}", &complet);
		//println!("Integer: {}", &compnum);
		v.push(complet);
		v.push(compnum);
		v = v.to_ascii_uppercase().replace(" ", "");
		v
		}
	
fn get_input(mut v: String) -> String {
    io::stdin().read_line(&mut v).unwrap();
    v = v.to_ascii_uppercase().replace(" ", "");
    v.pop();
	v
	}
	
fn check_valid(a: &String) -> bool {
	match a.as_str() {
	"A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => true,
	_ => false
	}
}

fn check_correct(a: &String) -> bool {
	match a.as_str() {
	"A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" => true,
	_ => false
	}
}

fn check_position(a: &i32, c: &Vec<char>) -> bool {
	let pos: usize = a.clone().try_into().unwrap();
	let z = c[pos].clone();
	if z == 'Z' {
		return true
		}
	else {
		return false
	}
}

fn check_direction_e(a: &i32, c: &Vec<char>, d:&String) -> bool {
	let pos: usize = a.clone().try_into().unwrap();
	let mut slen = 0;
	match d.as_str() {
	"5" => slen += 5,
	"4" => slen += 4,
	"3" => slen += 3,
	"2" => slen += 2,
	_ => ()
	}
	let mut i = 1;
	loop {
		if i == slen {
			break;
		}
		if pos + slen - 1 >= 100 {
			return false;
		}
		if pos / 10 != (pos + (slen - 1)) / 10 {
			return false;
		}
		let z = c[pos + i].clone();
		if z == 'Z' {
			i += 1;
			continue;
			}
		else {
			return false;
		}
	}
	true
}

fn check_direction_w(a: &i32, c: &Vec<char>, d:&String) -> bool {
	let pos: usize = a.clone().try_into().unwrap();
	let mut slen = 0;
	match d.as_str() {
	"5" => slen += 5,
	"4" => slen += 4,
	"3" => slen += 3,
	"2" => slen += 2,
	_ => ()
	}
	let mut i = 0;
	loop {
		if i == slen {
			break;
		}
		if pos + 1 < slen {
		return false;
		}
		if pos / 10 != (pos - (slen - 1)) / 10 {
			return false;
		}
		let z = c[pos - i].clone();
		if z == 'Z' {
			i += 1;
			continue;
			}
		else {
			return false;
		}
	}
	true
}

fn check_direction_s(a: &i32, c: &Vec<char>, d: &String) -> bool {
	let pos: usize = a.clone().try_into().unwrap();
	let mut slen = 0;
	
	match d.as_str() {
	"5" => slen += 40,
	"4" => slen += 30,
	"3" => slen += 20,
	"2" => slen += 10,
	_ => ()
	}
	let mut i = 0;
	loop {
		if i == slen {
			break;
		}
		if pos + slen >= 100 {
			return false;
		}
		i += 10;
		let z = c[pos + i].clone();
		if z == 'Z' {
			continue;
			}
		else {
			return false;
		}
	}
	true
}	
	
fn check_direction_n(a: &i32, c: &Vec<char>, d: &String) -> bool {
	let pos: usize = a.clone().try_into().unwrap();
	let mut slen = 0;
	match d.as_str() {
	"5" => slen += 40,
	"4" => slen += 30,
	"3" => slen += 20,
	"2" => slen += 10,
	_ => ()
	}
	let mut i = 0;
	loop {
		if i == slen {
			break;
		}
		if pos < slen {
		return false;
		}
		i += 10;
		let z = c[pos - i].clone();
		if z == 'Z' {
			continue;
			}
		else {
			return false;
		}
	}
	true
}

fn set_direction_s(a: &i32, mut c: Vec<char>, d: &String, mut e:Vec<char>) -> Vec<char> {
	let pos: usize = a.clone().try_into().unwrap();
	let mut slen = 0;
	match d.as_str() {
	"5" => slen += 40,
	"4" => slen += 30,
	"3" => slen += 20,
	"2" => slen += 10,
	_ => ()
	}
	let mut i = 0;
	loop {	
		if i == slen + 10{
			break;
			}
		let z = c[pos + i].clone();
		if z == 'Z' {
			let o = e.remove(0);
			c.remove(pos + i);
			c.insert(pos + i, o);
			i += 10;
			continue;
			}
	}
	return c
}	

fn set_direction_n(a: &i32, mut c: Vec<char>, d: &String, mut e:Vec<char>) -> Vec<char> {
	let pos: usize = a.clone().try_into().unwrap();
	let mut slen = 0;
	match d.as_str() {
	"5" => slen += 40,
	"4" => slen += 30,
	"3" => slen += 20,
	"2" => slen += 10,
	_ => ()
	}
	let mut i = 0;
	loop {
		if i == slen + 10{
			break;
		}
		let z = c[pos - i].clone();
		if z == 'Z' {
			let o = e.remove(0);
			c.remove(pos - i);
			c.insert(pos - i, o);
			i += 10;
			continue;
			}
	}
	return c
}

fn set_direction_e(a: &i32, mut c: Vec<char>, d: &String, mut e:Vec<char>) -> Vec<char> {
	let pos: usize = a.clone().try_into().unwrap();
	let mut slen = 0;
	match d.as_str() {
	"5" => slen += 5,
	"4" => slen += 4,
	"3" => slen += 3,
	"2" => slen += 2,
	_ => ()
	}
	let mut i = 0;
	loop {
		if i == slen {
			break;
		}
		let z = c[pos + i].clone();
		if z == 'Z' {
			let o = e.remove(0);
			c.remove(pos + i);
			c.insert(pos + i, o);
			i += 1;
			continue;
			}
		}
	return c
}

fn set_direction_w(a: &i32, mut c: Vec<char>, d: &String, mut e:Vec<char>) -> Vec<char> {
	let pos: usize = a.clone().try_into().unwrap();
	let mut slen = 0;
	match d.as_str() {
	"5" => slen += 5,
	"4" => slen += 4,
	"3" => slen += 3,
	"2" => slen += 2,
	_ => ()
	}
	let mut i = 0;
	loop {
		if i == slen {
			break;
		}
		let z = c[pos - i].clone();
		if z == 'Z' {
			let o = e.remove(0);
			c.remove(pos - i);
			c.insert(pos - i, o);
			i += 1;
			continue;
			}
		}
	return c
}

fn set_shot(a: &i32, mut c: Vec<char>) -> Vec<char> {
	let pos: usize = a.clone().try_into().unwrap();
		let z = c[pos].clone();
		if z == 'Z' {
			let o = 'T';
			c.remove(pos);
			c.insert(pos, o);
			} 
	return c
}

fn take_shot(a: &i32, mut c: Vec<char>) -> Vec<char> {
	let pos: usize = a.clone().try_into().unwrap();
		let z = c[pos].clone();
		if z == 'Z' {
			let o = 'M';
			c.remove(pos);
			c.insert(pos, o);
			} else {
			let o = 'H';
			c.remove(pos);
			c.insert(pos, o);
			}
	
	return c
}


fn ref_shot(a: &i32, mut c: Vec<char>, d: bool) -> Vec<char> {
	let pos: usize = a.clone().try_into().unwrap();
		if d == true {
			let o = 'H';
			c.remove(pos);
			c.insert(pos, o);
			} else {
			let o = 'M';
			c.remove(pos);
			c.insert(pos, o);
			}	
	return c
}

fn convert_pos(a: &String, b: &String) -> i32 {
	let mut pos = 0;
	match a.as_str() {
	"A" => pos += 0,
	"B" => pos += 10,
	"C" => pos += 20,
	"D" => pos += 30,
	"E" => pos += 40,
	"F" => pos += 50,
	"G" => pos += 60,
	"H" => pos += 70,
	"I" => pos += 80,
	"J" => pos += 90,
	"1" => pos += 1,
	"2" => pos += 2,
	"3" => pos += 3,
	"4" => pos += 4,
	"5" => pos += 5,
	"6" => pos += 6,
	"7" => pos += 7,
	"8" => pos += 8,
	"9" => pos += 9,
	"0" => pos += 0,
	_ => ()
	};
	match b.as_str() {
	"A" => pos += 0,
	"B" => pos += 10,
	"C" => pos += 20,
	"D" => pos += 30,
	"E" => pos += 40,
	"F" => pos += 50,
	"G" => pos += 60,
	"H" => pos += 70,
	"I" => pos += 80,
	"J" => pos += 90,
	"1" => pos += 1,
	"2" => pos += 2,
	"3" => pos += 3,
	"4" => pos += 4,
	"5" => pos += 5,
	"6" => pos += 6,
	"7" => pos += 7,
	"8" => pos += 8,
	"9" => pos += 9,
	"0" => pos += 0,
	_ => ()
	};
	
	pos
}

fn set_shot_e(a: &i32, mut c: Vec<char>) -> Vec<char> {
	let mut pos: usize = a.clone().try_into().unwrap();
	pos += 1;
		let z = c[pos].clone();
		if z == 'Z' {
			let o = 'T';
			c.remove(pos);
			c.insert(pos, o);
			} 
	return c
}

//fn set_shot_w(a: &i32, mut c: Vec<char>) -> Vec<char> {
//	let mut pos: usize = a.clone().try_into().unwrap();
//	pos -= 1;
//		let z = c[pos].clone();
//		if z == 'Z' {
//			let mut o = 'T';
//			c.remove(pos);
//			c.insert(pos, o);
//			} 
//	return c
//}

fn set_shot_s(a: &i32, mut c: Vec<char>) -> Vec<char> {
	let mut pos: usize = a.clone().try_into().unwrap();
	pos += 10;
		let z = c[pos].clone();
		if z == 'Z' {
			let o = 'T';
			c.remove(pos);
			c.insert(pos, o);
			} 
	return c
}

fn set_shot_n(a: &i32, mut c: Vec<char>) -> Vec<char> {
	let mut pos: usize = a.clone().try_into().unwrap();
	pos -= 10;
		let z = c[pos].clone();
		if z == 'Z' {
			let o = 'T';
			c.remove(pos);
			c.insert(pos, o);
			} 
	return c
}



fn convert_ind(a: &i32) -> String {
	let mut pos: usize = a.clone().try_into().unwrap();
	let mut ltr = String::new();
	
		if pos >= 90 {
			ltr.push('J');
			pos -= 90;
		} else
		if pos >= 80 {
			ltr.push('I');
			pos -= 80;
		} else
		if pos >= 70 {
			ltr.push('H');
			pos -= 70;
		} else
		if pos >= 60 {
			ltr.push('G');
			pos -= 60;
		} else
		if pos >= 50 {
			ltr.push('F');
			pos -= 50;
		} else
		if pos >= 40 {
			ltr.push('E');
			pos -= 40;
		} else
		if pos >= 30 {
			ltr.push('D');
			pos -= 30;
		} else
		if pos >= 20 {
			ltr.push('C');
			pos -= 20;
		} else
		if pos >= 10 {
			ltr.push('B');
			pos -= 10;
		} else {
		ltr.push('A');
		}
		if pos == 0 {
			ltr.push('0');
		} else
		if pos == 9 {
			ltr.push('9');
		
		} else
		if pos == 8 {
			ltr.push('8');
		
		} else
		if pos == 7 {
			ltr.push('7');
		
		} else
		if pos == 6 {
			ltr.push('6');
		
		} else
		if pos == 5 {
			ltr.push('5');
		
		} else
		if pos == 4 {
			ltr.push('4');
	
		} else
		if pos == 3 {
			ltr.push('3');
			
		} else
		if pos == 2 {
			ltr.push('2');
			
		} else
		if pos == 1 {
			ltr.push('1');
		
		};
		
	ltr
	
}

fn makescreen(a: &Vec<char>) {

println!("\x1b[48;5;130m\x1b[38;5;21m[ ][0][1][2][3][4][5][6][7][8][9]\x1b[0m");

print!("\x1b[48;5;130m\x1b[38;5;21m[A]\x1b[0m\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[0]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[1]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[2]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[3]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[4]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[5]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[6]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[7]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[8]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[9]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m\n");


print!("\x1b[48;5;130m\x1b[38;5;21m[B]\x1b[0m\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[10]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[11]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[12]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[13]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[14]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[15]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[16]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[17]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[18]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[19]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m\n");


print!("\x1b[48;5;130m\x1b[38;5;21m[C]\x1b[0m\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[20]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[21]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[22]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[23]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[24]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[25]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[26]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[27]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[28]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[29]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m\n");


print!("\x1b[48;5;130m\x1b[38;5;21m[D]\x1b[0m\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[30]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[31]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[32]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[33]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[34]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[35]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[36]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[37]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[38]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[39]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m\n");


print!("\x1b[48;5;130m\x1b[38;5;21m[E]\x1b[0m\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[40]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[41]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[42]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[43]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[44]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[45]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[46]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[47]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[48]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[49]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m\n");


print!("\x1b[48;5;130m\x1b[38;5;21m[F]\x1b[0m\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[50]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[51]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[52]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[53]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[54]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[55]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[56]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[57]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[58]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[59]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m\n");


print!("\x1b[48;5;130m\x1b[38;5;21m[G]\x1b[0m\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[60]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[61]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[62]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[63]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[64]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[65]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[66]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[67]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[68]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[69]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m\n");


print!("\x1b[48;5;130m\x1b[38;5;21m[H]\x1b[0m\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[70]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[71]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[72]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[73]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[74]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[75]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[76]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[77]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[78]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[79]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m\n");


print!("\x1b[48;5;130m\x1b[38;5;21m[I]\x1b[0m\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[80]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[81]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[82]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[83]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[84]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[85]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[86]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[87]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[88]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[89]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m\n");


print!("\x1b[48;5;130m\x1b[38;5;21m[J]\x1b[0m\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[90]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[91]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[92]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[93]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[94]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[95]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[96]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[97]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[98]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m");
print!("\x1b[48;5;27m\x1b[38;5;112m[\x1b[0m");
collett(&a[99]);
print!("\x1b[48;5;27m\x1b[38;5;112m]\x1b[0m\n");

}

fn collett(a: &char) {
	match a {
		'A' => print!("\x1b[48;5;27m\x1b[38;5;16mA\x1b[0m"),
		'B' => print!("\x1b[48;5;27m\x1b[38;5;16mB\x1b[0m"),
		'C' => print!("\x1b[48;5;27m\x1b[38;5;16mC\x1b[0m"),
		'D' => print!("\x1b[48;5;27m\x1b[38;5;16mD\x1b[0m"),
		'E' => print!("\x1b[48;5;27m\x1b[38;5;16mE\x1b[0m"),
		'H' => print!("\x1b[48;5;27m\x1b[38;5;125mH\x1b[0m"),
		'M' => print!("\x1b[48;5;27m\x1b[38;5;226mM\x1b[0m"),
		'T' => print!("\x1b[48;5;27m\x1b[28;5;130mT\x1b[0m"),
		'Z' => print!("\x1b[48;5;27m\x1b[38;5;115mZ\x1b[0m"),
		_ => ()
	}
}

fn thead() {
	println!("\x1b[48;5;125m\x1b[38;5;118m          TARGET BOARD           \x1b[0m");
}

fn phead() {
	println!("\x1b[48;5;125m\x1b[38;5;118m          PLAYER BOARD           \x1b[0m");
}
fn invchar () {
	println!("\x1b[48;5;130m\x1b[38;5;21m   Invalid character, try again  \x1b[0m");
}
fn invleng() {
	println!("\x1b[48;5;130m\x1b[38;5;21m    Invalid length, try again    \x1b[0m");
}

fn mtch4(a: &Vec<String>) {
	let lowed = a.clone();
	match lowed.len() {
					0 => print!("\x1b[48;5;112m\x1b[38;5;52m                                 \x1b[0m\n"),
					1 => print!("\x1b[48;5;112m\x1b[38;5;52m Targeting {}                    \x1b[0m\n", &lowed[0]),
					2 => print!("\x1b[48;5;112m\x1b[38;5;52m Targeting {} {}                 \x1b[0m\n", &lowed[0], &lowed[1]),
					3 => print!("\x1b[48;5;112m\x1b[38;5;52m Targeting {} {} {}              \x1b[0m\n", &lowed[0], lowed[1], lowed[2]),
					4 => print!("\x1b[48;5;112m\x1b[38;5;52m Targeting {} {} {} {}           \x1b[0m\n", &lowed[0], lowed[1], lowed[2], lowed[3]),
					5 => print!("\x1b[48;5;112m\x1b[38;5;52m Targeting {} {} {} {} {}        \x1b[0m\n", &lowed[0], lowed[1], lowed[2], lowed[3], lowed[4]),
					_ => ()
				}
}
