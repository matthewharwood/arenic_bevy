#[cfg(test)]
mod tests {
    // Explicit imports to avoid issues with super::*
    use crate::recording::{
        RecordingCountdown, RecordingMode, RecordingState, StopReason, TransitionReason,
        RecordingCommand, RecordingStateChanged, process_recording_commands
    };
    use bevy::app::App;
    use bevy::prelude::*;
    use std::time::Duration;

    #[test]
    fn test_recording_countdown() {
        let mut countdown = RecordingCountdown::new(Duration::from_secs(3));

        assert_eq!(countdown.get_display_number(), Some(3));

        countdown.tick(Duration::from_millis(1500));
        assert_eq!(countdown.get_display_number(), Some(2));

        countdown.tick(Duration::from_secs(1));
        assert_eq!(countdown.get_display_number(), Some(1));

        assert!(!countdown.tick(Duration::from_millis(400))); // Not done yet
        assert!(countdown.tick(Duration::from_millis(200)));  // Now it's done
    }

    #[test]
    fn test_recording_mode_display() {
        assert_eq!(RecordingMode::Idle.to_string(), "Idle");
        assert_eq!(RecordingMode::Countdown.to_string(), "Countdown");
        assert_eq!(RecordingMode::Recording.to_string(), "Recording");
        assert_eq!(RecordingMode::DialogPause.to_string(), "Dialog Pause");
    }

    #[test]
    fn test_recording_state_default() {
        let state = RecordingState::default();
        assert_eq!(state.mode, RecordingMode::Idle);
        assert!(state.recording_entity.is_none());
    }

    #[test]
    fn test_stop_reason_variants() {
        // Ensure all variants are covered
        let reasons = vec![
            StopReason::UserInterrupted,
            StopReason::TimeComplete,
            StopReason::ArenaTransition,
            StopReason::CharacterSwitch,
        ];

        // Test that Debug is implemented
        for reason in &reasons {
            let debug_str = format!("{:?}", reason);
            assert!(!debug_str.is_empty());
        }
    }

    #[test]
    fn test_transition_reason_completeness() {
        // Create a dummy entity for the test
        let dummy_entity = Entity::from_raw(0);
        
        // Ensure all transition reasons are handled
        let reasons = vec![
            TransitionReason::StartRequest(dummy_entity),
            TransitionReason::CountdownComplete,
            TransitionReason::UserInterrupted,
            TransitionReason::TimeComplete,
            TransitionReason::ArenaTransition,
            TransitionReason::CharacterSwitch,
            TransitionReason::DialogOpened,
            TransitionReason::DialogClosed,
        ];

        for reason in &reasons {
            let debug_str = format!("{:?}", reason);
            assert!(!debug_str.is_empty());
        }
    }
    
    #[test]
    fn test_command_driven_state_transitions() {
        let mut app = App::new();
        app.init_resource::<RecordingState>();
        app.add_event::<RecordingCommand>();
        app.add_event::<RecordingStateChanged>();
        app.add_systems(Update, process_recording_commands);
        
        // Create a real entity for testing
        let test_entity = app.world_mut().spawn_empty().id();
        
        // Send start recording command
        app.world_mut().send_event(RecordingCommand::StartRecording {
            entity: test_entity,
        });
        
        // Process the command
        app.update();
        
        // Verify state changed to countdown
        let state = app.world().resource::<RecordingState>();
        assert_eq!(state.mode, RecordingMode::Countdown);
        assert_eq!(state.recording_entity, Some(test_entity));
        
        // Test invalid command (start recording when already in countdown)
        app.world_mut().send_event(RecordingCommand::StartRecording {
            entity: test_entity,
        });
        
        app.update();
        
        // State should remain unchanged (invalid transition rejected)
        let state = app.world().resource::<RecordingState>();
        assert_eq!(state.mode, RecordingMode::Countdown);
    }
    
    #[test] 
    fn test_command_stop_recording_transitions() {
        let mut app = App::new();
        app.init_resource::<RecordingState>();
        app.add_event::<RecordingCommand>();
        app.add_event::<RecordingStateChanged>();
        app.add_systems(Update, process_recording_commands);
        
        // Create a real entity for testing
        let test_entity = app.world_mut().spawn_empty().id();
        
        // Manually set state to recording mode
        {
            let mut state = app.world_mut().resource_mut::<RecordingState>();
            state.mode = RecordingMode::Recording;
            state.recording_entity = Some(test_entity);
        }
        
        // Send stop recording command
        app.world_mut().send_event(RecordingCommand::StopRecording {
            reason: StopReason::UserInterrupted,
        });
        
        app.update();
        
        // Verify state changed back to idle
        let state = app.world().resource::<RecordingState>();
        assert_eq!(state.mode, RecordingMode::Idle);
        assert!(state.recording_entity.is_none());
    }

    #[test]
    fn test_command_pause_resume_cycle() {
        let mut app = App::new();
        app.init_resource::<RecordingState>();
        app.add_event::<RecordingCommand>();
        app.add_event::<RecordingStateChanged>();
        app.add_systems(Update, process_recording_commands);
        
        // Manually set state to recording mode
        {
            let mut state = app.world_mut().resource_mut::<RecordingState>();
            state.mode = RecordingMode::Recording;
        }
        
        // Send pause command
        app.world_mut().send_event(RecordingCommand::PauseForDialog);
        app.update();
        
        // Verify state changed to dialog pause
        let state = app.world().resource::<RecordingState>();
        assert_eq!(state.mode, RecordingMode::DialogPause);
        
        // Send resume command
        app.world_mut().send_event(RecordingCommand::ResumeFromDialog);
        app.update();
        
        // Verify state changed back to recording
        let state = app.world().resource::<RecordingState>();
        assert_eq!(state.mode, RecordingMode::Recording);
    }
}