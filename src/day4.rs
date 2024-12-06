use crate::util;

pub fn run(example: bool) {
    let data = if example {
        include_str!("../data/example/day4.1.txt")
    } else {
        include_str!("../data/day4.1.txt")
    };

    tracing::info!("day 4 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 4 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data),
    );

    if example {
        tracing::warn!("used example data");
    }

    tracing::info!("day 4 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 4 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data), // same data
    );
}

fn get_plane(data: &str) -> Vec<Vec<char>> {
    let mut plane = Vec::new();

    for row in data.lines() {
        plane.push(Vec::new());
        for col in row.chars() {
            plane.last_mut().unwrap().push(col);
        }
    }

    plane
}

fn part1(data: &str) -> usize {
    let mut got = Vec::<[(char, usize, usize, util::Direction); 4]>::new();

    let plane = get_plane(data);

    for (xrow, r) in plane.iter().enumerate() {
        for (xcol, &x) in r.iter().enumerate() {
            if x == 'X' {
                tracing::trace!("got an X");
                for (&m, mrow, mcol, mdir) in
                    util::surrounding(plane.as_slice(), xrow, xcol).filter(|(&m, _, _, _)| m == 'M')
                {
                    tracing::trace!("got an M {:?}", mdir);
                    'next_a: for (&a, arow, acol, adir) in
                        util::surrounding(plane.as_slice(), mrow, mcol)
                            .filter(|(&a, _, _, _)| a == 'A')
                    {
                        tracing::trace!("got an A {:?}", adir);
                        if adir != mdir {
                            tracing::trace!("wrong direction");
                            continue 'next_a;
                        }

                        'next_s: for (&s, srow, scol, sdir) in
                            util::surrounding(plane.as_slice(), arow, acol)
                                .filter(|(&s, _, _, _)| s == 'S')
                        {
                            tracing::trace!("got an S");
                            if sdir != adir {
                                tracing::trace!("wrong direction");
                                continue 'next_s;
                            }

                            tracing::debug!(
                                "gotcha! X={},{} M={},{} A={},{} S={},{}",
                                xrow,
                                xcol,
                                mrow,
                                mcol,
                                arow,
                                acol,
                                srow,
                                scol
                            );
                            got.push([
                                (x, xrow, xcol, util::Direction::E),
                                (m, mrow, mcol, mdir),
                                (a, arow, acol, adir),
                                (s, srow, scol, sdir),
                            ]);
                        }
                    }
                }
            }
        }
    }

    let mut vis = vec![vec!['.'; plane[0].len()]; plane.len()];
    for [(x, xrow, xcol, _), (m, mrow, mcol, _), (a, arow, acol, _), (s, srow, scol, _)] in
        got.iter()
    {
        vis[*xrow][*xcol] = *x;
        vis[*mrow][*mcol] = *m;
        vis[*arow][*acol] = *a;
        vis[*srow][*scol] = *s;
    }

    for row in vis {
        tracing::debug!("{:?}", row);
    }

    got.len()
}

fn part2(data: &str) -> usize {
    let mut got = Vec::new();

    let plane = get_plane(data);
    for (row, r) in plane.iter().enumerate().take(plane.len() - 1).skip(1) {
        'next_a: for (col, &c) in r.iter().enumerate().take(r.len() - 1).skip(1) {
            if c != 'A' {
                continue 'next_a;
            }

            let around = util::surrounding(plane.as_slice(), row, col).collect::<Vec<_>>();
            match around.as_slice() {
                // M M      M S      S S      S M
                //  A        A        A        A
                // S S      M S      M M      S M
                [(c1 @ &'M', row1, col1, _), _, (c2 @ &'M', row2, col2, _), _, _, (c3 @ &'S', row3, col3, _), _, (c4 @ &'S', row4, col4, _)]
                | [(c1 @ &'M', row1, col1, _), _, (c2 @ &'S', row2, col2, _), _, _, (c3 @ &'M', row3, col3, _), _, (c4 @ &'S', row4, col4, _)]
                | [(c1 @ &'S', row1, col1, _), _, (c2 @ &'S', row2, col2, _), _, _, (c3 @ &'M', row3, col3, _), _, (c4 @ &'M', row4, col4, _)]
                | [(c1 @ &'S', row1, col1, _), _, (c2 @ &'M', row2, col2, _), _, _, (c3 @ &'S', row3, col3, _), _, (c4 @ &'M', row4, col4, _)] =>
                {
                    got.push([
                        (**c1, *row1, *col1),
                        (**c2, *row2, *col2),
                        (**c3, *row3, *col3),
                        (**c4, *row4, *col4),
                    ]);
                }

                _ => {}
            }
        }
    }

    let mut vis = vec![vec!['.'; plane[0].len()]; plane.len()];
    for [(x, xrow, xcol), (m, mrow, mcol), (a, arow, acol), (s, srow, scol)] in got.iter() {
        vis[*xrow][*xcol] = *x;
        vis[*mrow][*mcol] = *m;
        vis[*arow][*acol] = *a;
        vis[*srow][*scol] = *s;
        vis[xrow + 1][xcol + 1] = 'A';
    }

    for row in vis {
        tracing::debug!("{:?}", row);
    }

    got.len()
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day4.1.txt")), 2644);
}

#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day4.1.txt")), 1952);
}
