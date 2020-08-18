use fixedbitset::FixedBitSet;
use petgraph::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{DAGSearcher, Filter, Word, WordSearcher};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BoardSolver {
    letters: String,
    relations: Relations,
}

impl BoardSolver {
    pub fn from_board<B: Into<RawBoard>>(letters: &str, board: B) -> Self {
        Self {
            letters: letters.to_string(),
            relations: Relations::from(&board.into()),
        }
    }

    pub fn first_n_solutions(&self, n: usize) -> Vec<Relations> {
        // let start = std::time::Instant::now();
        let words = DAGSearcher::default().lookup(&*self.letters);
        // println!(
        //     "Took {:?} to load searcher and lookup letters: {:?}",
        //     start.elapsed(),
        //     words
        // );

        // let start = std::time::Instant::now();
        let mut cs = ConstraintSet::new(words, self.relations.clone());
        // println!("Took {:?} to construct ConstraintSet", start.elapsed());

        // let start = std::time::Instant::now();
        let solutions = cs.find_n(n);
        // println!(
        //     "Took {:?} to find {}({}) solutions",
        //     start.elapsed(),
        //     n,
        //     solutions.len()
        // );

        return solutions;
    }
}

struct ConstraintSet {
    words: HashMap<usize, (FixedBitSet, Vec<Word>)>,
    relations: Relations,
    visit_order: Vec<NodeIndex>,
}

impl ConstraintSet {
    pub fn new(words: Vec<Word>, relations: Relations) -> Self {
        // TODO: fix this so that we visit by "BFS from most constrained node" order
        let mut visit_order = Vec::new();
        let mut most_constrained: Vec<_> = relations.node_indices().collect();
        most_constrained.sort_unstable_by_key(|&nx| relations.neighbors_undirected(nx).count());

        let mut visited = FixedBitSet::with_capacity(relations.node_count());
        let mut queue = std::collections::VecDeque::new();

        while let Some(mc) = most_constrained.pop() {
            queue.push_back(mc);
            while let Some(nx) = queue.pop_front() {
                if visited.contains(nx.index()) {
                    continue;
                }
                visited.put(nx.index());
                visit_order.push(nx);

                let mut neighbors: Vec<_> = relations.neighbors_undirected(nx).collect();
                neighbors.sort_unstable_by_key(|&nx| {
                    -(relations.neighbors_undirected(nx).count() as isize)
                });

                queue.extend(neighbors);
            }
        }

        let mut wordmap = HashMap::new();
        for word in words {
            wordmap
                .entry(word.len())
                .or_insert_with(Vec::new)
                .push(word);
        }
        let wordmap = wordmap
            .into_iter()
            .map(|(k, mut v)| {
                v.sort_unstable_by_key(|w| -(w.frequency() as isize));

                (k, (FixedBitSet::with_capacity(v.len()), v))
            })
            .collect();

        Self {
            words: wordmap,
            relations,
            visit_order,
        }
    }

    /// Find the first `n` solutions
    pub fn find_n(&mut self, n: usize) -> Vec<Relations> {
        self.find_n_impl(n, 0)
    }

    fn find_n_impl(&mut self, n: usize, visit_ind: usize) -> Vec<Relations> {
        if visit_ind >= self.relations.node_count() {
            return vec![self.relations.clone()];
        }
        let nx = self.visit_order[visit_ind];

        let word_len = self.relations[nx].filter_constraint.len();
        let num_constrainted_words = self.words[&word_len].1.len();

        let mut solutions = Vec::new();

        for word_ind in 0..num_constrainted_words {
            if self.words[&word_len].0.contains(word_ind) {
                continue;
            }
            let candidate_word = self.words[&word_len].1[word_ind].clone();

            // TODO: this works to limit a while filled in word, but it can be more efficient
            if !self.relations[nx]
                .filter_constraint
                .matches(&*candidate_word)
            {
                continue;
            }

            self.words.get_mut(&word_len).unwrap().0.put(word_ind);
            self.relations[nx].candidate = Some(candidate_word);

            if self.check_constraint_local(nx) {
                solutions.extend(self.find_n_impl(n - solutions.len(), visit_ind + 1));
            }

            self.words
                .get_mut(&word_len)
                .unwrap()
                .0
                .set(word_ind, false);

            if solutions.len() >= n {
                break;
            }
        }
        self.relations[nx].candidate = None;

        return solutions;
    }

