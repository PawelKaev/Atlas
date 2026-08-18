use grammalang_core::social::*;

#[test]
fn test_collective_trace_basic() {
    let mut trace = CollectiveTrace::new();
    
    trace.add_participant(Participant {
        machine_id: "m1".to_string(),
        name: "Machine 1".to_string(),
        role: ParticipantRole::Leader,
        contribution: 10,
    });
    
    trace.record_event("m1", TraceEventType::NodeAdded, "node_a");
    trace.record_event("m1", TraceEventType::Synthesis, "synthesis_1");
    
    assert_eq!(trace.participant_count(), 1);
    assert_eq!(trace.event_count(), 2);
    
    println!("Basic trace: {} participants, {} events", 
             trace.participant_count(), trace.event_count());
}

#[test]
fn test_trace_merge() {
    let mut trace1 = CollectiveTrace::new();
    let mut trace2 = CollectiveTrace::new();
    
    trace1.add_participant(Participant {
        machine_id: "m1".to_string(),
        name: "Machine 1".to_string(),
        role: ParticipantRole::Leader,
        contribution: 5,
    });
    
    trace2.add_participant(Participant {
        machine_id: "m2".to_string(),
        name: "Machine 2".to_string(),
        role: ParticipantRole::Member,
        contribution: 3,
    });
    
    trace2.record_event("m2", TraceEventType::NodeAdded, "node_from_m2");
    trace2.add_genealogy("node_from_m2", vec!["parent_a".to_string()]);
    
    let merged = trace1.merge(&trace2).unwrap();
    
    assert!(merged > 0);
    assert_eq!(trace1.participant_count(), 2);
    
    println!("Merged: {} items from trace2", merged);
    println!("Total participants: {}", trace1.participant_count());
}

#[test]
fn test_trace_genealogy() {
    let mut trace = CollectiveTrace::new();
    
    trace.add_genealogy("synthesis_1", vec!["thesis".to_string(), "antithesis".to_string()]);
    trace.add_genealogy("thesis", vec!["origin".to_string()]);
    
    let history = trace.get_history("synthesis_1").unwrap();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0], "thesis");
    assert_eq!(history[1], "antithesis");
    
    println!("Genealogy of synthesis_1: {:?}", history);
}

#[test]
fn test_trace_sync() {
    let mut trace = CollectiveTrace::new();
    
    trace.start_sync();
    assert!(trace.sync_state.is_syncing);
    
    trace.record_event("m1", TraceEventType::NodeAdded, "during_sync");
    trace.sync_state.pending_changes += 1;
    
    trace.finish_sync();
    assert!(!trace.sync_state.is_syncing);
    assert_eq!(trace.sync_state.pending_changes, 1);
    
    println!("Sync completed with {} pending changes", trace.sync_state.pending_changes);
}

#[test]
fn test_trace_stats() {
    let mut trace = CollectiveTrace::new();
    
    trace.add_participant(Participant {
        machine_id: "m1".to_string(),
        name: "M1".to_string(),
        role: ParticipantRole::Leader,
        contribution: 7,
    });
    
    trace.record_event("m1", TraceEventType::NodeAdded, "node1");
    trace.record_event("m1", TraceEventType::Synthesis, "synth1");
    trace.add_genealogy("synth1", vec!["node1".to_string()]);
    
    let stats = trace.stats();
    
    assert_eq!(stats.participants, 1);
    assert_eq!(stats.events, 2);
    assert_eq!(stats.genealogy_entries, 1);
    
    println!("Stats: {} participants, {} events, {} genealogy entries", 
             stats.participants, stats.events, stats.genealogy_entries);
}

#[test]
fn test_find_events_by_type() {
    let mut trace = CollectiveTrace::new();
    
    trace.record_event("m1", TraceEventType::NodeAdded, "node1");
    trace.record_event("m1", TraceEventType::Synthesis, "synth1");
    trace.record_event("m2", TraceEventType::NodeAdded, "node2");
    
    let node_events = trace.find_events(&TraceEventType::NodeAdded);
    assert_eq!(node_events.len(), 2);
    
    let synthesis_events = trace.find_events(&TraceEventType::Synthesis);
    assert_eq!(synthesis_events.len(), 1);
    
    println!("Found {} node events, {} synthesis events", 
             node_events.len(), synthesis_events.len());
}

#[test]
fn test_protocol_mismatch() {
    let mut trace1 = CollectiveTrace::new();
    let mut trace2 = CollectiveTrace::new();
    trace2.protocol_version = "0.7.0".to_string();
    
    let result = trace1.merge(&trace2);
    assert!(result.is_err());
    
    println!("Protocol mismatch detected correctly");
}
