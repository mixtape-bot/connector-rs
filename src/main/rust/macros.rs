// A JNI call that does not check for exceptions or verify
// error codes (if any).
macro_rules! jni_unchecked {
    ( $jnienv:expr, $name:tt $(, $args:expr )* ) => ({
        unsafe {
            jni_method!($jnienv, $name)($jnienv, $($args),*)
        }
    })
}

macro_rules! jni_method {
    ( $jnienv:expr, $name:tt ) => {{
        let env = $jnienv;
        match deref!(deref!(env, "JNIEnv"), "*JNIEnv").$name {
            Some(method) => {
                method
            }
            None => {
                return Err(jni::errors::Error::JNIEnvMethodNotFound(stringify!(
                    $name
                )));
            }
        }
    }};
}

macro_rules! deref {
    ( $obj:expr, $ctx:expr ) => {
        if $obj.is_null() {
            return Err(jni::errors::Error::NullDeref($ctx));
        } else {
            #[allow(unused_unsafe)]
            unsafe {
                *$obj
            }
        }
    };
}

macro_rules! to_ptr {
    ($obj:expr) => {
        Box::into_raw(Box::new($obj))
    };
}

macro_rules! from_ptr {
    ($ptr:expr) => {
        unsafe { &mut *$ptr }
    };
}
