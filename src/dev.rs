// #[derive(Debug, thiserror::Error)]
// pub enum OneError {
//     One(String)
// }
// impl std::fmt::Display for OneError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         todo!()
//     }
// }
// pub async fn one() -> Result<(), OneError> {
//     Err(OneError::One(String::from("one")))
// }
// #[derive(Debug, thiserror::Error)]
// pub enum TwoError {
//     Two(String)
// }
// impl std::fmt::Display for TwoError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         todo!()
//     }
// }
// pub async fn two() -> Result<(), TwoError> {
//     Err(TwoError::Two(String::from("two")))
// }
// #[derive(Debug, thiserror::Error)]
// pub enum ThreeError {
//     One {
//         #[source]
//         one: OneError,
//     },
//     Two {
//         #[source]
//         two: TwoError,
//     },
//     OneTwo {
//         #[source]
//         one: OneError,
//         // #[source] //error: duplicate #[source] attribute - и что с этим делать?
//         two: TwoError,
//     }
// }
// impl std::fmt::Display for ThreeError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         todo!()
//     }
// }
// pub async fn three() -> Result<(), ThreeError> {
//     match futures::join!(
//         one(),
//         two()
//     ) {
//         (Ok(_), Ok(_)) => Ok(()),
//         (Err(one), Ok(_)) => Err(ThreeError::One {
//             one,
//         }),
//         (Ok(_), Err(two)) => Err(ThreeError::Two {
//             two,
//         }),
//         (Err(one), Err(two)) => Err(ThreeError::OneTwo {
//             //что с этим делать?
//             one,
//             two,
//         }),
//     }
// }
// // pub trait Error: Debug + Display {
// //     fn source(&self) -> Option<&(dyn Error + 'static)> { ... }
// //     fn description(&self) -> &str { ... }
// //     fn cause(&self) -> Option<&dyn Error> { ... }
// //     fn provide<'a>(&'a self, demand: &mut Demand<'a>) { ... }
// // }

pub fn dev() {}
