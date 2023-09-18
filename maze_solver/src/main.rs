#[derive(Clone, Copy, PartialEq, Debug)]
struct MazePoint {
    x: usize,
    y: usize,
}

const DIR: [[isize; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

fn main() {
    let maze = vec![
        "xxxxxxxxxx x",
        "x        x x",
        "x        x x",
        "x xxxxxxxx x",
        "x          x",
        "x xxxxxxxxxx",
    ];

    let path = maze_solver(
        &maze,
        "#",
        MazePoint { x: 10, y: 0 },
        MazePoint { x: 1, y: 5 },
    );

    dbg!(path);
}

fn walk(
    maze: &Vec<&str>,
    wall: &str,
    curr: MazePoint,
    end: MazePoint,
    seen: &mut Vec<Vec<bool>>,
    path: &mut Vec<MazePoint>,
) -> bool {
    // off the map
    if curr.x >= maze[0].len() || curr.y >= maze.len() {
        return false;
    }

    // on a wall
    if maze[curr.y][curr.x..curr.x + 1] == *wall {
        return false;
    }

    // at the end
    if curr.y == end.y && curr.x == end.x {
        path.push(curr);
        return true;
    }

    if seen[curr.y][curr.x] {
        return false;
    }

    seen[curr.y][curr.x] = true;
    path.push(curr);

    for i in 0..DIR.len() {
        let [x, y] = DIR[i];
        if walk(
            maze,
            wall,
            MazePoint {
                x: (curr.x as isize + x) as usize,
                y: (curr.y as isize + y) as usize,
            },
            end,
            seen,
            path,
        ) {
            return true;
        }
    }

    path.pop();
    return false;
}

fn maze_solver(maze: &Vec<&str>, wall: &str, start: MazePoint, end: MazePoint) -> Vec<MazePoint> {
    let mut seen: Vec<Vec<bool>> = vec![];
    let mut path: Vec<MazePoint> = vec![];

    for _ in 0..maze.len() {
        seen.push(vec![false; maze[0].len()]);
    }

    walk(&maze, wall, start, end, &mut seen, &mut path);

    return path;
}

#[cfg(test)]
mod tests {
    use super::{maze_solver, MazePoint};

    #[test]
    fn maze_test() {
        let maze = vec![
            "xxxxxxxxxx x",
            "x        x x",
            "x        x x",
            "x xxxxxxxx x",
            "x          x",
            "x xxxxxxxxxx",
        ];

        let maze_result = vec![
            MazePoint { x: 10, y: 0 },
            MazePoint { x: 10, y: 1 },
            MazePoint { x: 10, y: 2 },
            MazePoint { x: 10, y: 3 },
            MazePoint { x: 10, y: 4 },
            MazePoint { x: 9, y: 4 },
            MazePoint { x: 8, y: 4 },
            MazePoint { x: 7, y: 4 },
            MazePoint { x: 6, y: 4 },
            MazePoint { x: 5, y: 4 },
            MazePoint { x: 4, y: 4 },
            MazePoint { x: 3, y: 4 },
            MazePoint { x: 2, y: 4 },
            MazePoint { x: 1, y: 4 },
            MazePoint { x: 1, y: 5 },
        ];

        let result = maze_solver(
            &maze,
            "x",
            MazePoint { x: 10, y: 0 },
            MazePoint { x: 1, y: 5 },
        );

        assert_eq!(draw_path(&maze, &maze_result), draw_path(&maze, &result));
    }

    fn draw_path(data: &Vec<&str>, path: &Vec<MazePoint>) -> Vec<String> {
        let mut data2: Vec<Vec<char>> = data.iter().map(|row| row.chars().collect()).collect();

        for p in path {
            if let Some(row) = data2.get_mut(p.y) {
                if let Some(cell) = row.get_mut(p.x) {
                    *cell = '*';
                }
            }
        }

        data2
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect()
    }
}
