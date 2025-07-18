#include "cel-cxx-ffi/include/absl.h"
#include "cel-cxx-ffi/src/absl.rs.h"
#include "absl/log/log_sink_registry.h"

namespace rust::cel_cxx {

static bool g_log_bridged = false;

void SetLogCallback() {
    if (g_log_bridged) return;  // Prevent duplicate setup
    
    static RustLogSink rust_sink;
    absl::AddLogSink(&rust_sink);
    g_log_bridged = true;
}

void RustLogSink::Send(const absl::LogEntry& entry) {
    log_callback(entry);
}

} // namespace rust::cel_cxx 
