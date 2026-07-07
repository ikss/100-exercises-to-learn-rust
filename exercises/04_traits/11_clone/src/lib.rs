// TODO: add the necessary `Clone` implementations (and invocations)
//  to get the code to compile.

impl Clone for Ticket {
    fn clone(&self) -> Self {
        let newTitle = self.title.clone();
        let newDescription = self.description.clone();
        let newStatus = self.status.clone();

        Ticket {
            title: newTitle,
            description: newDescription,
            status: newStatus,
        }
    }
}

pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    (ticket.clone(), ticket.summary())
}

pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

pub struct Summary {
    pub title: String,
    pub status: String,
}
