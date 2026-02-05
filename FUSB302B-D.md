# Page 1

onsemi

DATA SHEET

www.onsemi.com

# FUSB302B Programmable USB Type-C Controller w/PD

## FUSB302B

### Description

The FUSB302B targets system designers looking to implement a DRP/SRC/SNK USB Type-C connector with low amount of programmability.

The FUSB302B enables the USB Type-C detection including attach, and orientation. The FUSB302B integrates the physical layer of the USB BMC power delivery protocol to allow up to 100 W of power and role swap. The BMC PD block enables full support for alternative interfaces of the Type-C specification.

### Features

- Dual-role Functionality with Autonomous DRP Toggle
- Ability to Connect as Either a Host or a Device Based on What Has Been Attached
- Software Configurable Either as a Dedicated Host, Dedicated Device, or Dual Role
- Dedicated Devices can Operate both on a Type-C Receptacle or a Type-C Plug with a Fixed CC and VCONN Channel
- Full Type-C 1.2 Support. Integrates the Following Functionality of the CC Pin:
- Attach/Detach Detection as Host
- Current Capability Indication as Host
- Current Capability Detection as Device
- Audio Adapter Accessory Mode
- Debug Accessory Mode
- Active Cable Detection
- Integrates CCx to VCONN Switch with Over-current Limiting for Powering USB3.1 Full Featured Cables
- USB Power Delivery (PD) 2.0, Version 1.2 Support:
- Automatic GoodCRC Packet Response
- Automatic Retries of Sending a Packet if a GoodCRC is Not Received
- Automatic Soft Reset Packet Sent with Retries if Needed
- Automatic Hard Reset Ordered Set Sent
- Dead Battery Support (SNK Mode Support when No Power Applied)
- Low Power Operation: I_CC = 25 μA (Typical)
- AEC-Q100 Automotive Qualified Temperature Grade 2: (-40°C to +105°C)
- Packaged in:
- 9-ball WLCSP (1.215 mm × 1.260 mm)
- 14-lead MLP (2.5 mm × 2.5 mm, 0.5 mm Pitch)

### Applications

- Smartphones
- Tablets
- Laptops
- Notebooks
- Power Adapters
- Cameras

WLCSP9
CASE 567TN

WQFN14
CASE 510BR

## ORDERING INFORMATION

See detailed ordering and shipping information on page 2 of this data sheet.

© Semiconductor Components Industries, LLC, 2015

August, 2021 – Rev. 5

Publication Order Number:

FUSB302B/D

# Page 2

FUSB302B

- Dongles
- Automotive

![img-0.jpeg](img-0.jpeg)
Figure 1. Block Diagram

Table 1. ORDERING INFORMATION

|  Part Number | Top Mark | Operating Temperature Range | Package | Shipping†  |
| --- | --- | --- | --- | --- |
|  FUSB302BUCX | H4 | -40 to 85°C | 9-ball Wafer-level Chip Scale Package (WLCSP), 0.4 mm Pitch | 3,000 / Tape and Reel  |
|  FUSB302BMPX | UA | -40 to 85°C | 14-lead MLP 2.5 mm × 2.5 mm, 0.5 mm Pitch  |   |
|  FUSB302B01MPX | UP  |   |   |   |
|  FUSB302B10MPX | US  |   |   |   |
|  FUSB302B11MPX | UT  |   |   |   |
|  FUSB302BVMPX | DA | -40 to 105°C  |   |   |

†For information on tape and reel specifications, including part orientation and tape sizes, please refer to our Tape and Reel Packaging Specifications Brochure, BRD8011/D.

www.onsemi.com

ICI

Share Feedback

Your Opinion Matters

# Page 3

# FUSB302B

# TYPICAL APPLICATION

![img-1.jpeg](img-1.jpeg)
Figure 2. Typical Application

# BLOCK DIAGRAM

![img-2.jpeg](img-2.jpeg)
Figure 3. Functional Block Diagram

www.onsemi.com

ICL

Share Feedback

Your Opinion Matters

# Page 4

# FUSB302B

# PIN CONFIGURATION

![img-3.jpeg](img-3.jpeg)
TOP Through View

![img-4.jpeg](img-4.jpeg)
Bottom view
Figure 4. FUSB302BUCX Pin Assignment

Table 2. PIN MAP

|   | Column 1 | Column 2 | Column 3  |
| --- | --- | --- | --- |
|  Row A | CC2 | VBUS | VDD  |
|  Row B | VCONN | INT_N | SCL  |
|  Row C | CC1 | GND | SDA  |

www.onsemi.com

4

#

Share Feedback

Your Opinion Matters

# Page 5

FUSB302B

![img-5.jpeg](img-5.jpeg)
Figure 5. FUSB302BMPX Pin Assignment (N/C = No Connect)

![img-6.jpeg](img-6.jpeg)

Table 3. PIN DESCRIPTION

|  Name | Type | Description  |
| --- | --- | --- |
|  USB TYPE-C CONNECTOR INTERFACE  |   |   |
|  CC1/CC2 | I/O | Type-C connector Configuration Channel (CC) pins. Initially used to determine when an attach has occurred and what the orientation of the insertion is. Functionality after attach depends on mode of operation detected. Operating as a host: 1. Sets the allowable charging current for VBUS to be sensed by the attached device 2. Used to communicate with devices using USB BMC Power Delivery 3. Used to detect when a detach has occurred Operating as a device: 1. Indicates what the allowable sink current is from the attached host. Used to communicate with devices using USB BMC Power Delivery  |
|  GND | Ground | Ground  |
|  VBUS | Input | VBUS input pin for attach and detach detection when operating as an upstream facing port (Device). Expected to be an OVP protected input.  |

POWER INTERFACE

|  VDD | Power | Input supply voltage.  |
| --- | --- | --- |
|  VCONN | Power Switch | Regulated input to be switched to correct CC pin as VCONN to power USB3.1 full-featured cables and other accessories.  |

SIGNAL INTERFACE

|  SCL | Input | I²C serial clock signal to be connected to the phone-based I²C master.  |
| --- | --- | --- |
|  SDA | Open-Drain I/O | I²C serial data signal to be connected to the phone-based I²C master  |
|  INT_N | Open-Drain Output | Active LOW open drain interrupt output used to prompt the processor to read the I²C register bits  |

www.onsemi.com

#

6

Share Feedback

Your Opinion Matters

# Page 6

FUSB302B

# CONFIGURATION CHANNEL SWITCH

The FUSB302B integrates the control and detection functionality required to implement a USB Type-C host, device or dual-role port including:

- Device Port Pull-Down (RD)
- Host Port Pull-Up (IP)
- VCONN Power Switch with OCP for Full-Featured USB3.1 Cables

- USB BMC Power Delivery Physical Layer
- Configuration Channel (CC) Threshold Comparators

Each CC pin contains a flexible switch matrix that allows the host software to control what type of Type-C port is implemented. The switches are shown in Figure 6.

![img-7.jpeg](img-7.jpeg)
Figure 6. Configuration Channel Switch Functionality

# TYPE-C DETECTION

The FUSB302B implements multiple comparators and a programmable DAC that can be used by software to determine the state of the CC and VBUS pins. This status information provides the processor all of the information required to determine attach, detach and charging current configuration of the Type-C port connection.

The FUSB302B has three fixed threshold comparators that match the USB Type-C specification for the three charging current levels that can be detected by a Type-C device. These comparators automatically cause BC_LVL and COMP interrupts to occur when there is a change of state. In addition to the fixed threshold comparators, the host software can use the 6-bit DAC to determine the state of the CC lines more accurately.

The FUSB302B also has a fixed comparator that monitors if VBUS has reached a valid threshold or not. The DAC can be used to measure VBUS up to  $20\mathrm{V}$  which allows the software to confirm that changes to the VBUS line have occurred as expected based on PD or other communication methods to change the charging level.

# Detection through Autonomous Device Toggle

The FUSB302B has the capability to do autonomous DRP toggle. In autonomous toggle the FUSB302B internally controls the PDWN1, PDWN2, PU_EN1 and PU_EN2, MEAS_CC1 and MEAS_CC2 and implements a fixed DRP toggle between presenting as a SRC and presenting as a SNK. Alternately, it can present as a SRC or SNK only and poll CC1 and CC2 continuously.

www.onsemi.com

ICI

Share Feedback

Your Opinion Matters

# Page 7

FUSB302B

Table 4. PROCESSOR CONFIGURES THE FUSB302B THROUGH I²C

|  I²C Registers/Bits | Value  |
| --- | --- |
|  TOGGLE | 1  |
|  PWR | 07H  |
|  HOST_CUR0 | 1  |
|  HOST_CUR1 | 0  |
|  MEAS_VBUS | 0  |
|  VCONN_CC1 | 0  |
|  VCONN_CC2 | 0  |
|  Mask Register | 0xFE  |
|  Maska Register | 0xBF  |
|  Maskb Register (Except I_TOGDONE and I_BC_LVL Interrupt) | 0x01  |
|  PWR[3:0] | 0xBF  |

1. Once it has been determined what the role is of the FUSB302B, it returns I_TOGDONE and TOGSS1/2.
2. Processor then can perform a final manual check through I²C.

## Manual Device Toggle

The FUSB302B has the capability to do manual DRP toggle. In manual toggle the FUSB302B is configurable by the processor software by I²C and setting TOGGLE = 0.

## Manual Device Detection and Configuration

A Type-C device must monitor VBUS to determine if it is attached or detached. The FUSB302B provides this information through the VBUSOK interrupt. After the Type-C device knows that a Type-C host has been attached, it needs to determine what type of termination is applied to each CC pin. The software determines if an Ra or Rd termination is present based on the BC_LVL and COMP interrupt and status bits.

Additionally, for Rd terminations, the software can further determine what charging current is allowed by the Type-C host by reading the BC_LVL status bits. This is summarized in Table 5.

## Toggle Functionality

