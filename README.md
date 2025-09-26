# Basedpyright (ALT) Extension for Zed

## What Is This?
This is a `Zed` extension that fixes an issue I was personally having when working on a python project which had a workspace (using `uv` specifically). If you can find a better way than this, please let me know (seriously).

### Do I Need This?

If you've had issues with a python workspace using `basedpyright` when your package manager places the `.venv` at the root of your directory, you may have noticed occassionally opening a sub-package's files and getting a ton of import errors, implying that the language server doesn't know where the `.venv` is. If this is the case you *may* be interested. If not, feel free to go on and enjoy your life.

### The Problem

Let's say you're working on a directory that looks like this:

```sh
my_project /
    .venv
    packages /
        one /
            main.py
            pyproject.toml
        two /
            pyproject.toml
    pyproject.toml
    uv.lock
```

When using the [original](https://github.com/pub-struct/basedpyright-zed) extension, even with the [recommended settings](#Configure), if you opened `one/main.py`, `basedpyright` will attempt to look for a `.venv` in the `one` directory. It will not find one, so it will go off of the global interpreter which will have no idea what you're trying to import, what packages you're trying to import, etc.

I had this issue when working with a `uv` workspace, but this could potentially apply to other package managers as well, I'm just not as familiar with how they place their `.venv`.

#### But What About `pythonPath`

In the [recommended settings](#Configure) you'll notice that one of the options is to add this to your configuration:

```jsonc
{
  "lsp": {
    ...,
    "settings": {
      ...,
      "python": {
        "pythonPath": ".venv/bin/python"
      }
    }
  }
}
```

This definitely helps, but again, not with a workspace. What happens is if you're opening up, say again `one/main.py`, you can also open your `extension` logs simultaneously and see what's happening. The server is now thinking that instead of your root directory, the package (`one`) is your root and looks for a `.venv` within that package.

### The Solution

This package is a clone of the original, with one small tweak - if you specify `pythonPath`, it will now attempt to resolve this path to your **ROOT** directory regardless if you're in a sub-package or not. The `pythonPath` option will be replaced with an absolute path to the `.venv`, which will be passed on to the language server.

Again, this is a fix for `uv` which places the `.venv` in the root directory for all child packages. If your package manager does this, this extension may help.

### Is There an Easier Way?

Yes! You may not need this extension at all, here's a couple of possibilities:

#### 1. Use Local Project Settings

Just create a local `Zed` settings for your project and add the absolute path of the `.venv` to the `pythonPath`. Again, it has to be absolute or you're running into the same issue as if it were a global setting.

```jsonc
// my_project/.zed/settings.json
{
  "lsp": {
    ...,
    "settings": {
      ...,
      "python": {
        "pythonPath": "/absolute/path/to/.venv/bin/python"
      }
    }
  }
}
```

#### 2. Link Your `.venv`

Add symbolic links to the `.venv` in each package.

```sh
ln -s /absolute/path/to/.venv/bin/python packages/one/.venv
ln -s /absolute/path/to/.venv/bin/python packages/two/.venv
```

When `basedpyright` looks for the `.venv` it will always be there!

#### 3. Some Other Combination of `pyproject.toml`, `Zed`, or `uv` Settings That I Haven't Thought Of

I spent about 4-5 hours trying to figure out how to get this to work correctly. I couldn't find anything on issues, the web, or anywhere else on how to make `basedpyright` work consistently in a workspace. If you've figured it out, **PLEASE** for the love of everything message me and let me know how to do it so I can get rid of this extension.

<br/>

Below are is the remainder of the original `README` (with `alt` added). Enjoy!

---

## Pre-requisites

* Python 3.9+
* `basedpyright` installed into your python environment

## Installation

Search `basedpyright-alt` in the zed extensions. Click to install.

## Enable

Disable `pyright` and enable `basedpyright-alt` in your settings.

```jsonc
{
  "languages": {
    "Python": {
      "language_servers": ["basedpyright-alt", "!pyright"]
  },
}
```

## Configure

Configure under `lsp.basedpyright-alt.settings` as required.

The "binary" setting is optional, if not set, `basedpyright` will be searched for in your `PATH`.

```jsonc
{
  "lsp": {
    "basedpyright-alt": {
      "binary": {
        "path": ".venv/bin/basedpyright-langserver",
        "arguments": ["--stdio"]
      },
      "settings": {
        "python": {
          "pythonPath": ".venv/bin/python"
        },
        "basedpyright.analysis": {
          "diagnosticMode": "workspace",
          "inlayHints": {
            "callArgumentNames": false
          }
        }
      }
    }
  }
}
```
