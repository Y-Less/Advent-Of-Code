use std::{ vec::Vec, cmp::Ordering, collections::{ BinaryHeap, HashMap, HashSet } };

pub fn dijkstra(
    start: (usize, usize),
    adjacency_list: &HashMap<(usize, usize), Vec<((usize, usize), usize)>>,
) -> HashMap<(usize, usize), usize>
{
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();

    distances.insert(start, 0);
    to_visit.push(Visit
	{
        vertex: start,
        distance: 0,
    });

    while let Some(Visit { vertex, distance }) = to_visit.pop()
	{
        if !visited.insert(vertex)
		{
            // Already visited this node
            continue;
        }

        if let Some(neighbors) = adjacency_list.get(&vertex)
		{
            for (neighbor, cost) in neighbors {
                let new_distance = distance + cost;
                let is_shorter = distances
                    .get(&neighbor)
                    .map_or(true, |&current| new_distance < current);

                if is_shorter {
                    distances.insert(*neighbor, new_distance);
                    to_visit.push(Visit {
                        vertex: *neighbor,
                        distance: new_distance,
                    });
                }
            }
        }
    }

    distances
}

fn is_wall(ch: u8) -> bool
{
	ch == 0x23 || (ch >= 0x41 && ch <= 0x5A)
}

pub fn build_adjacency(grid: &Vec<Vec<u8>>) -> HashMap<(usize, usize), Vec<((usize, usize), usize)>>
{
	let mut adj: HashMap<(usize, usize), Vec<((usize, usize), usize)>> = HashMap::new();

	for (y, row) in grid.iter().enumerate()
	{
		for (x, dot) in row.iter().enumerate()
		{
			if is_wall(*dot)
			{
				continue;
			}
			let mut vec = Vec::new();
			if !is_wall(grid[y + 1][x])
			{
				vec.push(((x, y + 1), 1));
			}
			if !is_wall(grid[y][x + 1])
			{
				vec.push(((x + 1, y), 1));
				//matrix[base + (y + 1) * DIM + x] = 1;
			}
			if !is_wall(grid[y - 1][x])
			{
				vec.push(((x, y - 1), 1));
			}
			if !is_wall(grid[y][x - 1])
			{
				vec.push(((x - 1, y), 1));
				//matrix[base + (y + 1) * DIM + x] = 1;
			}
			adj.insert((x, y), vec);
		}
	}

	adj
}

#[derive(Debug)]
struct Visit<V> {
    vertex: V,
    distance: usize,
}

impl<V> Ord for Visit<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<V> PartialOrd for Visit<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Visit<V> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<V> Eq for Visit<V> {}

