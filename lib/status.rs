#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status<Machine, Result> {
    Parsing(Machine),
    Done(Result),
}

// impl<Machine, Result> Status<Machine, Result> {
//     pub fn map<U, F>(self, f: F) -> Status<U, Result>
//     where
//         F: FnOnce(Machine) -> U,
//     {
//         match self {
//             Self::Parsing(machine) => Status::Parsing(f(machine)),
//             Self::Done(result) => Status::Done(result),
//         }
//     }
// }
