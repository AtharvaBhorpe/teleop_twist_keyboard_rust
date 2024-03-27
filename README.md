
# teleop_twist_keyboard_rust

Rust Implementation of the Generic Keyboard Teleop for ROS2: https://github.com/ros-teleop/teleop_twist_keyboard

This node is a rust implementation of https://github.com/aarsht7/teleop_cpp_ros2. Please give this repo some love!


> This node is implemented using the [r2r](https://github.com/sequenceplanner/r2r) client library.
> For the official [rclrust](https://github.com/ros2-rust/ros2_rust) client library implementation, please refer [this branch](https://github.com/AtharvaBhorpe/teleop_twist_keyboard_rust/tree/rclrust).

  

## Features

  

This particular implementation does away with keeping the history of previous speed settings, and heavily cuts down on the amount of printing that is done to the terminal via the use of carriage returns (\r).

  

Furthermore, the last command that was sent is reflected, and invalid commands are identified as such.

  
  
  

## Installing the Package

  

> Note: 
> 1. This package does not implement ament/colcon/Cmake.
> 2. The code is synchronous.

  

As this package is implemented using [r2r](https://github.com/sequenceplanner/r2r), the standard ROS practice of creating a workspace is not needed (although can be implemented).

Clone this repository, then run `cargo run` in the *teleop_twist_keyboard_rust* folder, and ***voilà***!


## Prerequisites
Install the following if the `cargo run` fails.
```bash

sudo apt-get update
sudo apt install make clang pkg-config libssl-dev

```


```bash

git clone https://github.com/AtharvaBhorpe/teleop_twist_keyboard_rust

cd teleop_twist_keyboard_rust  # Change to the parent directory

cargo run  # This command will build and run the code.

```

  
  
  

## Running the Node

  

```bash

cd teleop_twist_keyboard_rust
./target/debug/teleop_twist_keyboard_rust # or .\target\debug\teleop_twist_keyboard_rust.exe on Windows
  
  

# If you want to see the outputs, check the /cmd_vel topic

ros2  topic  echo  /cmd_vel

```

  
  
  

## Usage

  

Same as the original + some addons

  

```

Reading from the keyboard and Publishing to Twist!

---------------------------

Moving around:

u i o

j k l

m , .

  

For Holonomic mode (strafing), hold down the shift key:

---------------------------

U I O

J K L

M < >

---------------------------

Simple Teleoperation with arrow keys

⇧

⇦ ⇨

⇩

  

A

D C

B

This increases/decreases speed linearly.

---------------------------

t : up (+z)

b : down (-z)

s/S : stop

q/z : increase/decrease max speeds by 10%

w/x : increase/decrease only linear speed by 10%

e/c : increase/decrease only angular speed by 10%

NOTE: Increasing or Decreasing will take affect live on the moving robot.

Consider Stopping the robot before changing it.

CTRL-C to quit

```

  
  
  

------


## TODO

 - [ ] Implement asynchronous methods.
 - [x] Implement the same node using official [rclrust](https://github.com/ros2-rust/ros2_rust).