When TOGGLE bit (Control2 register) is set the FUSB302B implements a fixed DRP toggle between presenting as a SRC and as a SNK. It can also be configured to present as a SRC only or SNK only and poll CC1 and CC2 continuously. This operation is turned on with TOGGLE = 1 and the processor should initially write HOST_CUR1 = 0, HOST_CUR0 = 1 (for default current), VCONN_CC1 = VCONN_CC2 = 0, Mask Register = 0xFE, Maska register = 0xBF, and Maskb register = 0x01, and PWR = 0x01. The processor should also read the interrupt register to clear them prior to setting the TOGGLE bit.

Table 5. DEVICE INTERRUPT SUMMARY

|  Status Type | Interrupt Status |   |   |   | Meaning  |
| --- | --- | --- | --- | --- | --- |
|   |  BC_LVL[1:0] | COMP | COMP Setting | VBUSOK  |   |
|  CC Detection | 2'b00 | NA | NA | 1 | vRA  |
|   |  2'b01 | NA | NA | 1 | vRd-Connect and vRd-USB  |
|   |  2'b10 | NA | NA | 1 | vRd-Connect and vRd-1.5  |
|   |  2'b11 | 0 | 6'b11_0100 (2.05 V) | 1 | vRd-Connect and vRd-3.0  |
|  Attach | NA | NA | NA | 1 | Host Attached, VBUS Valid  |
|  Detach | NA | NA | NA | 0 | Host Detached, VBUS Invalid  |

The high level software flow diagram for a Type-C device (SNK) is shown in Figure 7.

www.onsemi.com

ICO

Share Feedback

Your Opinion Matters

# Page 8

FUSB302B

![img-8.jpeg](img-8.jpeg)
Figure 7. SNK Software Flow

# Manual Host Detection and Configuration

When the FUSB302B is configured as a Type-C host, the software can use the status of the comparators and DAC to determine when a Type-C device has been attached or detached and what termination type has been attached to each CC pin.

The FUSB302B allows the host software to change the charging current capabilities of the port through the

HOST_CUR control bits. If the HOST_CUR bits are changed prior to attach, the FUSB302B automatically indicates the programmed current capability when a device is attached. If the current capabilities are changed after a device is attached, the FUSB302B immediately changes the CC line to the programmed capability.

![img-9.jpeg](img-9.jpeg)
Figure 8. HOST_CUR Changed after Attach

www.onsemi.com

ICO

Share Feedback

Your Opinion Matters

# Page 9

FUSB302B

![img-10.jpeg](img-10.jpeg)
Figure 9. HOST_CUR Changed prior to Attach

The Type-C specification outlines different attach and detach thresholds for a Type-C host that are based on how much current is supplied to each CC pin. Based on the programmed HOST_CUR setting, the software adjusts the

DAC comparator threshold to match the Type-C specification requirements. The BC_LVL comparators can also be used as part of the Ra detection flow. This is summarized in Table 6.

Table 6. HOST INTERRUPT SUMMARY

|  Termination | HOST_CUR[1:0] | Interrupt Status |   |   | Attach/Detach  |
| --- | --- | --- | --- | --- | --- |
|   |   |  BC_LVL[1:0] | COMP | COMP Setting  |   |
|  Ra | 2'b01 | 2'b00 | NA | NA | NA  |
|   |  2'b10 | 2'b01 | 0 | 6'b00_1010 (0.42 V)  |   |
|   |  2'b11 | 2'b10 | 0 | 6'b01_0011 (0.8 V)  |   |
|  Rd | 2'b01, 2'b10 | NA | 0 | 6'b10_0110 (1.6 V) | Attach  |
|   |   |  NA | 1 | 6'b10_0110 (1.6 V) | Detach  |
|   |  2'b11 | NA | 0 | 6'b11_1110 (2.6 V) | Attach  |
|   |   |  NA | 1 | 6'b11_1110 (2.6 V) | Detach  |

The high level software flow diagram for a Type-C Host (SRC) is shown below in Figure 10.

www.onsemi.com

9

#

Share Feedback

Your Opinion Matters

# Page 10

FUSB302B

Figure 10. SRC Software Flow
![img-11.jpeg](img-11.jpeg)
FUSB302B | COMP and | VBUSOK interrupts alert host software that a accessory detach has occurred

# Manual Dual-Role Detection and Configuration

The Type-C specification allows ports to be both a device and a host depending on what type of port has attached. This functionality is similar to USB OTG ports with the current USB connectors and is called a dual-role port. The

FUSB302B can be used to implement a dual-role port. A Type-C dual role port toggles between presenting as a Type-C device and a Type-C host. The host software controls the toggle time and configuration of the FUSB302B in each state as shown in Figure 11.

![img-12.jpeg](img-12.jpeg)
Figure 11. DRP Software Flow

www.onsemi.com

10

Share Feedback

Your Opinion Matters

# Page 11

FUSB302B

# BMC POWER DELIVERY

The Type-C connector allows USB Power Delivery (PD) to be communicated over the connected CC pin between two ports. The communication method is the BMC Power Delivery protocol and is used for many different reasons with the Type-C connector. Possible uses are outlined below.

- Negotiating and controlling charging power levels
- Alternative Interfaces such as MHL, Display Port
- Vendor specific interfaces for use with custom docks or accessories
- Role swap for dual-role ports that want to switch who is the host or device
- Communication with USB3.1 full featured cables

The FUSB302B integrates a thin BMC PD client which includes the BMC physical layer and packet FIFOs (48 bytes for transmit and 80 bytes for receive) which allows packets to be sent and received by the host software through I²C accesses. The FUSB302B allows host software to implement all features of USB BMC PD through writes and

reads of the FIFO and control of the FUSB302B physical interface.

The FUSB302B uses tokens to control the transmission of BMC PD packets. These tokens are written to the transmit FIFO and control how the packet is transmitted on the CC pin. The tokens are designed to be flexible and support all aspects of the USB PD specification. The FUSB302B additionally enables control of the BMC transmitter through tokens. The transmitter can be enabled or disabled by specific token writes which allow faster packet processing by burst writing the FIFO with all the information required to transmit a packet.

The FUSB302B receiver stores the received data and the received CRC in the receive FIFO when a valid packet is received on the CC pin. The BMC receiver automatically enables the internal oscillator when an Activity is sensed on the CC pin and load to the FIFO when a packet is received. The I_ACTIVITY and I_CRC_CHK interrupts alert the host software that a valid packet was received.

![img-13.jpeg](img-13.jpeg)
Figure 12. USB BMC Power Delivery Blocks

# Power Level Determination

The Type-C specification outlines the order of precedence for power level determination which covers power levels from basic USB2.0 levels to the highest levels of USB PD. The host software is expected to follow the USB Type-C specification for charging current priority based on feedback from the FUSB302B detection, external BC1.2 detection and any USB Power Delivery communication.

The FUSB302B does not integrate BC1.2 charger detection which is assumed available in the USB transceiver or USB charger in the system.

# Power Up, Initialization and Reset

When power is first applied through VDD, the FUSB302B is reset and registers are initialized to the default values shown in the register map.

The FUSB302B can be reset through software by programming the SW_RES bit in the RESET register.

If no power applied to VDD then the SRC can recognize the FUSB302B as a SNK.

# PD Automatic Receive GoodCRC

The power delivery packets require a GoodCRC acknowledge packet to be sent for each received packet where the calculated CRC is the correct value. This calculation is done by the FUSB302B and triggers the I_CRC_CHK interrupt if the CRC is good. If the AUTO_CRC (Switches1 register bit) is set and AUTO_PRE = 0, then the FUSB302B will automatically send the GoodCRC control packet in response to alleviate the local processor from responding quickly to the received packet. If GoodCRC is required for anything beyond SOP, then enable SOP*.

www.onsemi.com

ICO

Share Feedback

Your Opinion Matters

# Page 12

FUSB302B

# PD Send

The FUSB302B implements part of the PD protocol layer for sending packets in an autonomous fashion.

![img-14.jpeg](img-14.jpeg)
Figure 13.

# PD Automatic Sending Retries

If GoodCRC packet is not received and AUTO_RETRY is set, then a retry of the same message that was in the TxFIFO written by the processor is executed within  $t_{\text{Retry}}$  and that is repeated for NRETRY times.

# PD Send Soft Reset

