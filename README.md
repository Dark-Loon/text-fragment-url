# stf — Scroll to Text Fragments

Produce a URL that links directly to specific text in a web page. When opened, the browser highlights the text and scrolls it into view.

`stf` is a command-line tool for generating these links: give it a URL and a block of text, get back a shareable deep link that works in any modern browser.

<!-- TODO: embed demo GIF here showing all three modes -->

## Platform support

- Linux, macOS, Windows (native)
- Android, via [Termux](https://termux.dev)

## Install

```bash
cargo install --git https://github.com/you/text-fragment-url --locked -p stf_cli
```

> Requires the Rust toolchain. Prebuilt binaries and a crates.io release are planned — not yet available.

## Usage

`stf` supports three ways of providing input.

### 1. Direct mode

```bash
stf https://example.com "short simple text"
```

### 2. Clipboard / pipe mode

Pipe text in from anywhere — a clipboard tool, a file, another command. On Termux:

```bash
termux-clipboard-get | stf https://example.com
```

On Linux with Wayland:

```bash
wl-paste | stf https://example.com
```

On Linux with X11:

```bash
xclip -selection clipboard -o | stf https://example.com
```

### 3. Interactive mode

Run `stf` with no arguments for a guided prompt — it asks for the URL and text, shows you a live preview, and offers to walk you through disambiguation if needed.

```bash
stf
```

## Disambiguating repeated text

If your text appears more than once on the page, use `--prefix`/`-p` and `--suffix`/`-s` to anchor the match to surrounding text:

```bash
stf https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a \
  "The Referer" \
  --prefix "downgrade:" \
  --suffix "to origins"
```

## Non-English and right-to-left scripts

Text fragments work with any script — `stf` percent-encodes everything correctly, including right-to-left text:

```bash
stf https://example.com "مِصر" --prefix "البحرين"
# -> https://example.com/#:~:text=%D8%A7%D9%84%D8%A8%D8%AD%D8%B1%D9%8A%D9%86-,%D9%85%D9%90%D8%B5%D8%B1
```

## Flags

| Flag | Description |
|---|---|
| `-p`, `--prefix` | Text immediately before the match, to disambiguate repeated occurrences |
| `-s`, `--suffix` | Text immediately after the match, to disambiguate repeated occurrences |
| `-v`, `--verbose` | Print details about how the URL was constructed |
| `--completions <SHELL>` | Generate a shell completion script and print it to stdout |
| `-h`, `--help` | Print usage and an exhaustive list of flags |

## Autocompletions

```bash
# bash
stf --completions bash > ~/.local/share/bash-completion/completions/stf

# zsh (needs a directory on $fpath; ~/.zfunc is a common choice)
mkdir -p ~/.zfunc && stf --completions zsh > ~/.zfunc/_stf
# then add to ~/.zshrc, before compinit: fpath+=~/.zfunc

# fish (auto-loaded, no shell config needed)
stf --completions fish > ~/.config/fish/completions/stf.fish

# powershell
stf --completions power-shell

# nushell -- note: o>, not >, since nushell's redirection syntax differs from POSIX shells
mkdir ~/.cache/stf
stf --completions nushell o> ~/.cache/stf/completions.nu
# then in config.nu: source ~/.cache/stf/completions.nu
```

## Notes and limitations

- If the text fragment doesn't match anything in the linked document, or the browser doesn't support text fragments, the fragment is silently ignored and the page just loads at the top.
- If a fragment doesn't seem to highlight even though the syntax looks right, you may be matching a different occurrence than the one you expected — it might be highlighted, just offscreen.
- All major browsers added support by late 2024. Older devices (iOS 15 or earlier, very old Chrome/Firefox) open the link fine but silently ignore the fragment.
- Some sites opt out of this feature entirely via the `Document-Policy: force-load-at-top` header — GitHub is one example, and links to it won't scroll or highlight regardless of how correct the URL is.

## How it works

A text fragment directive looks like:

```
#:~:text=[prefix-,]textStart[,textEnd][,-suffix]
```

`stf` currently builds `textStart` plus optional `prefix`/`suffix`. Multi-segment ranges (`textStart,textEnd`) are already supported internally by the underlying library and are next up on the roadmap for the CLI — see below.

## Development plan

- expose `textEnd` range-matching as a CLI flag (the library already supports it; not yet wired up to `stf`'s arguments) — this is the natural fit for long, multi-sentence passages
- multiple highlights in one URL
- accept `text == "-"` as an unambiguous "force stdin" override (not needed for v1)
- notify when a text passage isn't found on the page — would require fetching the page over HTTP to verify; watch out for double-encoding when using HTTP clients (like `reqwest`) that have their own URL-encoding methods
- crates.io release, prebuilt binaries

## Acknowledgements

Text Fragments was implemented and specified by [Nick Burris](https://github.com/nickburris) and [David Bokan](https://github.com/bokand), with contributions from [Grant Wang](https://github.com/grantjwang).

Built with assistance from [Claude](https://claude.ai).
