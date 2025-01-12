#![no_std]
#![no_main]

use common::root::env;
use common::root::*;
use common::*;

use core::mem::size_of_val;

type ActionFunc = fn(cid: ContractID);
type ActionsMap<'a> = &'a [(&'a str, ActionFunc)];

// MANAGER ACTIONS

fn on_action_create_contract(_unused: ContractID) {
    let args = CtorParams {};
    env::generate_kernel(
        0 as *const ContractID,
        CtorParams::METHOD,
        &args,
        size_of_val(&args) as u32,
        0 as *const FundsChange,
        0,
        0 as *const SigRequest,
        0,
        "Create contract\0".as_ptr(),
        0,
    );
}

fn on_action_destroy_contract(cid: ContractID) {
    let args = DtorParams {};
    env::generate_kernel(
        &cid,
        DtorParams::METHOD,
        &args,
        size_of_val(&args) as u32,
        0 as *const FundsChange,
        0,
        0 as *const SigRequest,
        0,
        "Destroy contract\0".as_ptr(),
        0,
    );
}

fn on_action_view_contracts(_unused: ContractID) {
    env::enum_and_dump_contracts(&::common::SID);
}

#[no_mangle]
#[allow(non_snake_case)]
fn Method_0() {
    // Add documentation for roles and actions
    env::doc_add_group("\0");
    env::doc_add_group("roles\0");

    // Manager role documentation
    env::doc_add_group("manager\0");
    env::doc_add_group("create\0");
    env::doc_close_group(); // create

    env::doc_add_group("destroy\0");
    env::doc_add_text("cid\0", "ContractID\0".as_ptr());
    env::doc_close_group(); // destroy

    env::doc_add_group("view\0");
    env::doc_close_group(); // view

    env::doc_close_group(); // manager

    // Test role documentation
    env::doc_add_group("test\0");
    env::doc_add_group("hw\0");
    env::doc_add_text("message", "Outputs 'Hello World!'\0".as_ptr());
    env::doc_close_group(); // hw
    env::doc_close_group(); // test

    env::doc_close_group(); // roles
    env::doc_close_group(); // \0
}

#[no_mangle]
#[allow(non_snake_case)]
fn Method_1() {
    const INVALID_ROLE_ACTIONS: [(&str, ActionFunc); 0] = [];

    const VALID_MANAGER_ACTIONS: [(&str, ActionFunc); 3] = [
        ("create\0", on_action_create_contract),
        ("destroy\0", on_action_destroy_contract),
        ("view\0", on_action_view_contracts),
    ];

    const VALID_ROLES: [(&str, ActionsMap); 2] = [
        ("manager\0", &VALID_MANAGER_ACTIONS),
        ("test\0", &INVALID_ROLE_ACTIONS),
    ];

    // Get role input
    let mut role: [u8; 32] = Default::default();
    if env::doc_get_text("role\0", &mut role, size_of_val(&role) as u32) == 0 {
        env::doc_add_text("error\0", "Missing or invalid role\0".as_ptr());
        return;
    }

    if env::memcmp(&role, b"test\0".as_ptr(), 4) == 0 {
        // Directly call Method_2 for the "hw" role
        Method_2();
        return;
    }

    let mut action_map: ActionsMap = &INVALID_ROLE_ACTIONS;
    for &(valid_role, actions) in VALID_ROLES.iter() {
        if env::memcmp(&role, valid_role.as_ptr(), valid_role.len() as u32) == 0 {
            action_map = actions;
            break;
        }
    }

    // Handle invalid role
    if action_map == &INVALID_ROLE_ACTIONS {
        env::doc_add_text("error\0", "Invalid role\0".as_ptr());
        return;
    }

    // Get and execute action
    let mut action: [u8; 32] = Default::default();
    if env::doc_get_text("action\0", &mut action, size_of_val(&action) as u32) == 0 {
        env::doc_add_text("error\0", "Missing or invalid action\0".as_ptr());
        return;
    }

    for &(valid_action, func) in action_map.iter() {
        if env::memcmp(&action, valid_action.as_ptr(), valid_action.len() as u32) == 0 {
            let mut cid: ContractID = Default::default();
            env::doc_get_blob("cid\0", &mut cid, size_of_val(&cid) as u32);
            func(cid);
            return;
        }
    }

    env::doc_add_text("error\0", "Invalid action\0".as_ptr());
}

#[no_mangle]
#[allow(non_snake_case)]
fn Method_2() {
    // Handle the "test" role and "hw" action directly
    let mut role: [u8; 32] = Default::default();
    if env::doc_get_text("role\0", &mut role, size_of_val(&role) as u32) == 0 {
        env::doc_add_text("error\0", "Missing or invalid role\0".as_ptr());
        return;
    }

    if env::memcmp(&role, b"test\0".as_ptr(), 4) == 0 {
        let mut action: [u8; 32] = Default::default();
        if env::doc_get_text("action\0", &mut action, size_of_val(&action) as u32) > 0 {
            if env::memcmp(&action, b"hw\0".as_ptr(), 2) == 0 {
                env::doc_add_group("\0");
                env::doc_add_group("test\0");
                env::doc_add_text("hw\0", "Hello World!\0".as_ptr());
                env::doc_close_group(); // test
                env::doc_close_group(); // \0
                return;
            }
        }
        env::doc_add_text("error\0", "Invalid or missing action\0".as_ptr());
        return;
    }

    env::doc_add_text("error\0", "Invalid role\0".as_ptr());
}