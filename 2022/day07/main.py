# Author:  mateusz.janda@gmail.com
# Ad maiorem Dei gloriam

from typing import Iterator
from typing import Optional
from typing import Tuple


def read_line() -> Iterator[Tuple[str, str]]:
    """Read line from stdin."""
    while True:
        try:
            line = input()
        except EOFError as _:
            return

        words = line.strip().split()
        if line.startswith("$ cd"):
            yield words[1], words[2]
        elif line.startswith("$ ls"):
            yield words[1], None
        else:
            yield words[0], words[1]


class Node:
    """Filesystem node."""

    def __init__(
        self,
        name: str,
        is_file: bool,
        parent: "Node",
        size: Optional[int] = None,
    ) -> None:
        self.name = name
        self.is_file = is_file
        self.parent = parent
        self.size = size

        self.children = []

    def add_folder(self, name: str) -> None:
        """Add folder if doesn't exist yet."""
        for node in self.children:
            if node.name == name and not node.is_folder:
                return

        node = Node(name, is_file=False, parent=self)
        self.children.append(node)

    def add_file(self, name: str, size: int) -> None:
        """Add file to current directory."""
        for node in self.children:
            if node.name == name and node.is_file:
                return

        node = Node(name, is_file=True, parent=self, size=size)
        self.children.append(node)

    def go_to_folder(self, name: str) -> "Node":
        """Get folder node."""
        if name == "..":
            return self.parent

        for node in self.children:
            if node.name == name and not node.is_file:
                return node

        raise Exception("Folder not found")


SIZE_LIMIT = 100_000
UNUSED_SIZE_LIMIT = 30_000_000
DISK_SIZE = 70_000_000


def main() -> None:
    """Main body."""
    root = Node("/", is_file=False, parent=None)
    current_node = root

    # Build node tree
    for info, name in read_line():
        if info == "ls":
            pass
        elif info == "cd" and name == "/":
            current_node = root
        elif info == "cd":
            current_node = current_node.go_to_folder(name)
        elif info == "dir":
            current_node.add_folder(name)
        elif info.isdigit():
            current_node.add_file(name, int(info))

    # Part 1
    dfs = Dfs()
    root_size = dfs.search_folders_below_limit(root)
    print(dfs.all_below_limit)

    # Part 2
    unused_size = DISK_SIZE - root_size
    space_to_delete = UNUSED_SIZE_LIMIT - unused_size
    dfs = Dfs(space_to_delete)
    dfs.search_folder_to_delete(root)
    print(dfs.best_to_delete)


class Dfs:
    """Depth First Search."""

    def __init__(self, space_to_delete: Optional[int] = None) -> None:
        self.space_to_delete = space_to_delete

        self.all_below_limit = 0
        self.best_to_delete = DISK_SIZE

    def search_folders_below_limit(self, folder_node: "Node") -> int:
        """Search all folders and find this below SIZE_LIMIT."""
        folder_size = 0

        for node in folder_node.children:
            if not node.is_file:
                folder_size += self.search_folders_below_limit(node)
            else:
                folder_size += node.size

        if folder_size < SIZE_LIMIT:
            self.all_below_limit += folder_size

        return folder_size

    def search_folder_to_delete(self, folder_node: "Node") -> int:
        """Search all folder, and find best folder to delete."""
        folder_size = 0

        for node in folder_node.children:
            if not node.is_file:
                folder_size += self.search_folder_to_delete(node)
            else:
                folder_size += node.size

        if folder_size >= self.space_to_delete and folder_size < self.best_to_delete:
            self.best_to_delete = folder_size

        return folder_size


if __name__ == "__main__":
    main()
