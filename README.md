# My Boss

## About

This is software that tells me what to do and when to do it. It's My Boss. Some might call it an ERP system, but there's nothing "enterprise" about it.

My Boss is a command-line application that stores its data in local files. It's designed to be used together with Git or another version control system, to assure proper management of its data.

## Status

Contact management (primitive CRM functionality) has been implemented. As of this writing, no other features are available.

## Usage

Run the following from the repository root, to install My Boss:
```
cargo install --path .
```

After that run `mb --help` (also works for subcommand, like `mb contacts --help`) to figure out how to use My Boss.

## License

This project is open source software, licensed under the terms of the [Zero Clause BSD License] (0BSD, for short). This basically means you can do anything with the software, without any restrictions, but you can't hold the authors liable for problems.

See [LICENSE.md] for full details.

[Zero Clause BSD License]: https://opensource.org/licenses/0BSD
[LICENSE.md]: https://github.com/hannobraun/my-boss/blob/main/LICENSE.md
