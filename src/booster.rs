use crate::admin::AdminHandle;
use tokio::sync::{mpsc, oneshot};

// ##################################################### //
// ################### ACTOR BACKEND ################### //
// ##################################################### //

struct Booster {
    receiver: mpsc::Receiver<BoosterMessage>,
    admin: AdminHandle,
}

#[derive(Debug)]
enum BoosterMessage {
    GradeBOOST { reply_to: oneshot::Sender<()> },
}

impl Booster {
    fn new(receiver: mpsc::Receiver<BoosterMessage>, admin: AdminHandle) -> Self {
        Self { receiver, admin }
    }

    async fn handle_message(&mut self, msg: BoosterMessage) {
        println!(
            "[Actor] Booster is running handle_message() with new BoosterMessage: {:?}",
            msg
        );
        match msg {
            BoosterMessage::GradeBOOST { reply_to } => {
                let current_grades = self.admin.get_all_student_grades().await;

                let boosted_grades: Vec<f64> = current_grades
                    .into_iter()
                    .map(|grade| grade + 60.0)
                    .collect();

                self.admin.submit_student_grades(boosted_grades).await;
                let _ = reply_to.send(());
            }
        };
    }
}

// ###################################################### //
// ################### ACTOR FRONTEND ################### //
// ###################################################### //

async fn run_booster_actor(mut actor: Booster) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg).await;
    }
}

#[derive(Clone, Debug)]
pub struct BoosterHandle {
    sender: mpsc::Sender<BoosterMessage>,
}

impl BoosterHandle {
    pub async fn new(admin_handle: AdminHandle) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = Booster::new(receiver, admin_handle);
        tokio::spawn(run_booster_actor(actor));
        Self { sender }
    }

    pub async fn grade_boost(&self) {
        let (tx, rx) = oneshot::channel();
        let _ = self
            .sender
            .send(BoosterMessage::GradeBOOST { reply_to: tx })
            .await;
        let _ = rx.await;
    }
}
