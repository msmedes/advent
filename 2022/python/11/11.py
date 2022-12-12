from collections import deque
import math
import operator


def main():
    with open('../inputs/day11.txt') as f:
        monkeys = [Monkey.from_blob(line) for line in f.read().split('\n\n')]
    print(monkeys)
    answer1 = part1(monkeys)
    print(answer1)

def part1(monkeys):
    lcm = math.lcm(*[m.test for m in monkeys])
    for _ in range(10000):
        for monkey in monkeys:
            monkey.play_round(monkeys, lcm)
    
    sorted_monkeys = sorted(monkeys, key=lambda x: x.items_inspected, reverse=True)
    print('sorted monkeys', sorted_monkeys)
    return sorted_monkeys[0].items_inspected * sorted_monkeys[1].items_inspected

class Monkey:
    ops = {
        "+": operator.add,
        "-": operator.sub,
        "*": operator.mul,
        "/": operator.floordiv
    }   

    def __init__(self, number, items, operation, test, true, false):
        self.number = number
        self.items = items
        self.operation = operation
        self.test = test
        self.true = true
        self.false = false
        self.items_inspected = 0
    
    def __repr__(self):
        rep =  f'\n\nMonkey {self.number}\nitems: {self.items}'
        rep += f'\noperation: {self.operation}'
        rep += f'\ntest: {self.test}'
        rep += f'\ntrue: {self.true}'
        rep += f'\nfalse: {self.false}'
        rep += f'\nitems inspected: {self.items_inspected}'
        return rep

    @classmethod
    def from_blob(cls, blob):
        lines = blob.split('\n')
        number, items, operation, test, true, false = lines
        number = int(number[-2])
        items = cls._get_items(items)
        test = int(test.split(' ')[-1])
        true = int(true[-1])
        false = int(false[-1])
        operation = cls._get_operation(operation)

        return Monkey(number, items, operation, test, true, false)


    def _get_items(items):
        return deque(int(item) for item in items[18:].split(', '))

    def _get_operation(operation):
        return operation[19:].split(" ")

    def play_round(self, monkeys, lcm):
        while self.items:
            self.items_inspected += 1 
            current_item = self.items.popleft()
            current_item = self.increase_worry(current_item)
            current_item = self.get_bored(current_item, lcm)
            next_monkey = self.throw_to_friend(current_item)
            monkeys[next_monkey].items.append(current_item)

    def increase_worry(self, current_item):
        a = current_item if self.operation[0] == 'old' else int(self.operation[0])
        b = current_item if self.operation[2] == 'old' else int(self.operation[2])
        op =  self.ops[self.operation[1]](a, b)
        return op
    
    def get_bored(self, current_item, lcm):
        return current_item % lcm

    def throw_to_friend(self, current_item):
        if current_item % self.test == 0:
            return self.true
        return self.false

        
main()