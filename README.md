# flow

Command prefixing for continuous workflow using a single tool.

## Install

```bash
cargo install flow-cmd
```

## Usage

Start with a prefix:

```bash
flow git
```

Then run commands without repeating the prefix:

```text
$ git> status
$ git> log --oneline
```

Prefix management:

- `+add` adds to the prefix.
- `-` or `-N` drops elements from the end.
- `!commit` replaces the last element.
- `:ls -la` runs a shell command without the prefix.
- `:q` / `:exit` or Ctrl-D exits.

History is stored in `~/.flow_history`.

## Acknowledgements

This project was inspired by [with](https://github.com/mchav/with). Thanks to the author for the great idea!

## License

MIT
