#!/usr/bin/env python3

from pathlib import Path
import subprocess


ROOT = Path(__file__).resolve().parent
ROM_DIR = ROOT / "test_rom"


def list_all_roms():
    return sorted(
        path
        for path in ROM_DIR.rglob("*.gb")
        if path.parent.name not in {"rom_singles", "individual"}
    )


def list_grouped_roms(group_name):
    suites = {}

    for path in sorted(ROM_DIR.rglob("*.gb")):
        if path.parent.name != group_name:
            continue

        suite_name = path.parent.parent.name
        suites.setdefault(suite_name, []).append(path)

    return suites


def run_rom(path):
    print(f"\nRunning {path.relative_to(ROOT)}\n")
    subprocess.run(["cargo", "run", "--", str(path)], cwd=ROOT, check=False)


def run_many(roms):
    for rom in roms:
        run_rom(rom)


def choose_suite(title, suites):
    suite_names = sorted(suites)
    if not suite_names:
        print("No suites found.")
        return None

    print(f"\n{title}:")
    for index, suite_name in enumerate(suite_names, start=1):
        print(f"{index}. {suite_name}")

    print("\nb. back")
    print("q. quit")

    choice = input("\nSelect suite: ").strip().lower()

    if choice == "q":
        raise SystemExit

    if choice == "b":
        return None

    if choice.isdigit():
        index = int(choice) - 1
        if 0 <= index < len(suite_names):
            return suite_names[index]

    print("Invalid selection.")
    return None


def choose_rom(title, roms):
    if not roms:
        print("No ROMs found.")
        return

    print(f"\n{title}:")
    for index, rom in enumerate(roms, start=1):
        print(f"{index}. {rom.relative_to(ROOT)}")

    print("\na. run all")
    print("b. back")
    print("q. quit")

    choice = input("\nSelect ROM: ").strip().lower()

    if choice == "q":
        raise SystemExit

    if choice == "b":
        return

    if choice == "a":
        run_many(roms)
        return

    if choice.isdigit():
        index = int(choice) - 1
        if 0 <= index < len(roms):
            run_rom(roms[index])
            return

    print("Invalid selection.")


def main():
    all_roms = list_all_roms()
    singles = list_grouped_roms("rom_singles")
    individuals = list_grouped_roms("individual")

    if not all_roms and not singles and not individuals:
        print("No test ROMs found.")
        return

    while True:
        print("\nTest ROM Menu:")
        print("1. all")
        print("2. singles")
        print("3. individual")
        print("q. quit")

        choice = input("\nSelect group: ").strip().lower()

        if choice == "q":
            return

        if choice == "1":
            choose_rom("All", all_roms)
            continue

        if choice == "2":
            suite_name = choose_suite("Singles Suites", singles)
            if suite_name:
                choose_rom(f"Singles: {suite_name}", singles[suite_name])
            continue

        if choice == "3":
            suite_name = choose_suite("Individual Suites", individuals)
            if suite_name:
                choose_rom(f"Individual: {suite_name}", individuals[suite_name])
            continue

        print("Invalid selection.")


if __name__ == "__main__":
    main()
