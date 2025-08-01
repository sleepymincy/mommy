# mommy - affirmations in your terminal 💞

![screenshot](https://github.com/sleepymincy/sleepymincy/blob/main/.gitfiles/repos/images/mommy.png)

Clearly inspired by by [Gankra/cargo-mommy](<https://github.com/Gankra/cargo-mommy>) and original (in Bash) [sudofox/shell-mommy](<https://github.com/sudofox/shell-mommy>).

After using Bash  implementation for a bit, I've decided to try writing my own implementation in Rust for the sake of learning new things. ~~In the process I think I got too far lost in the cult of Rust.~~

## Quick Links:
- [How to build](#how-to-build)
- [Easy install](#easy-install)
- [Configuration](#configuration)
- [Known bugs / limitations](#known-bugs--limitations)
- [License information](#license-information)

## How to build:
- Get [Rust](https://rustup.rs/)
- `git clone https://github.com/sleepymincy/mommy`
- `cd mommy`
- `cargo build` or `cargo build -r` for release version (recommended)
- Compiled binary will be in `./target/release/`

## Easy install:
- Get [Rust](https://rustup.rs/)
- `cargo install shell-mommy`

## Configuration:
Available environment variables: 
- `SHELL_MOMMYS_EMOTES` - to set the emotes to anything u want
- `SHELL_MOMMYS_LITTLE` - to set the petnames mommy is using towards u
- `SHELL_MOMMYS_ROLES` - to change mommy to daddy or whatever else
- `SHELL_MOMMYS_PRONOUNS` - to change mommy's pronouns
- `SHELL_MOMMYS_COLOR` - to change text color
- `SHELL_MOMMYS_STYLE` - to change text style
- `SHELL_MOMMYS_COLOR_RGB` - to set custom rgb color for the text
- `SHELL_MOMMYS_ALIASES` - provide path to your aliases file for mommy to source
- `SHELL_MOMMYS_AFFIRMATIONS` - provide a path to a valid `.json` file, formatted exactly like [assets/affirmations.json](https://github.com/sleepymincy/mommy/blob/master/assets/affirmations.json), otherwise the code will fall back to built-in default affirmations
- `SHELL_MOMMYS_NEEDY` - can be `1`, or `0` (default), decides if mommy is running at all times or only when u call her
- `SHELL_MOMMY_ONLY_NEGATIVE` - can be `1` or `0` (default), decides if mommy only talks when exit code is not 0

You can either specify environment variables every time you run mommy:
```ansi
you@archbtw:~$ SHELL_MOMMYS_COLOR="blue" SHELL_MOMMYS_STYLE="bold" mommy ls -l
drwxr-xr-x - you 20 April 04:20 📁 dir1
drwxr-xr-x - you 20 April 04:20 📁 dir2
drwxr-xr-x - you 20 April 04:20 📁 dir3
you're doing so well~! 💓 <- will be blue and bold
```

Or all add this to your `.bashrc` (or any other rc file) to customize it user wide, for example:
```sh
export SHELL_MOMMYS_PRONOUNS="his"
export SHELL_MOMMYS_ROLES="daddy"
export SHELL_MOMMYS_LITTLE="discord kitten/kitty"
export SHELL_MOMMYS_EMOTES="🤤/💕/🥺/💋"
export SHELL_MOMMYS_COLOR="blue/red" # Will be randomly rotated between blue and red colors.
export SHELL_MOMMYS_STYLE="bold,italic/bold" # Will be randomly rotated between bold italic style and just bold style.
export SHELL_MOMMYS_COLOR_RGB="255,164,243/255,50,50" # Will be randomly rotated between lilac and red colors in this example. Note, that this setting will overwrite SHELL_MOMMYS_COLOR !!!
export SHELL_MOMMYS_ALIASES="$HOME/.config/aliases"
export SHELL_MOMMYS_AFFIRMATIONS="$HOME/.config/affirmations.json"
export SHELL_MOMMYS_NEEDY=1 # Will make mommy run ALWAYS.
export SHELL_MOMMY_ONLY_NEGATIVE=1 # Will make mommy only print affirmations if exit code is not 0
```

When you set `SHELL_MOMMYS_NEEDY` variable to `1`, mommy will accept exit codes instead of commands as an argument. Examples: 
- `sjdfhsdjkfhsdf; mommy $?` <- returns exit code `127`, which will result in negative response from mommy
- `ls; mommy $?` <- returns exit code `0`, which will make mommy give a positive response

To make this behavior consistent, you can add these to your relevant rc files:

```bash
# ~/.bashrc
export PROMPT_COMMAND="mommy \$?; $PROMPT_COMMAND"
```

```bash
# ~/.zshrc
precmd() { mommy $? }
```

```bash
# Others (not tested)
export PS1="\$(mommy \$?)$PS1"
```

You can also change `affirmations.json` before building, or load your own with `SHELL_MOMMYS_AFFIRMATIONS` during runtime, to un-degenerate this piece of software or make it worse. I'm not the one to judge.

## Known bugs / limitations:
- No known ones, but I'm sure there are. Open up an [issue](https://github.com/sleepymincy/mommy/issues/new) if you find one.

## License information:
This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <https://unlicense.org>
