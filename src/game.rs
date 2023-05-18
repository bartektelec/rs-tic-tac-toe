struct Game {
    state: [[i8; 3]; 3],
}

impl Game {
    fn player_move(&self, cell: usize) -> Option<()> {
        if cell < 1 || cell > 9 {
            return None;
        };

        let row = (cell / 3).floor();
        let col = (cell - 1) % 3;

        Some(())
    }
}