    fn check_constraint_local(&self, nx: NodeIndex) -> bool {
        let edge_iter = self
            .relations
            .edges_directed(nx, Outgoing)
            .chain(self.relations.edges_directed(nx, Incoming));
        for edge_ref in edge_iter {
            let a = &self.relations[edge_ref.source()];
            let b = &self.relations[edge_ref.target()];
            match (&a.candidate, &b.candidate) {
                (None, _) | (_, None) => continue,
                (Some(a), Some(b)) => {
                    let cc = edge_ref.weight();

                    // println!("{}, {} | {}, {}", a.len(), cc.0, b.len(), cc.1);7

                    if a.as_bytes()[cc.0] != b.as_bytes()[cc.1] {
                        return false;
                    }
                }
            }
        }

        return true;
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Serialize, Deserialize)]
pub struct Node {
    /// a potential candidate word
    pub candidate: Option<Word>,

    /// any matching word must fit this filter
    pub filter_constraint: Filter,

    /// position on board where word starts
    pub start_pos: [usize; 2],

    /// either [0, 1] or [1, 0]
    pub dir_vector: [usize; 2],
}

/// Specifies that character at index `self.0` in word 1 must match character at index `self.1` in word 2
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default, Serialize, Deserialize)]
pub struct CharacterConstraint(pub usize, pub usize);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Serialize, Deserialize)]
pub enum BoardTile {
    Empty,
    BlankChar,
    Char(u8),
}

impl BoardTile {
    pub fn is_empty(&self) -> bool {
        matches!(self, BoardTile::Empty)
    }

    pub fn is_blank(&self) -> bool {
        matches!(self, BoardTile::BlankChar)
    }

    pub fn is_char(&self) -> bool {
        matches!(self, BoardTile::Char(_))
    }
}

impl Default for BoardTile {
    fn default() -> Self {
        BoardTile::Empty
    }
}

impl std::convert::From<u8> for BoardTile {
    fn from(c: u8) -> Self {
        if (c as char).is_alphabetic() {
            BoardTile::Char(c)
        } else if c == b'#' {
            BoardTile::BlankChar
        } else {
            BoardTile::Empty
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RawBoard(ndarray::Array2<BoardTile>);

impl RawBoard {
    pub fn new_empty(height: usize, width: usize) -> Self {
        Self(ndarray::Array2::default((height, width)))
    }

    pub fn height(&self) -> usize {
        self.0.raw_dim()[0]
    }

    pub fn width(&self) -> usize {
        self.0.raw_dim()[1]
    }
}

impl std::fmt::Display for RawBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for _ in 0..self.width() + 2 {
            write!(f, "_")?;
        }
        writeln!(f)?;

        for r in 0..self.height() - 1 {
            write!(f, "|")?;
            for c in 0..self.width() {
                write!(
                    f,
                    "{}",
                    match self[[r, c]] {
                        BoardTile::Empty => ' ',
                        BoardTile::BlankChar => '#',
                        BoardTile::Char(c) => c as char,
                    }
                )?;
            }
            writeln!(f, "|")?;
        }

        write!(f, "|")?;
        for c in 0..self.width() {
            write!(
                f,
                "{}",
                match self[[self.height() - 1, c]] {
                    BoardTile::Empty => '_',
                    BoardTile::BlankChar => '#',
                    BoardTile::Char(c) => c as char,
                }
            )?;
        }
        writeln!(f, "|")?;

        Ok(())
    }
}

impl<'a> std::convert::From<&'a str> for RawBoard {
    fn from(text: &'a str) -> Self {
        let lines: Vec<_> = text
            .trim_matches(|c| c == '\n' || c == '\r')
            .lines()
            .map(|l| l.as_bytes())
            .collect();

        debug_assert!(lines.len() > 0);

        for i in 0..lines.len() - 1 {
            debug_assert_eq!(
                lines[i].len(),
                lines[i + 1].len(),
                "Lines {} and {} have different lengths ({} vs {})",
                i,
                i + 1,
                lines[i].len(),
                lines[i + 1].len()
            );
        }

        let width = lines[0].len();
        let height = lines.len();

        let mut arr = ndarray::Array2::default((height, width));

        for r in 0..height {
            for c in 0..width {
                arr[[r, c]] = BoardTile::from(lines[r][c]);
            }
        }

        Self(arr)
    }
}

impl std::ops::Index<[usize; 2]> for RawBoard {
    type Output = BoardTile;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        if index[0] == self.height() || index[1] == self.width() {
            return &BoardTile::Empty;
        }
        &self.0[index]
    }
}

