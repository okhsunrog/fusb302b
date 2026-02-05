# TODO

## High priority

### `wait_for_vbus()` is a no-op

**File:** `src/lib.rs:345`

Currently returns immediately. The FUSB302B has `VBUSOK` in STATUS0 (bit 7)
and `I_VBUSOK` in INTERRUPT (bit 7). The sink state machine calls
`wait_for_vbus()` during the `Discovery` state before attempting PD
negotiation. Without actually waiting, the driver may try to negotiate before
the source provides VBUS power.

Should poll `STATUS0.VBUSOK` until it reads true, with a timeout.

### SPECREV in switches_1 never updated after revision negotiation

**File:** `src/lib.rs:288-293, 308-313`

`detect_cc_pin()` configures `switches_1` with `auto_crc = true` but leaves
`SPECREV` at its reset default (Rev 2.0). Since `HAS_AUTO_GOOD_CRC = true`,
the FUSB302B generates GoodCRC responses autonomously using this register's
SPECREV/POWERROLE/DATAROLE fields.

When the protocol layer negotiates a different spec revision (e.g. Rev 3.0),
it updates its own internal header but has no way to update the FUSB302B's
switches_1 register. The hardware-generated GoodCRC will still carry Rev 2.0.
Some USB PD sources may reject GoodCRC with a mismatched revision.

Options:
- Add a method like `update_spec_revision()` that the protocol layer or
  policy engine can call after revision negotiation.
- Extend the `Driver` trait with a revision update callback.
- Set SPECREV to Rev 3.0 initially (most modern sources).

## Medium priority

### `transmit()` toggles oscillator on/off unnecessarily

**File:** `src/lib.rs:378-382, 474-478`

`init()` enables PWR[3] (internal oscillator) for all blocks. But
`transmit()` explicitly enables it before TX and disables it after. This
means after the first transmit, the oscillator is off until the next transmit
or until the receiver auto-enables it on CC activity.

Per the datasheet, the BMC receiver auto-enables the oscillator on activity
detection, but there's a window between the disable and the next incoming
preamble where the start of a response packet could be missed. This is
especially risky when the source responds quickly after our TX.

Consider removing the oscillator toggle from `transmit()` entirely, since
`init()` already enables it. If low-power operation is needed, it should be
managed at a higher level.

### Clear pending interrupts in `init()`

**File:** `src/lib.rs:238-245`

After SW_RES and PD_RESET, pending interrupts from the pre-reset state may
linger. The datasheet (page 7) recommends reading the interrupt registers to
clear them before starting operations. Should read and discard `interrupt()`,
`interrupta()`, and `interruptb()` after PD_RESET and before returning.

### `receive()` doesn't verify CRC validity

**File:** `src/lib.rs:482-511`

The receive loop breaks out when `!RX_EMPTY`, but doesn't check
`STATUS0.CRC_CHK` (bit 4, R-only) to confirm the packet has valid CRC. The
FUSB302B puts packets with bad CRC into the FIFO too (with `CRC_CHK = 0`).

With `AUTO_CRC` enabled, GoodCRC is only sent for valid packets, so the
source would retry on bad CRC. But the driver would still read and return
the corrupt packet to the protocol layer. Adding a `STATUS0.CRC_CHK` check
before reading FIFO data (and returning `Discarded` + flushing on failure)
would be more robust.

### `detect_cc_pin()` doesn't handle no-connection case

**File:** `src/lib.rs:281`

```rust
let selected_cc = if cc1 > cc2 { CcPin::Cc1 } else { CcPin::Cc2 };
```

If `cc1 == cc2 == 0` (nothing connected), it silently defaults to CC2 and
configures the switches as if a connection exists. Should check for this case
and return an error or a distinct result indicating no connection detected.

## Low priority

### Wrong error variant for unknown device version

**File:** `src/lib.rs:186`

```rust
return Err(FusbError::LenExceedsBuffer); // TODO: add proper error variant
```

Reuses `LenExceedsBuffer` for an unrecognized device version, which is
misleading. Add a dedicated `FusbError::UnknownDevice` or
`FusbError::UnsupportedVersion` variant.

### `Cargo.toml` has 6 keywords (max 5 for crates.io)

**File:** `Cargo.toml:10`

```toml
keywords = ["nostd", "usbpd", "driver", "embedded", "fusb302b", "async"]
```

crates.io allows at most 5 keywords. Publishing will fail. Drop one
(probably `"async"`).

### Redundant `modify_async` on R/C interrupt registers

**Files:** `src/lib.rs:364-367, 430-434, 440-444, 460-464`

After `read_async()` on an R/C register (INTERRUPTA, INTERRUPT), all bits
are already cleared. The subsequent `modify_async(|r| r.set_i_txsent(true))`
does another read (clearing again) then writes back, adding an unnecessary
I2C round-trip. These calls can be removed since the initial read is
sufficient to acknowledge the interrupt.

### Missing doc comments on public API

Public types (`Fusb302b`, `DeviceInterface`, `CcPin`, `FusbError`) and
methods (`init`, `get_device_info`) lack rustdoc comments. Important for
discoverability if the crate is published.
