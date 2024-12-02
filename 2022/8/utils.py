import os

import requests
from dotenv import load_dotenv

load_dotenv()


def get_input(day):
    """Get input from local file or download it from AoC website"""

    # if input file exists, return it
    if os.path.exists(f"input.txt"):
        print("Using local input file")
        with open(f"input.txt") as f:
            return f.read()

    # else download it
    else:
        print("Downloading input file")
        url = f"https://adventofcode.com/2022/day/{day}/input"
        cookies = {"session": os.getenv("COOKIE")}
        r = requests.get(url, cookies=cookies)
        with open(f"input.txt", "w") as f:
            f.write(r.text)
        return r.text
