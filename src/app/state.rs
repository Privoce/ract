#[derive(Debug, Clone, Copy)]
pub enum ComponentState<S: State> {
    Start,
    Run(S),
    Pause,
    Quit,
}

#[allow(unused)]
impl<S: State> ComponentState<S> {
    pub fn is_start(&self) -> bool {
        matches!(self, ComponentState::Start)
    }
    pub fn is_run(&self) -> bool {
        matches!(self, ComponentState::Run(_))
    }
    pub fn is_pause(&self) -> bool {
        matches!(self, ComponentState::Pause)
    }
    pub fn is_quit(&self) -> bool {
        matches!(self, ComponentState::Quit)
    }
    pub fn quit(&mut self) {
        *self = ComponentState::Quit;
    }
    pub fn to_pause(&mut self) {
        *self = ComponentState::Pause;
    }
}

impl<S: State> State for ComponentState<S> {
    fn next(&mut self) -> () {
        match self {
            ComponentState::Start => {
                *self = ComponentState::Run(S::default());
            }
            ComponentState::Run(r) => {
                if r.is_run_end() {
                    *self = ComponentState::Pause;
                } else {
                    let _ = r.next();
                    *self = ComponentState::Run(*r);
                }
            }
            ComponentState::Pause => {
                *self = ComponentState::Quit;
            }
            ComponentState::Quit => {}
        }
    }

    fn is_run_end(&self) -> bool {
        if let ComponentState::Run(r) = self {
            r.is_run_end()
        } else {
            false
        }
    }

    fn to_run_end(&mut self) -> () {
        if let ComponentState::Run(r) = self {
            let _ = r.to_run_end();
            *self = ComponentState::Run(*r);
        }
    }
}

impl<S: State> Default for ComponentState<S> {
    fn default() -> Self {
        Self::Start
    }
}

pub trait State: Default + Clone + Copy {
    fn next(&mut self) -> ();
    /// ## Check is current state is end of the run state
    fn is_run_end(&self) -> bool;
    fn to_run_end(&mut self) -> ();
}


#[derive(Clone, Copy, Debug, Default)]
pub enum BaseRunState{
    #[default]
    Running
}

impl State for BaseRunState {
    fn next(&mut self) -> () {
        ()
    }

    fn is_run_end(&self) -> bool {
        true
    }

    fn to_run_end(&mut self) -> () {
        ()
    }
}