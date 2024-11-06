# Other devices to communicate with


## chardev_ascii
| port addr | description                                                     |
|-----------|-----------------------------------------------------------------|
| d0        | Char out low. The low nibble of the character to be outputted   |
| d1        | Char out high. The high nibble of the character to be outputted |
| d2        | When a character is available. The low nib is put here else 0   |
| d3        | When a character is available. The high nib is put here else 0  |

