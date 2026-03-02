# Aspects definition for the Disinfo-Nesy-Detector
#
# This Nickel configuration defines the "Aspects" — cross-cutting concerns 
# like messaging and monitoring — for the neurosymbolic detector.

{
  aspects = {
    # JETSTREAM: Configuration for NATS-based message orchestration.
    jetstream = {
      message_handling = {
        ack = true,           # Ensure delivery confirmation.
        data_access = "Data()", # Formal accessor for payload bytes.
        log = true,           # Trace individual message transitions.
      },
    },
    # PROMETHEUS: Telemetry hooks for operational visibility.
    prometheus = {
      metrics = {
        latency = true,
        errors = true,
        messages_processed = true,
      },
    },
  },
  
  # HOOKS: Event-driven logic injected into the message processing pipeline.
  hooks = {
    pre_process = "log.Printf('Pre-processing message: %v', msg.Subject())",
    post_process = "log.Printf('Post-processing message: %v', msg.Subject())",
  },
}
