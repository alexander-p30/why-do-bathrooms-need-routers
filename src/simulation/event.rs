use std::sync::mpsc::Sender;
use uuid::Uuid;
//
// Person events
pub const EV_NEW_PERSON: &str = "new_person";
pub const EV_PERSON_JOINED_THE_QUEUE: &str = "person_joined_the_queue";
pub const EV_PERSON_ENTERED_THE_BATHROOM: &str = "person_entered_the_bathroom";
pub const EV_PERSON_FINISHED_USING_BATHROOM: &str = "person_finished_using_bathroom";
pub const EV_PERSON_LEFT_THE_BATHROOM: &str = "person_left_the_bathroom";
// Bathroom events
pub const EV_NEW_BATHROOM: &str = "new_bathroom";
pub const EV_BATHROOM_SWITCHED_GENDERS: &str = "bathroom_switched_genders";

#[derive(Clone, Debug)]
pub struct Event {
    pub name: String,
    pub producer_id: Uuid,
    pub destination_id: Option<Uuid>,
    pub producer_sender: Option<Sender<Event>>,
    pub person_snapshot: Option<super::person::Person>,
    pub bathroom_snapshot: Option<super::bathroom::Bathroom>,
}

pub fn new_event(
    name: String,
    producer_id: Uuid,
    destination_id: Option<Uuid>,
    person: Option<super::person::Person>,
    bathroom: Option<super::bathroom::Bathroom>,
) -> Event {
    return Event {
        name,
        producer_id,
        destination_id,
        producer_sender: None,
        person_snapshot: person,
        bathroom_snapshot: bathroom,
    };
}

pub fn new_creation_event(
    name: String,
    producer_id: Uuid,
    destination_id: Option<Uuid>,
    producer_sender: Sender<Event>,
    person: Option<super::person::Person>,
) -> Event {
    return Event {
        name,
        producer_id,
        destination_id,
        producer_sender: Some(producer_sender),
        person_snapshot: person,
        bathroom_snapshot: None,
    };
}