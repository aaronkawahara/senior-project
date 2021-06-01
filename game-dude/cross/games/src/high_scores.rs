const HIGH_SCORES_SAVED: usize = 3;

pub static mut SQUARE_FIELD_HIGH_SCORES: HighScoreTable<u32> =
    HighScoreTable::<u32>::new(SortOrder::Descending);
pub static mut ONLY_LEVEL_HIGH_SCORES: HighScoreTable<u32> =
    HighScoreTable::<u32>::new(SortOrder::Ascending);

pub struct HighScoreTable<S: ScoreConstraints> {
    entries: [Entry<S>; HIGH_SCORES_SAVED],
    sort_by: SortOrder,
}

impl<S: ScoreConstraints> HighScoreTable<S> {
    pub const fn new(sort_by: SortOrder) -> Self {
        HighScoreTable {
            entries: [Entry::<S>::default(); HIGH_SCORES_SAVED],
            sort_by,
        }
    }

    pub fn is_new_high_score(&self, new_score: S) -> bool {
        self.sort_by
            .comes_before(new_score, self.entries.last().unwrap().score)
    }

    pub fn insert_new_high_score(&mut self, entry: Entry<S>) {
        let mut i: usize = 0;

        while self
            .sort_by
            .comes_before(self.entries[i].score(), entry.score())
        {
            i += 1;
        }

        let mut next: Entry<S> = entry;

        for j in i..self.entries.len() {
            let temp: Entry<S> = self.entries[j];
            self.entries[j] = next;
            next = temp;
        }
    }
}

pub enum SortOrder {
    Ascending,
    Descending,
}

impl SortOrder {
    pub fn comes_before<T: core::cmp::Ord + Copy>(&self, val: T, other: T) -> bool {
        match self {
            SortOrder::Ascending => val < other,
            SortOrder::Descending => val > other,
        }
    }

    pub fn comes_after<T: core::cmp::Ord + Copy>(&self, val: T, other: T) -> bool {
        match self {
            SortOrder::Ascending => val > other,
            SortOrder::Descending => val < other,
        }
    }
}

pub trait ScoreConstraints: core::cmp::Ord + ConstDefault + Clone + Copy {}
pub type Initials = [char; 3];

#[derive(Clone, Copy)]
pub struct Entry<S: ScoreConstraints> {
    initials: Initials,
    score: S,
}

impl<S: ScoreConstraints> Entry<S> {
    const fn default() -> Self {
        Entry {
            initials: Initials::DEFAULT,
            score: S::DEFAULT,
        }
    }

    pub fn new(initials: Initials, score: S) -> Self {
        Entry { initials, score }
    }

    fn score(&self) -> S {
        self.score
    }

    fn initials(&self) -> &Initials {
        &self.initials
    }
}

pub trait ConstDefault {
    const DEFAULT: Self;
}

impl ConstDefault for u32 {
    const DEFAULT: Self = 0;
}

impl ConstDefault for Initials {
    const DEFAULT: Self = ['A', 'A', 'A'];
}

impl ScoreConstraints for u32 {}
