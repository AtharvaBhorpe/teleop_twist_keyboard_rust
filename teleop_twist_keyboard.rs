use ros2_rust::{publish};
use ros2_rust::rosmsg::geometry_msgs::msg::Twist;
use crossterm::{event::{self, Event, KeyCode}, terminal}; 
use std::{thread, time};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = ros2_rust::init_node("keyboard_controller")?;
    let mut vel_pub = publish(&node, "/cmd_vel", 1)?;
    
    terminal::enable_raw_mode()?;

    println!("Use WASD keys to control the robot");

    loop {
        if let Event::Key(key) = event::read()? {
            let mut twist = Twist::new();

            if let KeyCode::Char(c) = key.code {
                match c {
                    'w' => twist.linear.x = 0.1,
                    's' => twist.linear.x = -0.1,
                    'a' => twist.angular.z = 0.1,
                    'd' => twist.angular.z = -0.1,
                    _ => {}  // Do nothing for unused keys
                }
            }

            vel_pub.publish(twist)?;
        }

        thread::sleep(time::Duration::from_millis(100)); 
    }

    terminal::disable_raw_mode()?;
    ros2_rust::destroy_node(node);
    Ok(())
}
