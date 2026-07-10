use bevy::prelude::Resource;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObserverAnswer {
    pub observer: String,
    pub identified_player: bool,
    pub identified_selection: bool,
    pub identified_objective: bool,
    pub identified_next_command: bool,
    pub elapsed_millis: u32,
}

#[derive(Debug, Clone, Resource)]
pub struct FirstContactVisualAcceptance {
    pub questions: [&'static str; 4],
    pub observers: Vec<ObserverAnswer>,
    pub screenshot_baseline_reviewed: bool,
    pub occlusion_reviewed: bool,
    pub contrast_reviewed: bool,
}

impl Default for FirstContactVisualAcceptance {
    fn default() -> Self {
        Self {
            questions: [
                "Who am I?",
                "What is selected?",
                "Where is the objective?",
                "What command should I press next?",
            ],
            observers: Vec::new(),
            screenshot_baseline_reviewed: false,
            occlusion_reviewed: false,
            contrast_reviewed: false,
        }
    }
}

impl FirstContactVisualAcceptance {
    pub fn human_five_second_gate(&self) -> bool {
        self.observers.len() >= 3
            && self.observers.iter().take(3).all(|answer| {
                answer.identified_player
                    && answer.identified_selection
                    && answer.identified_objective
                    && answer.identified_next_command
                    && answer.elapsed_millis <= 5_000
            })
    }

    pub fn visual_review_gate(&self) -> bool {
        self.human_five_second_gate()
            && self.screenshot_baseline_reviewed
            && self.occlusion_reviewed
            && self.contrast_reviewed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acceptance_never_fabricates_human_observers() {
        let acceptance = FirstContactVisualAcceptance::default();
        assert_eq!(acceptance.questions.len(), 4);
        assert!(acceptance.observers.is_empty());
        assert!(!acceptance.human_five_second_gate());
    }
}
