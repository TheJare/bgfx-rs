// Copyright (c) 2015-2016, Johan Sköld.
// License: http://opensource.org/licenses/ISC

use std::env;
use std::io::{Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
    let target = env::var("TARGET").unwrap();
    let profile = env::var("PROFILE").unwrap();

    let first_div = target.find('-').unwrap();
    let last_div = target.rfind('-').unwrap();

    let arch = &target[..first_div];
    let platform = &target[(first_div + 1)..last_div];
    let compiler = &target[(last_div + 1)..];
    let bitness = if arch == "x86_64" { 64 } else { 32 };

    match compiler {
        "msvc" => build_msvc(bitness),
        "gnu" | "darwin" => build_gmake(bitness, &profile, platform),
        _ => panic!("Unsupported compiler"),
    }
}

/// Builds the bgfx binaries for `msvc` targets.
fn build_msvc(bitness: u32) {
    let vs_version = env::var("VisualStudioVersion").expect("Visual Studio version not detected");
    let platform = if bitness == 32 { "X86" } else { "X64" };

    let vs_release = match vs_version.as_ref() {
        "12.0" => "2013",
        "14.0" => "2015",
        "15.0" => "2017",
        _ => panic!(format!("Unknown Visual Studio version: {:?}", vs_version)),
    };

    Command::new("bx/tools/bin/windows/genie.exe")
        .current_dir("bgfx")
        .arg("--with-dynamic-runtime")
        .arg("--with-tools")
        // .arg(format!("--with-windows={}",
        //              env::var("WindowsSDKVersion")
        //                  .unwrap_or("8.1".to_string())
        //                  .replace("\\", "")))
        .arg(format!("vs{}", vs_release))
        .output()
        .expect("Failed to generate project files");

    let status = Command::new("MSBuild.exe")
        .current_dir("bgfx")
        .arg("/p:Configuration=Release")
        .arg(format!("/p:Platform={}", platform))
        .arg(format!(".build/projects/vs{}/bgfx.vcxproj", vs_release))
        .status()
        .expect("Failed to build bgfx");

    if status.code().unwrap() != 0 {
        panic!("Failed to build bgfx");
    }

    let status = Command::new("MSBuild.exe")
        .current_dir("bgfx")
        .arg("/p:Configuration=Release")
        .arg(format!("/p:Platform={}", platform))
        .arg(format!(".build/projects/vs{}/bx.vcxproj", vs_release))
        .status()
        .expect("Failed to build bx");

    if status.code().unwrap() != 0 {
        panic!("Failed to build bx");
    }

    let status = Command::new("MSBuild.exe")
        .current_dir("bgfx")
        .arg("/p:Configuration=Release")
        .arg(format!("/p:Platform={}", platform))
        .arg(format!(".build/projects/vs{}/bimg.vcxproj", vs_release))
        .status()
        .expect("Failed to build bimg");

    if status.code().unwrap() != 0 {
        panic!("Failed to build bimg");
    }

    // Build tools
    let status = Command::new("devenv.exe")
        .current_dir("bgfx")
        .arg(format!(".build/projects/vs{}/bgfx.sln", vs_release))
        .arg("/Build")
        .arg(format!("Release|x{}", bitness))
        .status()
        .expect("Failed to build tools");

    if status.code().unwrap() != 0 {
        panic!("Failed to build tools");
    }

    Command::new("cmd")
        .current_dir("bgfx")
        .arg("/C")
        .arg(format!("copy .build\\win{}_vs{}\\bin\\geometrycRelease.exe tools\\bin\\windows\\geometryc.exe", bitness, vs_release))
        .status().ok()
        .and_then(|s| s.code())
        .and_then(|s| if s == 0 {Some(0)} else {None})
        .expect("Failed to copy geometryc.exe");

    Command::new("cmd")
        .current_dir("bgfx")
        .arg("/C")
        .arg(format!("copy .build\\win{}_vs{}\\bin\\shadercRelease.exe tools\\bin\\windows\\shaderc.exe", bitness, vs_release))
        .status().ok()
        .and_then(|s| s.code())
        .and_then(|s| if s == 0 {Some(0)} else {None})
        .expect("Failed to copy shaderc.exe");

    Command::new("cmd")
        .current_dir("bgfx")
        .arg("/C")
        .arg(format!("copy .build\\win{}_vs{}\\bin\\texturecRelease.exe tools\\bin\\windows\\texturec.exe", bitness, vs_release))
        .status().ok()
        .and_then(|s| s.code())
        .and_then(|s| if s == 0 {Some(0)} else {None})
        .expect("Failed to copy texturec.exe");

    Command::new("cmd")
        .current_dir("bgfx")
        .arg("/C")
        .arg(format!("copy .build\\win{}_vs{}\\bin\\texturevRelease.exe tools\\bin\\windows\\texturev.exe", bitness, vs_release))
        .status().ok()
        .and_then(|s| s.code())
        .and_then(|s| if s == 0 {Some(0)} else {None})
        .expect("Failed to copy texturev.exe");

    let mut path = PathBuf::from(env::current_dir().unwrap());
    path.push("bgfx");
    path.push(".build");
    path.push(format!("win{}_vs{}", bitness, vs_release));
    path.push("bin");

    println!("cargo:rustc-link-lib=static=bgfxRelease");
    println!("cargo:rustc-link-lib=static=bxRelease");
    println!("cargo:rustc-link-lib=static=bimgRelease");
    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=psapi");
    println!("cargo:rustc-link-search=native={}", path.as_os_str().to_str().unwrap());
}

