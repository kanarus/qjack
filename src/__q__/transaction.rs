use crate::{__feature__, Error, pool, q};


pub struct X<'t>(
    pub(crate) &'t mut sqlx::Transaction<'static, __feature__::DB>
);

// #[allow(non_camel_case_types)]
// pub struct qX<'x>(
//     &'x mut X
// );
// impl<'x> std::ops::Index<&'x mut X> for q {
//     type Output = &'x mut X;
//     fn index(&self, index: &'x mut X) -> &Self::Output {
//         todo!()        
//     }
// }
// impl<'x> std::ops::IndexMut<&'x mut X> for q {
//     fn index_mut(&mut self, index: &'x mut X) -> &mut Self::Output {
//         
//     }
// }

pub enum TransactionResult {
    Commit,
    Rollback,
}




#[cfg(test)]
mod __ {
    use super::X;

    const _: () = {
        impl<'x> FnOnce<()> for X<'x> {
            type Output = i32;
            extern "rust-call" fn call_once(self, _: ()) -> Self::Output {
                42
            }
        }
        impl<'x> FnMut<()> for X<'x> {
            extern "rust-call" fn call_mut(&mut self, _: ()) -> Self::Output {
                42
            }
        }
        impl<'x> Fn<()> for X<'x> {
            extern "rust-call" fn call(&self, _: ()) -> Self::Output {
                42
            }
        }
    };

    fn __(x: X) {
        let _ = x();
        let _ = x();
        let _ = x();
    }
}

