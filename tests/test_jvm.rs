use jni::{InitArgsBuilder, JavaVM, JNIVersion};
use jni::errors::StartJvmError;
use jni::objects::JValue;
use jni::sys::jint;

#[test]
fn test_jvm() -> Result<(), StartJvmError> {
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
    let x = JValue::from(-10);
    let val: jint = env.call_static_method("java/lang/Math", "abs", "(I)I", &[x])?.i()?;

    assert_eq!(val, 10);
    Ok(())
}