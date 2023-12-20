# queue of (source, target, signal_type)
# data storage
# {
# flip-flop-name: { type: module_type[FLIPFLOP], target: [target], state: state }
# conjuntion-name: { type: module_type[CONJUNTION], target: [target], state: {vertex: state}}
# broadcast: { type: module_type[BROADCAST], target: [target1, target2], state: None}
# other: {type: module_type[OTHER], target: None, state: None}
# }

import sys
import math
from enum import Enum
import re
from collections import defaultdict, deque


class ModuleType(Enum):
    FLIPFLOP = 0
    CONJUNTION = 1
    BROADCAST = 2


F = open(sys.argv[1]).read().splitlines()

data = defaultdict(dict)

sep_reg = r"([%&]?)([a-z]+) -> ([a-z, ]+)"

for l in F:
    ty, n, tt = re.findall(sep_reg, l)[0]
    targets = list(map(lambda x: x.strip(), tt.split(",")))
    if ty == "%":
        data[n] = {
            "type": ModuleType.FLIPFLOP,
            "target": targets,
            "state": False,
        }
    elif ty == "&":
        data[n] = {
            "type": ModuleType.CONJUNTION,
            "target": targets,
            "state": {},
        }
    else:
        data[n] = {
            "type": ModuleType.BROADCAST,
            "target": targets,
            "state": None,
        }

for k, v in data.items():
    for t in v["target"]:
        if t in data and data[t]["type"] == ModuleType.CONJUNTION:
            data[t]["state"].update({k: False})

# print(data)


def part1():
    def push_button():
        global data

        res = {True: 0, False: 0}

        q = deque()
        q.append((None, "broadcaster", False))

        while q:
            source, target, current = q.popleft()

            res[current] += 1

            mod = data[target]
            if not mod:
                continue

            match (mod["type"], mod["target"], mod["state"], current):
                case (ModuleType.BROADCAST, targets, _, c):
                    for t in targets:
                        q.append((target, t, c))
                case (ModuleType.FLIPFLOP, _, _, True):
                    continue
                case (ModuleType.FLIPFLOP, targets, state, False):
                    data[target]["state"] = not state

                    for t in targets:
                        q.append((target, t, not state))
                case (ModuleType.CONJUNTION, targets, _, c):
                    data[target]["state"][source] = c

                    if all(data[target]["state"].values()):
                        pulse = False
                    else:
                        pulse = True

                    for t in targets:
                        q.append((target, t, pulse))
                case _:
                    continue

        return res[True], res[False]

    rest, resf = 0, 0
    for _ in range(1000):
        t, f = push_button()
        rest += t
        resf += f

    return rest * resf


def part2():
    global data

    ss = [k for k, v in data.items() if "rx" in v["target"]][0]
    cycles = {}
    seen = {k: 0 for k, v in data.items() if ss in v["target"]}

    npress = 0

    while True:
        npress += 1
        res = {True: 0, False: 0}

        q = deque()
        q.append((None, "broadcaster", False))

        while q:
            source, target, current = q.popleft()

            res[current] += 1

            mod = data[target]
            if not mod:
                continue

            if target == ss and current:
                seen[source] += 1

                if source not in cycles:
                    cycles[source] = npress

                if all(seen.values()):
                    res = 1
                    for cl in cycles.values():
                        res = res * cl // math.gcd(res, cl)
                    return res

            match (mod["type"], mod["target"], mod["state"], current):
                case (ModuleType.BROADCAST, targets, _, c):
                    for t in targets:
                        q.append((target, t, c))
                case (ModuleType.FLIPFLOP, _, _, True):
                    continue
                case (ModuleType.FLIPFLOP, targets, state, False):
                    data[target]["state"] = not state

                    for t in targets:
                        q.append((target, t, not state))
                case (ModuleType.CONJUNTION, targets, _, c):
                    data[target]["state"][source] = c

                    if all(data[target]["state"].values()):
                        pulse = False
                    else:
                        pulse = True

                    for t in targets:
                        q.append((target, t, pulse))
                case _:
                    continue


# print(f"{part1()}")
print(f"{part2()}")
