use itertools::Itertools;
use std::collections::VecDeque;

fn bfs(grid: &[Vec<u8>], start: &[(usize, usize)], goal: (usize, usize)) -> Option<usize> {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = start
        .iter()
        .map(|&(x, y)| (x, y, 0))
        .collect::<VecDeque<_>>();
    while let Some((x, y, len)) = queue.pop_front() {
        if (x, y) == goal {
            return Some(len);
        }
        for (dx, dy) in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            let Some(&square) = grid.get(nx).and_then(|row| row.get(ny)) else { continue };
            if grid[x][y] + 1 >= square && !visited[nx][ny] {
                visited[nx][ny] = true;
                queue.push_back((nx, ny, len + 1));
            }
        }
    }
    None
}

fn main() {
    // get input and create grid
    let input = include_str!("..\\..\\input\\day12_input.txt");
    let mut grid = input
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();

    // search entire grid for 'S' and 'E' coordinates
    let (sx, sy) = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(x, y)| grid[x][y] == b'S')
        .unwrap();
    let (gx, gy) = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(x, y)| grid[x][y] == b'E')
        .unwrap();

    // replace 'S' with 'a', 'E' with 'z' for easier bfs
    grid[sx][sy] = b'a';
    grid[gx][gy] = b'z';

    // 
    let positions = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .filter(|&(x, y)| grid[x][y] == b'a')
        .collect::<Vec<_>>();

    //
    let result = (
        bfs(&grid, &[(sx, sy)], (gx, gy)).unwrap(),
        bfs(&grid, &positions, (gx, gy)).unwrap(),
    );

    println!("part one: {} \npart two: {}", result.0, result.1);
}
