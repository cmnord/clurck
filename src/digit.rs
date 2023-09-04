pub const NUM_ROWS: usize = 13;
pub const NUM_COLS: usize = 4;

pub type DigitArray = [[u8; NUM_ROWS]; NUM_COLS];

// X is an alias for readability
const X: u8 = 1;

const DIGIT_ZERO: DigitArray = transpose([
    [0, X, X, 0],
    [X, X, X, X],
    [X, 0, 0, X],
    [X, 0, 0, X],
    [X, 0, 0, X],
    [X, 0, 0, X],
    [X, 0, 0, X],
    [X, 0, 0, X],
    [X, 0, 0, X],
    [X, 0, 0, X],
    [X, 0, 0, X],
    [X, X, X, X],
    [0, X, X, 0],
]);

const DIGIT_ONE: DigitArray = transpose([
    [0, X, X, 0],
    [X, X, X, 0],
    [X, 0, X, 0],
    [0, 0, X, 0],
    [0, 0, X, 0],
    [0, 0, X, 0],
    [0, 0, X, 0],
    [0, 0, X, 0],
    [0, 0, X, 0],
    [0, 0, X, 0],
    [0, 0, X, 0],
    [0, 0, X, 0],
    [0, 0, X, 0],
]);

const DIGIT_TWO: DigitArray = transpose([
    [0, X, X, 0],
    [X, X, X, X],
    [X, 0, 0, X],
    [0, 0, X, 0],
    [0, 0, X, 0],
    [0, 0, X, 0],
    [0, 0, X, 0],
    [0, 0, X, 0],
    [0, 0, X, 0],
    [0, 0, X, 0],
    [0, 0, X, 0],
    [0, 0, X, 0],
    [0, 0, X, 0],
]);

pub const DIGITS: [DigitArray; 3] = [DIGIT_ZERO, DIGIT_ONE, DIGIT_TWO];

const fn transpose(arr: [[u8; NUM_COLS]; NUM_ROWS]) -> DigitArray {
    let mut transposed = [[0; NUM_ROWS]; NUM_COLS];
    let mut i = 0;
    while i < NUM_ROWS {
        let mut j = 0;
        while j < NUM_COLS {
            transposed[j][i] = arr[i][j];
            j += 1;
        }
        i += 1;
    }
    transposed
}
