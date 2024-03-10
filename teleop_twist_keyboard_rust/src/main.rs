// use r2r::QosProfile;
// use tokio::task;
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let ctx = r2r::Context::create()?;
//     let mut node = r2r::Node::create(ctx, "testnode", "")?;
//     let duration = std::time::Duration::from_millis(2500);
//     let mut timer = node.create_wall_timer(duration)?;
//     let publisher =
//         node.create_publisher::<r2r::std_msgs::msg::String>("/hw_topic", QosProfile::default())?;
//     task::spawn(async move {
//         loop {
//             timer.tick().await.unwrap();
//             let msg = r2r::std_msgs::msg::String {
//                 data: "hello world".to_string(),
//             };
//             publisher.publish(&msg).unwrap();
//             std::thread::sleep(std::time::Duration::from_millis(100));
//         }
//     }); 
//     // here we spin the node in its own thread (but we could just busy wait in this thread)
//     let handle = std::thread::spawn(move || loop {
//         node.spin_once(std::time::Duration::from_millis(100));
//     });
//     handle.join().unwrap();
//     Ok(())
// }



///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


///////////////////     The following code publishes keyboard key presses to /cmd_vel      //////////////////////


// use std::io::{self, Read};
// use std::os::fd::AsRawFd;
// use termios::*;
// use r2r::QosProfile;
// use tokio::task;


// // Define the move bindings and speed bindings
// const MOVE_BINDINGS: [(char, f64, f64, f64, f64); 18] = [
//     ('i', 1.0, 0.0, 0.0, 0.0),
//     ('o', 1.0, 0.0, 0.0, -1.0),
//     ('j', 0.0, 0.0, 0.0, 1.0),
//     ('l', 0.0, 0.0, 0.0, -1.0),
//     ('u', 1.0, 0.0, 0.0, 1.0),
//     (',', -1.0, 0.0, 0.0, 0.0),
//     ('.', -1.0, 0.0, 0.0, 1.0),
//     ('m', -1.0, 0.0, 0.0, -1.0),
//     ('O', 1.0, -1.0, 0.0, 0.0),
//     ('I', 1.0, 0.0, 0.0, 0.0),
//     ('J', 0.0, 1.0, 0.0, 0.0),
//     ('L', 0.0, -1.0, 0.0, 0.0),
//     ('U', 1.0, 1.0, 0.0, 0.0),
//     ('<', -1.0, 0.0, 0.0, 0.0),
//     ('>', -1.0, -1.0, 0.0, 0.0),
//     ('M', -1.0, 1.0, 0.0, 0.0),
//     ('t', 0.0, 0.0, 1.0, 0.0),
//     ('b', 0.0, 0.0, -1.0, 0.0),
// ];

// const SPEED_BINDINGS: [(char, f64, f64); 6] = [
//     ('q', 1.1, 1.1),
//     ('z', 0.9, 0.9),
//     ('w', 1.1, 1.0),
//     ('x', 0.9, 1.0),
//     ('e', 1.0, 1.1),
//     ('c', 1.0, 0.9),
// ];

// // Function to get user input
// fn getch() -> char {
//     let mut ch: char;
//     let stdin = io::stdin();
//     let fileno = stdin.as_raw_fd();

//     // Store old settings
//     let mut oldt: Termios = Termios::from_fd(fileno).unwrap();
//     let mut newt: Termios = oldt;

//     // Make required changes and apply the settings
//     newt.c_lflag &= !(ICANON | ECHO | ECHOK | ECHOE | ECHONL | ISIG);
//     newt.c_iflag |= IGNBRK;
//     newt.c_iflag &= !(INLCR | ICRNL | IXON | IXOFF);
//     newt.c_cc[VMIN] = 1;
//     newt.c_cc[VTIME] = 0;

//     tcsetattr(fileno, TCSANOW, &newt).unwrap();

//     // Get the current character
//     ch = stdin.bytes().next().unwrap().unwrap() as char;

//     // Reapply old settings
//     tcsetattr(fileno, TCSANOW, &oldt).unwrap();

//     ch
// }


