use std::collections::BTreeSet;

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
    fn check_if_empty(&self, len: usize) -> bool {
        len == 0 || self.x_dim == 0 || self.y_dim == 0
    }

    pub fn get_horizontal(&self, len: usize) -> Vec::<Vec<[usize;2]>> {
        let mut res = Vec::<Vec<[usize;2]>>::new();
        if self.check_if_empty(len) {
            return res;
        }

        for jx in 0..self.y_dim {
            let this_v =
                (0..self.x_dim)
                .map(|x| [x, jx])
                .collect::<Vec<[usize;2]>>();
            if this_v.len() >= len {
                let mut these_vs = Vec::<Vec<[usize;2]>>::new();
                for ind in 0..(this_v.len()-(len - 1)) {
                    these_vs.push((&this_v[ind..ind+len]).to_vec())
                }
                res.append(&mut these_vs)
            }
        }
        res
    }

    // fn get_quadri_coords(&mut self) -> () {
    //     // 0 indexed for both x and y
    //     let mut quadri_coords = Vec::<Vec<[usize;2]>>::new();
    //     let goal_len : usize  = 4;
    //     //vertical
    //     for ix in 0..X_DIM {
    //         let this_v =
    //             (0..self.ydim)
    //             .map(|y| [ix, y])
    //             .collect::<Vec<[usize;2]>>();
    //         if this_v.len() >= goal_len {
    //             let mut these_vs = Vec::<Vec<[usize;2]>>::new();
    //             for ind in 0..(this_v.len()-(goal_len-1)) {
    //                 these_vs.push((&this_v[ind..ind+goal_len]).to_vec())
    //             }
    //             quadri_coords.append(&mut these_vs);
    //         }
    //     }
    //     //horizontal
    //     for jx in 0..Y_DIM {
    //         let this_v =
    //             (0..X_DIM)
    //             .map(|x| [x, jx])
    //             .collect::<Vec<[usize;2]>>();
    //         if this_v.len() >= goal_len {
    //             let mut these_vs = Vec::<Vec<[usize;2]>>::new();
    //             for ind in 0..(this_v.len()-(goal_len - 1)) {
    //                 these_vs.push((&this_v[ind..ind+goal_len]).to_vec())
    //             }
    //             quadri_coords.append(&mut these_vs);
    //         }
    //     }
    //     //diagonals
    //     //top-line diagonals
    //     for jx in 0..self.ydim {
    //         let this_v_right =
    //             (0..X_DIM)
    //             .filter_map(|k| {
    //                 let x : usize = k;
    //                 let y : usize = jx+k;
    //                 if x < X_DIM && y < Y_DIM { //x >= 0 &&  \y >= 0 per usize
    //                     return Some([x, y as usize])
    //                 } else {
    //                     return None
    //                 }
    //             })
    //             .collect::<Vec<[usize;2]>>();
    //         let this_v_left =
    //             (0..X_DIM)
    //             .filter_map(|k| {
    //                 let x : usize = k;
    //                 let y : i32 = jx as i32 - k as i32;
    //                 if x < X_DIM && y >= 0 && y < Y_DIM as i32 { //x >= 0 per usize
    //                     return Some([x, y as usize])
    //                 } else {
    //                     return None
    //                 }
    //             })
    //             .collect::<Vec<[usize;2]>>();
    //         if this_v_left.len() >= goal_len {
    //             let mut these_vs = Vec::<Vec<[usize;2]>>::new();
    //             for ind in 0..(this_v_left.len()-(goal_len-1)) {
    //                 these_vs.push((&this_v_left[ind..ind+goal_len]).to_vec())
    //             }
    //             quadri_coords.append(&mut these_vs);
    //         }
    //         if this_v_right.len() >= goal_len {
    //             let mut these_vs = Vec::<Vec<[usize;2]>>::new();
    //             for ind in 0..(this_v_right.len()-(goal_len-1)) {
    //                 these_vs.push((&this_v_right[ind..ind+goal_len]).to_vec())
    //             }
    //             quadri_coords.append(&mut these_vs);
    //         }
    //     }
    //     // Left side diagonals
    //     // Right side diagnoals
    //     for ix in 0..X_DIM {
    //         let this_v_right =
    //             (1..self.ydim)  //already handled x ==0
    //             .filter_map(|k| {
    //                 let x : usize = ix+k;
    //                 let y : usize = 0+k;
    //                 if x < X_DIM && y < Y_DIM { //x >= 0 && y >= 0 per usize
    //                     return Some([ix+k, 0+k])
    //                 } else {
    //                     return None
    //                 }
    //             })
    //             .collect::<Vec<[usize;2]>>();
    //         let this_v_left =
    //             (1..self.ydim)
    //             .filter_map(|k| {
    //                 let x : usize= ix + k;
    //                 let y : i32 = self.ydim as i32 - 1 - k as i32;
    //                 if x < X_DIM && y >= 0 && y < Y_DIM as i32 {  //x >= 0 per usize
    //                     return Some([x, y as usize])
    //                 } else {
    //                     return None
    //                 }
    //             })
    //             .collect::<Vec<[usize;2]>>();
    //         if this_v_left.len() >= goal_len {
    //             let mut these_vs = Vec::<Vec<[usize;2]>>::new();
    //             for ind in 0..(this_v_left.len()-(goal_len-1)) {
    //                 these_vs.push((&this_v_left[ind..ind+goal_len]).to_vec())
    //             }
    //             quadri_coords.append(&mut these_vs);
    //         }
    //         if this_v_right.len() >= goal_len {
    //             let mut these_vs = Vec::<Vec<[usize;2]>>::new();
    //             for ind in 0..(this_v_right.len()-(goal_len-1)) {
    //                 these_vs.push((&this_v_right[ind..ind+goal_len]).to_vec())
    //             }
    //             quadri_coords.append(&mut these_vs);
    //         }
    //     }

    //     //squares
    //     let max_square_distance = if X_DIM < Y_DIM { X_DIM - 1 } else {Y_DIM - 1};
    //     for ix in 0..X_DIM {
    //         for jx in 0..Y_DIM {
    //             let mut this_squares =
    //                 (1..max_square_distance+1)
    //                 .filter_map(|d| {
    //                     if ix+d >= X_DIM || jx+d >= Y_DIM {
    //                         return None
    //                     }
    //                     let this_square_a = [
    //                         [ix,jx],
    //                         [ix+d, jx],
    //                         [ix, jx + d],
    //                         [ix+d, jx + d]
    //                     ];
    //                     let this_square = this_square_a.to_vec();
    //                     Some(this_square)
    //                 })
    //                 .collect::<Vec<Vec<[usize;2]>>>();
    //                 quadri_coords.append(&mut this_squares);
    //         }
    //     }
    //     self.quadri_coords = quadri_coords;
    // }

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
    fn horizontal() {
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
    fn horizontal_short() {
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
    fn horizontal_empty() {
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
    fn horizontal_long() {
        let expected:  Vec<Vec<[usize; 2]>> = Vec::new();

        let q =  QuadriEnumerator::new(3,3);
        let horiz = q.get_horizontal(4);
        assert_eq!(vec_as_hs(horiz), vec_as_hs(expected));
    }

}