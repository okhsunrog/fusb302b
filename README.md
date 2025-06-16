# FUSB302B Driver for USB Power Delivery

This crate provides an `async` driver for the ONSEMI [FUSB302B](https://www.onsemi.com/products/interfaces/usb-type-c-and-power-delivery-controllers/fusb302b), a programmable USB Type-C Controller with Power Delivery (PD).

The driver is designed to work as a PHY (physical layer) driver for a higher-level USB-PD Policy Engine. It achieves this by implementing the `Driver` trait from the **[usbpd-traits](https://crates.io/crates/usbpd-traits)** crate, making it directly compatible with consumers of that trait, such as the [elagil/usbpd](https://github.com/elagil/usbpd) policy engine.

This project also serves as a real-world example of using the powerful [diondokter/device-driver](https://github.com/diondokter/device-driver) toolkit to create robust, type-safe peripheral drivers from a simple YAML definition.

## Features

*   **`async`/.await**: Fully asynchronous, non-blocking operation suitable for use with executors like `embassy`.
*   **PHY Driver**: Implements the `usbpd_traits::Driver` trait, providing a standard interface for sending and receiving USB-PD packets.
*   **Register-Safe**: All register access is defined in a clear `device.yaml` file and generated at compile time by the `device-driver` crate, preventing illegal register access and providing a type-safe API.
*   **Robust**: Correctly handles low-level hardware interactions, including FIFO management, interrupt polling, and token-based packet transmission as specified by the FUSB302B datasheet.
*   **Minimal and Focused**: The driver focuses solely on its role as a PHY, leaving all protocol logic, timing, and policy decisions to a higher-level library.

## Architecture

This driver demonstrates a clean, layered approach to embedded systems design, glued together by a shared trait:

1.  **Hardware Abstraction (`device-driver`)**: At the lowest level, the `device-driver` crate takes a `device.yaml` file describing the FUSB302B's registers and generates a safe, low-level API (`FusbLowLevel`). This completely abstracts away the I2C read/write operations and magic numbers associated with register bits.

2.  **PHY Driver (`Fusb302b`)**: This is the core of this crate. The `Fusb302b` struct wraps the generated low-level API and implements the `usbpd_traits::Driver` trait. Its sole responsibility is to translate the policy engine's high-level commands (e.g., "transmit this packet") into the specific sequence of operations required by the FUSB302B hardware.

3.  **Policy Engine (`usbpd`)**: A separate, higher-level crate that contains the entire USB-PD protocol state machine. It is generic over any type that implements `usbpd_traits::Driver`, allowing it to consume this driver to interact with the physical world.

## Usage

This driver is not intended to be used standalone. It should be initialized and then passed to a consumer that requires a `usbpd_traits::Driver`, such as the `usbpd` Sink Policy Engine.

Here is a brief example of how to set up the stack in an `embassy` application:

```rust
use embassy_executor::Spawner;
use your_hal::i2c::I2c; // Your specific HAL for I2C

// This crate
use fusb302b::Fusb302b; 

// The USB-PD policy engine crate and its traits
use usbpd::{
    sink::{device_policy_manager::DevicePolicyManager, policy_engine::Sink},
    timers::Timer as PdTimer,
};
use usbpd_traits::Driver; // The trait our driver implements

// 1. Implement the required timer and DPM traits for the policy engine...
struct AppTimer;
impl PdTimer for AppTimer { /* ... */ }

struct MyDevicePolicyManager;
impl DevicePolicyManager for MyDevicePolicyManager { /* ... */ }


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize your hardware peripherals
    let p = Peripherals::take();
    let i2c = I2c::new(p.I2C1, ...);

    // 1. Instantiate and initialize your FUSB302B driver
    let fusb_driver = match Fusb302b::new_and_init(i2c).await {
        Ok(driver) => driver,
        Err(e) => {
            error!("Failed to initialize FUSB302B: {:?}", e);
            return;
        }
    };

    // 2. Instantiate your application's policy manager
    let dpm = MyDevicePolicyManager;

    // 3. Instantiate the policy engine with your driver, timer, and DPM
    let mut policy_engine = Sink::<_, AppTimer, _>::new(fusb_driver, dpm);

    info!("Starting USB-PD Policy Engine...");

    // 4. Run the policy engine forever
    if let Err(e) = policy_engine.run().await {
        error!("Policy engine exited with error: {:?}", e);
    }
}
```

## Acknowledgements

This crate stands on the shoulders of giants and wouldn't be possible without the excellent work of the following projects:

*   **[elagil/usbpd](https://github.com/elagil/usbpd)**: Provides the hardware-agnostic USB-PD policy engine that this driver is designed to serve. The `usbpd-traits` sub-crate provides the essential `Driver` trait that enables this modular design.
*   **[diondokter/device-driver](https://github.com/diondokter/device-driver)**: The code generation toolkit that provides the foundation for safe and structured register access, making this driver significantly more robust and easier to develop.
*   **[fmckeogh/usb-pd-rs](https://github.com/fmckeogh/usb-pd-rs)**: The archived predecessor to the modern `usbpd` ecosystem. This `fusb302b` driver is a ground-up rewrite heavily inspired by the original driver logic found in that repository.