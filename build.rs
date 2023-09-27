use std::{fs::{self, File}, process::Command, path::Path, io::Write, ffi::OsStr};

const GLSLANG_PATH: &str = "C:\\VulkanSDK\\1.3.250.1\\Bin\\glslangValidator.exe";

fn main()
{
    println!("cargo:rerun-if-changed=shaders/src/");
    compile_shaders();
    generate_shader_modules();
}

fn compile_shaders()
{
    let out_folder: &Path = Path::new("./shaders/");
    let source_folder: &Path = Path::new("./shaders/src/");

    let shader_sources = fs::read_dir(source_folder).unwrap();
    for shader_source in shader_sources
    {
        match shader_source
        {
            Err(_) => continue,
            Ok(file) => {

                let out_file_path = out_folder.join(file.file_name().to_str().unwrap());
                let file_path = file.path();

                let in_file = file_path.to_str().unwrap();
                let out_file = out_file_path.to_str().unwrap();


                let output = if cfg!(target_os = "windows") {
                    Command::new(GLSLANG_PATH)
                        .args(["-V", "-o", out_file, in_file])
                        .output()
                        .expect("Failed to compile a shader")
                } else {
                    Command::new("glslangValidator")
                        .args(["-V", "-o", out_file, in_file])
                        .output()
                        .expect("Failed to compile a shader")
                };

                println!("{output:?}");
            }
        }
    }
}

fn generate_shader_modules()
{
    let module_file = "./src/graphics/shaders.rs";
    let out_folder = "./shaders";

    let mut module_file_handle = File::create(module_file).unwrap();
    write!(module_file_handle,
"\
/* ------------------------*/
/*   AUTO GENERATED FILE   */
/* ------------------------*/
"
    ).unwrap();

    let shader_sources = fs::read_dir(out_folder).unwrap();
    for shader_source in shader_sources
    {
        match shader_source
        {
            Err(_) => panic!(),
            Ok(file) => {
                if file.file_type().unwrap().is_dir() { continue; }

                let _file_name = file.file_name();
                let file_name = _file_name.to_str().unwrap();
                println!("file: {file_name:?}");

                let _path = file.path();
                let stem = _path.file_stem().and_then(OsStr::to_str).unwrap();
                let ext = _path.extension().and_then(OsStr::to_str).unwrap();
                let shader_type = match ext {
                    "vert" => "vertex",
                    "frag" => "fragment",
                    _ => "unknown"
                };

                write!(module_file_handle,
"
#[allow(non_snake_case)]
pub mod {ext}_{stem}
{{
    vulkano_shaders::shader! {{
        ty: \"{shader_type}\",
        bytes: \"{out_folder}/{file_name}\",
    }}
}}
"
                ).unwrap()
            }
        }
    }
}