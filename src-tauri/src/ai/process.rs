//! `codex app-server` as a long-lived child process speaking JSON lines on stdio.
use super::{AppServer, Connection};
use serde_json::Value;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{channel, Receiver};

pub struct CodexCli;

impl AppServer for CodexCli {
    fn spawn(&mut self) -> Result<Box<dyn Connection>, ()> {
        // stderr is discarded: it may carry account details.
        let mut child = Command::new("codex")
            .arg("app-server")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| ())?;
        let stdin = child.stdin.take().ok_or(())?;
        let stdout = child.stdout.take().ok_or(())?;
        let (tx, rx) = channel();
        std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                let Ok(line) = line else { break };
                // Lines that are not JSON are ignored, never logged.
                if let Ok(message) = serde_json::from_str::<Value>(&line) {
                    if tx.send(message).is_err() {
                        break;
                    }
                }
            }
        });
        Ok(Box::new(Process {
            child,
            stdin,
            messages: rx,
        }))
    }
}

struct Process {
    child: Child,
    stdin: ChildStdin,
    messages: Receiver<Value>,
}

impl Connection for Process {
    fn send(&mut self, message: &Value) -> Result<(), ()> {
        writeln!(self.stdin, "{message}").map_err(|_| ())?;
        self.stdin.flush().map_err(|_| ())
    }

    fn receive(&mut self) -> Vec<Value> {
        self.messages.try_iter().collect()
    }

    fn alive(&mut self) -> bool {
        matches!(self.child.try_wait(), Ok(None))
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
