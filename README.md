# mommy - affirmations in your terminal 💞

![screenshot](https://github.com/sleepymincy/sleepymincy/blob/main/.gitfiles/repos/images/mommy.png)

Clearly inspired by by [Gankra/cargo-mommy](<https://github.com/Gankra/cargo-mommy>) and original (in Bash) [sudofox/shell-mommy](<https://github.com/sudofox/shell-mommy>).

After using Bash  implementation for a bit, I've decided to try writing my own implementation in Rust for the sake of learning new things. ~~In the process I think I got too far lost in the cult of Rust.~~

## How to build:
- Get [Rust](https://rustup.rs/)
- `git clone https://github.com/sleepymincy/mommy`
- `cd mommy`
- `cargo build` or `cargo build -r` for release version (recommended)
- Compiled binary will be in `./target/release/`

## Easy install:
... TODO: fuck really hard with crates.io~ <3

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
- `SHELL_MOMMYS_NEEDY` - can be `1`, `0` or unset, decides if mommy is running at all times or only when u call her

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
export SHELL_MOMMYS_COLOR="blue" # Picking more than one will radomly rotate between colors.
export SHELL_MOMMYS_STYLE="bold,italic/bold" # Will be randomly rotated between bold italic and normal bold styles.
export SHELL_MOMMYS_COLOR_RGB="255,164,243/255,50,50" # Will be randomly rotated between lilac and red colors in this example. Note, that this setting will overwrite SHELL_MOMMYS_COLOR !!!
export SHELL_MOMMYS_ALIASES="$HOME/.config/aliases"
export SHELL_MOMMYS_NEEDY=1 # Will make mommy run ALWAYS.
```

You can also change `affirmations.json` file to un-degenerate this piece of software or make it worse by changing default affirmations. 
I'm not the one to judge.

## Known bugs / limitations:
- FIXED (**if** mommy is running at all times): ~~Does not work with `cd` ;~~
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
