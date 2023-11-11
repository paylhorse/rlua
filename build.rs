fn main() {
    // Specify that you're using Lua 5.4 with LuaJIT
    println!("cargo:rustc-cfg=rlua_lua54");
    println!("cargo:rustc-cfg=rlua_luajit");

    // Specify the path to the static library within the rlua repository
    let luajit_lib_path = "libs";

    // Add it to the linker search path
    println!("cargo:rustc-link-search=native={}", luajit_lib_path);

    // Specify the library to link against
    println!("cargo:rustc-link-lib=static=luajit");
}
