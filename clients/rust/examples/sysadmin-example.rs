extern crate sysadmin_bindings;
extern crate protobuf;
extern crate error_chain;

use std::time::Duration;

// note the wildcard import is recommended for error_chain
// see https://docs.rs/error-chain
use sysadmin_bindings::error_chain_generated_errors::*;
use sysadmin_bindings::{SysadminClient, Set, Commit, CommitConfig, CommitResponse};


// Available Commands:
//     Set
//     Get
//     Commit
//
// Coming soon:
//     Drop
//     FireHooks
//     EraseKey
//     Rollback
//     Reset
//     DumpHooks
//     TriggerHook
//     Blame
//     InFlight

fn main() {
    // Build the sysadmin server:
    //      ./docker_control.sh -t
    // and run it with:
    //      ./docker_control.sh -s

    if let Err(e) = run() {
        // set RUST_BACKTRACE=1 for more info
        use std::io::Write;
        use error_chain::ChainedError;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";
        writeln!(stderr, "{}", e.display_chain()).expect(errmsg);
        ::std::process::exit(1);
    }
}

// Note the "Result" being returned is a type alias generated by error-chaing
// https://docs.rs/error-chain/0.11.0/error_chain/example_generated/type.Result.html
fn run() -> Result<(())> {
    let mut client = SysadminClient::new(Duration::from_secs(2_u64), 1_u32, 1_u32);

    // init connection
    // if the connection fails we will report an error and exit
    client.connect("127.0.0.1:4000").chain_err(|| {
        "this will fail if sysadmin isn't running locally".to_string()
    })?;

    // commands can sent using functions
    let set_resp = client.set(String::from("foo"), 55_i32)?;
    println!("{:?}", set_resp);

    let com_resp: CommitResponse = client.commit(CommitConfig::default())?;
    assert_eq!(com_resp.status, sysadmin_bindings::StatusCode::SUCCESS);
    println!("{:?}", com_resp);

    // or you can create command-specific struct which implement
    // a method for sending themselves via a borrowed client
    let set_struct = Set::new("bar", 3);
    let bar_set_response = set_struct.send_command(&mut client)?;
    println!("{:?}", bar_set_response);

    let commit_struct = Commit::new(CommitConfig::NO_HOOKS);
    let com_resp = commit_struct.send_command(&mut client)?;
    assert_eq!(com_resp.status, sysadmin_bindings::StatusCode::SUCCESS);
    println!("{:?}", com_resp);

    let get_resp = client.get(String::from("foo"))?;
    println!("GetResponse.id {:?}", get_resp.id);
    println!("GetResponse.status {:?}", get_resp.status);
    get_resp.kvs.into_iter().for_each(|k| {
        println!(
            "GetResponse.kvs key={:?} value={:?}",
            k.key,
            k.value.unwrap()
        )
    });
    Ok(())
}
