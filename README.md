# skid

A command-line class assignment scheduler

## About

Using a paper planner is slow and memory unsafe. **skid** aims to provide a quick and useful environment for students to manage their assignments and classes.

## Installation

Install with cargo:

```sh
cargo install skid
```

## Usage

The welcome message and `help` command will guide you through the tool. Here's a quick rundown:

- Create classes with `create`
  - Change the metadata of a class later with `modify`
  - View class info with `info`
- Add assignments with `add`
  - Remove them if you need to with `remove`
  - Add them to the completed list with `complete`
- Delete a class with `delete`
  - Made a mistake? Prevent writing to config with `panic`
    - Show what would be written with `encode`
    - Write to the config anyways with `write`
- Track assignments before you complete them with [`klog`](https://klog.jotaen.net)

...And much more!

## License

MIT © 2021 Kyle P.