// // Function to print velocity information
// fn print_vels(speed: f64, turn: f64) {
//     println!("currently:\tspeed {}\tturn {}", speed, turn);
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let ctx: r2r::Context = r2r::Context::create()?;
//     let mut node: r2r::Node = r2r::Node::create(ctx, "teleop_twist_keyboard_rust", "")?;    // Initialize variables
//     let mut speed: f64 = 0.5;
//     let mut turn: f64 = 1.0;
//     let mut x: f64 = 0.0;
//     let mut y: f64 = 0.0;
//     let mut z: f64 = 0.0;
//     let mut th: f64 = 0.0;
//     let mut status: i32 = 0;

//     println!(
//         "This node takes keypresses from the keyboard and performs corresponding actions.\n\
//         Moving around:\n\
//         \tu    i    o\n\
//         \tj    k    l\n\
//         \tm    ,    .\n\
//         \n\
//         For Holonomic mode (strafing), hold down the shift key:\n\
//         \tU    I    O\n\
//         \tJ    K    L\n\
//         \tM    <    >\n\
//         \n\
//         t : up (+z)\n\
//         b : down (-z)\n\
//         \n\
//         anything else : stop\n\
//         \n\
//         q/z : increase/decrease max speeds by 10%\n\
//         w/x : increase/decrease only linear speed by 10%\n\
//         e/c : increase/decrease only angular speed by 10%\n\
//         \n\
//         CTRL-C to quit"
//     );

//     let publisher =
//         node.create_publisher::<r2r::geometry_msgs::msg::Twist>("/cmd_vel", QosProfile::default())?;
//     task::spawn(async move {
//         loop {
//         let key: char = getch();

//         if let Some((_k, new_speed, new_turn)) = SPEED_BINDINGS.iter().find(|&&(k, _, _)| k == key) {
//             // Handle speed bindings
//             speed *= new_speed;
//             turn *= new_turn;

//             print_vels(speed, turn);
//             if status == 14 {
//                 println!(
//                     "This node takes keypresses from the keyboard and performs corresponding actions.\n\
//                     Moving around:\n\
//                     \tu    i    o\n\
//                     \tj    k    l\n\
//                     \tm    ,    .\n\
//                     \n\
//                     For Holonomic mode (strafing), hold down the shift key:\n\
//                     \tU    I    O\n\
//                     \tJ    K    L\n\
//                     \tM    <    >\n\
//                     \n\
//                     t : up (+z)\n\
//                     b : down (-z)\n\
//                     \n\
//                     anything else : stop\n\
//                     \n\
//                     q/z : increase/decrease max speeds by 10%\n\
//                     w/x : increase/decrease only linear speed by 10%\n\
//                     e/c : increase/decrease only angular speed by 10%\n\
//                     \n\
//                     CTRL-C to quit"
//                 );
//             }
//             status = (status + 1) % 15;
//         } else if let Some(&(_k, nx, ny, nz, nth)) = MOVE_BINDINGS.iter().find(|&&(k, _, _, _, _)| k == key) {
//             // Handle move bindings
//             x = nx;
//             y = ny;
//             z = nz;
//             th = nth;
//         } else {
//             // Stop
//             x = 0.0;
//             y = 0.0;
//             z = 0.0;
//             th = 0.0;
//             if key == '\x03' {
//                 break;
//             }
//         }

//         // Perform actions based on user input
//         // (ROS-related code is omitted, and you need to implement it based on your needs)
//         // ...

//         // Print velocity information
//         print_vels(speed, turn);

        
//     }

//     let msg = r2r::geometry_msgs::msg::Twist {
//         linear: r2r::geometry_msgs::msg::Vector3{x: x, y: y, z: z},
//         angular: r2r::geometry_msgs::msg::Vector3{x: 0.0, y: 0.0, z: th}
//     };
//     publisher.publish(&msg).unwrap();
    
//     }); 
//     // here we spin the node in its own thread (but we could just busy wait in this thread)
//     let handle = std::thread::spawn(move || loop {
//         node.spin_once(std::time::Duration::from_millis(100));
//     });
//     handle.join().unwrap();
//     Ok(())
 
