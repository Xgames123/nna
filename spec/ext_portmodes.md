# Port modes
Provides a way to make the pins of a port act like all inputs.

## registers

| name                      | size | description        | location       | op access |
|---------------------------|------|--------------------|----------------|-----------|
| [pm](#port mode register) | 4    | port mode register | mapped at 0xE0 | no        |

## port mode register
Each bit of the pm register corresponds to the port with the same number. Ex. bit 0 of pm is p0

### Port mode 0 (default)
Port behaves like normal

### Port mode 1
When the pm of a port is set to 1 it causes all data pins (d0 -> d15) to become inputs.
