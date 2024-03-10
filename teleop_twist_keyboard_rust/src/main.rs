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
with some add-ons, implemented with Rust and ROS 2 Humble.
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
                println!("\n\n    ☺  Give it a Star :: https://github.com/AtharvaBhorpe/teleop_twist_keyboard_rust ☺ \n\n");
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
