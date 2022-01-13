pub struct State {
    pub input: String,
    pub selected_idx: usize,
    pub entries: Option<Vec<String>>,
}

impl State {
    pub fn default() -> Self {
        Self {
            input: "".to_string(),
            selected_idx: 0,
            entries: None,
        }
    }
}

pub enum Action {
    EntriesLoaded(Vec<String>),

    SelectedIndexIncreased,
    SelectedIndexDecreased,
}

/// Mutates the state of the application to simulate the given action. This
/// function should be the only function to ever mutate the State struct.
pub fn process_action(state: &mut State, action: Action) {
    match action {
        Action::EntriesLoaded(entries) => entries_loaded(state, entries),
        Action::SelectedIndexDecreased => selected_index_decreased(state),
        Action::SelectedIndexIncreased => selected_index_increased(state),
    };
}

fn entries_loaded(state: &mut State, entries: Vec<String>) {
    state.entries = Some(entries);
    state.selected_idx = 0;
}

fn selected_index_decreased(state: &mut State) {
    state.selected_idx = if state.entries.is_some() && state.selected_idx > 0 {
        state.selected_idx - 1
    } else {
        0
    }
}

fn selected_index_increased(state: &mut State) {
    state.selected_idx = if let Some(entries) = &state.entries {
        if state.selected_idx < entries.len() - 1 {
            state.selected_idx + 1
        } else {
            entries.len() - 1
        }
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn state_with_entries(num_entries: usize) -> State {
        let mut state = State::default();

        let entries: Vec<String> = vec![
            "main",
            "feat/my-feature",
            "bug/my-bug",
            "feat/another-feature",
        ]
        .iter()
        .map(|s| s.to_string())
        .take(num_entries)
        .collect();
        process_action(&mut state, Action::EntriesLoaded(entries));

        state
    }

    mod loading_entries {
        use super::*;

        #[test]
        pub fn when_empty_entries_are_loaded_the_entries_should_be_empty() {
            let mut state = State::default();
            process_action(&mut state, Action::EntriesLoaded(vec![]));
            assert_eq!(state.entries, Some(vec![]));
        }

        #[test]
        pub fn when_entries_are_loaded_the_entries_should_be_updated() {
            let mut state = State::default();
            let entries: Vec<String> = vec!["main", "feat/my-feature"]
                .iter()
                .map(|s| s.to_string())
                .collect();

            process_action(&mut state, Action::EntriesLoaded(entries.clone()));
            assert_eq!(state.entries, Some(entries));
        }
    }

    mod selected_index {
        use super::*;

        #[test]
        pub fn when_the_selected_index_is_increased_but_there_are_no_entries_it_should_be_zero() {
            let mut state = State::default();

            process_action(&mut state, Action::SelectedIndexIncreased);
            assert_eq!(state.selected_idx, 0);
        }

        #[test]
        pub fn when_the_selected_index_is_decreased_but_there_are_no_entries_it_should_be_zero() {
            let mut state = State::default();

            process_action(&mut state, Action::SelectedIndexDecreased);
            assert_eq!(state.selected_idx, 0);
        }

        #[test]
        pub fn when_the_selected_index_is_increased_it_should_increase() {
            let mut state = state_with_entries(3);

            process_action(&mut state, Action::SelectedIndexIncreased);
            assert_eq!(state.selected_idx, 1);
        }

        #[test]
        pub fn when_the_selected_index_is_decreased_it_should_decrease() {
            let mut state = state_with_entries(3);

            process_action(&mut state, Action::SelectedIndexIncreased);
            process_action(&mut state, Action::SelectedIndexDecreased);
            assert_eq!(state.selected_idx, 0);
        }

        #[test]
        pub fn when_the_selected_index_is_decreased_below_zero_it_should_remain_zero() {
            let mut state = state_with_entries(3);

            process_action(&mut state, Action::SelectedIndexIncreased);
            process_action(&mut state, Action::SelectedIndexDecreased);
            process_action(&mut state, Action::SelectedIndexDecreased);
            assert_eq!(state.selected_idx, 0);
        }

        #[test]
        pub fn when_the_selected_index_is_increased_beyond_the_entries_length_it_shouldnt_increase()
        {
            let mut state = state_with_entries(3);

            process_action(&mut state, Action::SelectedIndexIncreased);
            process_action(&mut state, Action::SelectedIndexIncreased);
            process_action(&mut state, Action::SelectedIndexIncreased);
            assert_eq!(state.selected_idx, 2);
        }
    }
}
