//! Exercise aiming to explore how to create a statically allocated matrix and draw a spiral on it.
//! The spiral is represented through an incremental number following its path through the matrix
//! and through the color scheme.

use colored::{Color, Colorize};
const ROWS: usize = 8;
const COLS: usize = 10;

const COLORS: [Color; 16] = [
    Color::Black,
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
    Color::White,
    Color::BrightBlack,
    Color::BrightRed,
    Color::BrightGreen,
    Color::BrightYellow,
    Color::BrightBlue,
    Color::BrightMagenta,
    Color::BrightCyan,
    Color::BrightWhite,
];

#[derive(Copy, Clone)]
struct ColoredCell {
    value: i32,
    color: Color,
}

struct ColoredMatrix {
    m: [[ColoredCell; COLS]; ROWS],
}

impl ColoredMatrix {
    fn new() -> ColoredMatrix {
        ColoredMatrix {
            m: [[ColoredCell {
                value: 0,
                color: Color::BrightWhite,
            }; COLS]; ROWS],
        }
    }

    fn print(&self) {
        const MAX_VALUE: usize = ROWS * COLS - 1;
        const DIGITS: u32 = MAX_VALUE.ilog10() + 1;
        const WIDTH: usize = DIGITS as usize + 1;
        self.m.iter().for_each(|row| {
            row.iter().for_each(|cell| {
                print!("{}", format!("{:>WIDTH$}", cell.value).color(cell.color));
            });
            println!()
        });
        println!();
    }

    fn draw_spiral(&mut self) {
        let mut elements = ROWS * COLS;
        let (mut rows, mut cols) = (ROWS, COLS);
        let (mut row_origin, mut col_origin) = (0_usize, 0_usize);
        let mut color_base_index = 0;
        let mut next_num = 0;
        while elements != 0 {
            #[cfg(debug_assertions)]
            println!(
                "entering draw_boundary with (\
                elements={elements}, \
                row_origin={row_origin}, \
                col_origin={col_origin}, \
                rows={rows}, \
                cols={cols}, \
                color_base_index={color_base_index}, \
                next_num={next_num}\
                )"
            );
            let (drawn_elements, used_colors) = self.draw_boundary(
                row_origin,
                col_origin,
                rows,
                cols,
                color_base_index,
                next_num,
            );
            elements -= drawn_elements;
            next_num += drawn_elements;
            color_base_index = (color_base_index + (used_colors as usize)) % COLORS.len();
            row_origin = row_origin.saturating_add(1);
            col_origin = col_origin.saturating_add(1);
            rows = rows.saturating_sub(2);
            cols = cols.saturating_sub(2);
        }
    }

    fn draw_boundary(
        &mut self,
        row_origin: usize,
        col_origin: usize,
        rows: usize,
        cols: usize,
        color_base_index: usize,
        base_number: usize,
    ) -> (usize, u8) {
        let elements = if rows == 1 {
            cols
        } else if cols == 1 {
            rows
        } else {
            (rows - 1) * 2 + (cols - 1) * 2
        };

        if elements == 1 {
            self.m[0][0] = ColoredCell {
                value: base_number as i32,
                color: COLORS[color_base_index],
            };
            return (1, 1);
        }

        // We can use at maximum 4 colors: use this variable as a 4-bit array that will be raised
        // individually, and count how many ones are raised at then end.
        let mut used_colors: u8 = 0;

        for v in 0..elements {
            let (i, j): (usize, usize);
            let color_index: usize;
            if v < cols - 1 {
                color_index = 0;
                (i, j) = (0, v);
            } else if v < cols + rows - 2 {
                color_index = 1;
                (i, j) = (v - (cols - 1), cols - 1);
            } else if v < 2 * cols + rows - 3 {
                color_index = 2;
                (i, j) = (rows - 1, cols - 1 - (v % (cols + rows - 2)));
            } else {
                color_index = 3;
                (i, j) = (rows - 1 - (v % (2 * cols + rows - 3)), 0);
            }
            self.m[i + row_origin][j + col_origin] = ColoredCell {
                value: (v + base_number) as i32,
                color: COLORS[(color_base_index + color_index) % COLORS.len()],
            };
            used_colors |= 1 << color_index;
        }

        (elements, used_colors.count_ones() as u8)
    }
}

fn main() {
    let mut m = ColoredMatrix::new();
    if cfg!(debug_assertions) {
        println!("\nMatrix before being spiralized:\n");
        m.print();
    }
    m.draw_spiral();
    println!("\nSpiralized matrix:\n");
    m.print();
}
