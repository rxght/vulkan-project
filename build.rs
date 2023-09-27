use std::{fs, process::Command, path::Path};

const GLSLANG_PATH: &str = "C:\\VulkanSDK\\1.3.250.1\\Bin\\glslangValidator.exe";


fn main()
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