const HIGH_SCORES_SAVED: usize = 3;

// #[used]
// #[link_section = ".square_field_high_score_table"]
pub static mut SQUARE_FIELD_HIGH_SCORES: HighScoreTable<u32> = 
    HighScoreTable::<u32>::new(SortOrder::Descending);
// pub static mut SQUARE_FIELD_HIGH_SCORES: *mut HighScoreTable<u32> = 0x80ffc00_u32 as *mut HighScoreTable<u32>;

// #[used]
// #[link_section = ".only_level_high_score_table"]
pub static mut ONLY_LEVEL_HIGH_SCORES: HighScoreTable<u32> = 
    HighScoreTable::<u32>::new(SortOrder::Ascending);
// pub static mut ONLY_LEVEL_HIGH_SCORES: *mut HighScoreTable<u32> = 0x80ffc40_u32 as *mut HighScoreTable<u32>;

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

    pub fn is_initialized(&self) -> bool {
        self.entries[0].initials[0].is_ascii_uppercase() &&
        self.entries[0].initials[1].is_ascii_uppercase() &&
        self.entries[0].initials[2].is_ascii_uppercase()
    }

    pub fn initialize(&mut self, sort_by: SortOrder) {
        self.entries = [Entry::<S>::default(); HIGH_SCORES_SAVED];
        self.sort_by = sort_by;
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

    pub fn entries(&self) -> &[Entry<S>; HIGH_SCORES_SAVED] {
        &self.entries
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
pub const NUM_INITIALS: usize = 3;
pub type Initials = [char; NUM_INITIALS];

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

    pub fn score(&self) -> S {
        self.score
    }

    pub fn initials(&self) -> &Initials {
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
