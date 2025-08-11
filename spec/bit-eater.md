# bit-eater
bit-eater is a custom mpu designed by me based on the nna8 architecture


# Memory map / Peripherals

## bit-eater1 (nna8v1)

| addr   | size   | function  |
|--------|--------|-----------|
| ..0xEE | (0xEE) | EEPROM rw |
| ..0xEF | (0x02) | keyboard  |
| ..0xFF | (0x10) | video mem |

> ![NOTE]
> Ranges don't include the lower bound
>

## bit-eater2 (nna8v2)

| bank   | size   | function  |
|--------|--------|-----------|
| ..0x0F | (0x10) | EEPROM    |
| ..0xFD | (0xEE) | unused    |
| 0xFE   | (0x01) | video mem |
| 0xFF   | (0x01) | io bank   |

> ![NOTE]
> Ranges don't include the lower bound
>

### io bank

| addr | size   | function   |
|------|--------|------------|
| 0x00 | (0x01) | port flags |
| 0x01 | (0x01) | p0         |
| 0x02 | (0x01) | p1         |

#### port flags
| bit | function |
|-----|----------|
| 1   | p0 ready |
| 2   | p1 ready |
| 3   | unused   |
| 4   | unused   |
| 5   | unused   |
| 6   | unused   |
| 7   | unused   |
| 8   | unused   |

> ![NOTE]
> when you read a 1 from the ready flag: the port has fully received the data.
> writing a 0 to the ready flag starts receiving the next byte from the port
>

# Video
| mpu                 | res   | mode |
|---------------------|-------|------|
| bit-eater1 (nna8v1) | 16x8  | bw   |
| bit-eater2 (nna8v2) | 64x32 | bw   |
