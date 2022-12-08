from collections import defaultdict
from itertools import accumulate
from operator import concat
from typing import Tuple, Union

# The methods below were heavily inspired by RikJux's solution.
# See:  https://github.com/RikJux/AdventOfCode2022/blob/master/day_7/day_7.py
# My focus was on readability and understanding whats going on, not on efficiency.


def change_dir_and_info(entry: str) -> Tuple[bool, Union[str, int]]:
    """Parses a line of the input file and returns a tuple of the form (change_dir, info).
    change_dir is True if the line is a directory change, False otherwise.
    info is the name of the directory if change_dir is True, the size of the file otherwise.

    Parameters
    ----------
    entry : str
        A line of the input file

    Returns
    -------
    Tuple[bool, Union[str, int]]
        A tuple of the form (change_dir, info)
    """
    return (
        # is change dir, folder or ..
        (True, entry[1])
        if entry[0] == "$ cd"
        # is not change dir, file size
        else (False, int(entry[0]))
        if entry[0][0].isdigit()
        else None
    )


def make_dir_tree(file_input: str) -> dict(str, int):
    """Parses the input file and returns a dictionary of the form {directory: size}.
    The size of a directory is the sum of the sizes of all the files in it and its subdirectories.

    Parameters
    ----------
    file_input : str
        The input file

    Returns
    -------
    dict(str, int)
        A dictionary of the form {directory: size}
    """
    curr_dir_path = list()
    tree = defaultdict(int)
    commands = list()

    for line in file_input.splitlines():
        command = change_dir_and_info(line.strip("\n").rsplit(None, 1))
        if command is not None and isinstance(command[0], bool):
            commands.append(command)

    # print(*commands, sep="\n")
    for change_dir, info in commands:
        # print("curr_dir_path: ", curr_dir_path)
        if change_dir:
            if info == "..":
                curr_dir_path.pop()
            else:
                curr_dir_path.append(info)
        else:
            size = info  # readability > efficiency
            for directory in accumulate(curr_dir_path, concat):
                # print("directory: ", directory, "size: ", size)
                tree[directory] += size
                # print("tree: ", tree[directory], "\n")
    return tree


example_input = """$ cd /
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

if __name__ == "__main__":
    max_size = 100_000
    # with open("input.txt") as f:
    #     raw_input = f.read()
    dir_tree = make_dir_tree(example_input)
    # print(*dir_tree.items(), sep="\n")

    print(sum(filter(lambda x: x <= max_size, dir_tree.values())))
