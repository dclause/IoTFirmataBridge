# Iot Firmata Bridge

**Lightweight Firmata server daemon to make your IoT devices (Raspberry Pi, Nvidia Jetson, etc.) Firmata-compatible over
TCP.**

---

## ❓ What is IotFirmataBridge?

IotFirmataBridge is a lightweight daemon designed to run on various IoT devices. It exposes their native hardware
interfaces (GPIO, I2C, SPI, PWM, etc.) via the Firmata protocol over TCP.  
This allows unified, remote control of devices like Raspberry Pi or Jetson using Firmata-compatible clients - for
example, using the popular [Johnny-Five](https://github.com/rwaldron/johnny-five) robotics framework, the
newer [Hermes-Five](https://github.com/dclause/hermes-five) framework
or [Hermes-Studio](https://github.com/dclause/hermes-studio) platform, or any other Firmata-compatible client.

---

## 💡 Why use IotFirmataBridge?

Firmata servers exist for microcontroller boards (Arduino, ESP32, NodeMCU, Spark.io, etc.), but not for more capable
boards like the Raspberry Pi or Nvidia Jetson.
IotFirmataBridge fills this gap by exposing these platforms to Firmata-compatible clients.

- **Unified control:** Control heterogeneous IoT boards using a consistent Firmata API
- **Remote operation:** Run your application on a central machine, while controlling distributed devices
- **Multi-platform:** Compatible with Raspberry Pi, Nvidia Jetson, and other (via feature flags)
- **Performance:** Built in Rust with async I/O via Tokio for high performance and reliability

---

## 🔄 How it works

1. **Daemon runs on the IoT device** exposing a TCP Firmata server
2. **Your client app connects via TCP** and sends Firmata commands (digitalWrite, analogRead, I2C, etc.)
3. **The daemon maps these commands to native hardware operations** using platform-specific libraries (`rppal`, etc.)
4. You get **real-time, remote hardware control** with a familiar and standard protocol

---

## 🚀 Getting started

### On your IoT device (requires Rust toolchain):

- Build and run the `IotFirmataBridge` daemon (Rust required):

```bash
git clone https://github.com/dclause/IotFirmataBridge.git
cd IotFirmataBridge
cargo build --release --features <raspberry|jetson>
sudo ./target/release/iotfirmatabridge
```

**Available features:**

- `raspberry`: enables support for Raspberry Pi
- `jetson`: enables support for Nvidia Jetson boards

> [!WARNING]
> Feature flags are mutually exclusive: choose one only.