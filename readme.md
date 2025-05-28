# commonnote
A simple library that defines a common data structure for musical note information exchange between hosts. Implementing this format allows seamless copy-paste between different music making applications.

### Basics
The `commonnote` data is stored on the system clipboard as plain JSON text. It should contain a `header` section that describes the data, as well as a `notes` array containing the actual data. The structure looks as follows:
```json5
{
  "header": {
    "resolution": 480,
    "language": "English",
    "origin": "mikoto-studio",
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

### Who uses it
The following hosts are known to use commonnote as their default note copying data structure:
- [Mikoto Studio](https://mikoto.studio/)