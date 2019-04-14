use stdweb::web::{
    File,
    FileReader,
    FileReaderReadyState,
    FileReaderResult,
    IEventTarget,
    event::LoadEndEvent,
};

use yew::{
    services::{
        ReaderService,
        Task,
    },
    callback::Callback,
};

pub trait ReaderServiceExt {
    fn read_file_as_dataurl(&mut self, file: &File, callback: Callback<String>) -> ReaderTask;
}

impl ReaderServiceExt for ReaderService {
    fn read_file_as_dataurl(&mut self, file: &File, callback: Callback<String>) -> ReaderTask {
        let file_reader = FileReader::new();
        let reader = file_reader.clone();
        file_reader.add_event_listener(move |_event: LoadEndEvent| match reader.result() {
            Some(FileReaderResult::String(dataurl)) => callback.emit(dataurl),
            Some(FileReaderResult::ArrayBuffer(_)) => unreachable!(),
            None => (),
        });
        file_reader.read_as_dataurl(file);
        ReaderTask(file_reader)
    }
}

trait FileReaderExt {
    fn read_as_dataurl(&self, file: &File);
}

impl FileReaderExt for FileReader {
    fn read_as_dataurl(&self, file: &File) {
        js! {
            var reader = @{self};
            reader.readAsDataURL(@{file});
        };
    }
}

#[must_use]
pub struct ReaderTask(FileReader);

impl Task for ReaderTask {
    fn is_active(&self) -> bool {
        self.0.ready_state() == FileReaderReadyState::Loading
    }

    fn cancel(&mut self) {
        self.0.abort();
    }
}

impl Drop for ReaderTask {
    fn drop(&mut self) {
        if self.is_active() {
            self.cancel();
        }
    }
}
