from shutil import which
from subprocess import run
import sys
import os

if not os.path.exists("pkg"):
    run(
        "python scripts/build_wasm.py",
        shell=True,
    )

if which("miniserve") is None:
    run(
        "cargo install miniserve",
        shell=True,
    )

try:
    run(
        "miniserve -p 3000 --spa --index web/index.html",
        shell=True,
    )
except KeyboardInterrupt:
    sys.exit(0)
