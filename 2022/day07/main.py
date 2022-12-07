# Author:  mateusz.janda@gmail.com
# Ad maiorem Dei gloriam

import sys

def read_line():
    """Read line from stdin."""
    # line = input()
    # return line
    return sys.stdin.read()


def main():
    """Main body."""

    print("Hello world")
    while True:
        line = read_line()
        print(line)
        break


if __name__ == "__main__":
    main()
