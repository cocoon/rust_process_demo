#[macro_use] extern crate rocket;

#[cfg(windows)]
use std::os::windows::process::ExitStatusExt;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
#[cfg(not(windows))]
use std::os::unix::process::ExitStatusExt;

use std::process::Stdio;
use std::thread;
use rocket::tokio::process::Command;
use std::process::ExitStatus;

use rocket::*;

const _CREATE_NO_WINDOW: u32 = 0x08000000;
const _DETACHED_PROCESS: u32 = 0x00000008;
const _CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;

#[cfg(windows)]
async fn process_win(mut command: &str) -> ExitStatus {
    
    // flag to spawn adb using cmd or directly
    let with_cmd = true; // [true + creation_flag _DETACHED_PROCESS] is causing shutdown error for rocket with ctrl + c: aborting due to failed shutdown

    println!("[process_win] command: {}", &command);

    let mut cmd = command.to_string();

    if with_cmd == true {
        cmd = format!("cmd /c {}", command);
        println!("[process_win] command with cmd: {}", &cmd);
    }

    let args = winsplit::split(&cmd);
    dbg!(&args);

    if args.len() > 0 {

        //let c = Command::new(&args[0]).args(&args[1..]).creation_flags(_DETACHED_PROCESS).stdout(Stdio::piped()).spawn().expect("failed to execute child");
        //let c = Command::new(&args[0]).args(&args[1..]).creation_flags(_CREATE_NEW_PROCESS_GROUP).stdout(Stdio::piped()).spawn().expect("failed to execute child");
        //let c = Command::new(&args[0]).args(&args[1..]).creation_flags(_DETACHED_PROCESS | _CREATE_NEW_PROCESS_GROUP).stdout(Stdio::piped()).spawn().expect("failed to execute child");
        //let c = Command::new(&args[0]).args(&args[1..]).creation_flags(_DETACHED_PROCESS | _CREATE_NEW_PROCESS_GROUP | _CREATE_NO_WINDOW).stdout(Stdio::piped()).spawn().expect("failed to execute child");
        let c = Command::new(&args[0]).args(&args[1..]).stdout(Stdio::piped()).spawn().expect("failed to execute child");
        
        let c_wait = c.wait_with_output().await;

        if c_wait.is_ok() {
                let out_res = c_wait.unwrap();
                println!("OUTPUT: {:?}", &out_res);

                let exit = out_res.status;

                return exit;           
        }

        return ExitStatus::from_raw(1);
	} else {
		let c = Command::new("cmd").arg("/c").arg(command).spawn().unwrap().wait().await.unwrap();
        return c;
	};
}

async fn process_lin(command: &str) -> ExitStatus {
    println!("[process_lin] command: {}", &command);

    let args = shell_words::split(command).unwrap();

	if args.len() > 0 {
		let c = Command::new(&args[0]).args(&args[1..]).spawn().unwrap().wait().await.unwrap();
        return c;
	} else {
		let c = Command::new("sh").arg("-c").arg(command).spawn().unwrap().wait().await.unwrap();
        return c;
	};
	
}

#[get("/run")]
async fn test1() -> String {

    #[cfg(windows)] {
        let adb_command = ".\\adb\\adb.exe connect 127.0.0.1";
        process_win(&adb_command).await;
    }

    #[cfg(not(windows))] {
        let adb_command = "adb connect 127.0.0.1";
        process_lin(&adb_command).await;
    }

    format!("run adb!")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![test1])
}