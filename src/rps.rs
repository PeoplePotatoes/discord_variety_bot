// Rock Paper Scissors
//Function to call Rock Paper scissors
/*
pub fn rps(player_a: &str, move_a: char, player_b: &str, move_b: char) {
    //let player_a = "Placeholder User #1";
    //let player_b = "Placeholder User #2";
    let move_a = 's';
    let move_b = 'p';
    let winner: char = logic(move_a, move_b);
    if winner == 't' {
        println!("Its a tie!");
    } else if winner == 'a' {
        println!("{} wins!", player_a);
    } else if winner == 'b' {
        println!("{} wins!", player_b);
    } else {
        println!("Uh-Oh! That's not supposed to happen!");
        eprintln!("There was an error while playing Rock, Paper, Scissors. An unexpected winner was returned.");
    }
}
*/
// Logic for rps module of the bot
pub fn rps(ma: char, mb: char) -> char {
    if ma == mb {
        't'
    } else if ma == 'r' {
        if mb == 'p' {
            'b'
        } else if mb == 's' {
            'a'
        } else {
            'e'
        }
    } else if ma == 'p' {
        if mb == 'r' {
            'a'
        } else if mb == 's' {
            'b'
        } else {
            'e'
        }
    } else if ma == 's' {
        if mb == 'r' {
            'b'
        } else if mb == 'p' {
            'a'
        } else {
            'e'
        }
    } else {
        'e'
    }
}
