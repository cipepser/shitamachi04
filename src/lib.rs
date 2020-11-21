// #![feature(test)]
// extern crate test;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test::Bencher;
//
//     #[bench]
//     fn bench_max(b: &mut Bencher) {
//         b.iter(|| {
//             let mut input = std::iter::repeat(0)
//                 .take(10000)
//                 .collect::<Vec<i32>>();
//             input.push(10);
//             max(input);
//         });
//     }
//     // #[test]
//     // fn test() {
//     //     let input = vec![0, 3, 1, 5, 10, 2, 4, 1, 3, 5, 8];
//     //     let actual = max(input);
//     //     let expect = 10;
//     //     assert!(actual.is_some());
//     //     assert_eq!(actual.unwrap(), expect);
//     // }
// }
// // pub fn add_two(a: i32) -> i32 {
// //     a + 2
// // }

pub fn max<T: PartialOrd + Default>(input: Vec<T>)
                                    -> Option<T> {
    if input.len() == 0 {
        return None;
    }
    let ret = input.into_iter()
        .fold(T::default(), |max, v| {
            if max < v {
                v
            } else {
                max
            }
        });
    Some(ret)
}


