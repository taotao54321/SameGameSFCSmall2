use std::path::PathBuf;

use anyhow::Context as _;
use clap::Parser;
use log::info;

use samegame_sfc_small_2::*;

/// 与えられた盤面に対する最大スコア手順を求める。
#[derive(Debug, Parser)]
struct Cli {
    /// 最終スコアがこの値を超えないとわかったノードを枝刈りする。
    #[arg(long, default_value_t = 0)]
    prune_score_max: Score,

    /// 盤面ファイル。
    path_board: PathBuf,
}

fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let cli = Cli::parse();

    let board = std::fs::read_to_string(&cli.path_board)
        .with_context(|| format!("問題ファイル '{}' を読めない", cli.path_board.display()))?;
    let board: Board = board
        .parse()
        .with_context(|| format!("問題ファイル '{}' のパースに失敗", cli.path_board.display()))?;

    let mut solver = Solver::new(cli.prune_score_max);

    if let Some((score, solution)) = solver.solve(board) {
        println!("{score}\t{solution}");
    } else {
        info!("NO SOLUTION");
    }

    Ok(())
}
