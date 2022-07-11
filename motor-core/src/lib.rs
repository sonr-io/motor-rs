use prs::account::{CreateAccountRequest, CreateAccountResponse};
use rayon::{prelude::*, /*ThreadPoolBuilder,*/ ThreadPoolBuildError};

pub fn create_account(
    _req: CreateAccountRequest,
) -> Result<CreateAccountResponse, ThreadPoolBuildError> {
    // let pool = ThreadPoolBuilder::new()
    //     .num_threads(3)
    //     // .spawn_handler(|thread| {
    //     //     let mut b = std::thread::Builder::new();
    //     //     if let Some(name) = thread.name() {
    //     //         b = b.name(name.to_owned());
    //     //     }
    //     //     if let Some(stack_size) = thread.stack_size() {
    //     //         b = b.stack_size(stack_size);
    //     //     }
    //     //     b.spawn(|| thread.run())?;
    //     //     Ok(())
    //     // })
    //     .build()?;

    // pool.install(|| long_task(1));
    // pool.install(|| long_task(2));
    // pool.install(|| long_task(3));
    let test: [u32; 100_000] = [1; 100_000];
    // for i in 0..1_000 {
    //     test[i] = i as u32;
    // }
    let val: u32 = test.par_iter().map(|&i| i * i).sum();

    Ok(CreateAccountResponse::default())
}

// fn long_task(num: i32) {
//     for _ in 0..1_000_000 {}
//     println!("done {}.", num);
// }