// }


////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////



///////////////////     The following code subscribes to /chatter and prints incoming message   //////////////////////
// use futures::future;
// use futures::stream::StreamExt;
// use r2r::QosProfile;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let ctx = r2r::Context::create()?;
//     let mut node = r2r::Node::create(ctx, "testnode", "")?;

//     let sub = node.subscribe::<r2r::std_msgs::msg::String>("/chatter", QosProfile::default())?;

//     let handle = tokio::task::spawn_blocking(move || loop {
//         node.spin_once(std::time::Duration::from_millis(100));
//     });

//     sub.for_each(|msg| {
//         println!("topic: new msg: {}", msg.data);
//         future::ready(())
//     })
//     .await;

//     handle.await?;

//     Ok(())
// }



/////////////////       The following code is the base on which ROS 2 part is added for teleop_twist_keyboard_rust      //////////////////

// use std::io::{self, Read};
// use std::os::fd::AsRawFd;
// use termios::*;

// // Define the move bindings and speed bindings
// const MOVE_BINDINGS: [(char, f32, f32, f32, f32); 18] = [
//     ('i', 1.0, 0.0, 0.0, 0.0),
//     ('o', 1.0, 0.0, 0.0, -1.0),
//     ('j', 0.0, 0.0, 0.0, 1.0),
//     ('l', 0.0, 0.0, 0.0, -1.0),
//     ('u', 1.0, 0.0, 0.0, 1.0),
//     (',', -1.0, 0.0, 0.0, 0.0),
//     ('.', -1.0, 0.0, 0.0, 1.0),
//     ('m', -1.0, 0.0, 0.0, -1.0),
//     ('O', 1.0, -1.0, 0.0, 0.0),
//     ('I', 1.0, 0.0, 0.0, 0.0),
//     ('J', 0.0, 1.0, 0.0, 0.0),
//     ('L', 0.0, -1.0, 0.0, 0.0),
//     ('U', 1.0, 1.0, 0.0, 0.0),
//     ('<', -1.0, 0.0, 0.0, 0.0),
//     ('>', -1.0, -1.0, 0.0, 0.0),
//     ('M', -1.0, 1.0, 0.0, 0.0),
//     ('t', 0.0, 0.0, 1.0, 0.0),
//     ('b', 0.0, 0.0, -1.0, 0.0),
// ];

// const SPEED_BINDINGS: [(char, f32, f32); 6] = [
//     ('q', 1.1, 1.1),
//     ('z', 0.9, 0.9),
//     ('w', 1.1, 1.0),
//     ('x', 0.9, 1.0),
//     ('e', 1.0, 1.1),
//     ('c', 1.0, 0.9),
// ];

// // Function to get user input
// fn getch() -> char {
//     let mut ch: char;
//     let stdin = io::stdin();
//     let fileno = stdin.as_raw_fd();

//     // Store old settings
//     let mut oldt: Termios = Termios::from_fd(fileno).unwrap();
//     let mut newt: Termios = oldt;

//     // Make required changes and apply the settings
//     newt.c_lflag &= !(ICANON | ECHO | ECHOK | ECHOE | ECHONL | ISIG);
//     newt.c_iflag |= IGNBRK;
//     newt.c_iflag &= !(INLCR | ICRNL | IXON | IXOFF);
//     newt.c_cc[VMIN] = 1;
//     newt.c_cc[VTIME] = 0;

//     tcsetattr(fileno, TCSANOW, &newt).unwrap();

//     // Get the current character
//     ch = stdin.bytes().next().unwrap().unwrap() as char;

//     // Reapply old settings
//     tcsetattr(fileno, TCSANOW, &oldt).unwrap();

//     ch
// }


// // Function to print velocity information
// fn print_vels(speed: f32, turn: f32) {
//     println!("currently:\tspeed {}\tturn {}", speed, turn);
// }

// fn main() {
//     // Initialize variables
//     let mut speed = 0.5;
//     let mut turn = 1.0;
//     let mut x = 0.0;
//     let mut y = 0.0;
//     let mut z = 0.0;
//     let mut th = 0.0;
//     let mut status = 0;

