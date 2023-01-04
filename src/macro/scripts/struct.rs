use core::str::FromStr;
use fixed_queue::VecDeque;
use heapless::String;

#[derive(Debug)]
pub struct Script {
    _lines_queue: VecDeque<String<255>, 1024>,
}

impl Script {
    pub fn new(action_text: &str) -> Self {
        let mut queue: VecDeque<String<255>, 1024> = VecDeque::new();
        let mut iter = action_text.split("\n");
        loop {
            if let Some(_line) = iter.next() {
                queue.push_back(String::from_str(_line).unwrap()).unwrap();
            } else {
                break;
            }
        }
        Script {
            _lines_queue: queue,
        }
    }
    pub fn lines_queue(&self) -> &VecDeque<String<255>, 1024> {
        let _queue = &self._lines_queue;
        return _queue;
    }
}
