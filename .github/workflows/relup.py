from pathlib import Path
from os import environ
from subprocess import run

r = environ["GITHUB_REPOSITORY"]
#
g = Path("releases")
if g.exists():
    with g.open() as o:
        for x in o:
            x = x.strip()
        if x:
            g = Path(x)
            c = ["gh", "-R", f"{r}", "release", "upload", "alpha", f"{g}"]
            print(c)
            run(c)
#
d = Path("release.d")
if d.is_dir():
    for g in d.iterdir():
        if g.is_file():
            c = ["gh", "-R", f"{r}", "release", "upload", "alpha", f"{g}"]
            print(c)
            run(c)
