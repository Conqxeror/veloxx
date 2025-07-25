use veloxx::audit::AuditTrail;

#[test]
fn test_audit_trail_log() {
    // Test that log function doesn't panic
    AuditTrail::log("Test audit event");
    AuditTrail::log("Another test event");

    // Since log just prints to stdout, we can't easily test the output
    // but we can ensure it doesn't crash - test passes if we reach this point
}

#[test]
fn test_audit_trail_log_empty_string() {
    AuditTrail::log("");
    // Test passes if we reach this point without panicking
}

#[test]
fn test_audit_trail_log_special_characters() {
    AuditTrail::log("Test with special chars: !@#$%^&*()");
    AuditTrail::log("Test with unicode: 🦀 Rust is awesome! 🚀");
    // Test passes if we reach this point without panicking
}

#[test]
fn test_audit_trail_log_multiline() {
    AuditTrail::log("Line 1\nLine 2\nLine 3");
    // Test passes if we reach this point without panicking
}
