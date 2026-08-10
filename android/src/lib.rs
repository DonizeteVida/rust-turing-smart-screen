use jni::objects::JClass;
use jni::sys::jstring;
use jni::Env;

#[unsafe(no_mangle)]
pub extern "C" fn Java_com_turing_smart_screen_MainActivity_helloworld(
    mut env: Env,
    _class: JClass,
) -> jstring {
    env
    .new_string("Hello World")
    .expect("Failed to create string")
    .into_raw()
}
