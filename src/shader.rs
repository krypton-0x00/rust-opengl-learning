use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::ptr;

use gl::{COMPILE_STATUS, FRAGMENT_SHADER, LINK_STATUS, VERTEX_SHADER};

pub fn load_shader_from_file(path: PathBuf) -> CString {
    //TODO: Add Link status handling
    if !path.exists() {
        panic!("Path: {:?} doesnt exist", path.as_os_str());
    } else {
        let mut data = String::new();
        let mut file = match File::open(path) {
            Err(e) => panic!("Couldnt open the shader file: {}", e),
            Ok(file) => file,
        };
        match file.read_to_string(&mut data) {
            Err(e) => panic!("Cant read the shader file: {}", e),
            Ok(_) => {}
        };
        CString::new(data).unwrap()
    }
}

fn check_shader_status(shader: u32, status_type: u32) {
    let mut success = 0;
    let info_log: [u8; 512] = [0; 512];
    unsafe {
        gl::GetShaderiv(shader, status_type, &mut success);
        if success == 0 {
            gl::GetShaderInfoLog(
                shader,
                512,
                ptr::null::<i32>() as *mut i32,
                info_log.as_ptr() as *mut i8,
            );
            //TODO: improve this
            panic!(
                "Error:Shader::{}::Compilation failed {:?}",
                shader, info_log
            )
        }
    }
}

pub fn compile_and_link_shader(vert_src: &str, frag_src: &str) -> u32 {
    let (mut vertex_shader, mut fragment_shader) = (0, 0);
    let vert_shader_src = load_shader_from_file(vert_src.into());
    let frag_shader_src = load_shader_from_file(frag_src.into());
    let mut shader_program = 0;

    unsafe {
        vertex_shader = gl::CreateShader(VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, &vert_shader_src.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);
        check_shader_status(vertex_shader, COMPILE_STATUS);

        fragment_shader = gl::CreateShader(FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader, 1, &frag_shader_src.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);
        check_shader_status(fragment_shader, COMPILE_STATUS);

        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        //TODO: check link status.

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
        shader_program
    }
}
