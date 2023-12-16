import sys


def hsh(s: str) -> int:
    curr = 0
    for c in s:
        curr += ord(c)
        curr *= 17
        curr = curr % 256

    return curr


I = open(sys.argv[1]).read().splitlines()[0].split(",")


def part1(steps: list[str]) -> int:
    res = sum(map(hsh, steps))
    return res


def part2(steps: list[str]) -> int:
    from collections import OrderedDict, defaultdict

    x = defaultdict(OrderedDict)

    for s in steps:
        match s:
            case equals if s.find("=") > 0:
                print(f"Part {s} has an equals: {equals}")
                l, f = equals.split("=")
                h = hsh(l)
                if l in x[h]:
                    x[h][l] = f
                else:
                    x[h].update({l: f})
            case minus if s.find("-") > 0:
                print(f"Part {s} has a minus: {minus}")
                l, _ = equals.split("-")
                h = hsh(l)

                x[h].pop(l, "")

            case _:
                print(f"None of the above")

    res = 0

    for b, c in x.items():
        for idx, l in enumerate(c.keys()):
            lens_res = (b + 1) * (idx + 1) * int(c[l])
            res += lens_res

    return res


print(f"{part2(I)}")
