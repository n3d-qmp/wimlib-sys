#[cfg(test)]

use sha2::{Sha256};
use sha2::digest::Digest;
use crate::*;
use std::ptr::null_mut;
use lazy_static::lazy_static;
use std::env::temp_dir;
use std::ffi::CString;
use std::path::PathBuf;
use std::str::FromStr;
use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::copy;

static CAT_IMAGE_FILE_NAME: &str = "cat.jpg";
static DOGS_IMAGE_FILE_NAME: &str = "dogs.jpg";
static WIM_IMAGE_NAME: &str = "I love cute animals";
static TMP_FILE_NAME: &str = "TEST.WIM";
lazy_static! {
    static ref TMP_DIR: String = {
        let mut tmp_path_buf = temp_dir();
        tmp_path_buf.push("wimlib-sys-test");
        tmp_path_buf.to_string_lossy().to_string()
    };

    static ref TMP_FILE_PATH: String = {
        let mut tmp_path_buf = PathBuf::from_str(&TMP_DIR).unwrap();
        tmp_path_buf.push(TMP_FILE_NAME);
        tmp_path_buf.to_string_lossy().to_string()
    };
}

#[test]
fn make_wim() {
    create_dir_all(TMP_DIR.to_string()).unwrap();

    unsafe {
        // create wim
        let wim_struct_ptr_box = Box::new(null_mut());
        let wim_struct_ptr_box_ptr = Box::into_raw(wim_struct_ptr_box);
        let create_res = wimlib_create_new_wim(wimlib_compression_type_WIMLIB_COMPRESSION_TYPE_LZMS, wim_struct_ptr_box_ptr);
        if create_res != 0 {
            panic!("wimlib_create_new_wim failed: error {}", create_res);
        }
        let wim_struct_ptr_box = Box::from_raw(wim_struct_ptr_box_ptr);
        let wim_struct_ptr = *wim_struct_ptr_box;

        // add new empty image to wim
        let mut wim_img_idx: i32 = 0;
        let wim_img_name = CString::new(WIM_IMAGE_NAME).unwrap();
        let add_image_res = wimlib_add_empty_image(wim_struct_ptr, wim_img_name.as_ptr() as *const i8, &mut wim_img_idx);
        if add_image_res != 0 {
            panic!("wimlib_add_empty_image failed: error {}", add_image_res);
        }

        // add cat image to the newly empty image
        let add_tree_src_path = CString::new(CAT_IMAGE_FILE_NAME).unwrap();
        let add_tree_dst_path = CString::new(CAT_IMAGE_FILE_NAME).unwrap();
        let add_tree_res = wimlib_add_tree(wim_struct_ptr, wim_img_idx, add_tree_src_path.as_ptr() as *const i8, add_tree_dst_path.as_ptr() as *const i8, 0);
        if add_tree_res != 0 {
            panic!("wimlib_add_tree failed: error {}", add_tree_res);
        }

        // add dog image
        let add_tree_src_path = CString::new(DOGS_IMAGE_FILE_NAME).unwrap();
        let add_tree_dst_path = CString::new(DOGS_IMAGE_FILE_NAME).unwrap();
        let add_tree_res = wimlib_add_tree(wim_struct_ptr, wim_img_idx, add_tree_src_path.as_ptr() as *const i8, add_tree_dst_path.as_ptr() as *const i8, 0);
        if add_tree_res != 0 {
            panic!("wimlib_add_tree failed: error {}", add_tree_res);
        }

        // write wim to TMP_FILE_PATH
        let write_dst_path = CString::new(TMP_FILE_PATH.to_string()).unwrap();
        let write_res = wimlib_write(wim_struct_ptr, write_dst_path.as_ptr() as *const i8, -1, 0, 0);
        if write_res != 0 {
            panic!("wimlib_write failed: error {}", write_res);
        }
    }
}

#[test]
fn unpack_wim_and_check_animal_image_hash() {
    unsafe {
        // read and hash cat and dogs images
        let mut file = File::open(CAT_IMAGE_FILE_NAME).unwrap();
        let mut hasher = Sha256::new();
        copy(&mut file, &mut hasher).unwrap();
        let cat_img_hash = hasher.result();
        let mut file = File::open(DOGS_IMAGE_FILE_NAME).unwrap();
        let mut hasher = Sha256::new();
        copy(&mut file, &mut hasher).unwrap();
        let dogs_img_hash = hasher.result();

        // open wim
        let wim_path = CString::new(TMP_FILE_PATH.to_string()).unwrap();
        let wim_struct_ptr_box = Box::new(null_mut());
        let wim_struct_ptr_box_ptr = Box::into_raw(wim_struct_ptr_box);
        let open_res = wimlib_open_wim(wim_path.as_ptr() as *const i8, 0, wim_struct_ptr_box_ptr);
        if open_res != 0 {
            panic!("wimlib_open_wim failed: error {}", open_res);
        }
        let wim_struct_ptr_box = Box::from_raw(wim_struct_ptr_box_ptr);
        let wim_struct_ptr = *wim_struct_ptr_box;

        // retrieve image index by name
        let wim_img_name = CString::new(WIM_IMAGE_NAME).unwrap();
        let wim_img_idx = wimlib_resolve_image(wim_struct_ptr, wim_img_name.as_ptr() as *const i8);
        if wim_img_idx < 1 {
            panic!("wimlib_resolve_image failed: invalid image number {}", wim_img_idx);
        }

        // unpack image
        let extract_target_path = CString::new(TMP_DIR.to_string()).unwrap();
        let cat_image_path = CString::new(CAT_IMAGE_FILE_NAME).unwrap();
        let dogs_image_path = CString::new(DOGS_IMAGE_FILE_NAME).unwrap();
        let extract_file_names = [cat_image_path.as_ptr(), dogs_image_path.as_ptr()];
        let extract_res = wimlib_extract_paths(wim_struct_ptr, wim_img_idx, extract_target_path.as_ptr() as *const i8, extract_file_names.as_ptr(), extract_file_names.len(), 0);
        if extract_res < 0 {
            panic!("wimlib_extract_paths failed: error {}", extract_res);
        }

        // read and hash unpacked cat and dogs images
        let mut unpacked_cat_img_path = PathBuf::from_str(&TMP_DIR).unwrap();
        unpacked_cat_img_path.push(CAT_IMAGE_FILE_NAME);
        let mut file = File::open(unpacked_cat_img_path).unwrap();
        let mut hasher = Sha256::new();
        copy(&mut file, &mut hasher).unwrap();
        let cat_img_unpacked_hash = hasher.result();

        let mut unpacked_dogs_img_path = PathBuf::from_str(&TMP_DIR).unwrap();
        unpacked_dogs_img_path.push(DOGS_IMAGE_FILE_NAME);
        let mut file = File::open(unpacked_dogs_img_path).unwrap();
        let mut hasher = Sha256::new();
        copy(&mut file, &mut hasher).unwrap();
        let dogs_img_unpacked_hash = hasher.result();

        // compare hashes
        assert_eq!(cat_img_hash, cat_img_unpacked_hash);
        assert_eq!(dogs_img_hash, dogs_img_unpacked_hash);
    }
}