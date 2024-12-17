use std::path::{Path, PathBuf};

use gen_utils::common::fs::FileState;
use notify::{event::ModifyKind, EventKind};

/// # 文件事件跟踪器
/// 在不同的操作系统上使用的文件监视器的RecommendedWatcher是不同的
/// 例如在Windows上文件的创建为Create,删除为Remove,修改为Modify
/// 而在Macos上当文件被创建时会先触发Create(File)再触发Modify(Metadata(Extended))
/// 而删除文件则触发Create(File) -> Modify(Name(Any)) -> Modify(Metadata(Extended))，
/// 而对文件重命名则会触发Create(File) -> Modify(Name(Any)) -> Modify(Metadata(Extended)) -> Modify(Name(Any))
/// 所以这里我这里打算使用一个状态机来跟踪处理文件的变化
/// 由于在Mac上文件的变化会触发多个事件，所以需要一个状态机来跟踪文件的变化
#[derive(Debug)]
pub struct FileEventTracker {
    pub path: Option<PathBuf>,
    /// Record the files that need to be compiled and their status
    pub states: Vec<EventKind>,
}

impl FileEventTracker {
    pub fn new() -> Self {
        Self {
            path: None,
            states: Vec::new(),
        }
    }

    pub fn set_path<P>(&mut self, path: P)
    where
        P: AsRef<Path>,
    {
        let flag = if let Some(self_path) = self.path.as_ref() {
            self_path == path.as_ref()
        } else {
            true
        };

        if !flag {
            self.flesh();
        }

        self.path = Some(path.as_ref().to_path_buf());
    }
    // pub fn new<P>(path: P) -> Self
    // where
    //     P: AsRef<Path>,
    // {
    //     Self {
    //         path: path.as_ref().to_path_buf(),
    //         states: Vec::new(),
    //     }
    // }

    /// ## 更新文件状态
    pub fn insert(&mut self, ekind: EventKind) {
        if self.path.is_some() {
            if let EventKind::Modify(ModifyKind::Metadata(_)) = ekind {
                // do nothing
            } else {
                self.states.push(ekind);
            }
        }
    }

    pub fn state(&self) -> Option<FileState> {
        if self.path.is_none() {
            return None;
        }

        if self.states.len() == 2 {
            if modify_name(self.states[0]) && modify_data(self.states[1]) {
                return Some(FileState::Deleted);
            }
            if modify_name(self.states[0]) && modify_name(self.states[1]) {
                return Some(FileState::Renamed);
            }
        } else if self.states.len() == 1 {
            if create(self.states[0]) {
                return Some(FileState::Created);
            }

            if modify_data(self.states[0]) {
                return Some(FileState::Modified);
            }

            if modify_name(self.states[0]) {
                return Some(FileState::Renamed);
            }
        }

        return None;
    }

    pub fn flesh(&mut self) {
        self.path = None;
        self.states.clear();
    }
}

fn modify_data(kind: EventKind) -> bool {
    matches!(kind, EventKind::Modify(ModifyKind::Data(_)))
}

fn modify_name(kind: EventKind) -> bool {
    matches!(kind, EventKind::Modify(ModifyKind::Name(_)))
}

fn create(kind: EventKind) -> bool {
    matches!(kind, EventKind::Create(_))
}