impl std::ops::IndexMut<[usize; 2]> for RawBoard {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl std::ops::Deref for RawBoard {
    type Target = ndarray::Array2<BoardTile>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Relations(DiGraph<Node, CharacterConstraint>);

impl Relations {
    pub fn word_list(&self) -> Vec<Word> {
        self.node_indices()
            .map(|nx| self[nx].candidate.clone().unwrap())
            .collect()
    }
}

impl std::fmt::Display for Relations {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Words used: ",)?;
        for (i, w) in self.word_list().into_iter().enumerate() {
            write!(f, "{}{}", if i > 0 { ", " } else { "" }, w)?;
        }
        writeln!(f)?;
        writeln!(f, "{}", Into::<RawBoard>::into(self.clone()))
    }
}

impl std::ops::Deref for Relations {
    type Target = DiGraph<Node, CharacterConstraint>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Relations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::convert::Into<RawBoard> for Relations {
    fn into(self) -> RawBoard {
        let [height, width] = self
            .0
            .node_indices()
            .map(|nx| {
                let node = &self.0[nx];
                let len = node.filter_constraint.len() - 1;
                [
                    node.start_pos[0] + len * node.dir_vector[0],
                    node.start_pos[1] + len * node.dir_vector[1],
                ]
            })
            .fold_first(|acc, e| [acc[0].max(e[0]), acc[1].max(e[1])])
            .unwrap();

        let mut board = RawBoard::new_empty(height + 1, width + 1);
        for nx in self.0.node_indices() {
            let node = &self.0[nx];
            let len = node.filter_constraint.len();
            for i in 0..len {
                let [r, c] = [
                    node.start_pos[0] + i * node.dir_vector[0],
                    node.start_pos[1] + i * node.dir_vector[1],
                ];

                board[[r, c]] = BoardTile::Char(node.candidate.as_ref().unwrap().as_bytes()[i]);
            }
        }

        return board;
    }
}

impl std::convert::From<&RawBoard> for Relations {
    fn from(board: &RawBoard) -> Self {
        #[derive(Debug)]
        struct Run {
            start_pos: [usize; 2],
            dir_vector: [usize; 2],
            filter: Option<Filter>,
        }
        fn id(x: [usize; 2]) -> [usize; 2] {
            x
        }
        fn transpose(x: [usize; 2]) -> [usize; 2] {
            [x[1], x[0]]
        }
        fn axis_diff(p1: [usize; 2], p2: [usize; 2]) -> usize {
            let p1 = [p1[0] as isize, p1[1] as _];
            let p2 = [p2[0] as isize, p2[1] as _];
            let dp = [p1[0] - p2[0], p1[1] - p2[1]];
            return dp[0].abs().max(dp[1].abs()) as _;
        }

        let mut runs = Vec::new();
        let mut run_map = ndarray::Array2::<Vec<usize>>::default(board.raw_dim());

        // track both horizontal and vertical runs by taking a transposed view of the matrix in the 2nd run
        for f in &[id, transpose] {
            let [height, width] = f([board.height(), board.width()]);

            for r in 0..height {
                let mut current_run_id = None;
                for c in 0..width {
                    if !board[f([r, c])].is_empty() && current_run_id.is_none() {
                        let run_id = runs.len();
                        runs.push(Run {
                            start_pos: f([r, c]),
                            dir_vector: f([0, 1]),
                            filter: None,
                        });
                        current_run_id = Some(run_id);
                    }

                    if let Some(run_id) = current_run_id {
                        run_map[f([r, c])].push(run_id);
                    }

                    if board[f([r, c + 1])].is_empty() && current_run_id.is_some() {
                        let current_run = &mut runs[current_run_id.unwrap()];

                        let run_len = axis_diff(f([r, c]), current_run.start_pos) + 1;
                        // println!("Run: [{}, {}] -> [{}, {}] (len {})", r, c, current_run.start_pos[0], current_run.start_pos[1], run_len+1);

                        let raw_chars: String = (0..run_len)
                            .map(|i| {
                                [
                                    current_run.start_pos[0] + current_run.dir_vector[0] * i,
                                    current_run.start_pos[1] + current_run.dir_vector[1] * i,
                                ]
                            })
                            .map(|[r, c]| {
                                if let BoardTile::Char(c) = board[[r, c]] {
                                    c as char
                                } else {
                                    '-'
                                }
                            })
                            .collect();

                        current_run.filter = Some(Filter::new(raw_chars));
                        current_run_id = None;
                    }
                }
            }
        }

        let mut relations = DiGraph::default();

        // add runs to graph as nodes
        // this adds length 1 runs, which will be pruned later
        for run in runs {
            relations.add_node(Node {
                candidate: None,
                filter_constraint: run.filter.unwrap(),
                start_pos: run.start_pos,
                dir_vector: run.dir_vector,
            });
        }
        // add edges between nodes (runs) which represent matching character constraints
        for r in 0..board.height() {
            for c in 0..board.width() {
                if run_map[[r, c]].len() > 1 {
                    let a = NodeIndex::new(run_map[[r, c]][0]);
                    let b = NodeIndex::new(run_map[[r, c]][1]);

                    let a_index = axis_diff(relations[a].start_pos, [r, c]);
                    let b_index = axis_diff(relations[b].start_pos, [r, c]);

                    debug_assert!(a_index < relations[a].filter_constraint.len());
                    debug_assert!(b_index < relations[b].filter_constraint.len());

                    relations.add_edge(a, b, CharacterConstraint(a_index, b_index));
                }
            }
        }
        // prune graph of length 1 runs
        for nx in relations.node_indices().rev() {
            if relations[nx].filter_constraint.len() <= 1 {
                relations.remove_node(nx);
            }
        }

        relations.shrink_to_fit();

        return Self(relations);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tricky_board_parse() {
        BoardSolver::from_board(
            "sass",
            r"
#####
_____
__###
__#__
__#__
",
        );
    }

    #[test]
    fn test_easy_solve() {
        let start = std::time::Instant::now();
        let solver = BoardSolver::from_board(
            "sassy",
            r"
_#___
_###_
_#___
_#___
",
        );
        println!("Took {:?} to parse the board", start.elapsed());

        let start = std::time::Instant::now();
        let solutions = solver.first_n_solutions(10);
        println!(
            "Took {:?} to suggest first solution: {:?}",
            start.elapsed(),
            solutions
        );

        // assert_eq!(solutions, Some(vec!["sass", "ass"]));
    }

    #[test]
    fn test_from_real() {
        let solver = BoardSolver::from_board(
            "bypass",
            r"
######___
___#_####
####_#__#
_#____#_#
_#_#__#__
_#_####__
_###__#__
",
        );

        let start = std::time::Instant::now();
        let solutions = solver.first_n_solutions(10);
        println!(
            "Took {:?} to suggest first solution: {:?}",
            start.elapsed(),
            solutions
        );
    }
}
