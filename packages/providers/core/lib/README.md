# nv-provider-core

## Messaging

Messaging w/ nv provider core's streaming socket listener is defined below.
The listener maintains a full duplex connection with the client,
requests and responses may be sent/received in any order.

### Config Collection Protocol

The Config Collection Protocol defines the structure of messages and behaviour of system components.

Tagged message w/ byte order:

```
Message: [HEADER][PAYLOAD]
  HEADER: [VERSION][OPCODE][CHECKSUM][PAYLOAD_LENGTH]
    VERSION: 1 byte
    OPCODE: 1 byte
    CHECKSUM: 1 byte
    PAYLOAD_LENGTH: 4 bytes

```
