struct Position {
    pub(crate) col: u8,
    pub(crate) row: u8,
}

fn _create_raw_qr_code(
    size: usize,
    rotation: u32,
    start_index_x: usize,
    start_index_y: usize,
) -> Vec<Vec<u8>> {
    let mut qr_code: Vec<Vec<u8>> = vec![vec![0; 32]; 32];

    // Fill the position patterns (top-left, top-right, bottom-left)
    for i in 0..7 {
        for j in 0..7 {
            if (i == 0 || i == 6) || (j == 0 || j == 6) || (i > 1 && i < 5 && j > 1 && j < 5) {
                qr_code[i + start_index_x][j + start_index_y] = 1; // Top-left
                qr_code[i + start_index_x][size - 1 - j + start_index_y] = 1; // Top-right
                qr_code[size - 1 - i + start_index_x][j + start_index_y] = 1; // Bottom-left
                qr_code[size - 1 - i + start_index_x][size - 1 - j + start_index_y] = 1;
                // Bottom-right
            }
        }
    }
    /*
    Timing Patterns, ignored for now
    qr_code[6][8] = 1;
    qr_code[6][10] = 1;
    qr_code[6][12] = 1;
    qr_code[8][6] = 1;
    qr_code[10][6] = 1;
    qr_code[12][6] = 1;
     */

    if rotation == 0 {
        for i in 0..7 {
            for j in 0..7 {
                qr_code[i + start_index_x][size - 1 - j + start_index_y] = 0; // Bottom-right
            }
        }
    } else if rotation == 90 {
        for i in 0..7 {
            for j in 0..7 {
                qr_code[size - 1 - i + start_index_x][j + start_index_y] = 0; // Bottom-left
            }
        }
    } else if rotation == 180 {
        for i in 0..7 {
            for j in 0..7 {
                qr_code[i + start_index_x][j + start_index_y] = 0; // Top-left
            }
        }
    } else if rotation == 270 {
        for i in 0..7 {
            for j in 0..7 {
                qr_code[i + start_index_x][size - 1 - j + start_index_y] = 0; // Top-right
            }
        }
    }

    // Print the QR code
    // println!("sample raw qrcode");
    for row in qr_code.iter() {
        for &val in row.iter() {
            print!(
                "{} ",
                match val {
                    0 => "█", // Position and alignment patterns
                    1 => "░", // Separation patterns
                    _ => " ", // Background
                }
            );
        }
        // println!();
    }
    return qr_code;
}




fn _main() {
    let sample_raw_qrcode = _create_raw_qr_code(25, 90, 2, 5);
    let qr_code = find_qr_code(&sample_raw_qrcode);
    // println!("real qrcode");
    for row in qr_code.iter() {
        for &val in row.iter() {
            print!(
                "{} ",
                match val {
                    0 => "█", // Position and alignment patterns
                    1 => "░", // Separation patterns
                    _ => " ", // Background
                }
            );
        }
        // println!();
    }
}
fn find_position_patterns(matrix: &Vec<Vec<u8>>) -> Vec<Position> {
    let mut positions = Vec::new();
    for i in 0..=matrix.len() - 7 {
        for j in 0..=matrix[0].len() - 7 {
            if is_position_pattern(i, j, matrix) {
                // println!("Position pattern found at: ({}, {})", i, j);
                positions.push(Position {
                    col: j as u8,
                    row: i as u8,
                });
            }
        }
    }
    return positions;
}

fn is_position_pattern(x: usize, y: usize, matrix: &Vec<Vec<u8>>) -> bool {
    // Assuming 1 is black and 0 is white for simplicity
    // Check the outermost 7x7 border for the alternating pattern
    for i in 0..7 {
        for j in 0..7 {
            // Check the 7x7 square
            if i == 0 || i == 6 || j == 0 || j == 6 {
                if matrix[x + i][y + j] != 1 {
                    return false;
                }
            }
            // Check the inner 5x5 square
            else if i == 1 || i == 5 || j == 1 || j == 5 {
                if matrix[x + i][y + j] != 0 {
                    return false;
                }
            }
            // Check the 3x3 core
            else {
                if matrix[x + i][y + j] != 1 {
                    return false;
                }
            }
        }
    }
    return true;
}

