use jni::JNIEnv;
use jni::objects::JClass;

#[no_mangle]
pub extern "system" fn Java_gg_mixtape_natives_connector_ConnectorDebugLibrary_configureLogging(_: JNIEnv, _: JClass) {
    env_logger::init();
}