//     println!(
//         "This node takes keypresses from the keyboard and performs corresponding actions.\n\
//         Moving around:\n\
//         \tu    i    o\n\
//         \tj    k    l\n\
//         \tm    ,    .\n\
//         \n\
//         For Holonomic mode (strafing), hold down the shift key:\n\
//         \tU    I    O\n\
//         \tJ    K    L\n\
//         \tM    <    >\n\
//         \n\
//         t : up (+z)\n\
//         b : down (-z)\n\
//         \n\
//         anything else : stop\n\
//         \n\
//         q/z : increase/decrease max speeds by 10%\n\
//         w/x : increase/decrease only linear speed by 10%\n\
//         e/c : increase/decrease only angular speed by 10%\n\
//         \n\
//         CTRL-C to quit"
//     );

//     loop {
//         let key: char = getch();

//         if let Some((_k, new_speed, new_turn)) = SPEED_BINDINGS.iter().find(|&&(k, _, _)| k == key) {
//             // Handle speed bindings
//             speed *= new_speed;
//             turn *= new_turn;

//             print_vels(speed, turn);
//             if status == 14 {
//                 println!(
//                     "This node takes keypresses from the keyboard and performs corresponding actions.\n\
//                     Moving around:\n\
//                     \tu    i    o\n\
//                     \tj    k    l\n\
//                     \tm    ,    .\n\
//                     \n\
//                     For Holonomic mode (strafing), hold down the shift key:\n\
//                     \tU    I    O\n\
//                     \tJ    K    L\n\
//                     \tM    <    >\n\
//                     \n\
//                     t : up (+z)\n\
//                     b : down (-z)\n\
//                     \n\
//                     anything else : stop\n\
//                     \n\
//                     q/z : increase/decrease max speeds by 10%\n\
//                     w/x : increase/decrease only linear speed by 10%\n\
//                     e/c : increase/decrease only angular speed by 10%\n\
//                     \n\
//                     CTRL-C to quit"
//                 );
//             }
//             status = (status + 1) % 15;
//         } else if let Some(&(k, nx, ny, nz, nth)) = MOVE_BINDINGS.iter().find(|&&(k, _, _, _, _)| k == key) {
//             // Handle move bindings
//             x = nx;
//             y = ny;
//             z = nz;
//             th = nth;
//         } else {
//             // Stop
//             x = 0.0;
//             y = 0.0;
//             z = 0.0;
//             th = 0.0;
//             if key == '\x03' {
//                 break;
//             }
//         }

//         // Perform actions based on user input
//         // (ROS-related code is omitted, and you need to implement it based on your needs)
//         // ...

//         // Print velocity information
//         print_vels(speed, turn);
//     }
// }


use std::os::fd::AsRawFd;
use termios::*;use std::collections::HashMap;
use std::io::{self, Read};
use std::thread;
use std::time::Duration;
use r2r::*;
use geometry_msgs::msg::Twist;
use lazy_static::*;

// Map for movement keys
lazy_static! {
    static ref MOVE_BINDINGS: HashMap<char, Vec<f64>> = {
        let mut m = HashMap::new();
        m.insert('i', vec![1.0, 0.0, 0.0, 0.0]);
        m.insert('o', vec![1.0, 0.0, 0.0, -1.0]);
        m.insert('j', vec![0.0, 0.0, 0.0, 1.0]);
        m.insert('l', vec![0.0, 0.0, 0.0, -1.0]);
        m.insert('u', vec![1.0, 0.0, 0.0, 1.0]);
        m.insert(',', vec![-1.0, 0.0, 0.0, 0.0]);
        m.insert('.', vec![-1.0, 0.0, 0.0, 1.0]);
        m.insert('m', vec![-1.0, 0.0, 0.0, -1.0]);
        m.insert('O', vec![1.0, -1.0, 0.0, 0.0]);
        m.insert('I', vec![1.0, 0.0, 0.0, 0.0]);
        m.insert('J', vec![0.0, 1.0, 0.0, 0.0]);
        m.insert('L', vec![0.0, -1.0, 0.0, 0.0]);
        m.insert('U', vec![1.0, 1.0, 0.0, 0.0]);
        m.insert('<', vec![-1.0, 0.0, 0.0, 0.0]);
        m.insert('>', vec![-1.0, -1.0, 0.0, 0.0]);
        m.insert('M', vec![-1.0, 1.0, 0.0, 0.0]);
        m.insert('t', vec![0.0, 0.0, 1.0, 0.0]);
        m.insert('b', vec![0.0, 0.0, -1.0, 0.0]);
        m.insert('k', vec![0.0, 0.0, 0.0, 0.0]);
        m.insert('K', vec![0.0, 0.0, 0.0, 0.0]);
        m
    };
}

