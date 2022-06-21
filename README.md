<div align="center">

# Inrs
Simple CLI to (add, delete, update, create) i18n translation file

</div>

```
    Copyright (C) 2020-2022  TheAwiteb
    https://github.com/TheAwiteb/inrs

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
```

## Requirements
 * [Rust](https://www.rust-lang.org/)

## Features
- Write in Rust 🦀.
- Sort translations by key (in json file).
- Add the missing keys with an empty translation (in json file).
- Create new language with `create` command.
- Add/Update translation on specified language with `update` command.
- Delete translation by key in all languages with `delete` command.

## Disadvantages
- Only support json files

## Install
> Soon

## Using
```
USAGE:
    inrs --path <PATH> <SUBCOMMAND>

OPTIONS:
    -h, --help           Print help information
    -p, --path <PATH>    Path of i18n directory 📂
    -V, --version        Print version information

SUBCOMMANDS:
    create    Create new language file 🔤
    delete    Delete translations by key 🚧
    help      Print this message or the help of the given subcommand(s)
    update    Add/Update translation 🆕
```

## License
GNU General Public License version 3 of the license for more see <https://www.gnu.org/licenses/>
