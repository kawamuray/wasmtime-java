#[link(wasm_import_module = "xyz")]
extern "C" {
    fn poll_word(addr: i64, len: i32) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn do_work() {
    let mut buf = [0u8; 1024];
    let buf_ptr = buf.as_mut_ptr() as i64;
    eprintln!("Buffer base address: {}", buf_ptr);
    let len = poll_word(buf_ptr, buf.len() as i32);
    let s = std::str::from_utf8(&buf[..len as usize]).unwrap();
    eprintln!("RECV WORD: {}", s);
}