/// Builds the bgfx binaries for makefile based targets.
fn build_gmake(bitness: u32, profile: &str, platform: &str) {
    let project_name = match platform {
        "pc-windows" => "gmake-mingw-gcc",
        "unknown-linux" => "gmake-linux",
        "apple" => "gmake-osx",
        _ => panic!("Unsupported OS"),
    };

    let output_name = match platform {
        "pc-windows" => format!("win{}_mingw-gcc", bitness),
        "unknown-linux" => format!("linux{}_gcc", bitness),
        "apple" => format!("osx{}_clang", bitness),
        _ => unreachable!(),
    };

    // Generate makefiles
    let status = Command::new("make")
        .arg("-C")
        .arg("bgfx")
        .arg(format!(".build/projects/{}", project_name))
        .status()
        .expect("Failed to generate makefiles");

    if status.code().unwrap() != 0 {
        panic!("Failed to generate makefiles.");
    }

    // C flags
    let cflags = if platform == "pc-windows" && bitness == 32 {
        "-fPIC -DBGFX_CONFIG_MULTITHREADED=1 -mincoming-stack-boundary=2"
    } else {
        "-fPIC -DBGFX_CONFIG_MULTITHREADED=1"
    };

    // Build bgfx
    let status = Command::new("make")
        .env("CFLAGS", cflags)
        .arg("-R")
        .arg("-C")
        .arg(format!("bgfx/.build/projects/{}", project_name))
        .arg(format!("config={}{}", profile, bitness))
        .arg("verbose=1")
        .arg("bgfx")
        .arg("bx")
        .arg("bimg")
        .status()
        .expect("Failed to build bgfx");

    if status.code().unwrap() != 0 {
        panic!("Failed to build bgfx.");
    }

    // Generate tools
    let status = Command::new("make")
        .arg("-C")
        .arg("bgfx")
        .arg("tools")
        .status()
        .expect("Failed to generate tools");

    if status.code().unwrap() != 0 {
        panic!("Failed to generate tools.");
    }

    // Output linker config
    let mut path = PathBuf::from(env::current_dir().unwrap());
    path.push("bgfx");
    path.push(".build");
    path.push(output_name);
    path.push("bin");

    let config = if profile == "debug" {
        "Debug"
    } else {
        "Release"
    };
    println!("cargo:rustc-link-lib=bgfx{}", config);
    println!("cargo:rustc-link-lib=bx{}", config);
    println!("cargo:rustc-link-lib=bimg{}", config);
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-search=native={}", path.as_os_str().to_str().unwrap());

    match platform {
        "pc-windows" => {
            println!("cargo:rustc-link-lib=gdi32");
            println!("cargo:rustc-link-lib=opengl32");
            println!("cargo:rustc-link-lib=psapi");
        }
        "unknown-linux" => {
            println!("cargo:rustc-link-lib=GL");
            println!("cargo:rustc-link-lib=X11");
        }
        "apple" => {
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=QuartzCore");
            println!("cargo:rustc-link-lib=framework=OpenGL");

            if should_link_metal() {
                println!("cargo:rustc-link-lib=framework=Metal");
            }
        }
        _ => unreachable!(),
    }
}

/// Determines whether we should link with Metal on OSX. The Metal framework
/// is only included starting with OSX 10.11. We do this through the C
/// compiler so we can test the same macro bgfx tests for support with.
fn should_link_metal() -> bool {
    let test = "#ifdef __ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__\nv=__ENVIRONMENT_MAC_OS_X_VER\
                SION_MIN_REQUIRED__\n#else\nv=1\n#endif";

    let mut cc = Command::new("cc")
        .arg("-xc")
        .arg("-E")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let mut stdin = cc.stdin.take().unwrap();
        stdin.write_fmt(format_args!("{}", test)).unwrap();
    }

    let output = cc.wait_with_output().unwrap();
    let output_str = String::from_utf8(output.stdout).unwrap();
    let ver_line = output_str
        .lines()
        .find(|l| l.starts_with("v="))
        .unwrap();
    let ver_str = &ver_line[2..];
    let ver = ver_str.parse::<u32>().unwrap();

    ver >= 101100
}