If the correct GoodCRC packet is still not received for all retries then I_RETRYFAIL interrupt is triggered and if AUTO_SOFT_RESET is set, then a Soft Reset packet is created (MessageID is set to 0 and the processor upon servicing I_RETRYFAIL would set the true MessageIDCounter to 0.

If this Soft Reset is sent successfully where a GoodCRC control packet is received with a MessageID = 0 then I_TXSENT interrupt occurs.

If not, this Soft Reset packet is retried NRETRIES times (MessageID is always 0 for all retries) if a GoodCRC acknowledge packet is not received with CRCREceiveTimer expiring ( $t_{\text{Receive}}$  of  $1.1 \, \text{ms} \, \text{max}$ ). If all retries fail, then I_SOFTFAIL interrupt is triggered.

# PD Send Hard Reset

If all retries of the soft reset packet fail and if AUTO_HARD_RESET is set, then a hard reset ordered set is sent by loading up the TxFIFO with RESET1, RESET1, RESET1, RESET2 and sending a hard reset. Note only one

hard reset is sent since the typical retry mechanism doesn't apply. The processor's policy engine firmware is responsible for retrying the hard reset if it doesn't receive the required response.

# Flush Rx-FIFO with BIST (Built-In Self Test) Test Data

During PD compliance testing, BIST test packets are used to test physical layer of the PD interface such as, frequency derivation, Amplitude measure and etc. The one BIST test data packet has 7 data objects (28byte data), header and CRC, but the message ID doesn't change, the packet should be ignored and not acted on by the PD policy engine. The PD protocol layer does need to send a GoodCRC message back after every packet. The BIST data can arrive continuously from a tester, which could cause the FUSB302B Rx FIFO to overflow and the PD protocol layer to stop sending GoodCRC messages unless the FIFO is read or cleared quickly. The FUSB302B has a special register bit in the I²C registers, bit[5] of address 0x09, that when the bit is set, all the data received next will be flushed from the RxFIFO automatically and the PD protocol layer will keep sending GoodCRC messages back. Once BIST test is done, tester sends HardReset, so with the HardReset, processor has to write the bit back to disable. Also, if the bit can be de-selected anytime, then the coming packet has to be managed by protocol layer and policy engine.

# I²C INTERFACE

The FUSB302B includes a full I²C slave controller. The I²C slave fully complies with the I²C specification version 6 requieremnts. This block is designed for Fast Mode Plus traffic up to 1 MHz SCL operation.

The TOGGLE features allow for very low power operation with slow clocking thus may not be fully

compliant to the 1 MHz operation. Examples of an  $\mathrm{I}^2\mathrm{C}$  write and read sequence are shown in Figure 14 and Figure 15 respectively.

![img-15.jpeg](img-15.jpeg)
Note: Single Byte read is initiated by Master with P immediately following first data byte
Figure 14. I²C Write Example

www.onsemi.com

ICO

Share Feedback

Your Opinion Matters

# Page 13

# FUSB302B

![img-16.jpeg](img-16.jpeg)
Figure 15. I²C Read Example

Table 7. ABSOLUTE MAXIMUM RATINGS

|  Symbol | Parameter | Min | Max | Unit  |
| --- | --- | --- | --- | --- |
|  VVDD | Supply Voltage from VDD | -0.5 | 6.0 | V  |
|  VCC_HDDRP | CC pins when configured as Host, Device or Dual Role Port | -0.5 | 6.0 | V  |
|  VVBUS | VBUS Supply Voltage | -0.5 | 28.0 | V  |
|  TSTORAGE | Storage Temperature Range | -65 | +150 | °C  |
|  TJ | Maximum Junction Temperature | - | +150 | °C  |
|  TL | Lead Temperature (Soldering, 10 Seconds) | - | +260 | °C  |
|  ESD | Human Body Model, ANSI/ESDA/JEDEC JS-001-2012 | 4 | - | kV  |
|   |  Charged Device Model, JEDEC JESD22-C101 | 1 | -  |   |

Stresses exceeding those listed in the Maximum Ratings table may damage the device. If any of these limits are exceeded, device functionality should not be assumed, damage may occur and reliability may be affected.

Table 8. RECOMMENDED OPERATING CONDITIONS

|  Symbol | Parameter | Min | Typ | Max | Unit  |
| --- | --- | --- | --- | --- | --- |
|  VVBUS | VBUS Supply Voltage | 4.0 | 5.0 | 21.0 | V  |
|  VVDD | VDD Supply Voltage | 2.7 (Note 3) | 3.3 | 5.5 | V  |
|  VVCONN | VCONN Supply Voltage | 2.7 | - | 5.5 | V  |
|  IVCONN | VCONN Supply Current | - | - | 560 | mA  |
|  TA | Operating Temperature | -40 | - | +85 | °C  |
|  TA | Operating Temperature (Note 11) | -40 | - | +105 | °C  |

Functional operation above the stresses listed in the Recommended Operating Ranges is not implied. Extended exposure to stresses beyond the Recommended Operating Ranges limits may affect device reliability.

3. This is for functional operation only and not the lowest limit for all subsequent electrical specifications below. All electrical parameters have a minimum of  $3.0\mathrm{V}$  operation.

www.onsemi.com

ICL

Share Feedback

Your Opinion Matters

# Page 14

# FUSB302B

# DC AND TRANSIENT CHARACTERISTICS

All typical values are at  $\mathrm{T_A} = 25^{\circ}\mathrm{C}$  unless otherwise specified.

Table 9. BASEBAND PD

|  Symbol | Parameter | TA = -40 to +85°C TA = -40 to +105°C (Note 11) TJ = -40 to +125°C |   |   | Unit  |
| --- | --- | --- | --- | --- | --- |
|   |   |  Min | Typ | Max  |   |
|  UI | Unit Interval | 3.03 | - | 3.70 | μs  |

TRANSMITTER

|  zDriver | Transmitter Output Impedance | 33 | - | 75 | Ω  |
| --- | --- | --- | --- | --- | --- |
|  tEndDriveBMC | Time to Cease Driving the Line after the end of the last bit of the Frame | - | - | 23 | μs  |
|  tHoldLowBMC | Time to Cease Driving the Line after the final High-to-Low Transition | 1 | - | - | μs  |
|  VOH | Logic High Voltage | 1.05 | - | 1.20 | V  |
|  VOL | Logic Low Voltage | 0 | - | 75 | mV  |
|  tStartDrive | Time before the start of the first bit of the preamble when the transmitter shall start driving the line | -1 | - | 1 | μs  |
|  tRISE_TX | Rise Time | 300 | - | - | ns  |
|  tFALL_TX | Fall Time | 300 | - | - | ns  |

RECEIVER

|  cReceiver | Receiver Capacitance when Driver isn’t Turned On | - | 50 | - | pF  |
| --- | --- | --- | --- | --- | --- |
|  zBmcRx | Receiver Input Impedance | 1 | - | - | MΩ  |
|  tRxFilter | Rx Bandwidth Limiting Filter (Note 4) | 100 | - | - | ns  |

4. Guaranteed by Characterization and/or Design. Not production tested.

![img-17.jpeg](img-17.jpeg)
Figure 16. Transmitter Test Load

www.onsemi.com

ICL

Share Feedback

Your Opinion Matters

# Page 15

FUSB302B

Table 10. TYPE-C CC SWITCH

|  Symbol | Parameter | TA = -40 to +85°C
TA = -40 to +105°C (Note 11)
TJ = -40 to +125°C |   |   | Unit  |
| --- | --- | --- | --- | --- | --- |
|   |   |  Min | Typ | Max  |   |
|  RSW_CCX | RDSON for SW1_CC1 and SW1_CC2, VCONN to CC1 & CC2 | - | 0.4 | 1.2 | Ω  |
|  ISW_CCX | Over-Current Protection (OCP) limit at which VCONN switch shuts off over the entire VCONN voltage range (OCPreg = 0Fh) | 600 | 800 | 1000 | mA  |
|  tSoftStart | Time taken for the VCONN switch to turn on during which Over-Current Protection is disabled | - | 1.5 | - | ms  |
|  I80_CCX | SRC 80 μA CC current (Default) HOST_CUR1 = 0, HOST_CUR0 = 1 | 64 | 80 | 96 | μA  |
|  I180_CCX | SRC 180 μA CC Current (1.5 A) HOST_CUR1 = 1, HOST_CUR0 = 0 | 166 | 180 | 194 | μA  |
|  I330_CCX | SRC 330 μA CC Current (3 A) HOST_CUR1 = 1, HOST_CUR0 = 1 | 304 | 330 | 356 | μA  |
|  VUFPDB | SNK Pull-down Voltage in Dead Battery under all Pull-up SRC Loads | - | - | 2.18 | V  |
|  RDEVICE | Device Pull-down Resistance (Note 5) | 4.6 | 5.1 | 5.6 | kΩ  |
|  zOPEN | CC Resistance for Disabled State | 126 | - | - | kΩ  |
|  WAKElow | Wake threshold for CC pin SRC or SNK LOW value. Assumes bandgap and wake circuit turned on ie PWR[0] = 1 | - | 0.25 | - | V  |
|  WAKEhigh | Wake threshold for CC pin SRC or SNK HIGH value. Assumes bandgap and wake circuit turned on ie PWR[0] = 1 | - | 1.45 | - | V  |
|  vBC_LVLhys | Hysteresis on the Ra and Rd Comparators (Note 7) | - | 20 | - | mV  |
|  vBC_LVL | CC Pin Thresholds, Assumes PWR = 4'h7
BC = 2'b00
BC = 2'b01
BC = 2'b10 | 0.15
0.61
1.16 | 0.20
0.66
1.23 | 0.25
0.70
1.31 | V  |
|  vMDACstepCC | Measure block MDAC step size for each code in MDAC[5:0] register | - | 42 | - | mV  |
|  vMDACstepVBUS | Measure block MDAC step size for each code in MDAC[5:0] register for VBUS measurement | - | 420 | - | mV  |
|  vVBUSthr | VBUS threshold at which I_VBUSOK interrupt is triggered. Assumes measure block on ie PWR[2] = 1 | - | - | 4.0 | V  |
|  tTOG1 | When TOGGLE = 1, time at which internal versions of PU_EN1 = PU_EN2 = 0 and PWDN1 = PDWN2 = 1 selected to present externally as a SNK in the DRP toggle | 30 | 45 | 60 | ms  |
|  tTOG2 | When TOGGLE = 1, time at which internal versions of PU_EN1 = 1 or PU_EN2 = 1 and PWDN1 = PDWN2 = 0 selected to present externally as a SRC in the DRP toggle | 20 | 30 | 40 | ms  |
|  tDIS | Disable time after a full toggle (tTOG1 + tTOG2) cycle so as to save power
TOG_SAVE_PWR2:1 = 00
TOG_SAVE_PWR2:1 = 01
TOG_SAVE_PWR2:1 = 10
TOG_SAVE_PWR2:1 = 11 | -
-
-
- | 0
40
80
160 | -
-
-
- | ms  |
|  Tshut | Temp. for Vconn Switch Off | - | 145 | - | °C  |
|  Thys | Temp. Hysteresis for Vconn Switch Turn On | - | 10 | - | °C  |

5. RDEVICE minimum and maximum specifications are only guaranteed when power is applied.

www.onsemi.com

ICO

Share Feedback

Your Opinion Matters

# Page 16

FUSB302B

Table 11. CURRENT CONSUMPTION

|  Symbol | Parameter | V_{DD} (V) | Conditions | T_{A} = -40 to +85°C
T_{A} = -40 to +105°C (Note 11)
T_{J} = -40 to +125°C |   |   | Unit  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|   |   |   |   |  Min | Typ | Max  |   |
|  Idisable | Disabled Current | 3.0 to 5.5 | Nothing Attached, No I^{2}C Transactions | – | 0.37 | 5.0 | μA  |
|  Idisable | Disabled Current (Note 11) | 3.0 to 5.5 | Nothing Attached, No I^{2}C Transactions | – | 0.37 | 8.5 | μA  |
|  Itog | Unattached (standby) Toggle Current | 3.0 to 5.5 | Nothing attached, TOGGLE = 1, PWR[3:0] = 1h, WAKE_EN = 0, TOG_SAVE_PWR2:1 = 01 | – | 25 | 40 | μA  |
|  Ipd_stby_meas | BMC PD Standby Current | 3.0 to 5.5 | Device Attached, BMC PD Active But Not Sending or Receiving Anything, PWR[3:0] = 7h | – | 40 | – | μA  |

Table 12. USB PD SPECIFIC PARAMETERS

|  Symbol | Parameter | T_{A} = -40 to +85°C
T_{A} = -40 to +105°C (Note 11)
T_{J} = -40 to +125°C |   |   | Unit  |
| --- | --- | --- | --- | --- | --- |
|   |   |  Min | Typ | Max  |   |
|  tHardReset | If a Soft Reset message fails, a Hard Reset is sent after tHardReset of CRCReceiveTimer expiring | – | – | 5 | ms  |
|  tHardReset Complete | If the FUSB302B cannot send a Hard Reset within tHardResetComplete time because of a busy line, then a I_HARDFAIL interrupt is triggered | – | – | 5 | ms  |
|  tReceive | This is the value for which the CRCReceiveTimer expires. The CRCReceiveTimer is started upon the last bit of the EOP of the transmitted packet | 0.9 | – | 1.1 | ms  |
|  tRetry | Once the CRCReceiveTimer expires, a retry packet has to be sent out within tRetry time. This time is hard to separate externally from tReceive since they both happen sequentially with no visible difference in the CC output | – | – | 75 | μs  |
|  tSoftReset | If a GoodCRC packet is not received within tReceive for NRETRIES then a Soft Reset packet is sent within tSoftReset time. | – | – | 5 | ms  |
|  tTransmit | From receiving a packet, we have to send a GoodCRC in response within tTransmit time. It is measured from the last bit of the EOP of the received packet to the first bit sent of the preamble of the GoodCRC packet | – | – | 195 | μs  |

Table 13. IO SPECIFICATIONS

|  Symbol | Parameter | V_{DD} (V) | Conditions | T_{A} = -40 to +85°C
T_{A} = -40 to +105°C (Note 11)
T_{J} = -40 to +125°C |   |   | Unit  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|   |   |   |   |  Min | Typ | Max  |   |

HOST INTERFACE PINS (INT_N)

|  V_{OLINTN} | Output Low Voltage | 3.0 to 5.5 | I_{OL} = 4 mA | – | – | 0.4 | V  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  T_{INT_Mask} | Time from global interrupt mask bit cleared to when INT_N goes LOW | 3.0 to 5.5 |  | 50 | – | – | μs  |

I²C INTERFACE PINS – STANDARD, FAST, OR FAST MODE PLUS SPEED MODE (SDA, SCL) (Note 6)

|  V_{ILI2C} | Low-Level Input Voltage | 3.0 to 5.5 |  | – | – | 0.51 | V  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  V_{IHI2C} | High-Level Input Voltage | 3.0 to 5.5 |  | 1.32 | – | – | V  |

www.onsemi.com

ICO

Share Feedback

Your Opinion Matters

# Page 17

FUSB302B

Table 13. IO SPECIFICATIONS

|  Symbol | Parameter | V_{DD} (V) | Conditions | T_{A} = -40 to +85°C
T_{A} = -40 to +105°C (Note 11)
T_{J} = -40 to +125°C |   |   | Unit  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|   |   |   |   |  Min | Typ | Max  |   |

I²C INTERFACE PINS – STANDARD, FAST, OR FAST MODE PLUS SPEED MODE (SDA, SCL) (Note 6)

|  V_{HYS} | Hysteresis of Schmitt Trigger Inputs | 3.0 to 5.5 |  | 94 | – | – | mV  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  I_{12C} | Input Current of SDA and SCL Pins | 3.0 to 5.5 | Input Voltage 0.26 V to 2.0 V | –10 | – | 10 | μA  |
|  I_{CCT12C} | VDD Current when SDA or SCL is HIGH | 3.0 to 5.5 | Input Voltage 1.8 V | –10 | – | 10 | μA  |
|  V_{OLSDA} | Low-Level Output Voltage (Open-Drain) | 3.0 to 5.5 | I_{OL} = 2 mA | 0 | – | 0.35 | V  |
|  I_{OLSDA} | Low-Level Output Current (Open-Drain) | 3.0 to 5.5 | V_{OLSDA} = 0.4 V | 20 | – | – | mA  |
|  C_{I} | Capacitance for Each I/O Pin (Note 7) | 3.0 to 5.5 |  | – | 5 | – | pF  |

6. I²C pull up voltage is required to be between 1.71 V and V<sub>DD</sub>.

Table 14. I²C SPECIFICATIONS FAST MODE PLUS I²C SPECIFICATIONS

|  Symbol | Parameter | Fast Mode Plus |   | Unit  |
| --- | --- | --- | --- | --- |
|   |   |  Min | Max  |   |
|  f_{SCL} | I2C_SCL Clock Frequency | 0 | 1000 | kHz  |
|  t_{HD;STA} | Hold Time (Repeated) START Condition | 0.26 | – | μs  |
|  t_{LOW} | Low Period of I2C_SCL Clock | 0.5 | – | μs  |
|  t_{HIGH} | High Period of I2C_SCL Clock | 0.26 | – | μs  |
|  t_{SU;STA} | Set-up Time for Repeated START Condition | 0.26 | – | μs  |
|  t_{HD;DAT} | Data Hold Time | 0 | – | μs  |
|  t_{SU;DAT} | Data Set-up Time | 50 | – | ns  |
|  t_{r} | Rise Time of I2C_SDA and I2C_SCL Signals (Note 7) | – | 120 | ns  |
|  t_{f} | Fall Time of I2C_SDA and I2C_SCL Signals (Note 7) | 6 | 120 | ns  |
|  t_{SU;STO} | Set-up Time for STOP Condition | 0.26 | – | μs  |
|  t_{BUF} | Bus-Free Time between STOP and START Conditions (Note 7) | 0.5 | – | μs  |
|  t_{SP} | Pulse Width of Spikes that Must Be Suppressed by the Input Filter | 0 | 50 | ns  |
|  C_{b} | Capacitive Load for each Bus Line (Note 7) | – | 550 | pF  |
|  t_{VD-DAT} | Data Valid Time for Data from SCL LOW to SDA HIGH or LOW Output (Note 7) | 0 | 0.45 | μs  |
|  t_{VD-ACK} | Data Valid Time for acknowledge from SCL LOW to SDA HIGH or LOW Output (Note 7) | 0 | 0.45 | μs  |
|  V_{nL} | Noise Margin at the LOW Level (Note 7) | 0.2 | – | V  |
|  V_{nH} | Noise Margin at the HIGH Level (Note 7) | 0.4 | – | V  |

7. Guaranteed by Characterization and/or Design. Not production tested.

www.onsemi.com
ICL
Share Feedback
Your Opinion Matters

# Page 18

FUSB302B

![img-18.jpeg](img-18.jpeg)

![img-19.jpeg](img-19.jpeg)
Figure 17. Definition of Timing for Full-Speed Mode Devices on the I²C Bus

Table 15. I²C SLAVE ADDRESS

|  Name | Bit 7 | Bit 6 | Bit 5 | Bit 4 | Bit 3 | Bit 2 | Bit 1 | Bit 0  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  FUSB302BUCX, FUSB302BMPX, FUSB302BVMPX | 0 | 1 | 0 | 0 | 0 | 1 | 0 | R/W  |
|  FUSB302B01MPX | 0 | 1 | 0 | 0 | 0 | 1 | 1 | R/W  |
|  FUSB302B10MPX | 0 | 1 | 0 | 0 | 1 | 0 | 0 | R/W  |
|  FUSB302B11MPX | 0 | 1 | 0 | 0 | 1 | 0 | 1 | R/W  |

Table 16. REGISTER DEFINITIONS (Notes 8 and 9)

|  Address | Register Name | Type | Reg Value | Bit 7 | Bit 6 | Bit 5 | Bit 4 | Bit 3 | Bit 2 | Bit 1 | Bit 0  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  0x01 | Device ID | R | 9X | Version ID[3:0] |   |   |   | Product ID[1:0] |   | Revision ID[1:0]  |   |
|  0x02 | Switches0 | R/W | 3 | PU_EN2 | PU_EN1 | VCONN_CC2 | VCONN_CC1 | MEAS_CC2 | MEAS_CC1 | PDWN2 | PDWN1  |
|  0x03 | Switches1 | R/W | 20 | POWER ROLE | SPEC REV1 | SPEC REV0 | DATA ROLE |  | AUTO_CRC | TXCC2 | TXCC1  |
|  0x04 | Measure | R/W | 31 |  | MEAS_VBUS | MDAC5 | MDAC4 | MDAC3 | MDAC2 | MDAC1 | MDAC0  |
|  0x05 | Slice | R/W | 60 | SDAC_HYS1 | SDAC_HYS2 | SDAC5 | SDAC4 | SDAC3 | SDAC2 | SDAC1 | SDAC0  |
|  0x06 | Control0 | R/W/C | 24 |  | TX_FLUSH | INT_MASK |  | HOST_CUR1 | HOST_CUR0 | AUTO_PRE | TX_START  |
|  0x07 | Control1 | R/W/C | 0 |  | ENSOP 2DB | ENSOP 1DB | BIST_MODE2 |  | RX_FLUSH | ENSOP2 | ENSOP1  |
|  0x08 | Control2 | R/W | 2 | TOG_SAVE_PWR2 | TOG_SAVE_PWR1 | TOG_RD_ONLY |  | WAKE_EN | MODE[1:0] |   | TOGGLE  |
|  0x09 | Control3 | R/W | 6 |  | SEND_HARD_ | BIST_TMODE | AUTO_HARD_RESET | AUTO_ | N_RETRIES[1:0] | AUTO_RETRY  |   |
|   |   |   |   |   |  RESET |   |   | SOFTRES_ET  |   |   |   |

www.onsemi.com

ICL

Share Feedback

Your Opinion Matters

# Page 19

FUSB302B

Table 16. REGISTER DEFINITIONS (Notes 8 and 9)

|  Address | Register Name | Type | Reg Value | Bit 7 | Bit 6 | Bit 5 | Bit 4 | Bit 3 | Bit 2 | Bit 1 | Bit 0  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  0x0A | Mask1 | R/W | 0 | M_VBUSOK | M_ACTIVITY | M_COMP_CHNG | M_CRC_C_HK | M_ALERT | M_WAKE | M_COLLISION | M_BC_LVL  |
|  0x0B | Power | R/W | 1 |  |  |  |  | PWR3 | PWR2 | PWR1 | PWR0  |
|  0x0C | Reset | W/C | 0 |  |  |  |  |  |  | PD_RESET | SW_RES  |
|  0x0D | OCPreg | R/W | 0F |  |  |  |  | OCP_RANGE | OCP_CUR2 | OCP_CUR1 | OCP_CUR0  |
|  0x0E | Maska | R/W | 0 | M_OCP_TEMP | M_TOGDONE | M_SOFT_FAIL | M_RETRY_FAIL | M_HARD_SENT | M_TXSENT | M_SOFTRST | M_HARDRST  |
|  0x0F | Maskb | R/W | 0 |  |  |  |  |  |  |  | M_GCROSENT  |
|  0x10 | Control4 | R/W | 0 |  |  |  |  |  |  |  | TOG_EXIT_AUD  |
|  0x3C | Status0a | R | 0 |  |  | SOFTFAIL | RETRY_FAIL | POWER3 | POWER2 | SOFTRST | HARDRST  |
|  0x3D | Status1a | R | 0 |  |  | TOGSS3 | TOGSS2 | TOGSS1 | RXSOP 2DB | RXSOP 1DB | RXSOP  |
|  0x3E | Interrupta | R/C | 0 | I_OCP_TEMP | I_TOGDONE | I_SOFTFAIL | I_RETRY_FAIL | I_HARD_SENT | I_TXSENT | I_SOFT_RST | I_HARD_RST  |
|  0x3F | Interruptb | R/C | 0 |  |  |  |  |  |  |  | I_GCRCS_ENT  |
|  0x40 | Status0 | R | 0 | VBUSOK | ACTIVITY | COMP | CRC_CHK | ALERT | WAKE | BC_LVL1 | BC_LVL0  |
|  0x41 | Status1 | R | 28 | RXSOP2 | RXSOP1 | RX_EMPTY | RX_FULL | TX_EMPTY | TX_FULL | OVRTEMP | OCP  |
|  0x42 | Interrupt | R/C | 0 | I_VBUSOK | I_ACTIVITY | I_COMP_CHNG | I_CRC_CHK | I_ALERT | I_WAKE | I_COLLISION | I_BC_LVL  |
|  0x43 | FIFOs | R/W (Note 10) | 0 | Write to TX FIFO or read from RX FIFO repeatedly without address auto increment  |   |   |   |   |   |   |   |
|  Type C Bits | USB PD Bits | General Bits  |
| --- | --- | --- |

8. Do not use registers that are blank.
9. Values read from undefined register bits are not defined and invalid. Do not write to undefined registers.
10. FIFO register is serially read/written without auto address increment.
11. Automotive Part Only; FUSB302BVMPX

Table 17. DEVICE ID
(Address: 01h; Reset Value: 0x1001_XXXX; Type: Read)

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7:4 | Version ID | R | 4 | Device version ID by Trim or etc.
A_[Revision ID]: 1000 (e.g. A_revA)
B_[Revision ID]: 1001
C_[Revision ID]: 1010 etc  |
|  3:2 | Product ID | R | 2 | “01”, “10” and “11” applies to MLP only:
00: FUSB302BMPX/FUSB302BVMPX(Default) & FUSB302BUCX
01: FUSB302B01MPX
10: FUSB302B10MPX
11: FUSB302B11MPX  |
|  1:0 | Revision ID | R | 2 | Revision History of each version
[Version ID]_revA: 00(e.g. revA)
[Version ID]_revB: 01 (e.g. revB)
[Version ID]_revC: 10 (e.g. revC)
[Version ID]_revC: 11 (e.g. revD)  |

www.onsemi.com

ICS

Share Feedback

Your Opinion Matters

# Page 20

FUSB302B

Table 18. SWITCHES0
(Address: 02h; Reset Value: 0x0000_0011; Type: Read/Write)

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7 | PU_EN2 | R/W | 1 | 1: Apply host pull up current to CC2 pin  |
|  6 | PU_EN1 | R/W | 1 | 1: Apply host pull up current to CC1 pin  |
|  5 | VCONN_CC2 | R/W | 1 | 1: Turn on the VCONN current to CC2 pin  |
|  4 | VCONN_CC1 | R/W | 1 | 1: Turn on the VCONN current to CC1 pin  |
|  3 | MEAS_CC2 | R/W | 1 | 1: Use the measure block to monitor or measure the voltage on CC2  |
|  2 | MEAS_CC1 | R/W | 1 | 1: Use the measure block to monitor or measure the voltage on CC1  |
|  1 | PDWN2 | R/W | 1 | 1: Device pull down on CC2. 0: no pull down  |
|  0 | PDWN1 | R/W | 1 | 1: Device pull down on CC1. 0: no pull down  |

Table 19. SWITCHES1
(Address: 03h; Reset Value: 0x0010_0000; Type: Read/Write)

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7 | POWERROLE | R/W | 1 | Bit used for constructing the GoodCRC acknowledge packet. This bit corresponds to the Port Power Role bit in the message header if an SOP packet is received:
1: Source if SOP
0: Sink if SOP  |
|  6:5 | SPECREV1: SPECREV0 | R/W | 2 | Bit used for constructing the GoodCRC acknowledge packet. These bits correspond to the Specification Revision bits in the message header:
00: Revision 1.0
01: Revision 2.0
10: Do Not Use
11: Do Not Use  |
|  4 | DATAROLE | R/W | 1 | Bit used for constructing the GoodCRC acknowledge packet. This bit corresponds to the Port Data Role bit in the message header. For SOP:
1: SRC
0: SNK  |
|  3 | Reserved | N/A | 1 | Do Not Use  |
|  2 | AUTO_CRC | R/W | 1 | 1: Starts the transmitter automatically when a message with a good CRC is received and automatically sends a GoodCRC acknowledge packet back to the relevant SOP*
0: Feature disabled  |
|  1 | TXCC2 | R/W | 1 | 1: Enable BMC transmit driver on CC2 pin  |
|  0 | TXCC1 | R/W | 1 | 1: Enable BMC transmit driver on CC1 pin  |

www.onsemi.com

ICO

Share Feedback

Your Opinion Matters

# Page 21

FUSB302B

Table 20. MEASURE
(Address: 04h; ·Reset Value: 0x0011_0001; Type: Read/Write)

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7 | Reserved | N/A | 1 | Do Not Use  |
|  6 | MEAS_VBUS | R/W | 1 | 0: MDAC/comparator measurement is controlled by MEAS_CC* bits
1: Measure VBUS with the MDAC/comparator. This requires MEAS_CC* bits to be 0  |
|  5:0 | MDAC[5:0] | R/W | 6 | Measure Block DAC data input. LSB is equivalent to 42 mV of voltage which is compared to the measured CC voltage.
The measured CC is selected by MEAS_CC2, or MEAS_CC1 bits.
MDAC[5:0] MEAS_VBUS = 0 MEAS_VBUS = 1 Unit
00_0000 0.042 0.420 V
00_0001 0.084 0.840 V
11_0000 2.058 20.58 V
11_0011 2.184 21.84 V
11_1110 2.646 26.46 V
11_1111 > 2.688 26.88 V  |

Table 21. SLICE
(Address: 05h; Reset Value: 0x0110_0000; Type: Read/Write)

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7:6 | SDAC_HYS[1:0] | R/W | 2 | Adds hysteresis where there are now two thresholds, the lower threshold which is always the value programmed by SDAC[5:0] and the higher threshold that is:
11: 255 mV hysteresis: higher threshold = (SDAC value + 20hex)
10: 170 mV hysteresis: higher threshold = (SDAC value + Ahex)
01: 85 mV hysteresis: higher threshold = (SDAC value + 5)
00: No hysteresis: higher threshold = SDAC value  |
|  5:0 | SDAC[5:0] | R/W | 6 | BMC Slicer DAC data input. Allows for a programmable threshold so as to meet the BMC receive mask under all noise conditions.  |

Table 22. CONTROL0
(Address: 06h; Reset Value: 0x0010_0100; Type: (see column below))

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7 | Reserved | N/A | 1 | Do Not Use  |
|  6 | TX_FLUSH | W/C | 1 | 1: Self clearing bit to flush the content of the transmit FIFO  |
|  5 | INT_MASK | R/W | 1 | 1: Mask all interrupts
0: Interrupts to host are enabled  |
|  4 | Reserved | N/A | 1 | Do Not Use  |
|  3:2 | HOST_CUR[1:0] | R/W | 2 | 1: Controls the host pull up current enabled by PU_EN[2:1]:
00: No current
01: 80 μA – Default USB power
10: 180 μA – Medium Current Mode: 1.5 A
11: 330 μA – High Current Mode: 3 A  |
|  1 | AUTO_PRE | R/W | 1 | 1: Starts the transmitter automatically when a message with a good CRC is received. This allows the software to take as much as 300 μS to respond after the I_CRC_CHK interrupt is received. Before starting the transmitter, an internal timer waits for approximately 170 μS before executing the transmit start and preamble
0: Feature disabled  |
|  0 | TX_START | W/C | 1 | 1: Start transmitter using the data in the transmit FIFO. Preamble is started first. During the preamble period the transmit data can start to be written to the transmit FIFO. Self clearing.  |

www.onsemi.com

Share Feedback

Your Opinion Matters

# Page 22

FUSB302B

Table 23. CONTROL1
(Address: 07h; Reset Value: 0x0000_0000; Type: (see column below))

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7 | Reserved | N/A | 1 | Do Not Use  |
|  6 | ENSOP2DB | R/W | 1 | 1: Enable SOP"_DEBUG (SOP double prime debug) packets
0: Ignore SOP"_DEBUG (SOP double prime debug) packets  |
|  5 | ENSOP1DB | R/W | 1 | 1: Enable SOP"_DEBUG (SOP prime debug) packets
0: Ignore SOP"_DEBUG (SOP prime debug) packets  |
|  4 | BIST_MODE2 | R/W | 1 | 1: Sent BIST Mode 01s pattern for testing  |
|  3 | Reserved | N/A | 1 | Do Not Use  |
|  2 | RX_FLUSH | W/C | 1 | 1: Self clearing bit to flush the content of the receive FIFO  |
|  1 | ENSOP2 | R/W | 1 | 1: Enable SOP"(SOP double prime) packets
0: Ignore SOP"(SOP double prime) packets  |
|  0 | ENSOP1 | R/W | 1 | 1: Enable SOP"(SOP prime) packets
0: Ignore SOP"(SOP prime) packets  |

Table 24. CONTROL2
(Address: 08h; Reset Value: 0x0000_0010; Type: (see column below))

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7:6 | TOG_SAVE_PWR2: TOG_SAVE_PWR1 | N/A | 2 | 00: Don't go into the DISABLE state after one cycle of toggle
01: Wait between toggle cycles for tDIS time of 40 ms
10: Wait between toggle cycles for tDIS time of 80 ms
11: Wait between toggle cycles for tDIS time of 160 ms  |
|  5 | TOG_RD_ONLY | R/W | 1 | 1: When TOGGLE=1 only Rd values will cause the TOGGLE state machine to stop toggling and trigger the I_TOGGLE interrupt
0: When TOGGLE=1, Rd and Ra values will cause the TOGGLE state machine to stop toggling  |
|  4 | Reserved | N/A | 1 | Do Not Use  |
|  3 | WAKE_EN | R/W | 1 | 1: Enable Wake Detection functionality if the power state is correct
0: Disable Wake Detection functionality  |
|  2:1 | MODE | R/W | 2 | 11: Enable SRC polling functionality if TOGGLE=1
10: Enable SNK polling functionality if TOGGLE=1
01: Enable DRP polling functionality if TOGGLE=1
00: Do Not Use  |
|  0 | TOGGLE | R/W | 1 | 1: Enable DRP, SNK or SRC Toggle autonomous functionality
0: Disable DRP, SNK and SRC Toggle functionality  |

www.onsemi.com

Share Feedback

Your Opinion Matters

# Page 23

FUSB302B

Table 25. CONTROL3
(Address: 09h; Reset Value: 0x0000_0110; Type: (see column below))

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7 | Reserved | N/A | 1 | Do Not Use  |
|  6 | SEND_HARD_RESET | W/C | 1 | 1: Send a hard reset packet (highest priority)
0: Don’t send a soft reset packet  |
|  5 | BIST_TMODE | R/W | 1 | 1: BIST mode. Receive FIFO is cleared immediately after sending GoodCRC response
0: Normal operation, All packets are treated as usual  |
|  4 | AUTO_HARDRESET | R/W | 1 | 1: Enable automatic hard reset packet if soft reset fail
0: Disable automatic hard reset packet if soft reset fail  |
|  3 | AUTO_SOFTRESET | R/W | 1 | 1: Enable automatic soft reset packet if retries fail
0: Disable automatic soft reset packet if retries fail  |
|  2:1 | N_RETRIES[1:0] | R/W | 2 | 11: Three retries of packet (four total packets sent)
10: Two retries of packet (three total packets sent)
01: One retry of packet (two total packets sent)
00: No retries (similar to disabling auto retry)  |
|  0 | AUTO_RETRY | R/W | 1 | 1: Enable automatic packet retries if GoodCRC is not received
0: Disable automatic packet retries if GoodCRC not received  |

Table 26. MASK
(Address: 0Ah; Reset Value: 0x0000_0000; Type: Read/Write)

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7 | M_VBUSOK | R/W | 1 | 1: Mask I_VBUSOK interrupt bit
0: Do not mask  |
|  6 | M_ACTIVITY | R/W | 1 | 1: Mask interrupt for a transition in CC bus activity
0: Do not mask  |
|  5 | M_COMP_CHNG | R/W | 1 | 1: Mask I_COMP_CHNG interrupt for change is the value of COMP, the measure comparator
0: Do not mask  |
|  4 | M_CRC_CHK | R/W | 1 | 1: Mask interrupt from CRC_CHK bit
0: Do not mask  |
|  3 | M_ALERT | R/W | 1 | 1: Mask the I_ALERT interrupt bit
0: Do not mask  |
|  2 | M_WAKE | R/W | 1 | 1: Mask the I_WAKE interrupt bit
0: Do not mask  |
|  1 | M_COLLISION | R/W | 1 | 1: Mask the I_COLLISION interrupt bit
0: Do not mask  |
|  0 | M_BC_LVL | R/W | 1 | 1: Mask a change in host requested current level
0: Do not mask  |

Table 27. POWER
(Address: 0Bh; Reset Value: 0x0000_0001; Type: Read/Write)

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7:4 | Reserved | N/A | 4 | Do Not Use  |
|  3:0 | PWR[3:0] | R/W | 4 | Power enables:
PWR[0]: Bandgap and wake circuit
PWR[1]: Receiver powered and current references for Measure block
PWR[2]: Measure block powered
PWR[3]: Enable internal oscillator  |

www.onsemi.com

ICS

Share Feedback

Your Opinion Matters

# Page 24

FUSB302B

Table 28. RESET
(Address: 0Ch; Reset Value: 0x0000_0000; Type: Write/Clear)

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7:2 | Reserved | N/A | 6 | Do Not Use  |
|  1 | PD_RESET | W/C | 1 | 1: Reset just the PD logic for both the PD transmitter and receiver  |
|  0 | SW_RES | W/C | 1 | 1: Reset the FUSB302B including the I²C registers to their default values  |

Table 29. OCPREG
(Address: 0Dh; Reset Value: 0x0000_1111; Type: Read/Write)

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7:4 | Reserved | N/A | 4 | Do Not Use  |
|  3 | OCP_RANGE | R/W | 1 | 1: OCP range between 100–800 mA (max_range = 800 mA)
0: OCP range between 10–80 mA (max_range = 80 mA)  |
|  2:0 | OCP_CUR2, OCP_CUR1, OCP_CUR0 | R/W | 3 | 111: max_range (see bit definition above for OCP_RANGE)
110: 7 × max_range / 8
101: 6 × max_range / 8
100: 5 × max_range / 8
011: 4 × max_range / 8
010: 3 × max_range / 8
001: 2 × max_range / 8
000: max_range / 8  |

Table 30. MASKA
(Address: 0Eh; Reset Value: 0x0000_0000; Type: Read/Write)

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7 | M_OCP_TEMP | R/W | 1 | 1: Mask the I_OCP_TEMP interrupt  |
|  6 | M_TOGDONE | R/W | 1 | 1: Mask the I_TOGDONE interrupt  |
|  5 | M_SOFTFAIL | R/W | 1 | 1: Mask the I_SOFTFAIL interrupt  |
|  4 | M_RETRYFAIL | R/W | 1 | 1: Mask the I_RETRYFAIL interrupt  |
|  3 | M_HARDSENT | R/W | 1 | 1: Mask the I_HARDSENT interrupt  |
|  2 | M_TXSENT | R/W | 1 | 1: Mask the I_TXSENT interrupt  |
|  1 | M_SOFTRST | R/W | 1 | 1: Mask the I_SOFTRST interrupt  |
|  0 | M_HARDRST | R/W | 1 | 1: Mask the I_HARDRST interrupt  |

Table 31. MASKB
(Address: 0Fh; Reset Value: 0x0000_0000; Type: Read/Write)

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7:1 | Reserved | N/A | 6 | Do Not Use  |
|  0 | M_GCRCSENT | R/W | 1 | 1: Mask the I_GCRCSENT interrupt  |

Table 32. CONTROL4
(Address: 00h; Reset Value: 0x0000_0000; Type: Read/Write)

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7:1 | Reserved | N/A | 6 | Do Not Use  |
|  0 | TOG_EXIT_AUD | R/W | 1 | 1: In auto Rd only Toggle mode, stop Toggle at Audio accessory (Ra on both CC)  |

www.onsemi.com

ICS

Share Feedback

Your Opinion Matters

# Page 25

FUSB302B

Table 33. STATUS0A
(Address: 3Ch; Reset Value: 0x0000_0000; Type: Read)

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7:6 | Reserved | N/A | 2 | Do Not Use  |
|  5 | SOFTFAIL | R | 1 | 1: All soft reset packets with retries have failed to get a GoodCRC acknowledge. This status is cleared when a START_TX, TXON or SEND_HARD_RESET is executed  |
|  4 | RETRYFAIL | R | 1 | 1: All packet retries have failed to get a GoodCRC acknowledge. This status is cleared when a START_TX, TXON or SEND_HARD_RESET is executed  |
|  3:2 | POWER3:POWER2 | R | 2 | Internal power state when logic internals needs to control the power state. POWER3 corresponds to PWR3 bit and POWER2 corresponds to PWR2 bit. The power state is the higher of both PWR[3:0] and {POWER3, POWER2, PWR[1:0]} so that if one is 03 and the other is F then the internal power state is F  |
|  1 | SOFTRST | R | 1 | 1: One of the packets received was a soft reset packet  |
|  0 | HARDRST | R | 1 | 1: Hard Reset PD ordered set has been received  |

Table 34. STATUS1A
(Address: 3Dh; Reset Value: 0x0000_0000; Type: Read)

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7:6 | Reserved | N/A | 2 | Do Not Use  |
|  5:3 | TOGSS3, TOGSS2, TOGSS1 | R | 3 | 000: Toggle logic running (processor has previously written TOGGLE=1)
001: Toggle functionality has settled to SRCon CC1 (STOP_SRC1 state)
010: Toggle functionality has settled to SRCon CC2 (STOP_SRC2 state)
101: Toggle functionality has settled to SNKon CC1 (STOP_SNK1 state)
110: Toggle functionality has settled to SNKon CC2 (STOP_SNK2 state)
111: Toggle functionality has detected AudioAccessory with vRa on both CC1 and CC2 (settles to STOP_SRC1 state)
Otherwise: Not defined (do not interpret)  |
|  2 | RXSOP2DB | R | 1 | 1: Indicates the last packet placed in the RxFIFO is type SOP"_DEBUG (SOP double prime debug)  |
|  1 | RXSOP1DB | R | 1 | 1: Indicates the last packet placed in the RxFIFO is type SOP"_DEBUG (SOP prime debug)  |
|  0 | RXSOP | R | 1 | 1: Indicates the last packet placed in the RxFIFO is type SOP  |

www.onsemi.com

25

Share Feedback

Your Opinion Matters

# Page 26

FUSB302B

(Address: 3Eh; Reset Value: 0x0000_0000; Type: Read/Clear)

Table 35. INTERRUPTA

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7 | I_OCP_TEMP | R/C | 1 | 1: Interrupt from either a OCP event on one of the VCONN switches or an over-temperature event  |
|  6 | I_TOGDONE | R/C | 1 | 1: Interrupt indicating the TOGGLE functionality was terminated because a device was detected  |
|  5 | I_SOFTFAIL | R/C | 1 | 1: Interrupt from automatic soft reset packets with retries have failed  |
|  4 | I_RETRYFAIL | R/C | 1 | 1: Interrupt from automatic packet retries have failed  |
|  3 | I_HARDSENT | R/C | 1 | 1: Interrupt from successfully sending a hard reset ordered set  |
|  2 | I_TXSENT | R/C | 1 | 1: Interrupt to alert that we sent a packet that was acknowledged with a GoodCRC response packet  |
|  1 | I_SOFTRST | R/C | 1 | 1: Received a soft reset packet  |
|  0 | I_HARDRST | R/C | 1 | 1: Received a hard reset ordered set  |

(Address: 3Fh; Reset Value: 0x0000_0000; Type: Read/Clear)

Table 36. INTERRUPTB

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7 | Reserved | N/A | 6 | Do Not Use  |
|  0 | I_GCRCSENT | R/C | 1 | 1: Sent a GoodCRC acknowledge packet in response to an incoming packet that has the correct CRC value  |

www.onsemi.com

26

Share Feedback

Your Opinion Matters

# Page 27

FUSB302B

Table 37. STATUS0
(Address: 40h; Reset Value: 0x0000_0000; Type: Read)

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7 | VBUSOK | R | 1 | 1: Interrupt occurs when VBUS transitions through vVBUSthr. This bit typically is used to recognize port partner during startup  |
|  6 | ACTIVITY | R | 1 | 1: Transitions are detected on the active CC* line. This bit goes high after a minimum of 3 CC transitions, and goes low with no Transitions
0: Inactive  |
|  5 | COMP | R | 1 | 1: Measured CC* input is higher than reference level driven from the MDAC
0: Measured CC* input is lower than reference level driven from the MDAC  |
|  4 | CRC_CHK | R | 1 | 1: Indicates the last received packet had the correct CRC. This bit remains set until the SOP of the next packet
0: Packet received for an enabled SOP* and CRC for the enabled packet received was incorrect  |
|  3 | ALERT | R | 1 | 1: Alert software an error condition has occurred. An alert is caused by:
TX_FULL: the transmit FIFO is full
RX_FULL: the receive FIFO is full
See Status1 bits  |
|  2 | WAKE | R | 1 | 1: Voltage on CC indicated a device attempting to attach
0: WAKE either not enabled (WAKE_EN=0) or no device attached  |
|  1:0 | BC_LVL[1:0] | R | 2 | Current voltage status of the measured CC pin interpreted as host current levels as follows:
00: < 200 mV
01: > 200 mV, < 660 mV
10: > 660 mV, < 1.23 V
11: > 1.23 V
Note the software must measure these at an appropriate time, while there is no signaling activity on the selected CC line.
BC_LVL is only defined when Measure block is on which is when register bits PWR[2]=1 and either MEAS_CC1=1 or MEAS_CC2=1  |

Table 38. STATUS1
(Address: 41h; Reset Value: 0x0010_1000; Type: Read)

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7 | RXSOP2 | R | 1 | 1: Indicates the last packet placed in the RxFIFO is type SOP" (SOP double prime)  |
|  6 | RXSOP1 | R | 1 | 1: Indicates the last packet placed in the RxFIFO is type SOP" (SOP prime)  |
|  5 | RX_EMPTY | R | 1 | 1: The receive FIFO is empty  |
|  4 | RX_FULL | R | 1 | 1: The receive FIFO is full  |
|  3 | TX_EMPTY | R | 1 | 1: The transmit FIFO is empty  |
|  2 | TX_FULL | R | 1 | 1: The transmit FIFO is full  |
|  1 | OVRTEMP | R | 1 | 1: Temperature of the device is too high  |
|  0 | OCP | R | 1 | 1: Indicates an over-current or short condition has occurred on the VCONN switch  |

www.onsemi.com

ICS

Share Feedback

Your Opinion Matters

# Page 28

FUSB302B

Table 39. INTERRUPT
(Address: 42h; Reset Value: 0x0000_0000; Type: Read/Clear)

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7 | I_VBUSOK | R/C | 1 | 1: Interrupt occurs when VBUS transitions through 4.5 V. This bit typically is used to recognize port partner during startup  |
|  6 | I_ACTIVITY | R/C | 1 | 1: A change in the value of ACTIVITY of the CC bus has occurred  |
|  5 | I_COMP_CHNG | R/C | 1 | 1: A change in the value of COMP has occurred. Indicates selected CC line has tripped a threshold programmed into the MDAC  |
|  4 | I_CRC_CHK | R/C | 1 | 1: The value of CRC_CHK newly valid. I.e. The validity of the incoming packet has been checked  |
|  3 | I_ALERT | R/C | 1 | 1: Alert software an error condition has occurred. An alert is caused by:
TX_FULL: the transmit FIFO is full
RX_FULL: the receive FIFO is full
See Status1 bits  |
|  2 | I_WAKE | R/C | 1 | 1: Voltage on CC indicated a device attempting to attach. Software must then power up the clock and receiver blocks  |
|  1 | I_COLLISION | R/C | 1 | 1: When a transmit was attempted, activity was detected on the active CC line. Transmit is not done. The packet is received normally  |
|  0 | I_BC_LVL | R/C | 1 | 1: A change in host requested current level has occurred  |

Table 40. FIFOS
(Address: 43h; Reset Value: 0x0000_0000; Type: (see column below))

|  Bit # | Name | R/W/C | Size (Bits) | Description  |
| --- | --- | --- | --- | --- |
|  7:0 | TX/RX Token | Read or Write | 8 | Writing to this register writes a byte into the transmit FIFO. Reading from this register reads from the receive FIFO. Each byte is a coded token. Or a token followed by a fixed number of packed data byte (see token coding in Table 41)  |

Software Model

Port software interacts with the port chip in two primary ways:
- I²C Registers
- 8 bit data tokens sent to or received from the FIFO register
- All reserved bits written in the TxFIFO should be 0 and all reserved bit read from the RxFIFO should be ignored

Transmit Data Tokens

Transmit data tokens provide in-sequence transmit control and data for the transmit logic. Note that the token codes, and their equivalent USB PD K-Code are not the same. Tokens are read one at a time when they reach the end of the TX FIFO. I.e., the specified token action is performed before the next token is read from the TX FIFO.

The tokens are defined as follows:

www.onsemi.com

ICS

Share Feedback

Your Opinion Matters

# Page 29

FUSB302B

Table 41. TOKENS USED IN FIFO

|  Code | Name | Size (Bytes) | Description  |
| --- | --- | --- | --- |
|  101x-xxx1
(0xA1) | TXON | 1 | Alternative method for starting the transmitter with the TX-START bit. This is not a token written to the TxFIFO but a command much like TX_START but it is more convenient to write it while writing to the TxFIFO in one contiguous write operation. It is preferred that the TxFIFO is first written with data and then TXON or TX_START is executed. It is expected that A1h will be written for TXON not any other bits where x is non-zero such as B1h, BFh, etc  |
|  0x12 | SOP1 | 1 | When reaching the end of the FIFO causes a Sync-1 symbol to be transmitted  |
|  0x13 | SOP2 | 1 | When reaching the end of the FIFO causes a Sync-2 symbol to be transmitted  |
|  0x1B | SOP3 | 1 | When reaching the end of the FIFO causes a Sync-3 symbol to be transmitted  |
|  0x15 | RESET1 | 1 | When reaching the end of the FIFO causes a RST-1 symbol to be transmitted  |
|  0x16 | RESET2 | 1 | When reaching the end of the FIFO causes a RST-2 symbol to be transmitted  |
|  0x80 | PACKSYM | 1+N | This data token must be immediately followed by a sequence of N packed data bytes. This token is defined by the 3 MSB's being set to 3'b100. The 5 LSB's are the number of packed bytes being sent.
Note: N cannot be less than 2 since the minimum control packet has a header that is 2 bytes and N cannot be greater than 30 since the maximum data packet has 30 bytes (2 byte header + 7 data objects each having 4 bytes)
Packed data bytes have two 4 bit data fields. The 4 LSB's are sent first, after 4b5b conversion etc in the chip  |
|  0xFF | JAM_CRC | 1 | Causes the CRC, calculated by the hardware, to be inserted into the transmit stream when this token reaches the end of the TX FIFO  |
|  0x14 | EOP | 1 | Causes an EOP symbol to be sent when this token reaches the end of the TX FIFO  |
|  0xFE | TXOFF | 1 | Turn off the transmit driver. Typically the next symbol after EOP  |

# RECEIVE DATA TOKENS

Receive data tokens provide in-sequence receive control and data for the receive logic. The RxFIFO can absorb as many packets as the number of bytes in the RxFIFO (80 bytes). The tokens are defined as follows:

Table 42. TOKENS USED IN RxFIFO

|  Code | Name | Size (Bytes) | Description  |
| --- | --- | --- | --- |
|  111b_bbb | SOP | 1 | First byte of a received packet to indicate that the packet is an SOP packet ("b" is undefined and can be any bit)  |
|  110b_bbb | SOP1 | 1 | First byte of a received packet to indicate that the packet is an SOP' packet and occurs only if ENSOP1=1 ("b" is undefined and can be any bit)  |
|  101b_bbb | SOP2 | 1 | First byte of a received packet to indicate that the packet is an SOP" packet and occurs only if ENSOP2=1 ("b" is undefined and can be any bit)  |
|  100b_bbb | SOP1DB | 1 | First byte of a received packet to indicate that the packet is an SOP"_DEBUG packet and occurs only if ENSOP1DB=1 ("b" is undefined and can be any bit)  |
|  011b_bbb | SOP2DB | 1 | First byte of a received packet to indicate that the packet is an SOP"_DEBUG packet and occurs only if ENSOP2DB=1 ("b" is undefined and can be any bit)  |
|  010b_bbb/001b_bbb/000b_bbb | Do Not Use | 1 | These can be used in future versions of this device and should not be relied on to be any special value. ("b" is undefined and can be any bit)  |

www.onsemi.com

29

Share Feedback

Your Opinion Matters

# Page 30

# FUSB302B

# REFERENCE SCHEMATIC

![img-20.jpeg](img-20.jpeg)
Figure 18. FUSB302/FUSB302B Reference Schematic Diagram

Table 43. RECOMMENDED COMPONENT VALUES FOR REFERENCE SCHEMATIC

|  Symbol | Parameter | Recommended Value |   |   | Unit  |
| --- | --- | --- | --- | --- | --- |
|   |   |  Min | Typ | Max  |   |
|  CRECV | CCX Receiver Capacitance | 200 | - | 600 | pF  |
|  CBULK | VCONN Source Bulk Capacitance | 10 | - | 220 | μF  |
|  CVCONN | VCONN Decoupling Capacitance | - | 0.1 | - | μF  |
|  CVDD1 | VDD Decoupling Capacitance | - | 0.1 | - | μF  |
|  CVDD2 | VDD Decoupling Capacitance | - | 1.0 | - | μF  |
|  RPU | I2C Pull-up Resistors | - | 4.7 | - | kΩ  |
|  RPU_INT | INT_N Pull-up Resistor | 1.0 | 4.7 | - | kΩ  |
|  VPU | I2C Pull-up Voltage | 1.71 | - | VDD | V  |

The table below is in reference to the WLCSP package drawing on the following page.

Table 44. PRODUCT-SPECIFIC DIMENSIONS

|  Product | D | E | X | Y  |
| --- | --- | --- | --- | --- |
|  FUSB302BUCX | 1.260 mm | 1.215 mm | 0.2075 mm | 0.230 mm  |

All brand names and product names appearing in this document are registered trademarks or trademarks of their respective holders.

www.onsemi.com

30

Share Feedback

Your Opinion Matters

# Page 31

onsemi

MECHANICAL CASE OUTLINE

PACKAGE DIMENSIONS

![img-21.jpeg](img-21.jpeg)
SCALE 4:1

WQFN14 2.5x2.5, 0.5P

CASE 510BR

ISSUE O

DATE 31 AUG 2016

![img-22.jpeg](img-22.jpeg)

![img-23.jpeg](img-23.jpeg)

![img-24.jpeg](img-24.jpeg)

![img-25.jpeg](img-25.jpeg)
RECOMMENDED LAND PATTERN

NOTES:

A. NO JEDEC REGISTRATION.
B. DIMENSIONS ARE IN MILLIMETERS.
C. DIMENSIONS AND TOLERANCES PER ASME Y14.5M, 2009.
D. LAND PATTERN RECOMMENDATION IS EXISTING INDUSTRY LAND PATTERN.

|  DOCUMENT NUMBER: | 98AON13629G | Electronic versions are uncontrolled except when accessed directly from the Document Repository. Printed versions are uncontrolled except when stamped “CONTROLLED COPY” in red.  |   |
| --- | --- | --- | --- |
|  DESCRIPTION: | WQFN14 2.5X2.5, 0.5P |   | PAGE 1 OF 1  |

onsemi and onsemi are trademarks of Semiconductor Components Industries, LLC dba onsemi or its subsidiaries in the United States and/or other countries. onsemi reserves the right to make changes without further notice to any products herein. onsemi makes no warranty, representation or guarantee regarding the suitability of its products for any particular purpose, nor does onsemi assume any liability arising out of the application or use of any product or circuit, and specifically disclaims any and all liability, including without limitation special, consequential or incidental damages. onsemi does not convey any license under its patent rights nor the rights of others.

© Semiconductor Components Industries, LLC, 2016

www.onsemi.com

# Page 32

onsemi

MECHANICAL CASE OUTLINE

PACKAGE DIMENSIONS

SCALE 5:1

WLCSP9 1.26x1.215x0.526

CASE 567TN

ISSUE O

DATE 31 MAR 2017

![img-26.jpeg](img-26.jpeg)
TOP VIEW

![img-27.jpeg](img-27.jpeg)
LAND PATTERN RECOMMENDATION (NSMD PAD TYPE)

![img-28.jpeg](img-28.jpeg)

![img-29.jpeg](img-29.jpeg)

![img-30.jpeg](img-30.jpeg)
BOTTOM VIEW

# NOTES:

A. NO JEDEC REGISTRATION APPLIES.
B. DIMENSIONS ARE IN MILLIMETERS.
C. DIMENSIONS AND TOLERANCE PER ASME Y14.5M, 2009.
D. DATUM C IS DEFINED BY THE SPHERICAL CROWNS OF THE BALLS.
E. PACKAGE NOMINAL HEIGHT IS 488 MICRONS ±38 MICRONS (450-526 MICRONS).
F. FOR DIMENSIONS D, E, X, AND Y SEE PRODUCT DATASHEET.

|  DOCUMENT NUMBER: | 98AON13359G | Electronic versions are uncontrolled except when accessed directly from the Document Repository. Printed versions are uncontrolled except when stamped “CONTROLLED COPY” in red.  |
| --- | --- | --- |
|  DESCRIPTION: | WLCSP9 1.26x1.215x0.526 | PAGE 1 OF 1  |

onsemi and onsemi are trademarks of Semiconductor Components Industries, LLC dba onsemi or its subsidiaries in the United States and/or other countries. onsemi reserves the right to make changes without further notice to any products herein. onsemi makes no warranty, representation or guarantee regarding the suitability of its products for any particular purpose, nor does onsemi assume any liability arising out of the application or use of any product or circuit, and specifically disclaims any and all liability, including without limitation special, consequential or incidental damages. onsemi does not convey any license under its patent rights nor the rights of others.

© Semiconductor Components Industries, LLC, 2017

www.onsemi.com

# Page 33

onsemi, ONSEMTs, and other names, marks, and brands are registered and/or common law trademarks of Semiconductor Components Industries, LLC dba "onsemi" or its affiliates and/or subsidiaries in the United States and/or other countries. onsemi owns the rights to a number of patents, trademarks, copyrights, trade secrets, and other intellectual property. A listing of onsemi's product/patent coverage may be accessed at www.onsemi.com/site/pdf/Patent-Marking.pdf. onsemi reserves the right to make changes at any time to any products or information herein, without notice. The information herein is provided "as-is" and onsemi makes no warranty, representation or guarantee regarding the accuracy of the information, product features, availability, functionality, or suitability of its products for any particular purpose, nor does onsemi assume any liability arising out of the application or use of any product or circuit, and specifically disclaims any and all liability, including without limitation special, consequential or incidental damages. Buyer is responsible for its products and applications using onsemi products, including compliance with all laws, regulations and safety requirements or standards, regardless of any support or applications information provided by onsemi. "Typical" parameters which may be provided in onsemi data sheets and/or specifications can and do vary in different applications and actual performance may vary over time. All operating parameters, including "Typicals" must be validated for each customer application by customer's technical experts. onsemi does not convey any license under any of its intellectual property rights nor the rights of others. onsemi products are not designed, intended, or authorized for use as a critical component in life support systems or any FDA Class 3 medical devices or medical devices with a same or similar classification in a foreign jurisdiction or any devices intended for implantation in the human body. Should Buyer purchase or use onsemi products for any such unintended or unauthorized application, Buyer shall indemnify and hold onsemi and its officers, employees, subsidiaries, affiliates, and distributors harmless against all claims, costs, damages, and expenses, and reasonable attorney fees arising out of, directly or indirectly, any claim of personal injury or death associated with such unintended or unauthorized use, even if such claim alleges that onsemi was negligent regarding the design or manufacture of the part. onsemi is an Equal Opportunity/Affirmative Action Employer. This literature is subject to all applicable copyright laws and is not for resale in any manner.

ADDITIONAL INFORMATION

TECHNICAL PUBLICATIONS:

Technical Library: www.onsemi.com/design/resources/technical-documentation
onsemi Website: www.onsemi.com

ONLINE SUPPORT: www.onsemi.com/support

For additional information, please contact your local Sales Representative at
www.onsemi.com/support/sales

