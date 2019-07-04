use std::os::raw::c_char;
use cqrs::{gb18030, gb18030_decode};

static mut G_AUTH_CODE: i32 = -1;

use lazy_static::lazy_static;

lazy_static! {
    static ref CQP: cqrs::API = cqrs::API::new().unwrap();
}

static APP_NAME: &[u8] = b"9,me.cqp.demo\0";

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn AppInfo() -> *const c_char {
	return APP_NAME.as_ptr() as *const i8;
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn Initialize(auth_code: i32) -> i32 {
    unsafe { G_AUTH_CODE = auth_code; }
	return 0;
}

#[no_mangle]
pub extern "stdcall" fn app_on_startup() -> i32 { 0 }

#[no_mangle]
pub extern "stdcall" fn app_on_exit() -> i32 { 0 }

#[no_mangle]
pub extern "stdcall" fn app_on_enabled() -> i32 {
    unsafe {
        let qq = CQP.get_login_qq(G_AUTH_CODE);
        CQP.add_log(G_AUTH_CODE,
            cqrs::LogLevel::Info as i32,
            gb18030!("INFO"),
            gb18030!("App enabled, QQ = {}", qq));
    }
    0
}

#[no_mangle]
pub extern "stdcall" fn app_on_disable() -> i32 { 0 }

#[no_mangle]
pub extern "stdcall" fn app_on_private_message(_subtype: i32, _msgid: i32,
    from_qq: i64, msg: *const c_char, _font: i32) -> i32 {
    let msg = unsafe { gb18030_decode(msg).unwrap() };
    if msg == "hello" {
        unsafe {
            CQP.send_private_msg(G_AUTH_CODE,
                from_qq,
                gb18030!("hello"));
        }
    }
    return cqrs::EventResultCode::Ignore as i32;
}

#[no_mangle]
pub extern "stdcall" fn app_on_group_message(_subtype: i32, _msgid: i32,
    _from_group: i64, _from_qq: i64, _from_anonymous: *const c_char,
    _msg: *const c_char, _font: i32) -> i32 {
    return cqrs::EventResultCode::Block as i32;
}

#[no_mangle]
pub extern "stdcall" fn app_on_discuss_message(_subtype: i32, _msgid: i32,
    _from_discuss: i64, _from_qq: i64,
    _msg: *const c_char, _font: i32) -> i32 {
    return cqrs::EventResultCode::Ignore as i32;
}

#[no_mangle]
pub extern "stdcall" fn app_on_group_admin_changed(_subtype: i32, _sendtime: i32,
    _from_group: i64, _being_operate_qq: i64) -> i32 {
    return cqrs::EventResultCode::Ignore as i32;
}

#[no_mangle]
pub extern "stdcall" fn app_on_group_member_decrease(_subtype: i32, _sendtime: i32,
    _from_group: i64, _from_qq: i64, _being_operate_qq: i64) -> i32 {
    return cqrs::EventResultCode::Ignore as i32;
}

#[no_mangle]
pub extern "stdcall" fn app_on_group_member_increase(_subtype: i32, _sendtime: i32,
    _from_group: i64, _from_qq: i64, _being_operate_qq: i64) -> i32 {
    // if subtype == 1 {
    //     unsafe {
    //         CQP.send_group_msg(G_AUTH_CODE,
    //             from_group,
    //             gb18030!("[CQ:at,qq={}] welcome", from_qq));
    //     }
    // }
    return cqrs::EventResultCode::Ignore as i32;
}

#[no_mangle]
pub extern "stdcall" fn app_on_friend_added(_subtype: i32, _sendtime: i32, _from_qq: i64) -> i32 {
    return cqrs::EventResultCode::Ignore as i32;
}

#[no_mangle]
pub extern "stdcall" fn app_on_friend_request(_subtype: i32, _sendtime: i32, _from_qq: i64,
    _msg: *const c_char, _response_flag: *const c_char) -> i32 {
    return cqrs::EventResultCode::Ignore as i32;
}

#[no_mangle]
pub extern "stdcall" fn app_on_group_request(_subtype: i32, _sendtime: i32,
    _from_group: i64, _from_qq: i64,
    _msg: *const c_char, _response_flag: *const c_char) -> i32 {
    // match subtype {
    //     1 => {
    //         unsafe {
    //             CQP.set_group_add_request_v2(G_AUTH_CODE,
    //                 response_flag,
    //                 cqrs::GroupRequestType::Add as i32,
    //                 cqrs::RequestResultCode::Allow as i32,
    //                 gb18030!(""));
    //         }
    //     },
    //     2 => {
    //         unsafe {
    //             CQP.set_group_add_request_v2(G_AUTH_CODE,
    //                 response_flag,
    //                 cqrs::GroupRequestType::Invite as i32,
    //                 cqrs::RequestResultCode::Allow as i32,
    //                 gb18030!("");
    //         }
    //     },
    //     _ => (),
    // }
    return cqrs::EventResultCode::Ignore as i32;
}

#[no_mangle]
pub extern "stdcall" fn app_menu_a() -> i32 { 0 }

#[no_mangle]
pub extern "stdcall" fn app_menu_b() -> i32 { 0 }
