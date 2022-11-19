use std::collections::BTreeSet;
use std::cmp::min;

#[derive(Debug)]
pub struct QuadriEnumerator {
    pub x_dim: usize,
    pub y_dim: usize
}

impl QuadriEnumerator {

    pub fn new(x_dim: usize, y_dim: usize) -> QuadriEnumerator {
        QuadriEnumerator {
            x_dim: x_dim,
            y_dim: y_dim
        }
    }
    fn transpose_coordinates(coords: &mut Vec::<Vec<[usize;2]>>) -> () {
        for v in coords {
            for c in v {
                c.reverse()
            }
        }
    }
    fn get_full_horizontal_lines(x_dim: usize, y_dim:usize) -> Vec::<Vec<[usize;2]>> {
        let mut res = Vec::<Vec<[usize;2]>>::new();
        if x_dim == 0 || y_dim == 0 {
            return res;
        }
        for jx in 0..y_dim {
            let this_v =
                (0..x_dim)
                .map(|x| [x, jx])
                .collect::<Vec<[usize;2]>>();
            res.push(this_v);
        }
        res
    }
    fn get_full_right_diagonals(x_dim: usize, y_dim: usize) -> Vec::<Vec<[usize;2]>> {
        let mut res = Vec::<Vec<[usize;2]>>::new();
        if x_dim == 0 || y_dim == 0 {
            return res;
        }

        // left side coords
        let mut starting_coords =             (0..y_dim)
            .map(|y| [0, y])
            .collect::<Vec<[usize;2]>>();

        //top coords without origin
        starting_coords.append(
            &mut (1..x_dim)
            .map(|x| [x, 0])
            .collect::<Vec<[usize;2]>>()
        );

        for c in starting_coords {
            let num_steps_available = min(x_dim - c[0], y_dim - c[1]);
            let this_v =
                (0..num_steps_available)
                .map(|s| [c[0]+ s, c[1] + s])
                .collect::<Vec<[usize;2]>>();
            res.push(this_v);
        }
        res
    }
    fn get_full_left_diagonals(x_dim: usize, y_dim: usize) -> Vec::<Vec<[usize;2]>> {
        let mut res = Vec::<Vec<[usize;2]>>::new();
        if x_dim == 0 || y_dim == 0 {
            return res;
        }

        // right side coords
        let mut starting_coords = (0..y_dim)
            .map(|y| [x_dim - 1 , y])
            .collect::<Vec<[usize;2]>>();

        //top coords without top-right most
        starting_coords.append(
            &mut (0..x_dim-1)
            .map(|x| [x, 0])
            .collect::<Vec<[usize;2]>>()
        );

        for c in starting_coords {
            let num_steps_available = min(c[0] + 1, y_dim - c[1]);
            let this_v =
                (0..num_steps_available)
                .map(|s| [c[0]- s, c[1] + s])
                .collect::<Vec<[usize;2]>>();
            res.push(this_v);
        }
        res
    }

    fn sliding_windows_of_len<T: std::clone::Clone>(coord_sets: Vec::<Vec<T>>, len: usize) -> Vec::<Vec<T>> {
        let mut res = Vec::<Vec<T>>::new();
        if len == 0 {
            return res;
        }
        for v in coord_sets {
            if v.len() >= len {
                let mut these_vs = Vec::<Vec<T>>::new();
                for ind in 0..(v.len()-(len - 1)) {
                    these_vs.push((&v[ind..ind+len]).to_vec())
                }
                res.append(&mut these_vs)
            }
        }
        res
    }
    fn get_horizontal_lines(len: usize,  x_dim: usize, y_dim: usize) -> Vec::<Vec<[usize;2]>> {
        let coord_sets = QuadriEnumerator::get_full_horizontal_lines(x_dim, y_dim);
        QuadriEnumerator::sliding_windows_of_len(coord_sets, len)
    }

