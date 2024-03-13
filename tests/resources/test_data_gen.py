import math
import random
import pathlib

BASE_DIR = pathlib.Path(__file__).parent
TARGET_FILE = BASE_DIR / "tests" / "resources" / "large.txt"
CHANGED_FILE = BASE_DIR / "tests" / "resources" / "large_ch.txt"

with open(TARGET_FILE, "r", encoding="utf-8") as f:
    lines = f.read().split("\n")

random_count = math.floor(len(lines) * 0.4)
lines_for_change = [
    line
    for line in random.sample(lines, random_count)
    if line.strip() != ""
]

print(f"Found {len(lines)} lines. Start change {len(lines_for_change)} lines")

with open(CHANGED_FILE, "w", encoding="utf-8") as f:
    for line in lines:
        if line in lines_for_change:
            f.write("[diff]")
        f.write(line)
        f.write("\n")
