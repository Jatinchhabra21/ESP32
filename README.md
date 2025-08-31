# ESP32-S3 Embedded Development Projects  

This repository contains beginner-level projects developed on the **ESP32-S3-DevKitC-1-N8R8** board.  
The purpose of this repo is to document my learning process in **embedded systems development** by building small, self-contained applications.  

⚡ **Note:** All projects in this repository are implemented **exclusively using the Hardware Abstraction Layer (HAL)**.  
- No Arduino framework.  
- No ESP-IDF high-level drivers.  
- Only HAL APIs, to stay close to the hardware while keeping the code portable and structured.

## Projects  

### 1. **Blinky (NeoPixel Smooth Transition)**  
- Drives the onboard NeoPixel RGB LED.  
- Smoothly transitions the LED between random colors.  
- NeoPixel protocol is implemented using a **Tx channel (RMT peripheral)**:  
  - RGB values are manually converted into pulse sequences.  
  - Timing is generated according to WS2812 protocol requirements.  
- Demonstrates:  
  - Low-level control of addressable LEDs.  
  - Constructing waveform pulses from raw data.  
  - Implementing smooth color transitions.  

### 2. **Segment (7-Segment Display Counter)**  
- Displays a single digit (0–9) on a 7-segment display.  
- A push button is used to increment the number:  
  - Short press → increments the digit by 1.  
  - Long press → continuously increments until reaching `9`, then rolls over to `0`.  
- Demonstrates:  
  - GPIO input handling (button).  
  - Driving a 7-segment display using HAL.  
  - Basic counter logic.  

## Hardware  

- **Board:** ESP32-S3-DevKitC-1-N8R8  
- **Peripherals:**  
  - Onboard NeoPixel RGB LED (WS2812)  
  - External single-digit 7-segment display  
  - Tactile push button  

## Repository Structure  

/blinky -> NeoPixel smooth transition using Tx channel (manual pulse construction, HAL only)
/segment -> 7-segment display counter with button input (HAL only)

## Getting Started  

1. Clone this repository:  
   ```bash
   git clone https://github.com/Jatinchhabra21/ESP32.git
   cd ESP32

# Why HAL?

The Hardware Abstraction Layer (HAL) is used exclusively in this project to:

- Stay close to the hardware → direct control of peripherals without unnecessary abstractions.
- Improve portability → HAL APIs are consistent across chips and frameworks.
- Enhance learning → focusing on how peripherals work under the hood, instead of relying on prebuilt drivers.
- Avoid hidden complexity → HAL provides predictability, making it easier to debug and understand behavior.

# Goals

- Develop embedded applications directly on top of the ESP32-S3 HAL.
- Gain hands-on experience with GPIO, timing, and peripheral control.
- Build a foundation for more advanced, low-level embedded projects.
