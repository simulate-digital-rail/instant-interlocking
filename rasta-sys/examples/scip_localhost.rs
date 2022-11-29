use std::ffi::{c_void, CStr, CString};

use once_cell::sync::Lazy;
use rasta_sys::{
    self, add_timed_event, disable_timed_event, enable_timed_event, rasta_connection, rasta_handle,
    rasta_lib_configuration_t, rasta_lib_connection_t, rasta_lib_init_configuration,
    rasta_lib_start, rasta_notification_result, sci_get_name_string, sci_return_code_SUCCESS,
    scip_cleanup, scip_init, scip_on_rasta_receive, scip_point_location,
    scip_point_location_POINT_LOCATION_RIGHT, scip_point_target_location,
    scip_point_target_location_POINT_LOCATION_CHANGE_TO_RIGHT, scip_register_sci_name,
    scip_send_change_location, scip_send_location_status, scip_t, sr_cleanup, sr_connect,
    sr_get_received_data, timed_event, RastaIPData,
};

#[repr(C)]
struct ConnectEventData {
    handle: *mut rasta_handle,
    ip_data_arr: [RastaIPData; 2],
    connect_event: *mut timed_event,
    schwarzenegger: *mut timed_event,
}

const ID_S: u32 = 0x61;
const ID_C: u32 = 0x62;

static mut SCIP: Lazy<*mut scip_t> =
    Lazy::<*mut scip_t>::new(|| unsafe { std::mem::zeroed::<*mut scip_t>() });

extern "C" fn on_receive(result: *mut rasta_notification_result) {
    let message = unsafe { sr_get_received_data((*result).handle, &mut (*result).connection) };
    unsafe { scip_on_rasta_receive(*SCIP, message) };
}

extern "C" fn on_handshake_complete(result: *mut rasta_notification_result) {
    if unsafe { (*result).connection.my_id } == ID_C {
        println!("Sending change location command");
        let sci_name_s = CString::new("S").unwrap();
        let code = unsafe {
            scip_send_change_location(
                *SCIP,
                sci_name_s.as_ptr() as *mut _,
                scip_point_target_location_POINT_LOCATION_CHANGE_TO_RIGHT,
            )
        };
        if code == sci_return_code_SUCCESS {
            println!("Sent change location command to server");
        } else {
            println!(
                "Something went wrong, error code 0x{:02x} was returned",
                code
            );
        }
    }
}

extern "C" fn on_change_location(
    p: *mut scip_t,
    sender: *mut i8,
    location: scip_point_target_location,
) {
    let sender_name = unsafe { CStr::from_ptr(sender) };
    println!(
        "Received location change to 0x{:02x} from {:?}",
        location, sender_name
    );

    println!("Sending back location status...");
    let code =
        unsafe { scip_send_location_status(p, sender, scip_point_location_POINT_LOCATION_RIGHT) };
    if code == sci_return_code_SUCCESS {
        println!("Sent location status");
    } else {
        println!(
            "Something went wrong, error code 0x{:02x} was returned",
            code
        );
    }
}

extern "C" fn on_location_status(_p: *mut scip_t, sender: *mut i8, location: scip_point_location) {
    let name = unsafe { sci_get_name_string(sender) };
    println!(
        "Received location status from {:?}. Point is at position 0x{:02x}",
        unsafe { CString::from_raw(name) },
        location
    );
}

extern "C" fn terminator(h: *mut c_void) -> i32 {
    let handle: *mut rasta_handle = unsafe { std::mem::transmute(h) };
    unsafe { sr_cleanup(handle) };
    1
}

extern "C" fn connect_timed(carry_data: *mut c_void) -> i32 {
    println!("->  Connection request sent to 0x{:02x}", ID_S);
    let data: &mut ConnectEventData = unsafe { &mut *(carry_data as *mut ConnectEventData) };
    unsafe { sr_connect(data.handle, ID_S as u64, data.ip_data_arr.as_mut_ptr()) };
    unsafe { enable_timed_event(data.schwarzenegger) };
    unsafe { disable_timed_event(data.connect_event) };
    0
}

extern "C" fn on_con_start(_connection: *mut rasta_connection) -> *mut c_void {
    let con = Box::new(unsafe { std::mem::zeroed::<rasta_lib_connection_t>() });
    Box::leak(con) as *mut [rasta_connection; 1] as *mut c_void
}

extern "C" fn on_con_end(_connection: *mut rasta_connection, _memory: *mut c_void) {
    // TODO: Don't leak memory
    //unsafe { std::alloc::dealloc(memory as *mut _, Layout::for_value_raw(memory as *const _)) };
}

