#![feature(test)]

#[macro_use]
extern crate test;

const GRID_DIM: usize = 300;
const GRID_DIMS: usize = GRID_DIM * GRID_DIM;

struct Grid {
    cells: Vec<i64>,
    table: Vec<i64>
}

impl Grid {
    pub fn coords_to_cell(x: i32, y: i32) -> Option<usize> {
        let x_adj = x - 1;
        let y_adj = y - 1;
        if x_adj < 0 || y_adj < 0 || x_adj as usize >= GRID_DIM || y_adj as usize >= GRID_DIM {
            None
        }
        else {
            Some((y_adj * GRID_DIM as i32 + x_adj) as usize)
        }
    }

    pub fn cell_to_coords(idx: usize) -> (i32, i32) {
        let y = idx / GRID_DIM;
        let x = idx - y * GRID_DIM;
        (x as i32 + 1, y as i32 + 1)
    }

    pub fn get_cell_power_level(x: i32, y: i32, serial: u32) -> i64 {
        let rack_id = x as i64 + 10;
        let mut power_level: i64 = rack_id * y as i64;
        power_level += serial as i64;
        power_level *= rack_id;
        power_level = (power_level / 100) % 10;
        power_level - 5
    }

    pub fn sample_nxn(&self, idx: usize, n: usize) -> Option<(i32, i32, i64)> {
        let mut power: i64 = 0;
        let coords = Grid::cell_to_coords(idx);
        let x = coords.0;
        let y = coords.1;
        for i in x..x + n as i32 {
            for j in y..y + n as i32 {
                let mut result = Grid::coords_to_cell(i, j);
                if result.is_none() {
                    return None;
                }
                power += self.cells[result.unwrap()];
            }
        }
        Some((x, y, power))
    }

    pub fn build_table(&mut self) {
        for x in 0..GRID_DIM {
            self.table[x] = self.cells[x];
        }
        for i in 1..GRID_DIM {
            for j in 0..GRID_DIM {
                let dst = j * GRID_DIM + i;
                let src = j * GRID_DIM + i - 1;
                self.table[dst] = self.cells[dst] + self.table[src];
            }
        }
        for i in 0..GRID_DIM {
            for j in 1..GRID_DIM {
                let dst = j * GRID_DIM + i;
                let src = (j - 1) * GRID_DIM + i;
                self.table[dst] += self.table[src];
            }
        }
    }

    pub fn sample_table_nxn(&self, idx: usize, n: usize) -> Option<(i32, i32, i64)> {
        if n == 0 {
            return None;
        }

        let coords = Grid::cell_to_coords(idx);
        let x = coords.0;
        let y = coords.1;

        let n1 = n - 1;
        let xn = x + n1 as i32;
        let yn = y + n1 as i32;

        let cell = Grid::coords_to_cell(xn, yn);
        if cell.is_none() {
            return None;
        }
        let mut power = self.table[cell.unwrap()];
        if x > 1 {
            let dst = Grid::coords_to_cell(x-1, yn);
            if cell.is_none() {
                return None;
            }
            power -= self.table[dst.unwrap()];
        }

        if y > 1 {
            let dst = Grid::coords_to_cell(xn, y-1);
            if cell.is_none() {
                return None;
            }
            power -= self.table[dst.unwrap()];
        }

        if x > 1 && y > 1 {
            let dst = Grid::coords_to_cell(x-1, y-1);
            if cell.is_none() {
                return None;
            }
            power += self.table[dst.unwrap()];
        }
        Some((x, y, power))
    }
}

fn part1(input: u32) -> (i32, i32, i64) {
    let mut grid = Grid {cells: vec![0i64; GRID_DIMS], table: vec![0i64; GRID_DIMS]};
    for x in 0..GRID_DIMS {
        let coords = Grid::cell_to_coords(x);
        grid.cells[x] = Grid::get_cell_power_level(coords.0, coords.1, input);
    }
    let mut results = Vec::new();
    for x in 0..GRID_DIMS {
        let result = grid.sample_nxn(x, 3);
        if result.is_some() {
            results.push(result.unwrap());
        }
    }
    *results.iter().max_by_key(|x| x.2).unwrap()
}

// without summed area it's too slow
fn part2_summed_area(input: u32) -> (i32, i32, i64) {
    let mut grid = Grid {cells: vec![0i64; GRID_DIMS], table: vec![0i64; GRID_DIMS]};
    for x in 0..GRID_DIMS {
        let coords = Grid::cell_to_coords(x);
        grid.cells[x] = Grid::get_cell_power_level(coords.0, coords.1, input);
    }
    grid.build_table();

    let mut results = vec![(0i32, 0i32, 0i64); GRID_DIM];
    for n in 1..=GRID_DIM {
        let mut result = Vec::new();
        for x in 0..GRID_DIMS {
            let tmp = grid.sample_table_nxn(x, n);
            if tmp.is_some() {
                result.push(tmp.unwrap());
            }
        }
        results[n-1] = *result.iter().max_by_key(|x| x.2).unwrap();
    }

    let mut best: i64 = std::i64::MIN;
    let mut output: (i32, i32, i64) = (0, 0, 0);
    let mut n = 1;
    for x in &results {
        if x.2 > best {
            output = (x.0, x.1, n);
            best = x.2;
        }
        n += 1;
    }
    output
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_power_level() {
        use Grid;
        assert_eq!(Grid::get_cell_power_level(3, 5, 8), 4);
        assert_eq!(Grid::get_cell_power_level(122, 79, 57), -5);
        assert_eq!(Grid::get_cell_power_level(217, 196, 39), 0);
        assert_eq!(Grid::get_cell_power_level(101, 153, 71), 4);
    }

    #[test]
    fn test_part1_ex_0() {
        use part1;
        assert_eq!(part1(18), (33, 45, 29));
    }

    #[test]
    fn test_part1_ex_1() {
        use part1;
        assert_eq!(part1(42), (21, 61, 30));
    }

    #[test]
    fn test_part1_input() {
        use part1;
        assert_eq!(part1(9110), (21, 13, 28));
    }

    #[test]
    fn test_part2_ex_0() {
        use part2_summed_area;
        assert_eq!(part2_summed_area(18), (90, 269, 16));
    }

    #[test]
    fn test_part2_ex_1() {
        use part2_summed_area;
        assert_eq!(part2_summed_area(42), (232, 251, 12));
    }
}

fn main() {
    println!("result: {:?}", part2_summed_area(9110));
}