// Map for speed keys
lazy_static! {
    static ref SPEED_BINDINGS: HashMap<char, Vec<f64>> = {
        let mut m = HashMap::new();
        m.insert('q', vec![1.1, 1.1]);
        m.insert('z', vec![0.9, 0.9]);
        m.insert('w', vec![1.1, 1.0]);
        m.insert('x', vec![0.9, 1.0]);
        m.insert('e', vec![1.0, 1.1]);
        m.insert('c', vec![1.0, 0.9]);
        m
    };
}

const MSG: &str = r#"
Reading from the keyboard and Publishing to Twist!
---------------------------
Moving around:
   u    i    o
   j    k    l
   m    ,    .
For Holonomic mode (strafing), hold down the shift key:
---------------------------
   U    I    O
   J    K    L
   M    <    >
---------------------------
Simple Teleoperation with arrow keys
          ⇧
        ⇦   ⇨
          ⇩

          A
        D   C
          B

---------------------------
t : up (+z)
b : down (-z)
s/S : stop
q/z : increase/decrease max speeds by 10%
w/x : increase/decrease only linear speed by 10%
e/c : increase/decrease only angular speed by 10%
NOTE : Increasing or Decreasing will take affect live on the moving robot.
      Consider Stopping the robot before changing it.
CTRL-C to quit
This is an exact replica of https://github.com/ros-teleop/teleop_twist_keyboard 
with some add-ons, inplemented with Rust and ROS 2 Humble.
"#;

// Init variables
static mut SPEED: f64 = 0.5;
static mut TURN: f64 = 1.0;
static mut X: f64 = 0.0;
static mut Y: f64 = 0.0;
static mut Z: f64 = 0.0;
static mut TH: f64 = 0.0;
static mut KEY: char = ' ';

// Function to get user input
fn getch() -> char {
    let ch: char;
    let stdin = io::stdin();
    let fileno = stdin.as_raw_fd();

    // Store old settings
    let oldt: Termios = Termios::from_fd(fileno).unwrap();
    let mut newt: Termios = oldt;

    // Make required changes and apply the settings
    newt.c_lflag &= !(ICANON | ECHO | ECHOK | ECHOE | ECHONL | ISIG);
    newt.c_iflag |= IGNBRK;
    newt.c_iflag &= !(INLCR | ICRNL | IXON | IXOFF);
    newt.c_cc[VMIN] = 1;
    newt.c_cc[VTIME] = 0;

    tcsetattr(fileno, TCSANOW, &newt).unwrap();

    // Get the current character
    ch = stdin.bytes().next().unwrap().unwrap() as char;

    // Reapply old settings
    tcsetattr(fileno, TCSANOW, &oldt).unwrap();

    ch
}

// Function to check speed is in the range or not
// Used to linearly increase/decrease the speed
fn vel_check(curr: f64, decrease: bool) -> f64 {
    if decrease {
        if curr >= -0.95 {
            curr - 0.05
        } else {
            -1.0
        }
    } else if curr <= 0.95 {
        curr + 0.05
    } else {
        1.0
    }
}

// Linear vel for arrow keys
fn l_vel(key: char, x: f64) -> f64 {
    match key {
        'A' => vel_check(x, false),
        'B' => vel_check(x, true),
        _ => 0.0,
    }
}

