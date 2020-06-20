use std::hash::Hash;
use std::collections::HashMap;

use crate::graph::Error;
use super::Step;

/// Returns an adjacency map given the result of a traversal represented
/// as an iterable sequence of Steps.
pub fn to_adjacency<N, I> (
    traversal: I
) -> Result<HashMap<N, Vec<N>>, Error>
where N: Hash+Eq+Clone, I: IntoIterator<Item=Step<N>> {
    let mut result = HashMap::new();

    for step in traversal {
        if result.is_empty() {
            result.insert(step.source.clone(), vec![ step.target.clone() ]);
            result.insert(step.target, vec![ step.source ]);
        } else {
            match result.get_mut(&step.source) {
                Some(targets) => {
                    targets.push(step.target.clone());
                },
                None => return Err(Error::UnknownNode)
            }

            if step.cut {
                match result.get_mut(&step.target) {
                    Some(targets) => {
                        targets.push(step.source);
                    },
                    None => return Err(Error::UnknownNode)
                }
            } else {
                result.insert(step.target, vec![ step.source.clone() ]);
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! map(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
         };
    );

    #[test]
    fn empty() {
        let steps: Vec<Step<()>> = vec![ ];
        let result = to_adjacency(steps).unwrap();

        assert_eq!(result.is_empty(), true);
    }

    #[test]
    fn p2() {
        let edges = vec![
            Step::new(0, 1, false)
        ];
        let result = to_adjacency(edges).unwrap();

        assert_eq!(result, map!{
            0 => vec![ 1 ],
            1 => vec![ 0 ]
        });
    }

    #[test]
    fn p3() {
        let edges = vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false)
        ];
        let result = to_adjacency(edges).unwrap();

        assert_eq!(result, map!{
            0 => vec![ 1 ],
            1 => vec![ 0, 2 ],
            2 => vec![ 1 ]
        });
    }

    #[test]
    fn c3() {
        let edges = vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(2, 0, true)
        ];
        let result = to_adjacency(edges).unwrap();

        assert_eq!(result, map!{
            0 => vec![ 1, 2 ],
            1 => vec![ 0, 2 ],
            2 => vec![ 1, 0 ]
        });
    }

    #[test]
    fn s3() {
        let edges = vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(1, 3, false)
        ].into_iter();
        let result = to_adjacency(edges).unwrap();

        assert_eq!(result, map!{
            0 => vec![ 1 ],
            1 => vec![ 0, 2, 3 ],
            2 => vec![ 1 ],
            3 => vec![ 1 ]
        });
    }
}