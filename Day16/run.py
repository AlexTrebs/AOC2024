import heapq


def parse(lines):
    grid = []
    line = 0
    for line in range(len(lines)):
        grid.append(list(lines[line].strip()))

    s = None
    e = None
    for r, row in enumerate(grid):
        for c, ch in enumerate(row):
            if ch == "S":
                s = (r, c)
            elif ch == "E":
                e = (r, c)
    return grid, s, e


def dijkstra(grid, starts):
    delta = {"E": (0, 1), "W": (0, -1), "N": (-1, 0), "S": (1, 0)}

    dist = {}
    pq = []
    for sr, sc, dir in starts:
        dist[(sr, sc, dir)] = 0
        heapq.heappush(pq, (0, sr, sc, dir))

    while pq:
        (d, row, col, direction) = heapq.heappop(pq)
        if dist[(row, col, direction)] < d:
            continue
        for next_dir in "EWNS".replace(direction, ""):
            if (row, col, next_dir) not in dist or dist[
                (row, col, next_dir)
            ] > d + 1000:
                dist[(row, col, next_dir)] = d + 1000
                heapq.heappush(pq, (d + 1000, row, col, next_dir))
        dr, dc = delta[direction]
        next_row, next_col = row + dr, col + dc
        if (
            0 <= next_row < len(grid)
            and 0 <= next_col < len(grid[0])
            and grid[next_row][next_col] != "#"
            and (
                (next_row, next_col, direction) not in dist
                or dist[(next_row, next_col, direction)] > d + 1
            )
        ):
            dist[(next_row, next_col, direction)] = d + 1
            heapq.heappush(pq, (d + 1, next_row, next_col, direction))

    return dist


def part1(input):
    grid, (sr, sc), (er, ec) = input
    dist = dijkstra(grid, [(sr, sc, "E")])
    best = 1000000000
    for dir in "EWNS":
        if (er, ec, dir) in dist:
            best = min(best, dist[(er, ec, dir)])
    return best


def part2(input):
    grid, (sr, sc), (er, ec) = input
    from_start = dijkstra(grid, [(sr, sc, "E")])
    from_end = dijkstra(grid, [(er, ec, d) for d in "EWNS"])
    optimal = part1(input)
    flip = {"E": "W", "W": "E", "N": "S", "S": "N"}
    result = set()
    for row in range(len(grid)):
        for col in range(len(grid[0])):
            for dir in "EWNS":
                state_from_start = (row, col, dir)
                state_from_end = (row, col, flip[dir])
                if state_from_start in from_start and state_from_end in from_end:
                    if (
                        from_start[state_from_start] + from_end[state_from_end]
                        == optimal
                    ):
                        result.add((row, col))
    return len(result)


sample = parse(open("input.txt").readlines())
real = parse(open("input.txt").readlines())
input = real

print(part1(input))
print(part2(input))
