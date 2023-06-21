from __future__ import annotations
# read content of file and return list of lines
from dataclasses import dataclass
from typing import *
import re
import textwrap


def read_file(filename):
    with open(filename) as file:
        return [line.strip() for line in file.readlines()]


class Node:
    name: str
    def size(self) -> int:
        raise NotImplementedError


@dataclass
class File(Node):
    name: str
    sized: int
    def size(self) -> int:
        return self.sized

    def __repr__(self):
        return f"- {self.name} (file , size = {self.sized})"


class Directory(Node):
    name: str
    children: List[Node]
    parent: Optional[Directory]
    def __init__(self, name: str, parent: Optional[Directory]):
        self.name = name
        self.parent = parent
        self.children = []

    def create_child_directory(self, name: str) -> Directory:
        child = Directory(name, self)
        self.children.append(child)
        return child

    def add_child_file(self, name: str, size: int):
        child = File(name, size)
        self.children.append(child)

    def size(self) -> int:
        return sum([child.size() for child in self.children])


    def __repr__(self):
        first_line = f"- {self.name} (dir, size = {self.size()})"
        children_lines = [f"{textwrap.indent(child.__repr__(), '  ')}" for child in self.children]
        return "\n".join([first_line] + children_lines)

    def sum_all_dir_less_than_1000000(self) -> int:
        s = 0
        for child in self.children:
            if isinstance(child, Directory):
                s += child.sum_all_dir_less_than_1000000()
        if self.size() < 100000:
            s += self.size()
        return s

    def min_above(self, size: int) -> Optional[int]:
        s = self.size()
        if s < size:
            return None
        else:
            for child in self.children:
                if isinstance(child, Directory):
                    res = child.min_above(size)
                    if res is not None and res < s:
                        s = res
            return s



file = read_file("../../src/input/day7.txt")
def solve(file: List[str]) -> int:
    cur = Directory("/", None)
    root = cur
    ls_regex = re.compile(r"^\$ ls$")
    cd_regex = re.compile(r"^\$ cd (.*)$")
    file_regex = re.compile(r"^(\d+) (.+)$")
    folder_regex = re.compile(r"^dir (.*)$")


    for line in file:
        if ls_regex.match(line):
            continue
        elif cd_regex.match(line):
            match = cd_regex.match(line)
            name = match.group(1)
            if name == "..":
                cur = cur.parent
            else:
                for child in cur.children:
                    if child.name == name:
                        cur = child
                        break
        elif file_regex.match(line):
            match = file_regex.match(line)
            name = match.group(2)
            size = int(match.group(1))
            cur.add_child_file(name, size)
        elif folder_regex.match(line):
            match = folder_regex.match(line)
            name = match.group(1)
            cur.create_child_directory(name)
        else:
            print(line)
            print("Invalid command")

    print(root)
    min_size = root.size() - 40000000
    print(min_size)
    print(root.min_above(min_size))
    print(root.sum_all_dir_less_than_1000000())


file2 = """$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"""
# file = file.split("\n")
solve(file)