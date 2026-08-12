use jni::objects::{JClass, JList, JString};
use jni::{EnvUnowned, jni_sig, jni_str};

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_turing_smart_screen_TuringSmartScreen_helloWorld<'local>(
    mut unowned_env: EnvUnowned<'local>,
    _class: JClass<'local>,
) -> JString<'local> {
    unowned_env
        .with_env(|env| -> jni::errors::Result<_> { JString::from_str(env, "Hello from Rust") })
        .resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_turing_smart_screen_TuringSmartScreen_getSerialDevices<'local>(
    mut unowned_env: EnvUnowned<'local>,
    _class: JClass<'local>,
) -> JList<'local> {
    unowned_env
        .with_env(|env| -> jni::errors::Result<_> {
            let object = env.new_object(jni_str!("java/util/ArrayList"), jni_sig!("()V"), &[])?;

            let list = JList::cast_local(env, object)?;
            let stub_device = JString::from_str(env, "Stub device")?;

            list.add(env, &stub_device)?;

            Ok(list)
        })
        .resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}
