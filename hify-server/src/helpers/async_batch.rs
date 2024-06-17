use std::future::Future;

use anyhow::{bail, Result};
use log::{error, warn};
use tokio::task::JoinSet;

use super::logging::progress_bar;

pub trait AsyncContextualRunner: Clone + Send + 'static {
    type Input: Send + 'static;
    type Output: Send + 'static;

    fn run(
        self,
        item: Self::Input,
        warn: impl Fn(&str) + Send,
    ) -> impl Future<Output = Result<Option<Self::Output>>> + Send;

    async fn run_for_batch(
        self,
        items: impl ExactSizeIterator<Item = Self::Input>,
    ) -> Result<Vec<Self::Output>> {
        run_batch_gen_with_progress(items, self).await
    }
}

async fn run_batch_gen_with_progress<
    I: Send + 'static,
    O: Send + 'static,
    R: AsyncContextualRunner<Input = I, Output = O>,
>(
    items: impl ExactSizeIterator<Item = I>,
    runner: R,
) -> Result<Vec<O>> {
    let mut set = JoinSet::new();

    let pb = progress_bar(items.len());

    for item in items {
        let runner = runner.clone();
        let pb = pb.clone();

        set.spawn(async move {
            let res = runner.run(item, |msg| pb.suspend(|| warn!("{msg}"))).await;
            pb.inc(1);
            res
        });
    }

    let mut output = vec![];
    let mut errors = 0;

    while let Some(res) = set.join_next().await {
        match res? {
            Ok(art) => {
                if let Some(produced) = art {
                    output.push(produced)
                }
            }

            Err(err) => {
                pb.suspend(|| error!("Error: {err}"));
                errors += 1;
            }
        }
    }

    pb.finish();

    if errors > 0 {
        bail!("Encountered {errors} error(s)");
    }

    Ok(output)
}
