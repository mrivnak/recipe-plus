#!/usr/bin/env python3

import os
import signal
import subprocess
import time
import tempfile
import uuid

from pathlib import Path


def start_server() -> subprocess.Popen:
    """Start backend

    Returns:
        subprocess.Popen: _description_
    """
    db_url = Path(tempfile.gettempdir()).joinpath(str(uuid.uuid4()) + ".sqlite")
    print(f"Using database: {db_url}")
    env = os.environ.copy()
    env["DATABASE_URL"] = str(db_url)
    # Build backend
    subprocess.run(["cargo", "build", "--release"], check=True, env=env)
    # Run migrations
    subprocess.run(
        ["diesel", "migration", "run"],
        check=True,
        env=env,
    )

    # Start server
    proc = subprocess.Popen(
        ["target/release/recipe-plus-api"],
        env=env
    )
    time.sleep(1)  # wait for server to start
    return proc


def stop_server(proc: subprocess.Popen):
    """Stop the server process

    Args:
        proc (subprocess.Popen): Process to terminate
    """
    proc.send_signal(signal.SIGTERM)
    time.sleep(1)  # wait for server to stop
    if proc.poll() is None:
        print("terminating process failed, killing process")
        proc.kill()


def run_integration_tests():
    """Run integration tests"""
    cmd = ["pnpm", "test"]

    subprocess.run(cmd, cwd="./tests", check=False)


def main():
    """Main function"""
    proc = start_server()
    run_integration_tests()
    stop_server(proc)


if __name__ == "__main__":
    main()
