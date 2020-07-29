# DSMsg

##### Random generator for online messages from Dark Souls.

Generates messages in the format of one of any of the three in the series, chosen randomly. Messages from Dark Souls II and III may have a second part, in which case the two parts will be joined by a conjunction.

## Installation

With [Cargo](https://github.com/rust-lang/cargo) installed, the following command will build and install DSMsg from [crates.io](https://crates.io) automatically:

```
cargo install dsmsg
```

The default installation outputs compound messages on a single line:

> recklessness ahead so to speak visions of misfortune...

Alternatively, to break them apart, you may enable the `newline` Feature:

```
cargo install dsmsg --features newline
```

> recklessness ahead  
> so to speak visions of misfortune...

---

*In `(Lordran|Drangleic|Lothric)`, the flow of time is distorted, with heroes centuries old phasing in and out. The very fabric wavers, and messages can be sent between worlds.*

*This tool allows one to receive helpful guidance from other Undead. Whether these other Undead will tell the truth, on the other hand, cannot be known.*
