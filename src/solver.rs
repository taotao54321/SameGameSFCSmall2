use log::info;

use crate::action::ActionHistory;
use crate::board::Board;
use crate::cmp::chmax;
use crate::hash::U64HashMap;
use crate::position::Position;
use crate::score::{Score, SCORE_PERFECT};

type DpTable = U64HashMap<Position, Score>;

/// 最大スコア探索用ソルバー。複数の面を連続で解ける。
#[derive(Debug)]
pub struct Solver {
    /// 探索時の枝刈り用スコア閾値。
    /// 最終スコアがこの値を超えないと判明した時点でそのノードは枝刈りする。
    /// ただし終了局面については一応解を記録する。
    prune_score_max: Score,

    /// 各局面から追加で獲得しうるスコアの上界を記録する DP テーブル。
    /// メモリ効率は若干悪いが、スコア閾値を適切に設定すればメモリ不足になることはないはず。
    dp: DpTable,
}

impl Solver {
    /// 枝刈り用スコア閾値を `prune_score_max` としてソルバーを作る。
    pub fn new(prune_score_max: Score) -> Self {
        Self {
            prune_score_max,
            dp: DpTable::default(),
        }
    }

    /// 現時点での枝刈り用スコア閾値を返す。
    pub fn prune_score_max(&self) -> Score {
        self.prune_score_max
    }

    /// 枝刈り用スコア閾値を設定する。
    pub fn set_prune_score_max(&mut self, prune_score_max: Score) {
        self.prune_score_max = prune_score_max;
    }

    /// 枝刈り用スコア閾値を chmax する。
    pub fn chmax_prune_score_max(&mut self, score: Score) {
        chmax!(self.prune_score_max, score);
    }

    /// 与えられた盤面に対する最大スコアを探索する。
    pub fn solve(&mut self, board: Board) -> Option<(Score, ActionHistory)> {
        let sub_solver = SubSolver::new(self.prune_score_max, &mut self.dp);
        let res = sub_solver.solve(board);

        // 次の面に備え、DP テーブルをクリア。
        info!("DP entry count: {}", self.dp.len());
        self.dp.clear();

        res
    }
}

#[derive(Debug)]
struct SubSolver<'solver> {
    prune_score_max: Score,

    best_score: Score,
    best_solution: Option<ActionHistory>,
    history: ActionHistory,

    dp: &'solver mut DpTable,
}

impl<'solver> SubSolver<'solver> {
    fn new(prune_score_max: Score, dp: &'solver mut DpTable) -> Self {
        Self {
            prune_score_max,

            best_score: 0,
            best_solution: None,
            history: ActionHistory::new(),

            dp,
        }
    }

    fn solve(mut self, board: Board) -> Option<(Score, ActionHistory)> {
        // 前回の面を解いた後、DP テーブルはクリアされているはず。
        debug_assert!(self.dp.is_empty());

        let pos = Position::new(board);
        self.dfs(&pos, 0);

        self.best_solution
            .map(|solution| (self.best_score, solution))
    }

    /// 現スコアが `score` である局面 `pos` から追加で獲得しうるスコアの上界を返す。
    fn dfs(&mut self, pos: &Position, score: Score) -> Score {
        // pos が終了局面ならば解の更新処理を行い、追加の獲得スコアを返す。
        if let Some(gain) = final_gain(pos) {
            if chmax!(self.best_score, score + gain) {
                info!("Found {}: {}", self.best_score, self.history);
                self.best_solution.replace(self.history.clone());
            }
            return gain;
        }

        // pos から追加で獲得しうるスコアについて現時点で最良の上界を得る。
        // DP テーブルにエントリがあるならその値を使う。
        // さもなくば探索せずにわかる範囲で見積もり、DP テーブルにその値を記録する。
        let gain_ub = *self
            .dp
            .entry(pos.clone())
            .or_insert_with(|| pos.gain_upper_bound());

        // 最終スコアが prune_score_max を超えないなら枝刈り。
        if score + gain_ub <= self.prune_score_max {
            return gain_ub;
        }

        // 最終スコアが prune_score_max を超えうるなら、全ての子ノードを探索して追加スコア上界を更新。
        let mut gain_ub = 0;
        for action in pos.actions() {
            unsafe { self.history.push_unchecked(action.least_square()) }

            let pos_child = pos.do_action(&action);
            let gain_action = action.gain();
            let gain_ub_child = self.dfs(&pos_child, score + gain_action);
            chmax!(gain_ub, gain_action + gain_ub_child);

            unsafe { self.history.remove_last_unchecked() }
        }

        // 新たな追加スコア上界を DP テーブルに記録してから返す。
        // ここでは必ず DP テーブルにエントリがあるはず。
        // (NOTE: 所有権の都合上、DP テーブルエントリを 2 回探すことになるが、速度的には問題ない)
        *self.dp.get_mut(pos).unwrap() = gain_ub;
        gain_ub
    }
}

/// `pos` が終了局面ならば追加の獲得スコア (`SCORE_PERFECT` または 0) を返す。
fn final_gain(pos: &Position) -> Option<Score> {
    (!pos.has_action()).then(|| {
        if pos.board().is_empty() {
            SCORE_PERFECT
        } else {
            0
        }
    })
}
