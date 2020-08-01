pub mod data_types {

    use std::cmp::Ordering;
    use std::cmp::PartialEq;
    use std::ops::AddAssign;
    use serde::{Serialize, Deserialize};

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum SequenceMatchFunction{
        StrictEqual,
        AlgebraicDistance,
        SequenceLength
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[derive(Copy, Clone, Eq, PartialOrd, PartialEq, Ord, Hash)]
    pub enum DataTypes{
        Integer(usize),
        U8(u8)
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[derive(Copy, Clone, PartialOrd, PartialEq)]
    pub enum Scores{
        SimilarityScores(SimilarityScores),
        SequenceLength(usize)
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[derive(Copy, Clone, PartialOrd, PartialEq)]
    pub enum SimilarityScores{
        Similarity(f32),
        Distance(f32),
        Length(usize),
        IsEqual(bool),
        None
    }

    impl DataTypes{
        pub fn compute_similarity(self, match_function: SequenceMatchFunction, other: Self) -> SimilarityScores {
            // This functions matches a sequence matching function and a
            match match_function {
                SequenceMatchFunction::StrictEqual => SimilarityScores::IsEqual(true),
                SequenceMatchFunction::SequenceLength => SimilarityScores::Length(1),
                SequenceMatchFunction::AlgebraicDistance => {
                    match self {
                        DataTypes::Integer(self_value) => {
                            match other {
                                DataTypes::Integer(other_value) => {
                                    if self_value > other_value{
                                        SimilarityScores::Distance((self_value - other_value) as f32)
                                    }else{
                                        SimilarityScores::Distance((other_value - self_value) as f32)
                                    }
                                }
                                _ => panic!("Cannot compare {:?} with {:?}", self, other)
                            }
                        },
                        _ => panic!("Cannot compare {:?} with {:?}", self, other)
                    }
                }
            }
        }
    }

    impl Eq for SimilarityScores {}

    impl Ord for SimilarityScores {
        // This function differentiates Cost metrics from Reward metrics:
        // Example:
        //      The Distance being a cost function, the ordering will be reversed (further means more cost)
        //      The Similarity being a Reward function, the ordering will be preserved
        fn cmp(&self, other: &Self) -> Ordering {
            match self {
                SimilarityScores::Similarity(self_value) => {
                    match other {
                        SimilarityScores::Similarity(other_value) => {
                            if self_value == other_value { Ordering::Equal }
                            else{
                                if self_value > other_value{ Ordering::Greater }
                                else{ Ordering::Less }
                            }
                        }
                        _ => panic!("Cannot compare {:?} with {:?}", self, other)
                    }
                },
                SimilarityScores::Length(self_value) => {
                    match other {
                        SimilarityScores::Length(other_value) => {
                            if self_value == other_value { Ordering::Equal }
                            else{
                                if self_value > other_value{ Ordering::Greater }
                                else{ Ordering::Less }
                            }
                        }
                        _ => panic!("Cannot compare {:?} with {:?}", self, other)
                    }
                },
                SimilarityScores::Distance(self_value) => {
                    match other {
                        SimilarityScores::Distance(other_value) => {
                            if self_value == other_value { Ordering::Equal }
                            else{
                                if self_value > other_value{ Ordering::Less }
                                else{ Ordering::Greater }
                            }
                        }
                        _ => panic!("Cannot compare {:?} with {:?}", self, other)
                    }
                },
                _ => panic!("Cannot compare {:?} with {:?}", self, other)
            }
        }
    }

    impl SimilarityScores{
        pub fn get_zero(self: Self) -> SimilarityScores {
            match self {
                SimilarityScores::Similarity(_) => SimilarityScores::Similarity(0.0),
                SimilarityScores::Distance(_) => SimilarityScores::Distance(0.0),
                SimilarityScores::Length(_) => SimilarityScores::Length(0),
                SimilarityScores::IsEqual(_) => SimilarityScores::IsEqual(true),
                _ => panic!("No zero implemented for {:?}", self)
            }
        }
    }

    impl std::ops::Add for SimilarityScores {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            match self {
                SimilarityScores::Similarity(x) => {
                    match other {
                        SimilarityScores::Similarity(y) => {
                            SimilarityScores::Similarity(x + y)
                        },
                        _ => panic!("No addition function implemented between {:?} and {:?}", self, other)
                    }
                },
                SimilarityScores::Distance(x) => {
                    match other {
                        SimilarityScores::Distance(y) => {
                            SimilarityScores::Distance(x + y)
                        },
                        _ => panic!("No addition function implemented between {:?} and {:?}", self, other)
                    }
                },
                SimilarityScores::Length(x) => {
                    match other {
                        SimilarityScores::Length(y) => {
                            SimilarityScores::Length(x + y)
                        },
                        _ => panic!("No addition function implemented between {:?} and {:?}", self, other)
                    }
                },
                SimilarityScores::IsEqual(x) => {
                    match other {
                        SimilarityScores::IsEqual(y) => {
                            SimilarityScores::IsEqual(x && y)
                        },
                        _ => panic!("No addition function implemented between {:?} and {:?}", self, other)
                    }
                },
                _ => panic!("No addition function implemented between {:?} and {:?}", self, other)
            }
        }
    }

    impl AddAssign for SimilarityScores {
        fn add_assign(&mut self, other: Self) {
            *self = *self + other;
        }
    }

    impl std::iter::Sum<SimilarityScores> for SimilarityScores {
        fn sum<I>(iter: I) -> Self
        where
            I: Iterator<Item = SimilarityScores>,
        {
            let mut sum_score: SimilarityScores = SimilarityScores::None;
            iter.for_each(|x| {
                match x {
                    SimilarityScores::Similarity(_) => {
                        if sum_score == SimilarityScores::None { sum_score = SimilarityScores::Similarity(0.0) }
                    },
                    _ => ()
                }
                sum_score = sum_score + x
            });
            sum_score
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[derive(Clone, PartialOrd, PartialEq)]
    pub enum SequenceAttributes{
        ClassStr(String)
    }
}