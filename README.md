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

## Load XM file

1. Deserialize `XmModule` struct using `XmModule::load(&XM)`
2. Convert to struct `Module` using `.to_module()`

## Save XM file

1. Convert `Module` to `XmModule`: `XmModule::from_module(&module)`
2. Serialize using `XmModule` `save()` fn

Edit data using rustified structs, use `Module` struct.

## Load XMrs fileformat (deflate then bincode)

let mut mod = Module::load(&data)?;

## Save XMrs fileformat (bincode then deflate)

let data = mod.save()?;

data contain a small five bytes header b"XMrs" + version coming from CARGO_PKG_VERSION_MAJOR.
