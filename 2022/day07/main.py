# Author:  mateusz.janda@gmail.com
# Ad maiorem Dei gloriam

from typing import Optional
from typing import Tuple


def read_line() -> Optional[Tuple[str, str]]:
    """Read line from stdin."""
    try:
        line = input()
    except EOFError as _:
        return None

    words = line.strip().split()
    if line.startswith('$ cd'):
        return words[1], words[2]
    elif line.startswith('$ ls'):
        return words[1], None

    return words[0], words[1]


class Node:
    """Filesystem nodes."""

    def __init__(self, name: str, is_file: bool,
                parent_node: Optional["Node"]=None,
                size: Optional[int]=None) -> None:
        self.name = name
        self.is_file = is_file
        self.parent_node = parent_node
        self.size = size

        self.nodes = []

    def add_folder(self, name: str) -> None:
        """Add folder if doesn't exist yet."""
        for node in self.nodes:
            if node.name == name and not node.is_folder:
                return

        node = Node(name, is_file=False, parent_node=self)
        self.nodes.append(node)

    def add_file(self, name: str, size: int) -> None:
        """Add file to current directory."""
        for node in self.nodes:
            if node.name == name and node.is_file:
                return

        node = Node(name, is_file=True, parent_node=self, size=size)
        self.nodes.append(node)

    def go_to_folder(self, name: str) -> "Node":
        """Get folder node."""
        if name == "..":
            return self.parent_node

        for node in self.nodes:
            if node.name == name and not node.is_file:
                return node

        raise Exception("Folder not found")


def main() -> None:
    """Main body."""
    print("Hello world")

    root = Node("/", is_file=False)
    current_node = root

    # Build node tree
    while True:
        words = read_line()
        if words is None:
            break

        meta_data = words[0]
        name = words[1]

        if meta_data == "cd" and name == "/":
            pass
        elif meta_data == 'ls':
            pass
        elif meta_data == "cd":
            current_node = current_node.go_to_folder(name)
        elif meta_data == "dir":
            current_node.add_folder(name)
        elif meta_data.isdigit():
            current_node.add_file(name, int(meta_data))

    dfs = Dfs()
    dfs.search(root)
    print(dfs.result)


SIZE_LIMIT = 100000

class Dfs:
    """Depth First Search."""

    def __init__(self) -> None:
        self.result = 0

    def search(self, folder_node: "Node") -> int:
        """Search all folders."""
        folder_size = 0

        for node in folder_node.nodes:
            if not node.is_file:
                folder_size += self.search(node)
            else:
                folder_size += node.size

        if folder_size < SIZE_LIMIT:
            self.result += folder_size

        return folder_size



if __name__ == "__main__":
    main()
