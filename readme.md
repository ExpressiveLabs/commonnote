# commonnote
A simple library that defines a common data structure for musical note information exchange between hosts. Implementing this format allows seamless copy-paste between different music making applications.

## Basics
The `commonnote` data is stored on the system clipboard as plain JSON text. It should contain a `header` section that describes the data, as well as a `notes` array containing the actual data. The structure looks as follows:
```json5
{
  "identifier": "commonnote", // This must be present for initial validation
  "header": {
    "resolution": 480, // Required, number of units per quarter note
    "language": "English", // Optional
    "origin": "mikoto-studio", // Optional
    "extra": {} // Optional extra host-specific data
  },
  "notes": [
    {
      "start": 0,
      "length": 480,
      "label": "la",
      "pitch": 60,
      "extra": {
        // Host-specific data goes here (e.g. Mikoto Studio stores phoneme data in the "extra" field)
      }
    }
  ],
  "extra": {} // Optional extra host-specific data
}
```

## Fields
### Identifier field
The `identifier` field sits at the root of the specification and provides a quick and inexpensive way to detect whether the clipboard contents are actually compatible with the `commonnote` spec. Its value must always equal `commonnote`. If this field is not present, no data will be parsed.

### Header data
The header field contains a number of descriptors to help hosts correctly parse the data, no matter its origin.
- `resolution` (required) is the number of units that make up a quarter note. This can be used to translate timing values to a scale that the host understands (or to setup dynamic resolution when the host supports this).
- `language` (optional): a string that represents the language specified for the data. Parsing this string is left up to the implementing party.
- `origin` (optional) allows hosts to identify where data came from, and assign special functionality to their own clipboard data.
- `extra` (optional) provides space for hosts to define their own extra header fields, mostly for internal purposes.

### Note data
- `start` (required) in units per quarter note defines the start of the note. It is up to the receiver to decide how to handle this data. A data provider should always provide it as an absolute value measured from the sequence start.
- `length` (required) in units per quarter note defines the length of the note.
- `pitch` (required) is the MIDI note number of the note, defined as an 8-bit integer.
- `label` (required) is a string representing the note's content. Given that this specification is targeted at singing synthesizers, this field should usually contain the lyrics associated with the note.
- `extra` (optional) provides space for hosts to define their own extra note data fields. This can be used to encode fine-tuning parameters or phonetic information.

### Extra data
At the end of the specification, there is room for extra global data. This can be used to transfer other information in addition to notes, for example fine-tuning automation, vocalist descriptors, and other useful data. This field may be used by hosts to store their own internal data structures as well. Receiving hosts should always validate data in this field before parsing incompatible data meant for another host.

## Rust crate
This library provides a basic Rust interface to handle and validate `commonnote` data. It's available as a crate from [crates.io](https://crates.io/crates/commonnote):
```zsh
cargo add commonnote
```

### Features
This crate exposes the `util` feature, which enables utility functions that abstract away some common operations.

## Who uses it
The following hosts are known to use commonnote as their default note copying data structure:
- [Mikoto Studio](https://mikoto.studio/)
- [Maghni AI](https://maghni.ai) (work in progress)