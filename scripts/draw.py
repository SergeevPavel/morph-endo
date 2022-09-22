#!/usr/bin/env python3

import subprocess, os, time
import concurrent.futures

def tasks_to_preform():
    (_, pages, _) = next(os.walk("data/repair_guide"))
    pages = set(pages)
    (_, presented, _) = next(os.walk("cache/repair_guide"))
    presented = set(presented)
    return ["repair_guide/" + task for task in  pages.difference(presented)]

def perform(task):
    subprocess.check_call(["target/release/main", "interpreter", task])
    subprocess.check_call(["target/release/main", "drawer", task])

def main():
    os.chdir(os.path.join(os.path.dirname(__file__), os.pardir))
    to_preform = tasks_to_preform()
    if not to_preform:
        return
    subprocess.check_call(["cargo", "build", "--release"])
    with concurrent.futures.ThreadPoolExecutor(max_workers=8) as e:
        for task in to_preform:
            e.submit(perform, task)



if __name__ == "__main__":
    main()


