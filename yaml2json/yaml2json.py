import json
import sys

import yaml

def load_yaml(path: str):
    with open(path) as f:
        return yaml.safe_load(f.read())

def load_and_print(path: str):
    obj = load_yaml(path)
    print(json.dumps(obj, indent=2))

def main():
    assert (len(sys.argv) == 2)
    load_and_print(sys.argv[-1])

if __name__ == '__main__':
    main()
