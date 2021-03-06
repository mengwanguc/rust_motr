#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CStr;
use std::ffi::CString;

use anyhow::Result;
use std::os::raw::c_char;
use std::{slice, str};


use std::io::Write;


include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


pub struct MotrClient {
    pub init_instance_done: bool
}


impl MotrClient {

    pub fn new(
        ha_addr: *const ::std::os::raw::c_char,
        local_addr: *const ::std::os::raw::c_char,
        profile_fid: *const ::std::os::raw::c_char,
        process_fid: *const ::std::os::raw::c_char,
    ) -> Self {
        unsafe {
            m0_init_instance(ha_addr, local_addr, profile_fid, process_fid);
            MotrClient{
                init_instance_done: true,
            }
        }
    }

    pub fn read_object(&self, obj_hi:u64, obj_low:u64, start:u64, len:u64) -> *mut read_result{
        unsafe {
            println!("readning object in rust\n");
            m0_object_read(obj_hi, obj_low, start, len)
            // m0_object_read(12345, 9999999999, 4099, 8188)
        }
    }

    pub fn create_object(&self, obj_hi:u64, obj_low:u64) -> std::os::raw::c_int{
        unsafe {
            println!("creating object in rust\n");
            m0_object_create(obj_hi, obj_low)
        }
    }

    pub fn write_object(&self, obj_hi:u64, obj_low:u64, start:u64, len:u64, s: *mut c_char) -> std::os::raw::c_int{
        unsafe {
            println!("creating object in rust\n");
            m0_object_write(obj_hi, obj_low, start, len, s)
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    static ha_addr_str: &str = "inet:tcp:10.140.82.80@22001";
    static local_addr_str: &str = "inet:tcp:10.140.82.80@22501";
    static profile_fid_str: &str = "0x7000000000000001:0x0";
    static process_fid_str: &str = "0x7200000000000001:0x3";

    pub fn str_to_c_char(v: Vec<u8>) -> *mut c_char {
        let bytes: Vec<u8> = v;
        let mut c_chars: Vec<i8> = bytes.iter().map(| c | *c as i8).collect::<Vec<i8>>();

        c_chars.push(0); // null terminator

        let ptr: *mut c_char = c_chars.as_mut_ptr();
        ptr
    }

    #[test]
    fn test_read() {
        unsafe {
            let ha_addr = CString::new(ha_addr_str).expect("CString::new failed");
            let local_addr = CString::new(local_addr_str).expect("CString::new failed");
            let profile_fid = CString::new(profile_fid_str).expect("CString::new failed");
            let process_fid = CString::new(process_fid_str).expect("CString::new failed");
            let client = MotrClient::new(ha_addr.as_ptr(), local_addr.as_ptr(), 
                                         profile_fid.as_ptr(), process_fid.as_ptr());
            println!("\nreadingnow\n\n\n");

            let rres = client.read_object(12345, 9999999999, 400, 5000);
            println!("done reading\n\n\n");

            let res_bytes = slice::from_raw_parts((*rres).data as *const u8, (*rres).len as _);

            let mut file = std::fs::File::create("temp.txt").expect("create failed");
            file.write_all(res_bytes).expect("write failed");            
        }
    }

    #[test]
    fn test_create() {
        unsafe {
            let ha_addr = CString::new(ha_addr_str).expect("CString::new failed");
            let local_addr = CString::new(local_addr_str).expect("CString::new failed");
            let profile_fid = CString::new(profile_fid_str).expect("CString::new failed");
            let process_fid = CString::new(process_fid_str).expect("CString::new failed");
            let client = MotrClient::new(ha_addr.as_ptr(), local_addr.as_ptr(), 
                                         profile_fid.as_ptr(), process_fid.as_ptr());
            println!("\ncreatingingnow\n\n\n");

            let rres = client.create_object(12345, 9999999999);
            println!("done creating  res:{}\n\n\n", rres);        

        }
    }

    #[test]
    fn test_write() {
        unsafe {
            let ha_addr = CString::new(ha_addr_str).expect("CString::new failed");
            let local_addr = CString::new(local_addr_str).expect("CString::new failed");
            let profile_fid = CString::new(profile_fid_str).expect("CString::new failed");
            let process_fid = CString::new(process_fid_str).expect("CString::new failed");
            let client = MotrClient::new(ha_addr.as_ptr(), local_addr.as_ptr(), 
                                         profile_fid.as_ptr(), process_fid.as_ptr());
            println!("\nwriting now\n\n\n");

            let rres = client.write_object(12345, 9999999999, 0, 8192, str_to_c_char(vec![b'X'; 8192]));
            println!("done writing  res:{}\n\n\n", rres);        

        }
    }

}