fn main() {
    let role = std::env::args().nth(1).unwrap();

    let mut rc = unsafe { std::mem::zeroed::<rasta_lib_configuration_t>() }.as_mut_ptr();
    let mut to_server = [unsafe { std::mem::zeroed::<RastaIPData>() }; 2];
    to_server[0].ip =
        unsafe { std::mem::transmute::<[u8; 16], [i8; 16]>(*b"127.0.0.1\0\0\0\0\0\0\0") };
    to_server[1].ip =
        unsafe { std::mem::transmute::<[u8; 16], [i8; 16]>(*b"127.0.0.1\0\0\0\0\0\0\0") };
    to_server[0].port = 8888;
    to_server[1].port = 8889;

    let mut termination_event = unsafe { std::mem::zeroed::<timed_event>() };
    let mut connect_on_timeout_event = unsafe { std::mem::zeroed::<timed_event>() };

    let mut connect_on_stdin_event_data = ConnectEventData {
        handle: unsafe { &mut (*rc).h },
        ip_data_arr: to_server,
        connect_event: &mut termination_event,
        schwarzenegger: &mut connect_on_timeout_event,
    };

    termination_event.callback = Some(terminator);
    termination_event.carry_data = unsafe { &mut (*rc).h } as *const rasta_handle as *mut c_void;
    termination_event.interval = 30000000000;

    connect_on_timeout_event.callback = Some(connect_timed);
    connect_on_timeout_event.carry_data =
        &mut connect_on_stdin_event_data as *const ConnectEventData as *mut c_void;
    connect_on_timeout_event.interval = 3000000000;

    if role == "s" {
        println!("->   S (ID = 0x{:x}", ID_S);

        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        let cfg_path = CString::new("examples/config/rasta_server_local.cfg").unwrap();
        unsafe { rasta_lib_init_configuration(rc, cfg_path.as_ptr()) };

        unsafe { (*rc).h.notifications.on_receive = Some(on_receive) };
        unsafe { (*rc).h.notifications.on_handshake_complete = Some(on_handshake_complete) };

        let sci_name_s = CString::new("S").unwrap();
        unsafe { *SCIP = scip_init(&mut (*rc).h, sci_name_s.as_ptr() as *mut i8) };
        unsafe { (*(*SCIP)).notifications.on_change_location_received = Some(on_change_location) };

        unsafe { (*(*rc).h.user_handles).on_connection_start = Some(on_con_start) };
        unsafe { (*(*rc).h.user_handles).on_disconnect = Some(on_con_end) };

        unsafe { enable_timed_event(&mut termination_event) };
        unsafe { disable_timed_event(&mut connect_on_timeout_event) };
        unsafe { add_timed_event(&mut (*rc).rasta_lib_event_system, &mut termination_event) };
        unsafe { rasta_lib_start(rc, 0) };
    }

    if role == "c" {
        println!("->   C (ID = 0x{:x}", ID_C);

        let cfg_path = CString::new("examples/config/rasta_client1_local.cfg").unwrap();
        unsafe { rasta_lib_init_configuration(rc, cfg_path.as_ptr()) };

        unsafe { (*rc).h.notifications.on_receive = Some(on_receive) };
        unsafe { (*rc).h.notifications.on_handshake_complete = Some(on_handshake_complete) };

        unsafe { (*(*rc).h.user_handles).on_connection_start = Some(on_con_start) };
        unsafe { (*(*rc).h.user_handles).on_disconnect = Some(on_con_end) };

        println!("->  Press Enter to connect");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        unsafe { enable_timed_event(&mut termination_event) };
        unsafe { enable_timed_event(&mut connect_on_timeout_event) };
        unsafe { add_timed_event(&mut (*rc).rasta_lib_event_system, &mut termination_event) };
        unsafe {
            add_timed_event(
                &mut (*rc).rasta_lib_event_system,
                &mut connect_on_timeout_event,
            )
        };

        let sci_name_c = CString::new("C").unwrap();
        unsafe { *SCIP = scip_init(&mut (*rc).h, sci_name_c.as_ptr() as *mut i8) };
        unsafe { (*(*SCIP)).notifications.on_location_status_received = Some(on_location_status) };

        let sci_name_s = CString::new("S").unwrap();
        unsafe { scip_register_sci_name(*SCIP, sci_name_s.as_ptr() as *mut _, ID_S as u64) };

        unsafe { rasta_lib_start(rc, 0) };
    }

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    unsafe { scip_cleanup(*SCIP) };
    unsafe { sr_cleanup(&mut (*rc).h) };
}
