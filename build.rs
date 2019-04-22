extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system libwim
    // shared library.
    println!("cargo:rustc-link-lib=wim");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // whitelist functions
        // list of functions that should be included
        .whitelist_function("wimlib_add_empty_image")
        .whitelist_function("wimlib_add_image")
        .whitelist_function("wimlib_add_image_multisource")
        .whitelist_function("wimlib_add_tree")
        .whitelist_function("wimlib_compress")
        .whitelist_function("wimlib_create_compressor")
        .whitelist_function("wimlib_create_decompressor")
        .whitelist_function("wimlib_create_new_wim")
        .whitelist_function("wimlib_decompress")
        .whitelist_function("wimlib_delete_image")
        .whitelist_function("wimlib_delete_path")
        .whitelist_function("wimlib_export_image")
        .whitelist_function("wimlib_extract_image")
        .whitelist_function("wimlib_extract_image_from_pipe")
        .whitelist_function("wimlib_extract_image_from_pipe_with_progress")
        .whitelist_function("wimlib_extract_pathlist")
        .whitelist_function("wimlib_extract_paths")
        .whitelist_function("wimlib_extract_xml_data")
        .whitelist_function("wimlib_free")
        .whitelist_function("wimlib_free_compressor")
        .whitelist_function("wimlib_free_decompressor")
        .whitelist_function("wimlib_get_compression_type_string")
        .whitelist_function("wimlib_get_compressor_needed_memory")
        .whitelist_function("wimlib_get_error_string")
        .whitelist_function("wimlib_get_image_description")
        .whitelist_function("wimlib_get_image_name")
        .whitelist_function("wimlib_get_image_property")
        .whitelist_function("wimlib_get_version")
        .whitelist_function("wimlib_get_version_string")
        .whitelist_function("wimlib_get_wim_info")
        .whitelist_function("wimlib_get_xml_data")
        .whitelist_function("wimlib_global_cleanup")
        .whitelist_function("wimlib_global_init")
        .whitelist_function("wimlib_image_name_in_use")
        .whitelist_function("wimlib_iterate_dir_tree")
        .whitelist_function("wimlib_iterate_lookup_table")
        .whitelist_function("wimlib_join")
        .whitelist_function("wimlib_join_with_progress")
        .whitelist_function("wimlib_mount_image")
        .whitelist_function("wimlib_open_wim")
        .whitelist_function("wimlib_open_wim_with_progress")
        .whitelist_function("wimlib_overwrite")
        .whitelist_function("wimlib_print_available_images")
        .whitelist_function("wimlib_print_header")
        .whitelist_function("wimlib_reference_resource_files")
        .whitelist_function("wimlib_reference_resources")
        .whitelist_function("wimlib_reference_template_image")
        .whitelist_function("wimlib_register_progress_function")
        .whitelist_function("wimlib_rename_path")
        .whitelist_function("wimlib_resolve_image")
        .whitelist_function("wimlib_set_default_compression_level")
        .whitelist_function("wimlib_set_error_file")
        .whitelist_function("wimlib_set_error_file_by_name")
        .whitelist_function("wimlib_set_image_descripton")
        .whitelist_function("wimlib_set_image_flags")
        .whitelist_function("wimlib_set_image_name")
        .whitelist_function("wimlib_set_image_property")
        .whitelist_function("wimlib_set_memory_allocator")
        .whitelist_function("wimlib_set_output_chunk_size")
        .whitelist_function("wimlib_set_output_compression_type")
        .whitelist_function("wimlib_set_output_pack_chunk_size")
        .whitelist_function("wimlib_set_output_pack_compression_type")
        .whitelist_function("wimlib_set_print_errors")
        .whitelist_function("wimlib_set_wim_info")
        .whitelist_function("wimlib_split")
        .whitelist_function("wimlib_unmount_image")
        .whitelist_function("wimlib_unmount_image_with_progress")
        .whitelist_function("wimlib_update_image")
        .whitelist_function("wimlib_verify_wim")
        .whitelist_function("wimlib_write")
        .whitelist_function("wimlib_write_to_fd")
        // nocopy for handles
        .no_copy("WIMStruct")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
