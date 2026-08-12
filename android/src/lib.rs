use jni::EnvUnowned;
use jni::errors::ThrowRuntimeExAndDefault;
use jni::objects::{JClass, JString};

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_turing_smart_screen_TuringSmartScreen_helloWorld<'local>(
    mut unowned_env: EnvUnowned<'local>,
    _class: JClass<'local>,
) -> JString<'local> {
    unowned_env
        .with_env(|env| -> jni::errors::Result<JString<'local>> {
            JString::from_str(env, "Hello from Rust a")
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}
