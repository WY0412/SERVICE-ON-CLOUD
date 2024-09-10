


pub struct Position {
    pub(crate) col: u8,
    pub(crate) row: u8,
}

pub(crate) fn get_matrix_position_version1(index: u64) -> Position {
    if index < 26 {
        let up = index / 2;
        let left = index % 2;
        return Position {
            col: (20 - left) as u8,
            row: (20 - up) as u8,
        };
    } else if 26 <= index && index < 52 {
        let down = (index - 26) / 2;
        let left = (index - 26) % 2;
        return Position {
            col: (18 - left) as u8,
            row: (8 + down) as u8,
        };
    } else if 52 <= index && index < 78 {
        let up = (index - 52) / 2;
        let left = (index - 52) % 2;
        return Position {
            col: (16 - left) as u8,
            row: (20 - up) as u8,
        };
    } else if 78 <= index && index < 104 {
        let down = (index - 78) / 2;
        let left = (index - 78) % 2;
        return Position {
            col: (14 - left) as u8,
            row: (8 + down) as u8,
        };
    } else if 104 <= index && index < 132 {
        let up = (index - 104) / 2;
        let left = (index - 104) % 2;
        return Position {
            col: (12 - left) as u8,
            row: (20 - up) as u8,
        };
    } else if 132 <= index && index < 144 {
        let up = (index - 132) / 2;
        let left = (index - 132) % 2;
        return Position {
            col: (12 - left) as u8,
            row: (5 - up) as u8,
        };
    } else if 144 <= index && index < 156 {
        let down = (index - 144) / 2;
        let left = (index - 144) % 2;
        return Position {
            col: (10 - left) as u8,
            row: (0 + down) as u8,
        };
    } else if 156 <= index && index < 184 {
        let down = (index - 156) / 2;
        let left = (index - 156) % 2;
        return Position {
            col: (10 - left) as u8,
            row: (7 + down) as u8,
        };
    } else if 184 <= index && index < 194 {
        let up = (index - 184) / 2;
        let left = (index - 184) % 2;
        return Position {
            col: (8 - left) as u8,
            row: (12 - up) as u8,
        };
    } else if 194 <= index && index < 204 {
        let down = (index - 194) / 2;
        let left = (index - 194) % 2;
        return Position {
            col: (5 - left) as u8,
            row: (8 + down) as u8,
        };
    } else if 204 <= index && index < 214 {
        let up = (index - 204) / 2;
        let left = (index - 204) % 2;
        return Position {
            col: (3 - left) as u8,
            row: (12 - up) as u8,
        };
    } else if 214 <= index && index < 224 {
        let down = (index - 214) / 2;
        let left = (index - 214) % 2;
        return Position {
            col: (1 - left) as u8,
            row: (8 + down) as u8,
        };
    }
    return Position { col: 0, row: 0 };
}

pub fn get_matrix_position_version2(index: u64) -> Position {
    if index < 34 {
        let up = index / 2;
        let left = index % 2;
        return Position {
            col: (24 - left) as u8,
            row: (24 - up) as u8,
        };
    } else if 34 <= index && index < 68 {
        let down = (index - 34) / 2;
        let left = (index - 34) % 2;
        return Position {
            col: (22 - left) as u8,
            row: (8 + down) as u8,
        };
    } else if 68 <= index && index < 76 {
        let left = (index - 68) % 2;
        let up = (index - 68) / 2;
        return Position {
            col: (20 - left) as u8,
            row: (24 - up) as u8,
        };
    } else if 76 <= index && index < 92 {
        let left = (index - 76) % 2;
        let up = (index - 76) / 2;
        return Position {
            col: (20 - left) as u8,
            row: (15 - up) as u8,
        };
    } else if 92 <= index && index < 108 {
        let left = (index - 92) % 2;
        let down = (index - 92) / 2;
        return Position {
            col: (18 - left) as u8,
            row: (8 + down) as u8,
        };
    } else if 108 <= index && index < 116 {
        let left = (index - 108) % 2;
        let down = (index - 108) / 2;
        return Position {
            col: (18 - left) as u8,
            row: (21 + down) as u8,
        };
    } else if 116 <= index && index < 124 {
        let left = (index - 116) % 2;
        let up = (index - 116) / 2;
        return Position {
            col: (16 - left) as u8,
            row: (24 - up) as u8,
        };
    } else if 124 <= index && index < 142 {
        let left = (index - 124) % 2;
        let up = (index - 124) / 2;
        return Position {
            col: (16 - left) as u8,
            row: (15 - up) as u8,
        };
    } else if 142 <= index && index < 154 {
        let left = (index - 142) % 2;
        let up = (index - 142) / 2;
        return Position {
            col: (16 - left) as u8,
            row: (5 - up) as u8,
        };
    } else if 154 <= index && index < 166 {
        let left = (index - 154) % 2;
        let down = (index - 154) / 2;
        return Position {
            col: (14 - left) as u8,
            row: (0 + down) as u8,
        };
    } else if 166 <= index && index < 202 {
        let left = (index - 166) % 2;
        let down = (index - 166) / 2;
        return Position {
            col: (14 - left) as u8,
            row: (7 + down) as u8,
        };
    } else if 202 <= index && index < 238 {
        let left = (index - 202) % 2;
        let up = (index - 202) / 2;
        return Position {
            col: (12 - left) as u8,
            row: (24 - up) as u8,
        };
    } else if 238 <= index && index < 250 {
        let left = (index - 238) % 2;
        let up = (index - 238) / 2;
        return Position {
            col: (12 - left) as u8,
            row: (5 - up) as u8,
        };
    } else if 250 <= index && index < 262 {
        let left = (index - 250) % 2;
        let down = (index - 250) / 2;
        return Position {
            col: (10 - left) as u8,
            row: (0 + down) as u8,
        };
    } else if 262 <= index && index < 298 {
        let left = (index - 262) % 2;
        let down = (index - 262) / 2;
        return Position {
            col: (10 - left) as u8,
            row: (7 + down) as u8,
        };
    } else if 298 <= index && index < 316 {
        let left = (index - 298) % 2;
        let up = (index - 298) / 2;
        return Position {
            col: (8 - left) as u8,
            row: (16 - up) as u8,
        };
    } else if 316 <= index && index < 334 {
        let left = (index - 316) % 2;
        let down = (index - 316) / 2;
        return Position {
            col: (5 - left) as u8,
            row: (8 + down) as u8,
        };
    } else if 334 <= index && index < 352 {
        let left = (index - 334) % 2;
        let up = (index - 334) / 2;
        return Position {
            col: (3 - left) as u8,
            row: (16 - up) as u8,
        };
    } else if 352 <= index && index < 370 {
        let left = (index - 352) % 2;
        let down = (index - 352) / 2;
        return Position {
            col: (1 - left) as u8,
            row: (8 + down) as u8,
        };
    }
    return Position { col: 0, row: 0 };
}