fn rotate_90(matrix: &mut Vec<Vec<u8>>) {
    let n = matrix.len();
    // Transpose the matrix
    for i in 0..n {
        for j in i..n {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }
    // Reverse each row
    for row in matrix.iter_mut() {
        row.reverse();
    }
}

fn rotate_180(matrix: &mut Vec<Vec<u8>>) {
    // Reverse the rows
    matrix.reverse();
    // Reverse each row
    for row in matrix.iter_mut() {
        row.reverse();
    }
}

fn rotate_270(matrix: &mut Vec<Vec<u8>>) {
    let n = matrix.len();
    // Transpose the matrix
    for i in 0..n {
        for j in i..n {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }
    // Reverse each column
    for j in 0..n {
        let mut i1 = 0;
        let mut i2 = n - 1;
        while i1 < i2 {
            let temp = matrix[i1][j];
            matrix[i1][j] = matrix[i2][j];
            matrix[i2][j] = temp;
            i1 += 1;
            i2 -= 1;
        }
    }
}

pub fn find_qr_code(raw_qr_code: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut positions = find_position_patterns(raw_qr_code);
    // position pattern more than 3, more than 1 in positions can be invalid
    while positions.len() > 3 {
        let length = positions.len();
        for i in 0..length {
            let mut to_pop = true;
            for j in i + 1..length {
                // find pattern on the same row or col
                if positions[i].col == positions[j].col || positions[i].row == positions[j].row {
                    to_pop = false;
                    break;
                }
            }
            // find no corresponding position pattern, remove it
            if to_pop {
                positions.remove(i);
            }
        }
    }
    // already found 3 position patterns, just need to handle rotation
    let mut center_position = Position { col: 26, row: 26 };
    let mut pop_index = 0;
    for i in 0..3 {
        let mut is_center_position = 0;
        for j in 0..3 {
            // find corresponding position pattern on the same row or col
            if i != j
                && (positions[i].col == positions[j].col || positions[i].row == positions[j].row)
            {
                is_center_position += 1;
            }
        }
        if is_center_position == 2 {
            center_position.row = positions[i].row;
            center_position.col = positions[i].col;
            pop_index = i;
        }
    }
    positions.remove(pop_index);

    // invalid situation
    if center_position.row == 26 {
        return vec![vec![0; 21]; 21];
    }

    // find size and relative location of center_position
    let mut matrix_size = 0;
    let mut left = false;
    let mut right = false;
    let mut up = false;
    let mut down = false;
    // get start position to copy sub matrix
    let mut start_position = Position {
        col: center_position.col,
        row: center_position.row,
    };
    for position in positions.iter() {
        if position.row == center_position.row {
            if position.col > center_position.col {
                left = true;
                matrix_size = position.col - center_position.col + 7;
            } else {
                right = true;
                matrix_size = center_position.col - position.col + 7;
                start_position.col = position.col;
            }
        } else {
            if position.row > center_position.row {
                up = true;
            } else {
                start_position.row = position.row;
                down = true;
            }
        }
    }

    // clockwise rotation angle
    let mut rotation = -1;

    if left && up {
        rotation = 0;
    } else if left && down {
        rotation = 270;
    } else if right && up {
        rotation = 90;
    } else if right && down {
        rotation = 180;
    }

    // get sub matrix
    let mut res_matrix = vec![vec![0; matrix_size as usize]; matrix_size as usize];
    for i in 0..matrix_size {
        for j in 0..matrix_size {
            res_matrix[i as usize][j as usize] =
                raw_qr_code[(start_position.row + i) as usize][(start_position.col + j) as usize];
        }
    }
    if rotation == 90 {
        rotate_270(&mut res_matrix);
    } else if rotation == 180 {
        rotate_180(&mut res_matrix);
    } else if rotation == 270 {
        rotate_90(&mut res_matrix);
    }

    return res_matrix;
}
