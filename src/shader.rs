// See LICENSE file for copyright and license details.

use std::iter;
use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;
use cgmath::{Matrix4};
use gl;
use gl::types::{GLuint, GLint, GLenum, GLchar};
use zgl::{Zgl};
use core_types::{ZInt};
use visualizer_types::{ZFloat, Color4, ColorId, MatId, AttrId};

fn get_attr_location(program_id: GLuint, zgl: &Zgl, name: &str) -> AttrId {
    let name_c = CString::from_slice(name.as_bytes()).as_slice_with_nul().as_ptr();
    let attr_id = unsafe {
        zgl.gl.GetAttribLocation(program_id, name_c)
    };
    zgl.check();
    assert!(attr_id >= 0);
    AttrId{id: attr_id as GLuint}
}

pub struct Shader {
    program_id: GLuint,
    position_attr_id: AttrId,
}

impl Shader {
    pub fn new(zgl: &Zgl, vs_src: &str, fs_src: &str) -> Shader {
        let vs = compile_shader(zgl, vs_src, gl::VERTEX_SHADER);
        let fs = compile_shader(zgl, fs_src, gl::FRAGMENT_SHADER);
        let program_id = link_program(zgl, vs, fs);
        let position_attr_id = get_attr_location(program_id, zgl, "position");
        zgl.enable_vertex_attrib_array(&position_attr_id);
        Shader {
            program_id: program_id,
            position_attr_id: position_attr_id,
        }
    }

    // TODO: add some arg to spec what attr user needs
    pub fn get_position_attr_id(&self) -> AttrId {
        self.position_attr_id.clone()
    }

    pub fn enable_attr(&self, zgl: &Zgl, attr_id: &AttrId, components_count: ZInt) {
        let is_normalized = gl::FALSE;
        let stride = 0;
        unsafe {
            zgl.gl.VertexAttribPointer(
                attr_id.id,
                components_count,
                gl::FLOAT,
                is_normalized,
                stride,
                ptr::null_mut(),
            );
        }
        zgl.check();
    }

    pub fn activate(&self, zgl: &Zgl) {
        unsafe {
            zgl.gl.UseProgram(self.program_id);
        }
        zgl.check();
    }

    pub fn set_uniform_mat4f(&self, zgl: &Zgl, mat_id: &MatId, mat: &Matrix4<ZFloat>) {
        let count = 1;
        let transpose = gl::FALSE;
        unsafe {
            let data_ptr = mem::transmute(mat);
            zgl.gl.UniformMatrix4fv(
                mat_id.id as ZInt, count, transpose, data_ptr);
        }
        zgl.check();
    }

    pub fn set_uniform_color(&self, zgl: &Zgl, color_id: &ColorId, color: &Color4) {
        unsafe {
            let data_ptr = mem::transmute(color);
            zgl.gl.Uniform4fv(color_id.id as ZInt, 1, data_ptr);
        }
        zgl.check();
    }

    pub fn get_uniform_color(&self, zgl: &Zgl, name: &str) -> ColorId {
        let id = self.get_uniform(zgl, name);
        ColorId{id: id as GLuint}
    }

    pub fn get_uniform_mat(&self, zgl: &Zgl, name: &str) -> MatId {
        let id = self.get_uniform(zgl, name);
        MatId{id: id as GLuint}
    }

    fn get_uniform(&self, zgl: &Zgl, name: &str) -> GLuint {
        let name_c = CString::from_slice(name.as_bytes()).as_slice_with_nul().as_ptr();
        let id = unsafe {
            zgl.gl.GetUniformLocation(self.program_id, name_c) as GLuint
        };
        assert!(id != -1);
        zgl.check();
        id
    }
}

fn compile_shader(zgl: &Zgl, src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = zgl.gl.CreateShader(ty);
        zgl.check();
        let src_c = CString::from_slice(src.as_bytes())
            .as_slice_with_nul().as_ptr();
        zgl.gl.ShaderSource(shader, 1, &src_c, ptr::null());
        zgl.check();
        zgl.gl.CompileShader(shader);
        zgl.check();
        let mut status = gl::FALSE as GLint;
        zgl.gl.GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        if status != gl::TRUE as GLint {
            let mut len = 0;
            zgl.gl.GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut err_log = String::with_capacity(len as uint);
            err_log.extend(iter::repeat('\0').take(len as uint));
            let raw_ptr = err_log.as_slice().as_ptr() as *mut GLchar;
            zgl.gl.GetShaderInfoLog(shader, len, &mut len, raw_ptr);
            err_log.truncate(len as uint);
            panic!("{}", err_log);
        }
    }
    shader
}

fn link_program(zgl: &Zgl, vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = zgl.gl.CreateProgram();
        zgl.check();
        zgl.gl.AttachShader(program, vs);
        zgl.check();
        zgl.gl.AttachShader(program, fs);
        zgl.check();
        zgl.gl.LinkProgram(program);
        zgl.check();
        zgl.gl.DeleteShader(vs);
        zgl.check();
        zgl.gl.DeleteShader(fs);
        zgl.check();
        zgl.gl.UseProgram(program);
        zgl.check();
        zgl.gl.DeleteProgram(program); // mark for deletion
        zgl.check();
        let mut status = gl::FALSE as GLint;
        zgl.gl.GetProgramiv(program, gl::LINK_STATUS, &mut status);
        if status != gl::TRUE as GLint {
            let mut len = 0;
            zgl.gl.GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut err_log = String::with_capacity(len as uint);
            err_log.extend(iter::repeat('\0').take(len as uint));
            let raw_ptr = err_log.as_slice().as_ptr() as *mut GLchar;
            zgl.gl.GetProgramInfoLog(program, len, &mut len, raw_ptr);
            err_log.truncate(len as uint);
            panic!("{}", err_log);
        }
        program
    }
}

// vim: set tabstop=4 shiftwidth=4 softtabstop=4 expandtab:
