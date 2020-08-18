use wordscapes_helper::*;

fn main() {
    for sol in BoardSolver::from_board(
        "angryi",
        r"
____gain_n
__g______a
__r__g___r
__i__ra##y
rang_a____
_n_rain___
_g_a_n____
_rainy____
_y_n______
",
    )
    .first_n_solutions(5) {
        println!( "{}", sol);
    }
    println!(
        "{}",
        BoardSolver::from_board(
            "ranb",
            r"
 bar #
 # # #
#### #
#    #
###   
",
        )
        .first_n_solutions(1)[0]
    );

    println!(
        "{}",
        BoardSolver::from_board(
            "bypass",
            r"
######___
___#_####
####_#__#
_#____#_#
_#_#__#__
_#_####__
_###__#__
",
        )
        .first_n_solutions(1)[0]
    );
}
