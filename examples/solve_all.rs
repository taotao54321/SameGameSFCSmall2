use clap::Parser;
use log::info;

use samegame_sfc_small_2::*;

/// ゲーム内に現れうる全ての面の中での最大スコアを求める。
#[derive(Debug, Parser)]
struct Cli {
    /// 最終スコアがこの値を超えないとわかったノードを枝刈りする。
    /// 1 つの面を解き終えるたびに最大スコアで chmax される。
    #[arg(long, default_value_t = 0)]
    prune_score_max: Score,
}

fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let cli = Cli::parse();

    let mut solver = Solver::new(cli.prune_score_max);
    for (param, board, rng_after) in enumerate_all_legal_board() {
        let RandomBoardParam {
            rng_state,
            nmi_counter,
            nmi_timing,
            entropy,
        } = param;

        info!(
            "Search: rng_state=0x{rng_state:04X} nmi_counter=0x{nmi_counter:02X} nmi_timing={nmi_timing} entropy={entropy} rng_after=0x{:04X}",
            rng_after.state()
        );

        if let Some((score, solution)) = solver.solve(board) {
            println!("0x{rng_state:04X}\t0x{nmi_counter:02X}\t{nmi_timing}\t{entropy}\t{score}\t{solution}");
            // 同点の解は全て列挙したいので -1 する。
            solver.chmax_prune_score_max(score.saturating_sub(1));
        }
    }

    Ok(())
}
