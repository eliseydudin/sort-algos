from subprocess import run, DEVNULL
from sys import argv
from time import time


def main():
    try:
        to_run = argv[1]
    except IndexError as e:
        print("No args given")
        return

    PATH = f"tests/{to_run}"
    with open(PATH, "r") as f:
        results = {}

        for algo in ["bubble", "choice", "insert", "merge"]:
            start = time()
            process = run(
                ["cargo", "r", "--bin", f"{algo}_sort"],
                stdin=f,
                stdout=DEVNULL,
                stderr=DEVNULL,
            )
            end = time()

            process.check_returncode()

            results[algo] = end - start

            f.seek(0)
    print("╭─────────────────┬────────────────╮")
    print("│ algorithm       │ time           │")
    print("├─────────────────┼────────────────┤")
    for k, v in results.items():
        if k == "merge":
            k += " "
        print("│ %s          │ %s       │" % (k, str(v)[:8]))
    print("╰─────────────────┴────────────────╯")


if __name__ == "__main__":
    main()
