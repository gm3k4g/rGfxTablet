## What is GfxTablet Driver Rust edition?
 This is an input driver for the GfxTablet android app, but written in the Rust programming language.
## What about GfxTablet?
 GfxTablet is an android app. 
 However, the whole application is made out of 2 components:
* The GfxTablet android app
* An input driver on the destination device

Therefore, this project is about the input driver.
### Features
Testing has been done on a 64-bit Windows 10 PC.
There are some features not supported yet like pressure sensitivity, but they will be coming soon.
Extensive testing hasn't been done so the following features may not work on all platforms.
* Using the size of the canvas being sent from the android app to this client
* Holding the pen down on the tablet's surface counts as a mouse down event, and hovering it above the surface counts as a mouse up event, however there are some nitpicks and things that may not be working exactly correctly.

 NOTE: For now, this should at least only work on 64 bit Windows 10 PCs.
 In addition, there is no signal detection implemented in the driver yet, so you won't be able to have graceful shut downs. For now, if you want to quit from the driver, you will need to Ctrl+C.
### License
This project is licensed under the MIT license.
### Authors
The original author of this project [is here](https://rfc2822.github.io/GfxTablet/).