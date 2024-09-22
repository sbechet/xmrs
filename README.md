# XMrs File format library

A no_std library to edit Sound Tracker data with pleasure.

Because "Representation is the Essence of Programming".

For now MOD **Amiga Modules**, S3M **Scream Tracker III** and XM **FastTracker II** files are supported.

Useful struct parts:

- [Module](https://docs.rs/xmrs/latest/xmrs/module/struct.Module.html)
  - [Pattern](https://docs.rs/xmrs/latest/xmrs/module/type.Pattern.html)
    - [PatternSlot](https://docs.rs/xmrs/latest/xmrs/patternslot/struct.PatternSlot.html)
  - [Instrument](https://docs.rs/xmrs/latest/xmrs/instrument/struct.Instrument.html)
    - [InstrDefault](https://docs.rs/xmrs/latest/xmrs/instr_default/struct.InstrDefault.html) for Historical XM Instrument
      - [Envelope](https://docs.rs/xmrs/latest/xmrs/envelope/struct.Envelope.html)
      - [Vibrato](https://docs.rs/xmrs/latest/xmrs/instr_vibrato/struct.InstrVibrato.html)
      - [Sample](https://docs.rs/xmrs/latest/xmrs/sample/struct.Sample.html)
    - [InstrEkn](https://docs.rs/xmrs/latest/xmrs/instr_ekn/struct.InstrEkn.html) for Euclidian Rythm Instrument
    - [InstrMidi](https://docs.rs/xmrs/latest/xmrs/instr_midi/struct.InstrMidi.html) for Midi Instrument
    - [InstrOpl](https://docs.rs/xmrs/latest/xmrs/instr_opl/struct.InstrOpl.html) for Yamaha OPL Instrument
    - [InstrSid](https://docs.rs/xmrs/latest/xmrs/instr_sid/struct.InstrSid.html) for MOS6581 SID Instrument
    - [InstrRobSid](https://docs.rs/xmrs/latest/xmrs/instr_robsid/struct.InstrRobSid.html) for historical Rob Hubbard Instrument
      - [RobEffects](https://docs.rs/xmrs/latest/xmrs/instr_robsid/struct.RobEffects.html)

## Load MOD file

1. Deserialize `AmigaModule` struct using `AmigaModule::load(&amiga)`
2. Convert to struct `Module` using `.to_module()`

## Load XM file

1. Deserialize `XmModule` struct using `XmModule::load(&XM)`
2. Convert to struct `Module` using `.to_module()`

## Save XM file

1. Convert `Module` to `XmModule`: `XmModule::from_module(&module)`
2. Serialize using `XmModule` `save()` fn

Edit data using rustified structs, use `Module` struct.

Note: You can only save `InstrDefault` using XM fileformat.

## About no_std

micromath is used by default in no_std. If you prefer libm, use `cargo build --no-default-features --features=libm --release`.

## About std

if you want to use std feature use `cargo build --no-default-features --features=std --release`

## About std demo

if you want to test examples use `cargo build --no-default-features --features=std,demo --release`
