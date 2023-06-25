use crate::configuration::Config;
use super::code_package::CodePackage;

pub fn generate(config: &Config) -> CodePackage {
  let lib_executable_name = config.get_ios_lib_executable_name();
  let rust_file_name = &config.rust_file_name;
  let rust_dir = &config.rust_dir;
  let ios_dir = &config.ios_dir;
  let ios_dir_name = &config.get_ios_dir_name();
  let bridge_header_name = &config.get_bridge_h_file_name();
  let ios_libs_dir = &config.get_ios_libs_directory_path();

  let code = format!("#!/bin/sh

cd {rust_dir}
cargo lipo --release
cbindgen src/lib.rs -l c > {rust_file_name}.h

# mkdir {ios_dir}/include
mkdir {ios_libs_dir}
cp {rust_file_name}.h {ios_dir}/{ios_dir_name}/{bridge_header_name}
cp target/universal/release/{lib_executable_name} {ios_libs_dir}
");

  CodePackage { dir_rel_path: config.output_dir.clone(), file_name: "install.sh".to_string(), code }
}
