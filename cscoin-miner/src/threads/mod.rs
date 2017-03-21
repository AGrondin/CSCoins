
use std::ops::Deref;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

pub mod worker;
use self::worker::Worker;

//-----------------------------------------------------------------------------
// structs and enums
//-----------------------------------------------------------------------------

//The assignment given to the worker threads
#[derive(Clone)]
pub enum ThreadAssignment {
    Stop,
    //Last solution hash, hash prefix, nb_elements
    SortedList(String, String, u64),
    ReverseSortedList(String, String, u64),
    //Last solution hash, hash prefix, grid_size, nb_blockers
    ShortestPath(String, String, u64, u64)
}


//Worker manager

pub struct ThreadManager {
    num_threads:      u64,
    challenge_handle: Arc<Mutex<ThreadAssignment>>,
    threads:          Vec<JoinHandle<()>>,
    main_rx:          Receiver<String>, //Used to receive the solution (nonce) from any thread that has one
    main_tx:          Sender<String>    //The sender that will be cloned and given to the threads
}

impl ThreadManager {

    pub fn new(num_threads: u64) -> ThreadManager {
        assert!(num_threads >= 1, "Must have at least one worker thread!");

        let (main_tx, main_rx) = channel();

        ThreadManager {
            num_threads:      num_threads,
            challenge_handle: Arc::new(Mutex::new(ThreadAssignment::Stop)),
            threads:          Vec::new(),
            main_rx:          main_rx,
            main_tx:          main_tx
        }
    }

    pub fn setup(&mut self, first_challenge: ThreadAssignment) {

        self.challenge_handle = Arc::new(Mutex::new(first_challenge));

        for i in 1..self.num_threads {
            let thread_tx = Arc::new(Mutex::new(self.main_tx.clone()));
            let challenge = self.challenge_handle.clone();

            self.threads.push(thread::spawn(move || {
                Worker::new(thread_tx, challenge).do_work(true);
            }));
        }

    }

    pub fn set_new_assignment(&mut self, assignment: ThreadAssignment) {
        let assignment_mutex = self.challenge_handle.clone();
        *assignment_mutex.lock().unwrap() = assignment;
    }

    //Returns a solution if the threads found one
    //None otherwise
    //fn get_solution() -> Option<> {
    //
    //}

}
