# mommy - affirmations in your terminal 💞

Clearly inspired by by [cargo-mommy](<https://github.com/Gankra/cargo-mommy>) and original (in Bash) [shell-mommy](<https://github.com/sudofox/shell-mommy>).

After using Bash  implementation for a bit, I've decided to try writing my own implementation in Rust for the sake of learning new things. ~~In the process I think I got too far lost in the cult of Rust.~~

## How to build:
- Get [Rust](https://rustup.rs/)
- `git clone https://github.com/sleepymincy/mommy`
- `cd mommy`
- `cargo build` or `cargo build -r` for release version

## Easy install:
... TODO: fuck really hard with crates.io~ <3

## Configuration:
Available environment variables: 
- `SHELL_MOMMYS_EMOTES` - to set the emotes to anything u want
- `SHELL_MOMMYS_LITTLE` - to set the petnames mommy is using towards u
- `SHELL_MOMMYS_ROLES` - to change mommy to daddy or whatever else
- `SHELL_MOMMYS_PRONOUNS` - to change mommy's pronouns

You can all add this to your `.bashrc` (or any other rc file) to customize it system wide, for example:
```
export SHELL_MOMMYS_PRONOUNS="his"
export SHELL_MOMMYS_ROLES="daddy"
export SHELL_MOMMYS_LITTLE="discord kitten/kitty"
export SHELL_MOMMYS_EMOTES="🤤/💕/🥺/💋"
```

You can also change `affirmations.json` file to un-degenerate this piece of software or make it worse by changing default affirmations. 
I'm not the one to judge.

## Known bugs / limitations:
- Crashes whenever command was piped ;
- Is not compatible with Bash aliases (open for suggestions how to fix this) ;
- Probably way more. I'm stupid :3

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
