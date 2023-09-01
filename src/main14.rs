const INPUT: usize = 556061;
const INPUT_P2: [usize;6] = [5,5,6,0,6,1];

fn digits(sum: usize) -> (usize, usize) {
    (sum / 10, sum % 10)
}

fn print(board: &[usize], current: &(usize, usize)) {
    for i in 0..board.len() {
        if i == current.0 && i == current.1 {
            print!("[({})]", board[i]);
        } else if i == current.0 {
            print!("({})", board[i]);
        } else if i == current.1 {
            print!("[{}]", board[i]);
        } else {
            print!(" {} ", board[i]);
        }
    }
    println!();
}

fn try_part2(skip: bool, board: &[usize]) -> bool {
    let ref_len = INPUT_P2.len();
    if skip || board.len() < ref_len {
        return skip;
    }
    if board[board.len()-ref_len..board.len()] == INPUT_P2 {
        println!("Part 2: {}", board.len() - ref_len);
        true
    } else {
        false
    }
}

fn main() {
    let mut part2_done = false;
    let mut part1_done = false;
    let mut board = [3, 7].to_vec();
    let mut current = (0, 1);

    while !(part1_done && part2_done) {
            let next = digits(board[current.0] + board[current.1]);
        if next.0 > 0 {

            board.push(next.0);
            part2_done = try_part2(part2_done, &board);
        }
        board.push(next.1);
        part2_done = try_part2(part2_done, &board);

        current.0 = (current.0 + 1 + board[current.0]) % board.len();
        current.1 = (current.1 + 1 + board[current.1]) % board.len();

        if INPUT < 20 {
            print(&board, &current);
        }

        if !part1_done && board.len() >= INPUT + 10 {
            println!("Part 1: {}", board[INPUT..INPUT+10].iter().map(|c| c.to_string()).collect::<String>());
            part1_done = true;
        }

    }
}
