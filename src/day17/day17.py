import sys
import heapq

F = open(sys.argv[1]).read().splitlines()


def part1(lines: list[list[str]]):
    # Mapping is h, x, y, dx, dy, d
    initial = (0, 0, 0, 0, 0, 0)
    hpq = [initial]

    seen = set()

    while hpq:
        h, x, y, dx, dy, d = heapq.heappop(hpq)

        if x == len(lines) - 1 and y == len(lines) - 1:
            return h

        if (x, y, dx, dy, d) in seen:
            continue

        seen.add((x, y, dx, dy, d))

        for tx, ty in [(0, 1), (1, 0), (-1, 0), (0, -1)]:
            nx, ny = x + tx, y + ty

            if nx < 0 or nx >= len(lines) or ny < 0 or ny >= len(lines[0]):
                continue

            ht = int(lines[nx][ny])

            if (dx, dy) == (tx, ty):
                if d >= 3:
                    continue
                else:
                    heapq.heappush(hpq, (h + ht, nx, ny, tx, ty, d + 1))
            elif (dx, dy) == (-tx, -ty):
                continue
            else:
                heapq.heappush(hpq, (h + ht, nx, ny, tx, ty, 1))


def part2(lines: list[list[str]]):
    # Mapping is h, x, y, dx, dy, d
    initial = (0, 0, 0, 0, 0, 0)
    hpq = [initial]

    seen = set()

    while hpq:
        h, x, y, dx, dy, d = heapq.heappop(hpq)

        # print(f"Current data ", h, x, y, dx, dy, d)

        if x == len(lines) - 1 and y == len(lines[0]) - 1 and d >= 4:
            # print(f"Found the exit")
            return h

        if (x, y, dx, dy, d) in seen:
            # print(f"Combination already seen")
            continue

        seen.add((x, y, dx, dy, d))

        if d < 10 and (dx, dy) != (0, 0):
            nx, ny = x + dx, y + dy

            if nx >= 0 and nx < len(lines) and ny >= 0 and ny < len(lines[0]):
                # print(f"We can keep going to the same dir ", nx, ny, dx, dy)
                ht = int(lines[nx][ny])
                heapq.heappush(hpq, (h + ht, nx, ny, dx, dy, d + 1))

        if d >= 4 or (dx, dy) == (0, 0):
            for tx, ty in [(0, 1), (1, 0), (-1, 0), (0, -1)]:
                if (tx, ty) != (dx, dy) and (tx, ty) != (-dx, -dy):
                    nx, ny = x + tx, y + ty

                    if nx >= 0 and nx < len(lines) and ny >= 0 and ny < len(lines[0]):
                        # print(f"Next stop", nx, ny, tx, ty)
                        ht = int(lines[nx][ny])
                        heapq.heappush(hpq, (h + ht, nx, ny, tx, ty, 1))


# print(f"{part1(F)}")
print(f"{part2(F)}")
