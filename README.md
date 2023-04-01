# XMrs File format library

A library to edit Tracker data with pleasure.

Because "Representation is the Essence of Programming".

For now XM **FastTracker II** files are supported.

Useful struct parts:

- [Module](https://docs.rs/xmrs/latest/xmrs/module/struct.Module.html)
  - [Pattern](https://docs.rs/xmrs/latest/xmrs/module/type.Pattern.html)
    - [PatternSlot](https://docs.rs/xmrs/latest/xmrs/patternslot/struct.PatternSlot.html)
  - [Instrument](https://docs.rs/xmrs/latest/xmrs/instrument/struct.Instrument.html)
    - [InstrDefault](https://docs.rs/xmrs/latest/xmrs/instrument/struct.InstrDefault.html) for Historical XM Instrument
      - [Envelope](https://docs.rs/xmrs/latest/xmrs/envelope/struct.Envelope.html)
      - [Vibrato](https://docs.rs/xmrs/latest/xmrs/vibrato/struct.Vibrato.html)
      - [Sample](https://docs.rs/xmrs/latest/xmrs/sample/struct.Sample.html)
    - [InstrEkn](https://docs.rs/xmrs/latest/xmrs/instr_ekn/struct.InstrEkn.html) for Euclidian Rythm Instrument
    - [InstrMidi](https://docs.rs/xmrs/latest/xmrs/instr_midi/struct.InstrMidi.html) for Midi Instrument
    - [InstrSid](https://docs.rs/xmrs/latest/xmrs/instr_sid/struct.InstrSid.html) for MOS6581 SID Instrument
    - [InstrRobSid](https://docs.rs/xmrs/latest/xmrs/instr_robsid/struct.InstrRobSid.html) for historical Rob Hubbard Instrument
      - [RobEffects](https://docs.rs/xmrs/latest/xmrs/instr_robsid/struct.RobEffects.html)

## Load XM file

1. Deserialize `XmModule` struct using `XmModule::load(&XM)`
2. Convert to struct `Module` using `.to_module()`

## Save XM file

1. Convert `Module` to `XmModule`: `XmModule::from_module(&module)`
2. Serialize using `XmModule` `save()` fn

Edit data using rustified structs, use `Module` struct.

Note: You can only save `InstrDefault` using XM fileformat.

## Load XMrs fileformat (deflate then bincode)

let mut mod = Module::load(&data)?;

## Save XMrs fileformat (bincode then deflate)

let data = mod.save()?;

data contain a small five bytes header b"XMrs" + version coming from CARGO_PKG_VERSION_MAJOR.
