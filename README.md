# DSMsg

[
![Crates.io](https://img.shields.io/crates/v/dsmsg?logo=rust&style=for-the-badge&label=crate)
![Downloads](https://img.shields.io/crates/d/dsmsg?style=flat-square)
](https://crates.io/crates/dsmsg)  
[
![GitHub](https://img.shields.io/github/repo-size/yaulendil/dsmsg?logo=github&style=for-the-badge&label=repo)
](https://github.com/yaulendil/dsmsg)

[
![docs.rs](https://docs.rs/dsmsg/badge.svg?style=for-the-badge)
](https://docs.rs/dsmsg)

##### Random generator for online messages from Dark Souls, Demon's Souls, Bloodborne, and Sekiro.

Generates messages in the format of one of any game in the series, chosen randomly. Messages from the more recent titles may have a second part, in which case the two parts will be joined by a conjunction.

## Installation

With [Cargo](https://github.com/rust-lang/cargo) installed, the following command will build and install DSMsg from [crates.io](https://crates.io) automatically:

```
cargo install dsmsg
```

The default installation outputs compound messages on a single line:

> recklessness ahead so to speak visions of misfortune...

Alternatively, to break them apart, you may enable the `newline` Feature:

```
cargo install dsmsg --features "newline"
```

> recklessness ahead  
> so to speak visions of misfortune...

## Optional Message Sets

Messages from Bloodborne, Demon's Souls, and Sekiro are also available via compile options. To enable output from all available message groups:

```
cargo install dsmsg --features "all-sets"
```

Additionally, the messages from the Dark Souls series are implemented as default Features, and can be disabled. Thus, to install using messages (with line breaks) from **ONLY** Dark Souls III and Bloodborne, for example, use the following command:

```
cargo install dsmsg --no-default-features --features "bloodborne ds3 newline"
```

This will first **disable** all three Dark Souls message groups (including `ds3`), and then **enable** the groups `bloodborne` and `ds3`. The full set of message groups is as follows:
- `bloodborne`
- `demons`
- `ds1`
- `ds2`
- `ds3`
- `sekiro`


---

*In `(Lordran|Drangleic|Lothric)`, the flow of time is distorted, with heroes centuries old phasing in and out. The very fabric wavers, and messages can be sent between worlds.*

*This tool allows one to receive helpful guidance from other Undead. Whether these other Undead will tell the truth, on the other hand, cannot be known.*
