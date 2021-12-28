/// 4-way directions: up/down/left/right.
pub fn neighbors_4_way(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> impl Iterator<Item=(usize, usize)> {
    let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    neighbors_helper(grid, i, j, deltas.into_iter())
}

/// 8-way directions, including diagonals.
pub fn neighbors_8_way(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> impl Iterator<Item=(usize, usize)> {
    let deltas = [-1, 0, 1].into_iter().flat_map(|di| {
        [-1, 0, 1].into_iter().filter_map(move |dj| {
            if (di, dj) != (0, 0) {
                Some((di, dj))
            } else {
                None
            }
        })
    });

    neighbors_helper(grid, i, j, deltas)
}

/// Helper function for `neighbors_4_way` and `neighbors_8_way`.
fn neighbors_helper(
    grid: &Vec<Vec<u32>>,
    i: usize,
    j: usize,
    deltas: impl Iterator<Item=(isize, isize)>
) -> impl Iterator<Item=(usize, usize)> {
    let i = i as isize;
    let j = j as isize;

    let r = grid.len() as isize;
    let c = grid[0].len() as isize;

    deltas.filter_map(move |(di, dj)| {
        let i2 = i + di;
        let j2 = j + dj;

        if 0 <= i2 && i2 < r &&
            0 <= j2 && j2 < c
        {
            Some((i2 as usize, j2 as usize))
        } else {
            None
        }
    })
}
