
# teleop_twist_keyboard_rust

Rust Implementation of the Generic Keyboard Teleop for ROS2: https://github.com/ros-teleop/teleop_twist_keyboard

This node is a rust implementation of https://github.com/aarsht7/teleop_cpp_ros2. Please give this repo some love!

> This node is implemented using the official [rclrust](https://github.com/ros2-rust/ros2_rust) client library.
> For the [r2r](https://github.com/sequenceplanner/r2r) client library implementation, please refer [this branch](https://github.com/AtharvaBhorpe/teleop_twist_keyboard_rust).  

## Features

  

This particular implementation does away with keeping the history of previous speed settings, and heavily cuts down on the amount of printing that is done to the terminal via the use of carriage returns (\r).

  

Furthermore, the last command that was sent is reflected, and invalid commands are identified as such.

  
  
  

## Installing the Package

  

> Note: 
> 1. This package implements colcon.
> 2. The code is synchronous.

  

As this package is implemented using [rclrust](https://github.com/ros2-rust/ros2_rust) client library, the standard ROS practice of creating a workspace is followed.

1. Visit the [rclrust repo](https://github.com/ros2-rust/ros2_rust) to understand the installation process of rclrust.
2. Clone this repository branch in the ***src*** folder of your ROS 2 workspace.
3. Run `colcon build` in your ROS 2 workspace directory, and ***voilà***!


## Prerequisites
Install the following if the `cargo run` or `colcon build` fails.
```bash

sudo apt-get update
sudo apt install make clang pkg-config libssl-dev

```


```bash

cd your_ros2_workspace/src

git clone -b rclrust https://github.com/AtharvaBhorpe/teleop_twist_keyboard_rust

cd..  # Change to the parent directory

colcon build  # This command will build the package.

. ./install/setup.sh

```

  
  
  

## Running the Node

  

```bash

ros2 run teleop_twist_keyboard_rust teleop_twist_keyboard_rust



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
