# current_artist

#### 🟡 Incomplete
This API may have [additions](/api-stability/marker.md) in the future.

---

Access the [`Artist`](/common-objects/artist.md) of the currently set [`Song`](/common-objects/song.md).

#### Inputs

`None`

#### Outputs

| Field  | Type            | Description |
|--------|-----------------|-------------|
| artist | `Artist` object | See [`Artist`](/common-objects/artist.md)

#### Example Request
```bash
festival-cli current_artist
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"current_artist"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "artist": {
      "name": "Rex Orange County",
      "key": 65,
      "runtime": 7583,
      "albums": [
        237
      ],
      "songs": [
        2800,
        2803,
        2809
      ]
    }
  },
  "id": 0
}
```