// Angular vel for arrow keys
fn a_vel(key: char, th: f64) -> f64 {
    match key {
        'C' => vel_check(th, true),
        'D' => vel_check(th, false),
        _ => 0.0,
    }
}

fn main() -> Result<() > {
    // node init
    let context: Context = Context::create()?;
    let mut node: Node = Node::create(context, "teleop", "")?;    //  Node::create(ctx, name, namespace);

    // define publisher
    let _pub = node.create_publisher::<Twist>("/cmd_vel", QosProfile::default())?;

    let mut twist = Twist::default();

    println!("{}", MSG);
    println!("\nNow top Speed is {} and turn is {} | Last command: ", unsafe { SPEED }, unsafe { TURN });

    loop {
        // get the pressed key
        unsafe {
            KEY = getch();
        }

        //'A' and 'B' represent the Up and Down arrow keys consecutively
        if unsafe { KEY == 'A' || KEY == 'B' } {
            unsafe {
                X = l_vel(KEY, X);
                Y = 0.0;
                Z = 0.0;
                TH = 0.0;
            }
            println!(
                "\rCurrent: speed {}\tturn {} | Last command: {}   ",
                unsafe { SPEED * X },
                unsafe { TURN * TH },
                unsafe { KEY }
            );
        }

        //'C' and 'D' represent the Right and Left arrow keys consecutively
        else if unsafe { KEY == 'C' || KEY == 'D' } {
            unsafe {
                TH = a_vel(KEY, TH);
                Y = 0.0;
                Z = 0.0;
                X = 0.0;
            }
            println!(
                "\rCurrent: speed {}\tturn {} | Last command: {}   ",
                unsafe { SPEED * X },
                unsafe { TURN * TH },
                unsafe { KEY }
            );
        } else if MOVE_BINDINGS.contains_key(&unsafe { KEY }) {
            // Grab the direction data
            unsafe {
                X = MOVE_BINDINGS[&KEY][0];
                Y = MOVE_BINDINGS[&KEY][1];
                Z = MOVE_BINDINGS[&KEY][2];
                TH = MOVE_BINDINGS[&KEY][3];
            }

            println!(
                "\rCurrent: speed {}\tturn {} | Last command: {}   ",
                unsafe { SPEED },
                unsafe { TURN },
                unsafe { KEY }
            );
        }
        // Otherwise if it corresponds to a key in speedBindings
        else if SPEED_BINDINGS.contains_key(&unsafe { KEY }) {
            // Grab the speed data
            unsafe {
                SPEED = SPEED * SPEED_BINDINGS[&KEY][0];
                TURN = TURN * SPEED_BINDINGS[&KEY][1];
            }

            println!(
                "\nNow top Speed is {} and turn is {} | Last command: {} \n\t\tCurrent speed might be affected\n",
                unsafe { SPEED },
                unsafe { TURN },
                unsafe { KEY }
            );
        } else {
            if unsafe { KEY == 's' || KEY == 'S' } {
                unsafe {
                    X = 0.0;
                    Y = 0.0;
                    Z = 0.0;
                    TH = 0.0;
                }
                println!("\n\t\tRobot Stopped..!! \n");
                println!(
                    "\rCurrent: speed {}\tturn {} | Last command: {}   ",
                    unsafe { SPEED * X },
                    unsafe { TURN * TH },
                    unsafe { KEY }
                );
            } else if unsafe { KEY == '\x03' } {
                println!("\n\n    ☺  Give it a Star :: https://github.com/1at7/teleop_cpp_ros2 ☺ \n\n");
                break;
            } else {
                println!(
                    "\rCurrent: speed {}\tturn {} | Invalid command! {}",
                    unsafe { SPEED * X },
                    unsafe { TURN * TH },
                    unsafe { KEY }
                );
            }
        }

        // Update the Twist message
        unsafe {
            twist.linear.x = X * SPEED;
            twist.linear.y = Y * SPEED;
            twist.linear.z = Z * SPEED;
            twist.angular.x = 0.0;
            twist.angular.y = 0.0;
            twist.angular.z = TH * TURN;
        }

        let _ = _pub.publish(&twist);
        thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}
