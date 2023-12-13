// use crate::log::Log;

#[derive(PartialEq, Clone)]
pub enum Errors {
    MissingRequiredComponent,
    DuplicateComponents,
    FailedToSerialize,
}

// pub fn serialize_error(error: &Errors) -> String {
//     match error {
//         Errors::FailedToSerialize => "Failed to serialze".to_string(),
//         Errors::DuplicateComponents => "Duplicate components".to_string(),
//         Errors::MissingRequiredComponent => "Missing required component".to_string(),
//     }
// }

// pub struct ErrorManager {
//     stop_duplicates: bool,
//     pub error: Option<Errors>,
// }

// impl ErrorManager {
//     pub fn new() -> Self {
//         Self {
//             stop_duplicates: true,
//             error: None,
//         }
//     }
//     pub fn send_error(&mut self, error: Errors) {
//         if let Some(err) = &self.error {
//             if self.stop_duplicates && err == &error {
//                 return;
//             }
//         }
//         self.error = Some(error.clone());
//         Log::critical(&serialize_error(&error));
//     }
// }
