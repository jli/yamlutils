# yaml helper things

yaml is confusing and ambiguous sometimes. `yaml2json` converts it to json.

```sh
$ yaml2json.pex example.yml
{
  "build": {
    "ci": [
      "pytest\n  --long_option1\n  --long_option2\n"
    ],
    "on_success": [
      "# Comments can mess things up. pip install poetry-dynamic-versioning && poetry build\n"
    ]
  }
}
```

pipe to `jq .` for nice colors.


## Install

```sh
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
./bin/make_pex.sh
mv yaml2json.pex ~/bin/
```

pex packages up the python requirements with the code so you can execute
`yaml2json.pex` without installing the dependencies globally.