    pub fn get_horizontal(&self, len: usize) -> Vec::<Vec<[usize;2]>> {
        QuadriEnumerator::get_horizontal_lines(len, self.x_dim, self.y_dim)
    }
    pub fn get_vertical(&self, len: usize) -> Vec::<Vec<[usize;2]>> {
        // Get lines in transpose
        let mut coords = QuadriEnumerator::get_horizontal_lines(len, self.y_dim, self.x_dim);
        // Flip em back
        QuadriEnumerator::transpose_coordinates(&mut coords);
        coords
    }
    pub fn get_diagnoals(&self, len:usize) -> Vec::<Vec<[usize;2]>> {
        let mut all_diags = QuadriEnumerator::get_full_right_diagonals(self.x_dim, self.y_dim);
        all_diags.append(&mut QuadriEnumerator::get_full_left_diagonals(self.x_dim, self.y_dim));

        QuadriEnumerator::sliding_windows_of_len(all_diags, len)
    }

    pub fn get_square_corners(&self) -> Vec::<Vec<[usize;2]>> {
        let mut res = Vec::<Vec<[usize;2]>>::new();
        if self.x_dim <= 1 || self.y_dim <= 1 {
            return res;
        }

        // iterate through all top left coordinates of squares
        for ix in 0..(self.x_dim - 1) {
            for jx in 0..(self.y_dim - 1) {
                let max_square_distance = min(self.x_dim - ix, self.y_dim - jx);
                let mut these_squares =
                    (1..max_square_distance)
                    .map(|d| {
                        [
                            [ix,jx],
                            [ix+d, jx],
                            [ix, jx + d],
                            [ix+d, jx + d]
                        ].to_vec()
                    })
                    .collect::<Vec<Vec<[usize;2]>>>();
                res.append(&mut these_squares);
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn vec_as_hs(vs: Vec<Vec<[usize;2]>>) -> BTreeSet<BTreeSet<[usize;2]>> {
        let mut res = BTreeSet::<BTreeSet<[usize;2]>>::new();

        for v in vs{
            let this_hash_set: BTreeSet<[usize;2]> = v.into_iter().collect();
            res.insert(this_hash_set);
        }

        res
    }

    #[test]
    fn test_sliding_windows_of_len() {
        let coord_sets = vec![
            vec![[0,0],[1,1],[2,2],[3,3]]
        ];
        let expected = vec![
            vec![[0,0],[1,1],[2,2]],
            vec![[1,1],[2,2],[3,3]]
        ];
        let actual = QuadriEnumerator::sliding_windows_of_len(coord_sets, 3);
        assert_eq!(vec_as_hs(actual), vec_as_hs(expected))
    }

    #[test]
    fn test_sliding_windows_of_len_empty() {
        let coord_sets = vec![
            vec![[0,0],[1,1],[2,2],[3,3]]
        ];
        let expected = vec![
        ];
        let actual = QuadriEnumerator::sliding_windows_of_len(coord_sets, 6);
        assert_eq!(vec_as_hs(actual), vec_as_hs(expected))
    }

    #[test]
    fn test_sliding_windows_of_len_zero_len() {
        let coord_sets = vec![
            vec![[0,0],[1,1],[2,2],[3,3]]
        ];
        let expected = vec![
        ];
        let actual = QuadriEnumerator::sliding_windows_of_len(coord_sets, 0);
        assert_eq!(vec_as_hs(actual), vec_as_hs(expected))
    }

    #[test]
    fn test_transpose_coordinates() {
        let mut c: Vec<Vec<[usize; 2]>> = vec![
            vec![[0,1], [1,1], [2,1]],
            vec![[0,0], [2,0], [1,0]]
        ];
        let expected: Vec<Vec<[usize; 2]>> =  vec![
            vec![[1,0], [1,1], [1,2]],
            vec![[0,0], [0,2], [0,1]]
        ];
        QuadriEnumerator::transpose_coordinates(&mut c);
        assert_eq!(vec_as_hs(c), vec_as_hs(expected))
    }

    #[test]
    fn test_get_full_right_diagonals() {
        let expected = vec![
            vec![[0,1]],
            vec![[0,0], [1,1]],
            vec![[1,0]],
        ];
        let actual = QuadriEnumerator::get_full_right_diagonals(2, 2);
        assert_eq!(vec_as_hs(actual), vec_as_hs(expected))
    }

    #[test]
    fn test_get_full_right_diagonals_empty() {
        let expected = vec![        ];
        let actual = QuadriEnumerator::get_full_right_diagonals(0, 3);
        assert_eq!(vec_as_hs(actual), vec_as_hs(expected))
    }

    #[test]
    fn test_get_full_left_diagonals() {
        let expected = vec![
            vec![[0,0]],
            vec![[0,1],[1,0]],
            vec![[1,1]],
        ];
        let actual = QuadriEnumerator::get_full_left_diagonals(2, 2);
        assert_eq!(vec_as_hs(actual), vec_as_hs(expected))
    }

    #[test]
    fn test_get_full_left_diagonals_empty() {
        let expected = vec![        ];
        let actual = QuadriEnumerator::get_full_left_diagonals(0, 3);
        assert_eq!(vec_as_hs(actual), vec_as_hs(expected))
    }

    #[test]
    fn test_horizontal() {
        let q =  QuadriEnumerator::new(3,3);
        let horiz = q.get_horizontal(3);
        let expected : Vec<Vec<[usize; 2]>> = vec![
            vec![[0,1], [1,1], [2,1]],
            vec![[0,0], [2,0], [1,0]],
            vec![[0,2], [1,2], [2,2]]
        ];
        assert_eq!(vec_as_hs(horiz), vec_as_hs(expected))
    }

    #[test]
    fn test_horizontal_short() {
        let q =  QuadriEnumerator::new(3,3);
        let horiz = q.get_horizontal(2);
        let expected : Vec<Vec<[usize; 2]>> = vec![
            vec![[0,1], [1,1]],
            vec![[1,1], [2,1]],
            vec![[0,0], [1,0]],
            vec![[2,0], [1,0]],
            vec![[0,2], [1,2]],
            vec![[1,2], [2,2]]
        ];
        assert_eq!(vec_as_hs(horiz), vec_as_hs(expected))
    }
    #[test]
    fn test_horizontal_empty() {
        let expected:  Vec<Vec<[usize; 2]>> = Vec::new();

        let q =  QuadriEnumerator::new(3,3);
        let horiz = q.get_horizontal(0);
        assert_eq!(vec_as_hs(horiz), vec_as_hs(expected.clone()));

        let q2 = QuadriEnumerator::new(0, 3);
        let horiz2 = q2.get_horizontal(3);
        assert_eq!(vec_as_hs(horiz2), vec_as_hs(expected.clone()));

        let q3 = QuadriEnumerator::new(3, 0);
        let horiz3 = q3.get_horizontal(3);
        assert_eq!(vec_as_hs(horiz3), vec_as_hs(expected.clone()));
    }
    #[test]
    fn test_horizontal_long() {
        let expected:  Vec<Vec<[usize; 2]>> = Vec::new();

        let q =  QuadriEnumerator::new(3,3);
        let horiz = q.get_horizontal(4);
        assert_eq!(vec_as_hs(horiz), vec_as_hs(expected));
    }

    #[test]
    fn test_horizontal_non_square_1() {
        let q1 =  QuadriEnumerator::new(3,4);
        let horiz1 = q1.get_horizontal(3);
        let expected1 : Vec<Vec<[usize; 2]>> = vec![
            vec![[0,0], [1,0], [2,0]],
            vec![[0,1], [1,1], [2,1]],
            vec![[0,2], [1,2], [2,2]],
            vec![[0,3], [1,3], [2,3]],
        ];
        assert_eq!(vec_as_hs(horiz1), vec_as_hs(expected1));
    }

    #[test]
    fn test_horizontal_non_square_2() {
        let q2 =  QuadriEnumerator::new(4,3);
        let horiz2 = q2.get_horizontal(3);
        let expected2 : Vec<Vec<[usize; 2]>> = vec![
            vec![[0,0], [1,0], [2,0]],
            vec![[1,0], [2,0], [3,0]],
            vec![[0,1], [1,1], [2,1]],
            vec![[1,1], [2,1], [3,1]],
            vec![[0,2], [1,2], [2,2]],
            vec![[1,2], [2,2], [3,2]]
        ];
        assert_eq!(vec_as_hs(horiz2), vec_as_hs(expected2));
    }

    #[test]
    fn test_vertical() {
        let q =  QuadriEnumerator::new(3,3);
        let verts = q.get_vertical(3);
        let expected : Vec<Vec<[usize; 2]>> = vec![
            vec![[0,0], [0,1], [0,2]],
            vec![[1,0], [1,1], [1,2]],
            vec![[2,0], [2,1], [2,2]]
        ];
        assert_eq!(vec_as_hs(verts), vec_as_hs(expected))
    }

    #[test]
    fn test_diagnoals() {
        let q = QuadriEnumerator::new(3,3);
        let diags = q.get_diagnoals(3);
        let expected: Vec<Vec<[usize; 2]>> = vec![
            vec![[0,0], [1,1], [2,2]],
            vec![[0,2], [1,1], [2,0]]
        ];
        assert_eq!(vec_as_hs(diags), vec_as_hs(expected))
    }

    #[test]
    fn test_diagnoals_short() {
        let q = QuadriEnumerator::new(3,3);
        let diags = q.get_diagnoals(2);
        let expected: Vec<Vec<[usize; 2]>> = vec![
            vec![[0,0], [1,1]],
            vec![[1,1], [2,2]],
            vec![[0,2], [1,1]],
            vec![[1,1], [2,0]],
            vec![[1,0], [0,1]],
            vec![[1,2], [2,1]],
            vec![[0,1], [1,2]],
            vec![[1,0], [2,1]]
        ];
        assert_eq!(vec_as_hs(diags), vec_as_hs(expected))
    }

    #[test]
    fn test_diagnoals_empty() {
        let q = QuadriEnumerator::new(3,3);
        let diags = q.get_diagnoals(0);
        let expected: Vec<Vec<[usize; 2]>> = vec![];
        assert_eq!(vec_as_hs(diags), vec_as_hs(expected))
    }

    #[test]
    fn test_diagnoals_rectangle() {
        let q = QuadriEnumerator::new(2,3);
        let diags = q.get_diagnoals(2);
        let expected: Vec<Vec<[usize; 2]>> = vec![
            vec![[0,0], [1,1]],
            vec![[0,2], [1,1]],
            vec![[1,0], [0,1]],
            vec![[0,1], [1,2]],
        ];
        assert_eq!(vec_as_hs(diags), vec_as_hs(expected))
    }

    #[test]
    fn test_square_corners() {
        let q = QuadriEnumerator::new(3,3);
        let actual = q.get_square_corners();
        let expected: Vec<Vec<[usize; 2]>> = vec![
            vec![[0,0], [0,1], [1,0], [1,1]],
            vec![[1,0], [1,1], [2,0], [2,1]],
            vec![[0,1], [0,2], [1,1], [1,2]],
            vec![[1,1], [1,2], [2,1], [2,2]],
            vec![[0,0], [2,0], [0,2], [2,2]],
        ];
        assert_eq!(vec_as_hs(actual), vec_as_hs(expected))
    }

    #[test]
    fn test_square_corners_rec() {
        let q = QuadriEnumerator::new(2,3);
        let actual = q.get_square_corners();
        let expected: Vec<Vec<[usize; 2]>> = vec![
            vec![[0,0], [0,1], [1,0], [1,1]],
            vec![[0,1], [0,2], [1,1], [1,2]]
        ];
        assert_eq!(vec_as_hs(actual), vec_as_hs(expected))
    }

    #[test]
    fn test_square_empty() {
        let q = QuadriEnumerator::new(1,3);
        let actual = q.get_square_corners();
        let expected: Vec<Vec<[usize; 2]>> = vec![];
        assert_eq!(vec_as_hs(actual), vec_as_hs(expected))
    }
}