# XMrs File format library

A library to edit Tracker data with pleasure.

For now XM **FastTracker II** files are supported.

Useful struct parts:

- module
  - Pattern
    - PatternSlot
  - Instrument
    - Envelope
    - Vibrato
    - Sample

To load XM file:

1. Deserialize `XmModule` struct using `XmModule::load(&XM)`
2. Convert to struct `Module` using `.to_module()`

To save XM file:

1. Convert `Module` to `XmModule`: `XmModule::from_module(&module)`
2. Serialize using `XmModule` `save()` fn

Edit data using rustified structs, use `Module` struct.
