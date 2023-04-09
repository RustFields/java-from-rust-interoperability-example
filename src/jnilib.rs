use jni::errors::StartJvmError;
use jni::{InitArgsBuilder, JavaVM, JNIVersion};
use jni::objects::JValue;
use jni::sys::jint;

pub fn abs(i: i32) -> Result<jint, StartJvmError> {
    // Build the VM properties
    let jvm_args = InitArgsBuilder::new()
        // Pass the JNI API version (default is 8)
        .version(JNIVersion::V8)
        // You can additionally pass any JVM options (standard, like a system property,
        // or VM-specific).
        // Here we enable some extra JNI checks useful during development
        .option("-Xcheck:jni")
        .build()
        .unwrap();
// Create a new VM
    let jvm = JavaVM::new(jvm_args)?;

// Attach the current thread to call into Java — see extra options in
// "Attaching Native Threads" section.
//
// This method returns the guard that will detach the current thread when dropped,
// also freeing any local references created in it
    let mut env = jvm.attach_current_thread()?;
// Call Java Math#abs(-10)
    let x = JValue::from(i);
    let val: jint = env.call_static_method("java/lang/Math", "abs", "(I)I", &[x])?.i()?;
    Ok(val)
}

#[cfg(test)]
mod tests{
    use crate::jnilib::abs;
    #[test]
    fn test_abs() {
        assert_eq!(abs(-10).unwrap(), 10);
    }
}
