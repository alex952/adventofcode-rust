import sys
import re

F = open(sys.argv[1]).read().splitlines()

RULES = {}
PARTS = []


for l in F:
    if not l:
        continue

    if l[0] == "{":
        d = {x: int(y) for x, y in map(lambda p: p.split("="), l[1:-1].split(","))}
        PARTS.append(d)
    else:
        rn = l[0 : l.index("{")]
        r = [r for r in l[l.index("{") + 1 : l.index("}")].split(",")]

        r = [
            list(
                filter(
                    lambda x: x,
                    re.findall(r"(?:([a-z])([<>])(\d+):([a-z]+|A|R))|([a-z]+|A|R)", r)[
                        0
                    ],
                )
            )
            for r in r
        ]

        RULES[rn] = r


def part1(rules, parts):
    count = 0

    for p in parts:
        curr = "in"

        print(f"Current rule names is {curr}")

        while curr not in ["R", "A"]:
            rl = rules[curr]

            for r in rl:
                if len(r) == 1:
                    print(f"Rule {r} is direct")
                    curr = r[0]
                    break

                match r:
                    case [pn, "<", n, t]:
                        print(f"Rule {r} is <")
                        if p[pn] < int(n):
                            print(f"Is a match, moving to {t}")
                            curr = t
                            break
                    case [pn, ">", n, t]:
                        print(f"Rule {r} is >")
                        if p[pn] > int(n):
                            print(f"Is a match, moving to {t}")
                            curr = t
                            break

        print(f"Part {p} is {curr}")
        if curr == "A":
            count += sum(p.values())

    return count


def part2(rules, parts, ranges, rn="in"):
    if rn == "A":
        product = 1
        for lo, hi in ranges.values():
            product *= hi - lo + 1
        return product

    if rn == "R":
        return 0

    rls = rules[rn]

    count = 0

    for r in rls:
        match r:
            case [_]:
                count += part2(rules, parts, ranges, r[0])
                continue
            case [k, "<", n, _]:
                l, h = ranges[k]
                T = (l, min(int(n) - 1, h))
                F = (max(int(n), l), h)
            case [k, ">", n, _]:
                l, h = ranges[k]
                T = (max(int(n) + 1, l), h)
                F = (l, min(int(n), h))

        if T[0] <= T[1]:
            copy = dict(ranges)
            copy[r[0]] = T
            count += part2(rules, parts, copy, r[-1])
        if F[0] <= F[1]:
            ranges = dict(ranges)
            ranges[r[0]] = F
        else:
            break

    return count


# print(f"{part1(RULES, PARTS)}")
print(f"{part2(RULES, PARTS, {key: (1, 4000) for key in 'xmas'})}")
