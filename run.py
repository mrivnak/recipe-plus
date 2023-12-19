#!/usr/bin/env python3

import argparse
import os
import subprocess
import time

def run_backend(prod: bool = False) -> subprocess.Popen:
    if prod:
        return subprocess.Popen(["./recipe-plus"])
    else:
        return subprocess.Popen(["cargo", "run"], cwd="api")


def run_frontend(prod: bool = False) -> subprocess.Popen:
    if prod:
        return subprocess.Popen(["node", ".output/server/index.mjs"], env=os.environ, cwd="web")
    else:
        return subprocess.Popen(["pnpm", "dev"], cwd="web")


def is_exited(processes: list[subprocess.Popen]) -> bool:
    for process in processes:
        if process.poll() is not None:
            print("Process exited with code", process.returncode)
            return True
    return False


def main():
    # Development mode is default, use --prod to enable production mode
    # Prod expects a system-wide nginx installation, and for the server to be in the PATH

    parser = argparse.ArgumentParser(description='Your script description')

    parser.add_argument('--prod', action='store_true', help='Enable production mode')

    args = parser.parse_args()
    prod = bool(args.prod)
    procs = [
        run_backend(prod),
        run_frontend(prod)
    ]
    while True:
        time.sleep(0.5)

        try:
            if is_exited(procs):
                print("One of the processes exited, exiting")
                break
        except KeyboardInterrupt:
            for proc in procs:
                proc.terminate()
            break


if __name__ == '__main__':
    main()
